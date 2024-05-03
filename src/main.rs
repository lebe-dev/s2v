use std::env;
use std::process::exit;

use clap::ArgMatches;

use crate::cli::{IGNORE_BASE64_ERRORS_FLAG, init_cli_app, init_working_dir, K8S_NAMESPACE_ARG, LOG_LEVEL_ARGUMENT, LOG_LEVEL_DEFAULT_VALUE, SECRET_MASK_ARG, VAULT_DEST_PATH_ARG};
use crate::cmd::copy::copy_secrets_into_vault;
use crate::logging::get_logging_config;

pub mod cli;
pub mod logging;
pub mod cmd;
pub mod k8s;
pub mod vault;
pub mod exec;

fn main() {
    let matches = init_cli_app();
    init_logging(&matches);
    init_working_dir(&matches);

    match matches.subcommand() {
        Some(("copy", matches)) => {
            let namespace = matches.get_one::<&str>(K8S_NAMESPACE_ARG).unwrap();
            let vault_dest_path = matches.get_one::<&str>(VAULT_DEST_PATH_ARG).unwrap();
            let secret_mask = matches.get_one::<&str>(SECRET_MASK_ARG).unwrap();
            let ignore_base64_errors = matches.get_flag(IGNORE_BASE64_ERRORS_FLAG);

            println!("copy secrets from namespace '{namespace}' to vault '{vault_dest_path}'..");
            println!("- filter secrets by mask: '{secret_mask}'");
            println!("- ignore base64 errors: {ignore_base64_errors}");

            check_required_env_vars();

            match copy_secrets_into_vault(&namespace, &vault_dest_path, &secret_mask, ignore_base64_errors) {
                Ok(()) => println!("success"),
                Err(e) => eprintln!("error: {}", e)
            }
        }
        _ => unreachable!()
    }
}

fn init_logging(matches: &ArgMatches) {
    let log_level = match matches.get_one::<&str>(LOG_LEVEL_ARGUMENT) {
        Some(value) => {value}
        None => {
            LOG_LEVEL_DEFAULT_VALUE
        }
    };

    let logging_config = get_logging_config(log_level);
    log4rs::init_config(logging_config).unwrap();
}

fn check_required_env_vars() {
    let required_vars = vec!["KUBECONFIG", "VAULT_TOKEN", "VAULT_ADDR"];

    for var_name in required_vars {
        if env::var_os(var_name).is_none() {
            eprintln!("error: environment variable '{var_name}' is not defined. exit");
            exit(1)
        }
    }
}