use crate::file_actions::FileActions;
use crate::terminal::Stdio;
use crate::{files, terminal};
use std::fmt;

pub struct Actions {
  pub file_actions: FileActions,
  pub stdio: Stdio,
}

#[derive(Debug, Clone, Copy)]
pub enum Log {
  Warning = 1,
  Info = 2,
  Debug = 3,
  Trace = 4,
}

impl fmt::Display for Log {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Log::Warning => write!(f, "Warning"),
      Log::Info => write!(f, "Info"),
      Log::Debug => write!(f, "Debug"),
      Log::Trace => write!(f, "Trace"),
    }
  }
}

impl Actions {
  pub fn new(file_actions: FileActions) -> Self {
    Self {
      file_actions,
      stdio: Stdio::new(),
    }
  }

  fn log(&mut self, level: Log, message: String) {
    // TODO replace 4 with verbosity settings in clap
    if (level as u16 <= 4) || self.file_actions.settings.dry_run {
      self.stdio.writeln(message);
    }
  }

  fn trace(&mut self, message: String) {
    self.log(Log::Trace, message);
  }
  fn debug(&mut self, message: String) {
    self.log(Log::Debug, message);
  }
  fn info(&mut self, message: String) {
    self.log(Log::Info, message);
  }
  fn warn(&mut self, message: String) {
    self.log(Log::Warning, message);
  }

  fn run<F: FnOnce()>(&mut self, callback: F) {
    if !self.file_actions.settings.dry_run {
      callback();
    }
  }

  pub fn execute(&mut self) {
    self.trace(format!("{:?}", self.file_actions.settings));

    for (key, value) in self.file_actions.actions.clone() {
      if files::is_same_file(&value.path, &key).unwrap() {
        self.debug(format!("keeping current {:?}", key));
        continue;
      }

      if value.path.is_dir() {
        if !key.exists() {
          self.info(format!("mkdir {:?}", key));
          self.run(|| files::create_dir(&key));
        }
        continue;
      }

      if self.file_actions.settings.force {
        self.warn(format!("overwrite {:?} -> {:?}", value.path, key));
        self.run(|| files::overwrite_symlink(&value.path, &key));
        continue;
      }

      if key.exists() {
        match self
          .stdio
          .dialog(format!("Overwrite? {}", &key.to_string_lossy()))
        {
          terminal::Inputs::Exit => {
            self.trace("Exiting".to_string());
            break;
          }
          terminal::Inputs::No => {
            self.debug(format!("keeping current {:?}", key));
            continue;
          }
          terminal::Inputs::Yes => {
            self.warn(format!("deleting existing {:?}", key));
            self.run(|| files::delete_file(&key));
          }
        }
      }

      self.info(format!("linking {:?} -> {:?}", value.path, key));
      self.run(|| files::create_symlink(&value.path, &key));
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{files, parsers::clap::ClapParser};
  use std::{fs, path::PathBuf};
  use walkdir::WalkDir;

  fn test_dir() -> PathBuf {
    PathBuf::from("tests/destination")
  }

  fn list_test_dir() -> Vec<String> {
    let files = WalkDir::new(test_dir()).sort_by_file_name().into_iter();
    to_str_dest_files(files)
  }

  fn delete_test_dir() {
    if let Err(err) = fs::remove_dir_all(test_dir().as_path()) {
      print!("ERROR: {} {:?}\r\n", err, test_dir().as_path())
    }
  }

  fn prepare(args: Vec<&str>) {
    delete_test_dir();
    files::create_dir(test_dir().as_path());
    let settings = ClapParser::new().into_settings(args);
    let files_actions: FileActions = FileActions::new(settings).build();
    Actions::new(files_actions).execute();
  }

  fn to_str_dest_files(files: walkdir::IntoIter) -> Vec<String> {
    let mut str_dest_files: Vec<String> = files
      .map(|v| v.unwrap().path().to_string_lossy().to_string())
      .collect::<Vec<String>>();
    str_dest_files.sort();
    str_dest_files
  }

  #[test]
  fn integration_test() {
    // test with no args should create any file
    prepare(vec!["paro"]);
    assert_eq!(list_test_dir(), vec!["tests/destination"]);

    // basic redirection args should create some files
    prepare(vec![
      "paro",
      "-a",
      "tests/example-dotfiles",
      "-n",
      "tests/destination",
    ]);
    assert_eq!(
      list_test_dir(),
      vec![
        "tests/destination",
        "tests/destination/.folder",
        "tests/destination/.folder/something.txt",
        "tests/destination/.normal-file.txt",
      ]
    );

    // adds ignored file
    prepare(vec![
      "paro",
      "-a",
      "tests/example-dotfiles",
      "-n",
      "tests/destination",
      "-i",
      "tests/example-dotfiles/.ignored-file",
    ]);
    assert_eq!(
      list_test_dir(),
      vec![
        "tests/destination",
        "tests/destination/.folder",
        "tests/destination/.folder/something.txt",
        "tests/destination/.ignored-file",
        "tests/destination/.normal-file.txt",
      ]
    );

    // removes file
    prepare(vec![
      "paro",
      "-a",
      "tests/example-dotfiles",
      "-n",
      "tests/destination",
      "-x",
      "tests/example-dotfiles/normal-file.txt",
    ]);
    assert_eq!(
      list_test_dir(),
      vec![
        "tests/destination",
        "tests/destination/.folder",
        "tests/destination/.folder/something.txt",
      ]
    );

    // using hostname
    prepare(vec![
      "paro",
      "-a",
      "tests/example-dotfiles",
      "-n",
      "tests/destination",
      "-B",
      "dois",
    ]);
    assert_eq!(
      list_test_dir(),
      vec![
        "tests/destination",
        "tests/destination/.file.txt",
        "tests/destination/.folder",
        "tests/destination/.folder/something.txt",
        "tests/destination/.normal-file.txt"
      ]
    );

    // using hostname
    prepare(vec![
      "paro",
      "-a",
      "tests/example-dotfiles",
      "-n",
      "tests/destination",
      "-B",
      "dois",
    ]);
    assert_eq!(
      list_test_dir(),
      vec![
        "tests/destination",
        "tests/destination/.file.txt",
        "tests/destination/.folder",
        "tests/destination/.folder/something.txt",
        "tests/destination/.normal-file.txt"
      ]
    );

    // using tag
    prepare(vec![
      "paro",
      "-a",
      "tests/example-dotfiles",
      "-n",
      "tests/destination",
      "-t",
      "um",
    ]);
    assert_eq!(
      list_test_dir(),
      vec![
        "tests/destination",
        "tests/destination/.file.txt",
        "tests/destination/.file1.txt",
        "tests/destination/.folder",
        "tests/destination/.folder/something.txt",
        "tests/destination/.normal-file.txt"
      ]
    );

    // using dry_run
    prepare(vec![
      "paro",
      "-a",
      "tests/example-dotfiles",
      "-n",
      "tests/destination",
      "-t",
      "um",
      "--dry-run",
    ]);
    assert_eq!(list_test_dir(), vec!["tests/destination"]);

    delete_test_dir()
  }
}
