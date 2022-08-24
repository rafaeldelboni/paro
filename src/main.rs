mod files;
mod nix_helper;
mod parsers;
mod settings;

use crate::{
  files::FileActions, parsers::clap::ClapParser, parsers::config::ConfigParser,
};

fn main() {
  let mut config_files = nix_helper::get_default_config_files();
  config_files.push("tests/settings".to_string()); // just for now while testing
  let config = ConfigParser::new(&config_files).into_settings();
  let clap = ClapParser::new().into_settings(vec![]);
  let merged = config.merge(clap).with_defaults();

  let mut files: FileActions = FileActions::new(merged.clone());
  files.build();

  println!("merged: {:?}", merged);
  println!("files:");
  for (key, value) in files.actions {
    println!("{:?} {:?} -> {:?}", value.depth, value.path, key);
  }
}
