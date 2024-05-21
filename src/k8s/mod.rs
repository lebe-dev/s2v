use std::collections::HashMap;

use log::{debug, info, trace};
use serde::Deserialize;

use crate::exec::execute_shell_command;
use crate::LOG_LINE_SEPARATOR;

pub mod manifest;

#[cfg(test)]
pub mod mock;

pub const KUBECTL_EXEC_PATH: &str = "kubectl";

pub trait KubectlTool {
    fn get_secret_names(&self, namespace: &str, mask: &str) -> anyhow::Result<Vec<String>>;
    fn get_secret_manifest(&self, namespace: &str, secret_name: &str) -> anyhow::Result<String>;
}

pub struct KubectlToolImpl;

impl KubectlToolImpl {
    pub fn new() -> Self {
        Self
    }
}

impl KubectlTool for KubectlToolImpl {
    fn get_secret_names(&self, namespace: &str, mask: &str) -> anyhow::Result<Vec<String>> {
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

    fn get_secret_manifest(&self, namespace: &str, secret_name: &str) -> anyhow::Result<String> {
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
}

#[derive(Deserialize)]
pub struct KubernetesSecret {
    pub data: HashMap<String, String>
}