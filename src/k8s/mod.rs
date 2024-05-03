use std::collections::HashMap;

use log::{debug, info, trace};
use serde::Deserialize;

use crate::exec::execute_shell_command;

pub mod manifest;

pub const KUBECTL_EXEC_PATH: &str = "/usr/bin/kubectl";

#[derive(Deserialize)]
pub struct KubernetesSecret {
    pub data: HashMap<String, String>
}

pub fn get_secret_names_from_namespace(namespace: &str, mask: &str) -> anyhow::Result<Vec<String>> {
    info!("getting secret names from namespace '{namespace}', filtered by mask '{mask}'..");

    let args = format!("-n {namespace} get secrets --field-selector type=Opaque");

    let cmd_output = execute_shell_command(KUBECTL_EXEC_PATH, &args)?;

    let rows = cmd_output.split("\n").collect::<Vec<&str>>();

    trace!("------------------------");
    trace!("kubectl output:");
    trace!("{:?}", rows);
    trace!("------------------------");

    let mut secrets: Vec<String> = vec![];

    for row in rows {
        let row_parts = row.split(" ").collect::<Vec<&str>>();

        if row_parts.len() > 0 {
            let secret_name = row_parts.first().unwrap();

            if secret_name.contains(&mask) {
                secrets.push(secret_name.to_string());
                info!("added secret '{secret_name}'");
            }
        }
    }

    debug!("secrets received:");
    debug!("{:?}", secrets);

    Ok(secrets)
}