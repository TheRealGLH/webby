use std::{env, process::ExitCode};

use webby::config::Configuration;

fn main() -> ExitCode {
    let config = Configuration::build(env::args());
    match webby::init(config) {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{:?}", e);
            ExitCode::from(5)
        }
    }
}
