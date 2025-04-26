const DEFAULT_PORT: u16 = 7676;
pub fn print_help() {
    println!("usage: webbington [-p <port>] [-d <base_dir>]")
}

pub struct Configuration {
    pub directory: String,
    pub help: bool,
    pub port: u16,
}

impl Configuration {
    pub fn build(args: impl Iterator<Item = String>) -> Result<Self, String> {
        let mut path = String::from(".");
        let mut help = false;
        let mut port: u16 = DEFAULT_PORT;
        //TODO: parse the args iterator for our potential overrides
        let mut tmp = args;
        while let Some(argument) = tmp.next() {
            if argument == "-h" || argument == "--help" {
                help = true;
                break;
            }
            match argument.as_str() {
                "-h" | "--help" => {
                    help = true;
                }
                "-p" => {
                    if let Some(next_arg) = tmp.next() {
                        port = str::parse::<u16>(next_arg.as_str()).unwrap_or(DEFAULT_PORT);
                    }
                }
                "-d" => {
                    if let Some(next_arg) = tmp.next() {
                        path = next_arg;
                    }
                }
                _ => {}
            }
        }
        Ok(Self {
            directory: path,
            help,
            port,
        })
    }
}
