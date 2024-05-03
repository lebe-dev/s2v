use log::{debug, info, trace};

use crate::k8s::kubectl::execute_kubectl_command;

pub mod kubectl;
pub mod manifest;

pub fn get_secret_names_from_namespace(namespace: &str, mask: &str) -> anyhow::Result<Vec<String>> {
    info!("getting secret names from namespace '{namespace}', filtered by mask '{mask}'..");

    let args = format!("-n {namespace} get secrets --field-selector type=Opaque");

    let cmd_output = execute_kubectl_command(&args)?;

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