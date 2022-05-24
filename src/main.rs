use std::io::Read;
use std::fs::File;

fn load_config() -> String {
    let mut f = File::open("example/config.yml").expect("Configuration file is missing!");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Could not load the config file! (Maybe check the permissions?)");
    contents
}

fn main() {
    load_config();
}
