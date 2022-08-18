mod clap_parser;
mod config_parser;
mod files;
mod nix_helper;
mod types;

use crate::{clap_parser::ClapParser, config_parser::ConfigParser};

fn main() {
  let mut config_files = nix_helper::get_default_config_files();
  config_files.push("tests/settings".to_string()); // just for now while testing
  let config = ConfigParser::new(&config_files).into_settings();
  let clap = ClapParser::new().into_settings(vec![]);
  let merged = config.merge(clap).with_defaults();

  // { TODO move this to a function
  let mut files = files::walk_directories(
    merged.directories.clone(),
    merged.destination.clone(),
  );
  files::remove_files(&mut files, merged.excludes.clone());
  // }

  println!("merged: {:?}", merged);
  println!("files:");
  for file in files {
    println!("{:?} {:?} -> {:?}", file.0.depth(), file.0.path(), file.1);
  }
}
