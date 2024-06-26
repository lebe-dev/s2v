use std::env;
use std::process::exit;

use clap::ArgMatches;

use crate::cli::{DEST_K8S_NAMESPACE_ARG, FILENAME_ARG, IGNORE_BASE64_ERRORS_FLAG, IGNORE_UTF8_ERRORS_FLAG, init_cli_app, init_working_dir, LOG_LEVEL_ARGUMENT, LOG_LEVEL_DEFAULT_VALUE, SECRET_IGNORE_MASK_ARG, SECRET_MASK_ARG, SERVICE_NAME_ARG, SRC_K8S_NAMESPACE_ARG, VAULT_DEST_PATH_ARG, VAULT_SRC_PATH_ARG};
use crate::cmd::append::append_secrets_to_vault_path;
use crate::cmd::copy::copy_secrets_into_vault;
use crate::cmd::manifests::generate_manifest_with_vault_paths;
use crate::cmd::SecretOptions;
use crate::cmd::update::update_vault_paths_based_on_manifest_file;
use crate::k8s::KubectlToolImpl;
use crate::logging::{get_logging_config, LOG_LINE_SEPARATOR};
use crate::vault::VaultToolImpl;

pub mod cli;
pub mod logging;
pub mod cmd;
pub mod k8s;
pub mod exec;
pub mod vault;

