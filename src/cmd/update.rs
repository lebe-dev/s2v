use std::collections::HashMap;
use std::fs;
use std::path::Path;

use anyhow::anyhow;
use log::{error, info};

use crate::k8s::manifest::get_secrets_from_manifest;
use crate::vault::path::add_data_part_to_vault_path;

pub fn update_vault_paths_based_on_manifest_file(file_path: &str, new_vault_path: &str,
                                                 ignore_base64_errors: bool, ignore_utf8_errors: bool) -> anyhow::Result<HashMap<String,String>> {
    info!("update vault paths in manifest file '{file_path}' to '{new_vault_path}'");

    let manifest_file = Path::new(file_path);

    if manifest_file.exists() {
        let manifest = fs::read_to_string(manifest_file)?;

        let secrets = get_secrets_from_manifest(&manifest,
                                            ignore_base64_errors, ignore_utf8_errors)?;

        let new_vault_path = add_data_part_to_vault_path(&new_vault_path);

        let mut updated_paths: HashMap<String,String> = HashMap::new();

        for (k, _) in secrets {
            let updated_value = format!("vault:{new_vault_path}#{k}");
            info!("- '{k}': {updated_value}");
            let encoded_value = base64::encode(updated_value);

            updated_paths.insert(k, encoded_value);
        }

        Ok(updated_paths)

    } else {
        error!("file wasn't found '{file_path}'");
        return Err(anyhow!("manifest file doesn't exist"))
    }
}