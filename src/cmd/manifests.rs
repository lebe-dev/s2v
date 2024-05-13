use std::fs;
use std::path::Path;

use log::info;
use tera::{Context as TeraContext, Tera};

use crate::k8s::get_secret_names_from_namespace;
use crate::k8s::manifest::{get_secret_manifest, get_secrets_from_manifest};
use crate::vault::path::add_data_part_to_vault_path;

pub const TEMPLATE_FILENAME: &str = "template.yaml";

pub fn generate_manifest_with_vault_paths(src_k8s_namespace: &str, secret_mask: &str, service_name: &str,
                                          dest_k8s_namespace: &str, vault_dest_path: &str,
                      ignore_base64_errors: bool, ignore_utf8_errors: bool) -> anyhow::Result<()> {
    info!("generate manifests for secrets from namespace '{src_k8s_namespace}' with secret mask '{secret_mask}'");
    info!("- service name: '{service_name}'");
    info!("- vault destination path: '{vault_dest_path}'");
    info!("- ignore base64 errors: '{ignore_base64_errors}'");

    let vault_dest_path = add_data_part_to_vault_path(&vault_dest_path);

    info!("reading k8s secrets by name mask '{secret_mask}'..");

    let mut all_secrets_names: Vec<String> = vec![];

    let secret_names = get_secret_names_from_namespace(&src_k8s_namespace, &secret_mask)?;

    for secret_name in secret_names {
        let manifest = get_secret_manifest(&src_k8s_namespace, &secret_name)?;

        let secrets = get_secrets_from_manifest(&manifest, ignore_base64_errors, ignore_utf8_errors)?;

        for (k, _) in secrets {
            all_secrets_names.push(k)
        }
    }

    all_secrets_names.sort();

    info!("all secret have been read, generating a manifest file at path manifests/{service_name}.yaml");

    let template_file = Path::new(TEMPLATE_FILENAME);

    let mut tera = Tera::default();
    tera.add_template_file(&template_file, None)?;

    let mut context = TeraContext::new();

    info!("encoding (base64) secrets..");

    let mut secrets_block: String = "".to_string();

    for secret_name in all_secrets_names {
        let secret_value = format!("vault:{vault_dest_path}#{secret_name}");

        let encoded_value = base64::encode(&secret_value);

        info!("- {secret_name}: {encoded_value}");

        let secret_row = format!("  {secret_name}: {encoded_value}\n");

        secrets_block.push_str(&secret_row);
    }

    if !secrets_block.is_empty() {
        secrets_block.truncate(secrets_block.len()-1);
    }

    context.insert("secretsBlock", &secrets_block);
    context.insert("namespace", dest_k8s_namespace);
    context.insert("serviceName", service_name);

    let manifest_content = tera.render(TEMPLATE_FILENAME, &context)?;

    let manifests_dir = Path::new("manifests");

    if !manifests_dir.exists() {
        fs::create_dir(&manifests_dir)?;
    }

    let manifest_file = manifests_dir.join(format!("{service_name}.yaml"));

    fs::write(&manifest_file, &manifest_content)?;

    Ok(())
}