fn main() {
    let matches = init_cli_app();
    init_logging(&matches);
    init_working_dir(&matches);

    match matches.subcommand() {
        Some(("copy", matches)) => {
            let namespace = matches.get_one::<String>(SRC_K8S_NAMESPACE_ARG).unwrap();
            let vault_dest_path = matches.get_one::<String>(VAULT_DEST_PATH_ARG).unwrap();
            let secret_mask = matches.get_one::<String>(SECRET_MASK_ARG).unwrap();
            let secret_ignore_mask = matches.get_one::<String>(SECRET_IGNORE_MASK_ARG);
            let ignore_base64_errors = matches.get_flag(IGNORE_BASE64_ERRORS_FLAG);
            let ignore_utf8_errors = matches.get_flag(IGNORE_UTF8_ERRORS_FLAG);

            println!("copy secrets from namespace '{namespace}' to vault '{vault_dest_path}'..");
            println!("- filter secrets by mask: '{secret_mask}'");
            println!("- ignore base64 errors: {ignore_base64_errors}");
            println!("- ignore utf8-related errors: {ignore_utf8_errors}");

            check_required_copy_env_vars();

            let kubectl_tool = KubectlToolImpl::new();
            let vault_tool = VaultToolImpl::new();

            let secret_options = SecretOptions::new(
                secret_mask,
                secret_ignore_mask,
                ignore_base64_errors,
                ignore_utf8_errors
            );

            match copy_secrets_into_vault(&namespace, &vault_dest_path, &secret_options,
                                          &kubectl_tool, &vault_tool) {
                Ok(()) => println!("success"),
                Err(e) => eprintln!("error: {}", e)
            }
        },
        Some(("gen-manifest", matches)) => {
            let src_k8s_namespace = matches.get_one::<String>(SRC_K8S_NAMESPACE_ARG).unwrap();
            let secret_mask = matches.get_one::<String>(SECRET_MASK_ARG).unwrap();
            let secret_ignore_mask = matches.get_one::<String>(SECRET_IGNORE_MASK_ARG);
            let dest_k8s_namespace = matches.get_one::<String>(DEST_K8S_NAMESPACE_ARG).unwrap();
            let service_name = matches.get_one::<String>(SERVICE_NAME_ARG).unwrap();
            let vault_dest_path = matches.get_one::<String>(VAULT_DEST_PATH_ARG).unwrap();
            let ignore_base64_errors = matches.get_flag(IGNORE_BASE64_ERRORS_FLAG);
            let ignore_utf8_errors = matches.get_flag(IGNORE_UTF8_ERRORS_FLAG);

            println!("generate manifests for secrets from namespace '{src_k8s_namespace}'");
            println!("- filter secrets by mask: '{secret_mask}'");
            println!("- destination k8s namespace: '{dest_k8s_namespace}'");
            println!("- service name mask: '{service_name}'");
            println!("- vault destination path: '{vault_dest_path}'");
            println!("- ignore base64 errors: {ignore_base64_errors}");
            println!("- ignore utf8 errors: {ignore_utf8_errors}");

            check_required_gen_manifests_env_vars();

            let kubectl_tool = KubectlToolImpl::new();

            let secret_options = SecretOptions::new(
                secret_mask,
                secret_ignore_mask,
                ignore_base64_errors,
                ignore_utf8_errors
            );

            match generate_manifest_with_vault_paths(&src_k8s_namespace,  &service_name,
                                     &dest_k8s_namespace, &vault_dest_path, &secret_options,
                                         &kubectl_tool) {
                Ok(()) => println!("success"),
                Err(e) => eprintln!("error: {}", e)
            }
        },
        Some(("append", matches)) => {
            let vault_src_path = matches.get_one::<String>(VAULT_SRC_PATH_ARG).unwrap();
            let vault_dest_path = matches.get_one::<String>(VAULT_DEST_PATH_ARG).unwrap();

            println!("append vault secrets from path '{vault_src_path}' to '{vault_dest_path}'..");

            check_required_append_env_vars();

            let vault_tool = VaultToolImpl::new();

            match append_secrets_to_vault_path(&vault_src_path, &vault_dest_path, &vault_tool) {
                Ok(()) => println!("success"),
                Err(e) => eprintln!("error: {}, {}", e, e.root_cause())
            }
        },
        Some(("update-vault-path", matches)) => {
            let manifest_src_file = matches.get_one::<String>(FILENAME_ARG).unwrap();
            let vault_dest_path = matches.get_one::<String>(VAULT_DEST_PATH_ARG).unwrap();

            let ignore_base64_errors = matches.get_flag(IGNORE_BASE64_ERRORS_FLAG);
            let ignore_utf8_errors = matches.get_flag(IGNORE_UTF8_ERRORS_FLAG);

            println!("update vault path inside manifest '{manifest_src_file}' to '{vault_dest_path}'..");
            println!("- ignore base64 errors: {ignore_base64_errors}");
            println!("- ignore utf8 errors: {ignore_utf8_errors}");

            match update_vault_paths_based_on_manifest_file(&manifest_src_file, &vault_dest_path,
                                                        ignore_base64_errors, ignore_utf8_errors) {
                Ok(values) => {
                    println!("updated values:");
                    println!("{}", LOG_LINE_SEPARATOR);
                    for (k, v) in values {
                        println!("{k}: {v}")
                    }
                },
                Err(e) => eprintln!("error: {}, {}", e, e.root_cause())
            }
        }
        _ => println!("use -h to get help")
    }
}

fn init_logging(matches: &ArgMatches) {
    let log_level = match matches.get_one::<String>(LOG_LEVEL_ARGUMENT) {
        Some(value) => {value}
        None => {
            LOG_LEVEL_DEFAULT_VALUE
        }
    };

    let logging_config = get_logging_config(log_level);
    log4rs::init_config(logging_config).expect("logging init error");
}

fn check_required_copy_env_vars() {
    check_required_env_vars(&vec!["KUBECONFIG", "VAULT_TOKEN", "VAULT_ADDR"])
}

fn check_required_gen_manifests_env_vars() {
    check_required_env_vars(&vec!["KUBECONFIG"])
}

fn check_required_append_env_vars() {
    check_required_env_vars(&vec!["VAULT_TOKEN", "VAULT_ADDR"])
}

fn check_required_env_vars(required_vars: &Vec<&str>) {
    for var_name in required_vars {
        if env::var_os(var_name).is_none() {
            eprintln!("error: environment variable '{var_name}' is not defined. exit");
            exit(1)
        }
    }
}