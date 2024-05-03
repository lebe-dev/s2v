use std::process::Command;

use anyhow::anyhow;
use log::{debug, error};

pub const KUBECTL_PATH: &str = "/usr/bin/kubectl";

pub fn execute_kubectl_command(args_row: &str) -> anyhow::Result<String> {
    debug!("executable path '{KUBECTL_PATH}'");
    debug!("args '{}'", args_row);

    let args: Vec<&str> = args_row.split(" ").collect();

    let output = Command::new(KUBECTL_PATH).args(args).output()?;

    if output.status.success() {
        let stdout = format!("{}", String::from_utf8_lossy(&output.stdout));

        debug!("<stdout>");
        debug!("{}", stdout);
        debug!("</stdout>");

        Ok(stdout)

    } else {
        error!("kubectl execution error:");
        let stderr = String::from_utf8_lossy(&output.stderr);

        error!("<stderr>");
        error!("{}", stderr);
        error!("</stderr>");

        Err(anyhow!("kubectl execution error"))
    }
}