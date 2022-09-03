mod file_actions;
mod files;
mod nix_helper;
mod parsers;
mod settings;
mod terminal;

use crate::terminal::Stdio;
use crate::{
  file_actions::FileActions, parsers::clap::ClapParser,
  parsers::config::ConfigParser,
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

  // TODO move this to another file
  // TODO integration test
  let mut stdio = Stdio::new();

  for (key, value) in files.actions {
    if files::is_same_file(&value.path, &key).unwrap() {
      continue;
    }

    if value.path.is_dir() {
      if !key.exists() {
        stdio.writeln(format!("mkdir {:?}", key));
        files::create_dir(&key);
      }
      continue;
    }

    if merged.force {
      stdio.writeln(format!("overwrite {:?} -> {:?}", value.path, key));
      files::overwrite_symlink(&value.path, &key);
      continue;
    }

    if key.exists() {
      match stdio.dialog(format!("Overwrite? {}", &key.to_string_lossy())) {
        terminal::Inputs::Exit => {
          break;
        }
        terminal::Inputs::No => {
          stdio.writeln(format!("keeping current {:?}", key));
          continue;
        }
        terminal::Inputs::Yes => {
          stdio.writeln(format!("deleting existing {:?}", key));
          files::delete_file(&key);
        }
      }
    }

    stdio.writeln(format!("linking {:?} -> {:?}", value.path, key));
    files::create_symlink(&value.path, &key);
  }
}
