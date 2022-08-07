mod clap_parser;
mod config_parser;
mod settings;

use crate::{clap_parser::ClapParser, config_parser::ConfigParser};

fn main() {
  let clap = ClapParser::new().into_settings(vec![]);
  let config = ConfigParser::new(vec!["tests/settings"]).into_settings();

  println!("clap: {:?}\nconfig: {:?}", clap, config);
}
