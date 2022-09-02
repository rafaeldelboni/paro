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
    if value.path.is_dir() && !&key.exists() {
      write!(stdout, "mkdir {:?}\r\n", key).unwrap();
      continue;
    }

    if is_same_file(&value.path, &key).unwrap() {
      match can_i_overwrite(&mut stdout, &mut stdin, &key.to_string_lossy()) {
        terminal::Inputs::Exit => {
          break;
        }
        terminal::Inputs::No => {
          write!(stdout, "keeping current {:?}\r\n", key).unwrap();
          continue;
        }
        terminal::Inputs::Yes => (),
      }
    }

    write!(stdout, "linking {:?} -> {:?}\r\n", value.path, key).unwrap();
  }
}
