use crate::file_actions::FileActions;
use crate::terminal::Stdio;
use crate::{files, terminal};

// TODO integration test
pub fn execute(files: FileActions) {
  let mut stdio = Stdio::new();

  stdio.writeln(format!("settings: {:?}", files.settings));

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

    if files.settings.force {
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
