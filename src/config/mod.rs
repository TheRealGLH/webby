pub fn print_help() {
    println!("usage: webby [-p <port>] <base_dir>")
}

pub struct Configuration {
    pub directory: String,
    pub print_help: bool,
    pub port: u16,
}

impl Configuration {
    pub fn build(mut args: impl Iterator<Item = String>) -> Self {
        let mut directory = String::from(".");
        let mut print_help = false;
        let port: u16 = 7676;
        //TODO: parse the args iterator for our potential overrides
        Self {
            directory,
            print_help,
            port,
        }
    }
}
