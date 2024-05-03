use log::{debug, info};

use crate::k8s::kubectl::execute_kubectl_command;
use crate::logging::LOG_LINE_SEPARATOR;

pub fn get_secret_manifest(namespace: &str, secret_name: &str) -> anyhow::Result<String> {
    info!("getting secret manifest '{secret_name}' (namespace '{namespace}')..");

    let args = format!("-n {namespace} get secret {secret_name} -o yaml");

    let output = execute_kubectl_command(&args)?;

    debug!("{LOG_LINE_SEPARATOR}");
    debug!("kubectl output:");
    debug!("{output}");
    debug!("{LOG_LINE_SEPARATOR}");

    Ok(output)
}