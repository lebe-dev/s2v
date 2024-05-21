use std::collections::HashMap;

use anyhow::{anyhow, Context};
use log::{debug, error, info};

use crate::k8s::KubernetesSecret;
use crate::logging::LOG_LINE_SEPARATOR;

pub fn get_secrets_from_manifest(manifest: &str, ignore_base64_errors: bool,
                             ignore_utf8_errors: bool) -> anyhow::Result<HashMap<String,String>> {
    info!("getting secrets from manifest, ignore base64 errors: {ignore_base64_errors}, ignore utf-8 related errors {ignore_utf8_errors}..");

    debug!("{LOG_LINE_SEPARATOR}");
    debug!("manifest:");
    debug!("{manifest}");
    debug!("{LOG_LINE_SEPARATOR}");

    let secret: KubernetesSecret = serde_yaml::from_str(&manifest)
                                             .context("secret deserialization error")?;

    let mut secrets: HashMap<String, String> = HashMap::new();

    for (key, encoded_value) in secret.data {
        debug!("processing secret key '{key}'");
        match base64::decode(&encoded_value) {
            Ok(decoded) => {
                match String::from_utf8(decoded) {
                    Ok(value) => secrets.insert(key.to_string(), value),
                    Err(e) => {
                        error!("secret '{key}' contains non-utf8 secret value: {}", e);

                        if ignore_utf8_errors {
                            secrets.insert(key.to_string(), encoded_value.to_string())

                        } else {
                            return Err(anyhow!("secret value error"))
                        }
                    }
                };
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
        let secrets = get_secrets_from_manifest(
            &manifest, true, true).unwrap();

        let expected_values = HashMap::from([
            ("DATABASE_URL", "app.db"),
            ("DATABASE_USER", "demo-app-user"),
            ("DATABASE_PASSWORD", "1029j09qelDAm"),
            ("TOKEN", "non-encoded-value"),
            ("NON_UTF8_VALUE", "d6320dcf844b2b13561b47c647d6076c"),
        ]);

        assert_results(&secrets, &expected_values);
    }

    #[test]
    fn return_error_for_invalid_yaml_syntax() {
        assert!(get_secrets_from_manifest("invalid-syntax", true, true).is_err())
    }

    #[test]
    fn return_error_for_invalid_base64_values() {
        let manifest_file = Path::new("test-data").join("secret.yaml");
        let manifest = fs::read_to_string(manifest_file).unwrap();
        assert!(get_secrets_from_manifest(&manifest, false, false).is_err())
    }

    #[test]
    fn return_error_for_invalid_utf8_values() {
        let manifest_file = Path::new("test-data").join("invalid-utf8.yaml");
        let manifest = fs::read_to_string(manifest_file).unwrap();
        assert!(get_secrets_from_manifest(&manifest, false, false).is_err())
    }

    fn assert_results(results: &HashMap<String, String>, expected_results: &HashMap<&str, &str>) {
        for (k, v) in expected_results {
            assert!(results.contains_key(*k));

            let value = results.get(*k).unwrap();

            assert_eq!(value.to_string(), *v);
        }
    }
}