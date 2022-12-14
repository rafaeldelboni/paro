use crate::{files::canonicalize_path, settings::Settings};
use config::{Config, File, FileFormat};

pub struct ConfigParser {
  config: Config,
}

impl ConfigParser {
  pub fn new(files: &Vec<String>) -> Self {
    let mut builder = Config::builder();

    for file in files {
      builder = builder.add_source(
        File::new(&file.to_owned()[..], FileFormat::Toml).required(false),
      );
    }

    Self {
      config: builder
        .set_default("tags", Vec::<String>::new())
        .unwrap()
        .set_default("excludes", Vec::<String>::new())
        .unwrap()
        .set_default("includes", Vec::<String>::new())
        .unwrap()
        .set_default("directories", Vec::<String>::new())
        .unwrap()
        .set_default("destination", String::new())
        .unwrap()
        .set_default("hostname", String::new())
        .unwrap()
        .set_default("force", false)
        .unwrap()
        .set_default("down", false)
        .unwrap()
        .set_default("dry-run", false)
        .unwrap()
        .set_default("verbose", 0)
        .unwrap()
        .build()
        .unwrap(),
    }
  }

  pub fn into_settings(self) -> Settings {
    let settings: Settings = self.config.try_deserialize().unwrap();
    Settings {
      destination: canonicalize_path(settings.destination).unwrap(),
      ..settings
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use regex::Regex;

  fn config_file() -> Vec<String> {
    vec!["tests/settings".to_string()]
  }

  #[test]
  fn test_config_defaults() {
    let settings =
      ConfigParser::new(&vec!["tests/non-exist".to_string()]).into_settings();
    assert_eq!(settings.tags, Vec::<String>::new());
    assert_eq!(settings.excludes, Vec::<String>::new());
    assert_eq!(settings.includes, Vec::<String>::new());
    assert_eq!(settings.directories, Vec::<String>::new());
    assert_eq!(settings.destination, String::new());
    assert_eq!(settings.hostname, String::new());
    assert_eq!(settings.force, false);
    assert_eq!(settings.down, false);
    assert_eq!(settings.dry_run, false);
    assert_eq!(settings.verbose, 0);
  }

  #[test]
  fn test_config_tags() {
    let settings = ConfigParser::new(&config_file()).into_settings();
    assert_eq!(settings.tags, ["linux1", "macos2"]);
  }

  #[test]
  fn test_config_excludes() {
    let settings = ConfigParser::new(&config_file()).into_settings();
    assert_eq!(settings.excludes, ["file.txt", "file2.txt", "file3.txt"]);
  }

  #[test]
  fn test_config_includes() {
    let settings = ConfigParser::new(&config_file()).into_settings();
    assert_eq!(settings.includes, ["file.txt", "file2.txt", "file3.txt"]);
  }

  #[test]
  fn test_config_directories() {
    let settings = ConfigParser::new(&config_file()).into_settings();
    assert_eq!(settings.directories, ["home/", "dome/", "pombe/"]);
  }

  #[test]
  fn test_config_hostname() {
    let settings = ConfigParser::new(&config_file()).into_settings();
    assert_eq!(settings.hostname, "hostname-in-config");
  }

  #[test]
  fn test_config_destination() {
    let settings = ConfigParser::new(&config_file()).into_settings();

    let re = Regex::new(r"/tests$").unwrap();
    assert!(re.is_match(settings.destination.as_str()));
  }

  #[test]
  fn test_config_force() {
    let settings = ConfigParser::new(&config_file()).into_settings();
    assert_eq!(settings.force, true);
  }

  #[test]
  fn test_config_down() {
    let settings = ConfigParser::new(&config_file()).into_settings();
    assert_eq!(settings.down, true);
  }

  #[test]
  fn test_config_dry_run() {
    let settings = ConfigParser::new(&config_file()).into_settings();
    assert_eq!(settings.dry_run, true);
  }

  #[test]
  fn test_config_verbose() {
    let settings = ConfigParser::new(&config_file()).into_settings();
    assert_eq!(settings.verbose, 2);
  }
}
