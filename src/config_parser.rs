use crate::settings::ParoSettings;
use config::{Config, File, FileFormat};

pub struct ConfigParser {
  config: Config,
}

impl ConfigParser {
  pub fn new(files: Vec<&str>) -> Self {
    let mut builder = Config::builder();

    for file in files {
      println!("{:?}->", file);
      builder =
        builder.add_source(File::new(file, FileFormat::Toml).required(false));
    }

    Self {
      config: builder
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
  fn test_config_excludes() {
    let settings = ConfigParser::new(vec!["tests/settings"]).into_settings();
    assert_eq!(settings.excludes, ["file.txt", "file2.txt", "file3.txt"]);
  }
}
