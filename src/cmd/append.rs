use std::collections::HashMap;

use anyhow::Context;
use log::{debug, error, info};

use crate::vault::path::{add_data_part_to_vault_path, remove_data_part_from_vault_path};
use crate::vault::read::read_secrets_from_vault_path;
use crate::vault::VaultTool;

pub fn append_secrets_to_vault_path(vault_src_path: &str, vault_dest_path: &str,
                                    vault_tool: &dyn VaultTool) -> anyhow::Result<()> {
    info!("append secrets from vault path '{vault_src_path}' to '{vault_dest_path}'..");

    let vault_src_path = add_data_part_to_vault_path(&vault_src_path);
    let vault_dest_path_for_read = add_data_part_to_vault_path(&vault_dest_path);
    let vault_dest_path = remove_data_part_from_vault_path(&vault_dest_path);

    debug!("paths after post-processing 'data' section:");
    debug!("- src: '{vault_src_path}'");
    debug!("- dest: '{vault_dest_path_for_read}'");

    let mut dest_secrets: HashMap<String, String> = HashMap::new();

    match read_secrets_from_vault_path(&vault_dest_path_for_read) {
        Ok(secrets) => dest_secrets = secrets,
        Err(e) =>
            error!("unable to read secrets from destination path: '{}', it's ok if destination path doesn't exist yet. continue", e)
    }

    let src_secrets = read_secrets_from_vault_path(&vault_src_path)
                                        .context("unable to read secrets from source vault path")?;

    for (key, value) in src_secrets {
        dest_secrets.insert(key, value);
    }

    vault_tool.create_secrets(&vault_dest_path, &dest_secrets)
        .context("unable to put secrets to the destination vault path")?;

    Ok(())
}