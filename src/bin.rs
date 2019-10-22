#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;

use env_logger;
use std::io::Write;

pub mod cli;
pub mod errors;
pub mod matching;
pub mod qgram;
pub mod verification;

use crate::errors::*;
use crate::matching::ed_join;

fn main() -> Result<()> {
    // See https://docs.rs/env_logger/0.7.0/env_logger/ for details on controlling the log output
    env_logger::builder()
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}] \n {}",
                record.module_path().unwrap(),
                record.args()
            )
        })
        .init();

    // Parsing CLI Argument and get configurations
    let config: cli::Config = cli::parse_config().unwrap_or_else(|err| {
        eprintln!("Error when parsing CLI arguments:\n {}", err);
        std::process::exit(1);
    });

    info!(
        "input file: {:?}, q = {}, tau = {}",
        &config.filepath, config.q, config.tau
    );

    match ed_join(&config.filepath, config.q, config.tau) {
        Ok(()) => Ok(()),
        Err(e) => Err(e),
    }
}
