use std::env;
use std::path::Path;

use clap::{Arg, ArgAction, ArgMatches, Command};
use log::debug;

pub const WORKDIR: &str = ".";
pub const WORK_DIR_ARGUMENT: &str = "work-dir";
pub const WORK_DIR_SHORT_ARGUMENT: char = 'd';

pub const LOG_LEVEL_ARGUMENT: &str = "log-level";
pub const LOG_LEVEL_DEFAULT_VALUE: &str = "info";

pub const COPY_COMMAND: &str = "copy";

pub const GEN_MANIFEST_COMMAND: &str = "gen-manifest";

pub const APPEND_COMMAND: &str = "append";

pub const UPDATE_VAULT_PATH_COMMAND: &str = "update-vault-path";

pub const SRC_K8S_NAMESPACE_ARG: &str = "src-k8s-namespace";
pub const DEST_K8S_NAMESPACE_ARG: &str = "dest-k8s-namespace";

pub const VAULT_SRC_PATH_ARG: &str = "vault-src-path";
pub const VAULT_DEST_PATH_ARG: &str = "vault-dest-path";

pub const SECRET_MASK_ARG: &str = "secret-mask";

pub const SECRET_IGNORE_MASK_ARG: &str = "secret-ignore-mask";

pub const SERVICE_NAME_ARG: &str = "service-name";

pub const FILENAME_ARG: &str = "filename";

/// Use values AS IS if base64 related error occurs
pub const IGNORE_BASE64_ERRORS_FLAG: &str = "ignore-base64-errors";

/// Use values AS IS if utf-8 related error occurs
pub const IGNORE_UTF8_ERRORS_FLAG: &str = "ignore-utf8-errors";

pub fn init_cli_app() -> ArgMatches {
    Command::new("s2v")
        .version("1.2.0")
        .author("Eugene Lebedev <eugene.0x90@gmail.com>")
        .about("Migration tool for K8s vanilla secrets to HashiCorp Vault")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg(
            Arg::new(WORK_DIR_ARGUMENT)
                .short(WORK_DIR_SHORT_ARGUMENT)
                .help("set working directory")
                .long(WORK_DIR_ARGUMENT)
                .required(false)
        )
        .arg(
            Arg::new(LOG_LEVEL_ARGUMENT)
                .help("set logging-level")
                .long(LOG_LEVEL_ARGUMENT)
                .default_value(LOG_LEVEL_DEFAULT_VALUE)
                .required(false)
        )
        .subcommand(
            Command::new(COPY_COMMAND)
                .about("copy secrets from kubernetes to hashicorp vault")
                .arg(get_src_k8s_namespace_arg())
                .arg(get_secret_mask_arg())
                .arg(get_vault_dest_path_arg())
                .arg(get_secret_ignore_mask_arg())
                .arg(get_ignore_base64_errors_flag())
                .arg(get_ignore_utf8_errors_flag())
        )
        .subcommand(
            Command::new(GEN_MANIFEST_COMMAND)
                .about("generate secret manifest file with vault paths for a service. Merge secrets data if mask matches multiple secrets")
                .arg(get_src_k8s_namespace_arg())
                .arg(get_secret_mask_arg())
                .arg(get_service_name_arg())
                .arg(get_dest_k8s_namespace_arg())
                .arg(get_vault_dest_path_arg())
                .arg(get_secret_ignore_mask_arg())
                .arg(get_ignore_base64_errors_flag())
                .arg(get_ignore_utf8_errors_flag())
        )
        .subcommand(
            Command::new(APPEND_COMMAND)
                .about("append secrets from vault source path to destination path")
                .arg(get_vault_src_path_arg())
                .arg(get_vault_dest_path_arg())
        )
        .subcommand(
            Command::new(UPDATE_VAULT_PATH_COMMAND)
                .about("update vault paths inside secret manifest file")
                .arg(get_filename_arg())
                .arg(get_vault_dest_path_arg())
                .arg(get_ignore_base64_errors_flag())
                .arg(get_ignore_utf8_errors_flag())
        )
        .get_matches()
}

fn get_src_k8s_namespace_arg() -> Arg {
    Arg::new(SRC_K8S_NAMESPACE_ARG)
        .help("source kubernetes namespace. Example: demo")
        .required(true)
}

fn get_dest_k8s_namespace_arg() -> Arg {
    Arg::new(DEST_K8S_NAMESPACE_ARG)
        .help("destination kubernetes namespace. Example: demo")
        .required(true)
}

fn get_vault_src_path_arg() -> Arg {
    Arg::new(VAULT_SRC_PATH_ARG)
        .help("vault src path. Example: kv/data/demo/some-service")
        .required(true)
}

fn get_vault_dest_path_arg() -> Arg {
    Arg::new(VAULT_DEST_PATH_ARG)
        .help("vault dest path. Example: kv/demo/some-service")
        .required(true)
}

fn get_secret_mask_arg() -> Arg {
    Arg::new(SECRET_MASK_ARG)
        .help("filter secret names by mask. Example: some-service")
        .required(true)
}

fn get_secret_ignore_mask_arg() -> Arg {
    Arg::new(SECRET_IGNORE_MASK_ARG)
        .help("ignore secrets by mask. Example: some-service")
        .required(false)
}

fn get_service_name_arg() -> Arg {
    Arg::new(SERVICE_NAME_ARG)
        .help("set service name. Service name is will be used in secret manifests. Example: some-service")
        .required(true)
}

fn get_ignore_base64_errors_flag() -> Arg {
    Arg::new(IGNORE_BASE64_ERRORS_FLAG)
        .long(IGNORE_BASE64_ERRORS_FLAG)
        .help("ignore base64 decoding errors. Use secret value AS IS if base64 related error occurs")
        .action(ArgAction::SetTrue)
        .required(false)
}

fn get_ignore_utf8_errors_flag() -> Arg {
    Arg::new(IGNORE_UTF8_ERRORS_FLAG)
        .long(IGNORE_UTF8_ERRORS_FLAG)
        .help("ignore utf-8 related errors. Use secret value AS IS if utf-8 related error occurs")
        .action(ArgAction::SetTrue)
        .required(false)
}

fn get_filename_arg() -> Arg {
    Arg::new(FILENAME_ARG)
        .help("source file")
        .required(true)
}

pub fn init_working_dir(matches: &ArgMatches) {
    let working_directory: &Path = get_argument_path_value(
        &matches, WORK_DIR_ARGUMENT, WORKDIR);

    debug!("working directory '{}'", &working_directory.display());

    env::set_current_dir(&working_directory).expect("couldn't set working directory");
}

fn get_argument_path_value<'a>(matches: &'a ArgMatches, long_argument: &str,
                               default_path: &'a str) -> &'a Path {
    let mut path: &Path = Path::new(default_path);

    match matches.get_one::<String>(long_argument) {
        Some(value) => path = Path::new(value),
        None => {}
    }

    return path;
}