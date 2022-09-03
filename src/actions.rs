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

// TODO integration test
impl Actions {
  pub fn new(file_actions: FileActions) -> Self {
    Self {
      file_actions,
      stdio: Stdio::new(),
    }
  }

  // TODO consider level and verbosity and dry-run
  fn log(&mut self, _level: Log, message: String) {
    self.stdio.writeln(message);
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

  // TODO consider dry-run
  fn run<F: FnOnce()>(&mut self, cb: F) {
    cb();
  }

  pub fn execute(&mut self) {
    self.trace(format!("settings: {:?}", self.file_actions.settings));

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
