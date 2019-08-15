use clap::{App, load_yaml};

mod decode;
mod encode;
mod utils;

use decode::decode;
use encode::encode;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let result = if let Some(encode_matches) = matches.subcommand_matches("encode") {
      encode(encode_matches)
    } else if let Some(decode_matches) = matches.subcommand_matches("decode") {
      decode(decode_matches)
    } else {
      panic!("Must provide command: either 'encode' or 'decode'");
    };

    match result {
      Err(e) => println!("Error: {}", e),
      _ => (),
    }
}
