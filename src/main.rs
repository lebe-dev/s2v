use clap::ArgMatches;

use crate::cli::{init_cli_app, init_working_dir, LOG_LEVEL_ARGUMENT, LOG_LEVEL_DEFAULT_VALUE};
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

