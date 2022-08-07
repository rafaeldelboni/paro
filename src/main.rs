mod clap_parser;
mod config_parser;
mod settings;

use crate::{clap_parser::ClapParser, config_parser::ConfigParser};

fn main() {
  let clap = ClapParser::new().to_settings(vec![]);
  let config = ConfigParser::new(vec!["tests/settings"]).to_settings();

  println!("clap: {:?}\nconfig: {:?}", clap, config);
}
