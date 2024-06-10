use std::collections::HashMap;

use log::info;
use crate::cmd::SecretOptions;

use crate::k8s::KubectlTool;
use crate::k8s::manifest::get_secrets_from_manifest;
use crate::vault::VaultTool;

pub fn copy_secrets_into_vault(k8s_namespace: &str, vault_dest_path: &str,
                                secret_options: &SecretOptions,
               kubectl_tool: &dyn KubectlTool, vault_tool: &dyn VaultTool) -> anyhow::Result<()> {
    info!("copy k8s secrets from namespace '{k8s_namespace}' into vault '{vault_dest_path}'..");

    let secret_names = kubectl_tool.get_secret_names(&k8s_namespace,
                                                     &secret_options.secret_mask,
                                                     secret_options.secret_ignore_mask)?;

    let mut all_secrets: HashMap<String, String> = HashMap::new();

    for secret_name in secret_names {
        let secret_manifest = kubectl_tool.get_secret_manifest(&k8s_namespace, &secret_name)?;

        let secrets = get_secrets_from_manifest(&secret_manifest,
                                                            secret_options.ignore_base64_errors,
                                                            secret_options.ignore_utf8_errors)?;

        for (k, v) in secrets {
            _ = &all_secrets.insert(k, v);
        }
    }

    vault_tool.create_secrets(&vault_dest_path, &all_secrets)?;

    info!("all secrets have been copied to '{vault_dest_path}'");

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::cmd::copy::copy_secrets_into_vault;
    use crate::cmd::SecretOptions;
    use crate::k8s::mock::KubectlToolMockImpl;
    use crate::vault::mock::VaultToolMockImpl;

    #[test]
    fn secrets_merge_test() {
        let kubectl_tool = KubectlToolMockImpl::new(
            vec!["a".to_string(), "b".to_string()],
            false, false, false);

        let expected_secrets: HashMap<String, String> = HashMap::from([
            ("DATABASE_URL".to_string(), "app.db".to_string()),
            ("DATABASE_USER".to_string(), "demo-app-user".to_string()),
            ("REDIS_PASS".to_string(), "g590gj4g59j".to_string()),
        ]);

        let vault_tool = VaultToolMockImpl::new(&expected_secrets);

        let options = SecretOptions::new("whatever", None, true, true);

        assert!(
            copy_secrets_into_vault(
                "whatever", "whatever", &options,
                &kubectl_tool, &vault_tool
            ).is_ok()
        )
    }

    #[test]
    fn return_error_if_unable_to_get_secret_names() {
        let kubectl_tool = KubectlToolMockImpl::new(Vec::new(),
                    true, false,false);

        let options = SecretOptions::new("whatever", None, true, true);

        let vault_tool = VaultToolMockImpl::new_without_expectations();

        assert!(
            copy_secrets_into_vault(
                "whatever", "whatever", &options,
                &kubectl_tool, &vault_tool
            ).is_err()
        )
    }

    #[test]
    fn return_error_if_unable_to_get_secret_a_manifest() {
        let kubectl_tool = KubectlToolMockImpl::new(
            vec!["a".to_string()],
            false, true, true);

        let vault_tool = VaultToolMockImpl::new_without_expectations();

        let options = SecretOptions::new("whatever", None, true, true);

        assert!(
            copy_secrets_into_vault(
                "whatever", "whatever", &options,
                &kubectl_tool, &vault_tool
            ).is_err()
        )
    }

    #[test]
    fn return_error_if_unable_to_get_secret_b_manifest() {
        let kubectl_tool = KubectlToolMockImpl::new(
            vec!["a".to_string(), "b".to_string()],
            false, false, true);

        let vault_tool = VaultToolMockImpl::new_without_expectations();

        let options = SecretOptions::new("whatever", None, true, true);

        assert!(
            copy_secrets_into_vault(
                "whatever", "whatever", &options,
                &kubectl_tool, &vault_tool
            ).is_err()
        )
    }

    #[test]
    fn return_error_if_unable_to_create_secrets() {
        let kubectl_tool = KubectlToolMockImpl::new(
            vec!["a".to_string()],
            false, false, false);

        let vault_tool = VaultToolMockImpl::new_with_error();

        let options = SecretOptions::new("whatever", None, true, true);

        assert!(
            copy_secrets_into_vault(
                "whatever", "whatever", &options,
                &kubectl_tool, &vault_tool
            ).is_err()
        )
    }
}