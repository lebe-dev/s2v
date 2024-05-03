use std::env;
use std::path::Path;

use clap::{Arg, ArgMatches, Command};
use log::debug;

pub const WORKDIR: &str = ".";
pub const WORK_DIR_ARGUMENT: &str = "work-dir";
pub const WORK_DIR_SHORT_ARGUMENT: char = 'd';

pub const LOG_LEVEL_ARGUMENT: &str = "log-level";
pub const LOG_LEVEL_DEFAULT_VALUE: &str = "info";

pub fn init_cli_app(app_version: &str) -> ArgMatches {
    Command::new("s2v")
        .version(app_version)
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
        .get_matches()
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