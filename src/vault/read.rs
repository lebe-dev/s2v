use std::collections::HashMap;

use log::{info, trace};
use serde::Deserialize;

use crate::exec::execute_shell_command;
use crate::logging::LOG_LINE_SEPARATOR;
use crate::vault::VAULT_BIN_PATH;

pub fn read_secrets_from_vault_path(vault_path: &str) -> anyhow::Result<HashMap<String, String>> {
    info!("reading secrets from vault path '{vault_path}'");

    let args = format!("read {vault_path} -format=json");

    let json_output = execute_shell_command(VAULT_BIN_PATH, &args)?;

    trace!("{LOG_LINE_SEPARATOR}");
    trace!("vault read output:");
    trace!("{json_output}");
    trace!("{LOG_LINE_SEPARATOR}");

    let secrets = get_secrets_from_json_output(&json_output)?;

    trace!("{LOG_LINE_SEPARATOR}");
    trace!("secrets read:");
    trace!("{:?}", secrets);
    trace!("{LOG_LINE_SEPARATOR}");

    Ok(secrets)
}

fn get_secrets_from_json_output(json: &str) -> anyhow::Result<HashMap<String,String>> {
    let vault_read_output = serde_json::from_str::<VaultReadOutput>(&json)?;
    Ok(vault_read_output.data)
}

#[derive(Deserialize)]
struct VaultReadOutput {
    pub data: HashMap<String,String>
}