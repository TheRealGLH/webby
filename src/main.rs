use std::{env, process::ExitCode};

use webbington::config::Configuration;
use webbington::config::print_help;

fn main() -> ExitCode {
    let config = Configuration::build(env::args()).unwrap_or_else(|e| {
        eprintln!("Error while parsing command arguments: {e:?}");
        print_help();
        std::process::exit(4);
    });
    match webbington::init(config) {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{:?}", e);
            ExitCode::from(5)
        }
    }
}
