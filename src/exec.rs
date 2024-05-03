use std::process::Command;

use anyhow::anyhow;
use log::{debug, error};

pub fn execute_shell_command(executable_path: &str, args_row: &str) -> anyhow::Result<String> {
    debug!("executing shell command..");
    debug!("executable path '{executable_path}'");
    debug!("args '{}'", args_row);

    let args: Vec<&str> = args_row.split(" ").collect();

    let output = Command::new(executable_path).args(args).output()?;

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