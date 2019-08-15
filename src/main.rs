use clap::{App, load_yaml};


fn main() {
    println!("Hello, world!");
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
}
