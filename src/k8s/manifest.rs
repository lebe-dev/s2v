use std::collections::HashMap;

use anyhow::{anyhow, Context};
use log::{debug, error, info};

use crate::exec::execute_shell_command;
use crate::k8s::{KUBECTL_EXEC_PATH, KubernetesSecret};
use crate::logging::LOG_LINE_SEPARATOR;

pub fn get_secret_manifest(namespace: &str, secret_name: &str) -> anyhow::Result<String> {
    info!("getting secret manifest '{secret_name}' (namespace '{namespace}')..");

    let args = format!("-n {namespace} get secret {secret_name} -o yaml");

    let output = execute_shell_command(KUBECTL_EXEC_PATH, &args)?;

    debug!("{LOG_LINE_SEPARATOR}");
    debug!("kubectl output:");
    debug!("{output}");
    debug!("{LOG_LINE_SEPARATOR}");

    info!("secret manifest '{secret_name}' (namespace '{namespace}') has been read");

    Ok(output)
}

pub fn get_secrets_from_manifest(manifest: &str, ignore_base64_errors: bool) -> anyhow::Result<HashMap<String,String>> {
    info!("getting secrets from manifest, ignore base64 errors: {ignore_base64_errors}..");

    debug!("{LOG_LINE_SEPARATOR}");
    debug!("manifest:");
    debug!("{manifest}");
    debug!("{LOG_LINE_SEPARATOR}");

    let secret: KubernetesSecret = serde_yaml::from_str(&manifest).context("secret deserialization error")?;

    let mut secrets: HashMap<String, String> = HashMap::new();

    for (key, encoded_value) in secret.data {
        match base64::decode(&encoded_value) {
            Ok(decoded) => {
                let value = String::from_utf8(decoded).unwrap();
                secrets.insert(key.to_string(), value);
            }
            Err(e) => {
                error!("secret value decoding error '{}'", e);
                if ignore_base64_errors {
                    secrets.insert(key.to_string(), encoded_value.to_string());

                } else {
                    return Err(anyhow!("base64 decode error"))
                }
            }
        }
    }

    debug!("{LOG_LINE_SEPARATOR}");
    debug!("secret values:");
    debug!("{:?}", secrets);
    debug!("{LOG_LINE_SEPARATOR}");

    info!("secrets have been extracted from manifest");

    Ok(secrets)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs;
    use std::path::Path;

    use crate::k8s::manifest::get_secrets_from_manifest;

    #[test]
    fn return_secret_values() {
        let manifest_file = Path::new("test-data").join("secret.yaml");
        let manifest = fs::read_to_string(manifest_file).unwrap();
        let secrets = get_secrets_from_manifest(&manifest, true).unwrap();

        let expected_values = HashMap::from([
            ("DATABASE_URL", "app.db"),
            ("DATABASE_USER", "demo-app-user"),
            ("DATABASE_PASSWORD", "1029j09qelDAm"),
            ("TOKEN", "non-encoded-value"),
        ]);

        assert_results(&secrets, &expected_values);
    }

    #[test]
    fn return_error_for_invalid_yaml_syntax() {
        assert!(get_secrets_from_manifest("invalid-syntax", true).is_err())
    }

    #[test]
    fn return_error_for_invalid_base64_values() {
        let manifest_file = Path::new("test-data").join("secret.yaml");
        let manifest = fs::read_to_string(manifest_file).unwrap();
        assert!(get_secrets_from_manifest(&manifest, false).is_err())
    }

    fn assert_results(results: &HashMap<String, String>, expected_results: &HashMap<&str, &str>) {
        for (k, v) in expected_results {
            assert!(results.contains_key(*k));

            let value = results.get(*k).unwrap();

            assert_eq!(value.to_string(), *v);
        }
    }
}