use std::collections::HashMap;

use log::info;

use crate::k8s::KubectlTool;
use crate::k8s::manifest::get_secrets_from_manifest;
use crate::vault::create_secrets_in_vault;

pub fn copy_secrets_into_vault(k8s_namespace: &str, vault_dest_path: &str, secret_mask: &str,
                               ignore_base64_errors: bool, ignore_utf8_errors: bool,
                               kubectl_tool: &dyn KubectlTool) -> anyhow::Result<()> {
    info!("copy k8s secrets from namespace '{k8s_namespace}' into vault '{vault_dest_path}'..");

    let secret_names = kubectl_tool.get_secret_names(&k8s_namespace, &secret_mask)?;

    let mut all_secrets: HashMap<String, String> = HashMap::new();

    for secret_name in secret_names {
        let secret_manifest = kubectl_tool.get_secret_manifest(&k8s_namespace, &secret_name)?;

        let secrets = get_secrets_from_manifest(&secret_manifest,
                                                ignore_base64_errors, ignore_utf8_errors)?;

        for (k, v) in secrets {
            _ = &all_secrets.insert(k, v);
        }
    }

    create_secrets_in_vault(&vault_dest_path, &all_secrets)?;

    info!("all secrets have been copied to '{vault_dest_path}'");

    Ok(())
}