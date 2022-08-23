mod files;
mod nix_helper;
mod parsers;
mod settings;

use crate::{
  files::PathActions, parsers::clap::ClapParser, parsers::config::ConfigParser,
};

fn main() {
  let mut config_files = nix_helper::get_default_config_files();
  config_files.push("tests/settings".to_string()); // just for now while testing
  let config = ConfigParser::new(&config_files).into_settings();
  let clap = ClapParser::new().into_settings(vec![]);
  let merged = config.merge(clap).with_defaults();

  // { TODO move this to a function
  let mut files: Vec<PathActions> = Vec::<PathActions>::new();
  files::select_files(&mut files, &merged);
  files::exclude_files(&mut files, &merged);
  files::include_files(&mut files, &merged);
  files::cleanup_special_folders(&mut files, &merged);
  // }

  println!("merged: {:?}", merged);
  println!("files:");
  for file in files {
    println!("{:?} {:?} -> {:?}", file.0.depth(), file.0.path(), file.1);
  }
}
