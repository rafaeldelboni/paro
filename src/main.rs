mod file_actions;
mod files;
mod nix_helper;
mod parsers;
mod settings;
mod terminal;

use crate::terminal::{build_stdio, can_i_overwrite};
use crate::{
  file_actions::FileActions, files::is_same_file, parsers::clap::ClapParser,
  parsers::config::ConfigParser,
};
use std::io::Write;

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

  let (mut stdout, mut stdin) = build_stdio();

  for (key, value) in files.actions {
    write!(
      stdout,
      "{:?} {:?} -> {:?}\r\n",
      value.depth, value.path, key
    )
    .unwrap();
    if is_same_file(&value.path, &key).unwrap() {
      // TODO return ENUM of actions overwrite, keep, overwrite all, keep all
      // TODO maybe add into action this type of action and mutate
      let _sure = can_i_overwrite(&mut stdout, &mut stdin);
    }
  }
}
