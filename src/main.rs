mod clap_parser;
mod config_parser;
mod files;
mod nix_helper;
mod settings;

use crate::{clap_parser::ClapParser, config_parser::ConfigParser};

fn main() {
  let config_files = nix_helper::get_default_config_files();
  let defaults = settings::ParoSettings::defaults();
  let clap = ClapParser::new().into_settings(vec![]);
  let config = ConfigParser::new(&config_files).into_settings();

  println!(
    "config files: {:?}\n\
     defaults: {:?}\n\
     clap: {:?}\n\
     config: {:?}",
    config_files, defaults, clap, config
  );
}
