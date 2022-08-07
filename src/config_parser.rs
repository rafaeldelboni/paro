use crate::settings::ParoSettings;
use config::{Config, File, FileFormat};

pub struct ConfigParser {
  config: Config,
}

impl ConfigParser {
  pub fn new(files: Vec<&str>) -> Self {
    let mut builder = Config::builder();

    for file in files {
      builder =
        builder.add_source(File::new(file, FileFormat::Toml).required(false));
    }

    Self {
      config: builder
        .set_default("tags", Vec::<String>::new())
        .unwrap()
        .set_default("excludes", Vec::<String>::new())
        .unwrap()
        .build()
        .unwrap(),
    }
  }

  pub fn into_settings(self) -> ParoSettings {
    self.config.try_deserialize().unwrap()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_config_defaults() {
    let settings = ConfigParser::new(vec!["tests/non-exist"]).into_settings();
    assert_eq!(settings.tags, Vec::<String>::new());
    assert_eq!(settings.excludes, Vec::<String>::new());
  }

  #[test]
  fn test_config_tags() {
    let settings = ConfigParser::new(vec!["tests/settings"]).into_settings();
    assert_eq!(settings.tags, ["linux1", "macos2"]);
  }

  #[test]
  fn test_config_excludes() {
    let settings = ConfigParser::new(vec!["tests/settings"]).into_settings();
    assert_eq!(settings.excludes, ["file.txt", "file2.txt", "file3.txt"]);
  }
}
