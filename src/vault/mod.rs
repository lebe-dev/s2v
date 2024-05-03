use std::collections::HashMap;
use std::fs;
use std::path::Path;

use anyhow::Context;
use log::{debug, info, trace};

use crate::exec::execute_shell_command;
use crate::logging::LOG_LINE_SEPARATOR;

const VAULT_BIN_PATH: &str = "/usr/bin/vault";

const VAULT_SECRET_ENCODED_PREFIX: &str = "vault:";

const TEMP_DIR_NAME: &str = "tmp";

pub fn create_secrets_in_vault(vault_path: &str, secrets: &HashMap<String, String>) -> anyhow::Result<()> {
    info!("creating secrets in vault at path '{vault_path}'..");

    trace!("{LOG_LINE_SEPARATOR}");
    trace!("secrets:");
    trace!("{:?}", secrets);
    trace!("{LOG_LINE_SEPARATOR}");

    let mut args_builder: String = format!("kv put {vault_path} ");

    let mut complex_value_file_paths: Vec<String> = vec![];

    for (key, value) in secrets {
        debug!("secret '{key}'");

        if !value.starts_with(VAULT_SECRET_ENCODED_PREFIX) {
            if is_complex_secret_value(&value) {
                let tmp_file_path = write_complex_value_into_file(&key, &value)?;

                complex_value_file_paths.push(tmp_file_path.to_string());

                args_builder.push_str(&format!("{key}=@{tmp_file_path} "))

            } else {
                args_builder.push_str(&format!("{}={} ", key, value))
            }

        } else {
            info!("secret '{key}' contains vault path, skip")
        }
    }

    let output = execute_shell_command(VAULT_BIN_PATH, &args_builder).context("vault add error")?;

    trace!("{LOG_LINE_SEPARATOR}");
    trace!("vault put output:");
    trace!("{output}");
    trace!("{LOG_LINE_SEPARATOR}");

    debug!("cleaning up temporary files..");

    for file_path in complex_value_file_paths {
        let file_path = Path::new(&file_path);

        if file_path.exists() {
            fs::remove_file(&file!())?;
        }
    }

    info!("all secrets have been saved into vault at path '{vault_path}'");

    Ok(())
}

fn is_complex_secret_value(input: &str) -> bool {
    input.contains("\n") || input.contains("\"") ||
    input.contains(",") || input.contains(" ")
}

fn write_complex_value_into_file(key: &str, value: &str) -> anyhow::Result<String> {
    let temp_path = Path::new(TEMP_DIR_NAME);

    if !temp_path.exists() {
        fs::create_dir(&temp_path)?;
    }

    let temp_file = temp_path.join(format!("{key}.tmp"));

    fs::write(&temp_file, &value)?;

    Ok(format!("{}", temp_file.display()))
}

#[cfg(test)]
mod complex_secret_tests {
    use crate::vault::is_complex_secret_value;

    #[test]
    fn return_true_for_complex_secrets() {
        let value = r#"This is a
        raw string
        literal"#;

        assert!(is_complex_secret_value(&value))
    }

    #[test]
    fn return_true_for_simple_secrets() {
        assert!(!is_complex_secret_value("test"));
        assert!(!is_complex_secret_value("true"));
        assert!(!is_complex_secret_value("483947"));
    }
}