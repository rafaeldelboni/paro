use crate::nix_helper::{get_hostname, get_user_home};
use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ParoSettings {
  pub tags: Vec<String>,
  pub excludes: Vec<String>,
  pub includes: Vec<String>,
  pub directories: Vec<String>,
  pub hostname: String,
  pub force: bool,
  pub down: bool,
  pub dry_run: bool,
}

impl ParoSettings {
  pub fn defaults() -> ParoSettings {
    ParoSettings {
      tags: Vec::<String>::new(),
      excludes: Vec::<String>::new(),
      includes: Vec::<String>::new(),
      directories: vec![get_user_home() + "/.dotfiles"],
      hostname: get_hostname(),
      force: false,
      down: false,
      dry_run: false,
    }
  }

  pub fn merge(self, other: ParoSettings) -> Self {
    Self {
      tags: self
        .tags
        .into_iter()
        .chain(other.tags.into_iter())
        .collect(),
      excludes: self
        .excludes
        .into_iter()
        .chain(other.excludes.into_iter())
        .collect(),
      includes: self
        .includes
        .into_iter()
        .chain(other.includes.into_iter())
        .collect(),
      directories: self
        .directories
        .into_iter()
        .chain(other.directories.into_iter())
        .collect(),
      hostname: if other.hostname.is_empty() {
        self.hostname
      } else {
        other.hostname
      },
      force: self.force | other.force,
      down: self.down | other.down,
      dry_run: self.dry_run | other.dry_run,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn to_string_vec(vector: Vec<&str>) -> Vec<String> {
    vector.into_iter().map(|e| e.to_string()).collect()
  }

  #[test]
  fn test_merge() {
    let settings_empty = ParoSettings {
      tags: Vec::<String>::new(),
      excludes: Vec::<String>::new(),
      includes: Vec::<String>::new(),
      directories: Vec::<String>::new(),
      hostname: "".to_string(),
      force: false,
      down: false,
      dry_run: false,
    };
    let settings_1 = ParoSettings {
      tags: to_string_vec(vec!["t1", "t1"]),
      excludes: to_string_vec(vec!["e1", "e1"]),
      includes: to_string_vec(vec!["i1", "i1"]),
      directories: to_string_vec(vec!["d1", "d1"]),
      hostname: "h1".to_string(),
      force: true,
      down: true,
      dry_run: true,
    };
    let settings_2 = ParoSettings {
      tags: to_string_vec(vec!["t2", "t2"]),
      excludes: to_string_vec(vec!["e2", "e2"]),
      includes: to_string_vec(vec!["i2", "i2"]),
      directories: to_string_vec(vec!["d2", "d2"]),
      hostname: "h2".to_string(),
      force: false,
      down: false,
      dry_run: false,
    };

    let merged_settings = settings_empty.merge(settings_1.clone());
    assert_eq!(merged_settings.tags, settings_1.tags);
    assert_eq!(merged_settings.excludes, settings_1.excludes);
    assert_eq!(merged_settings.includes, settings_1.includes);
    assert_eq!(merged_settings.directories, settings_1.directories);
    assert_eq!(merged_settings.hostname, settings_1.hostname);
    assert_eq!(merged_settings.force, settings_1.force);
    assert_eq!(merged_settings.down, settings_1.down);
    assert_eq!(merged_settings.dry_run, settings_1.dry_run);

    let merged2_settings = settings_1.clone().merge(settings_2.clone());
    assert_eq!(
      merged2_settings.tags,
      to_string_vec(vec!["t1", "t1", "t2", "t2"])
    );
    assert_eq!(
      merged2_settings.excludes,
      to_string_vec(vec!["e1", "e1", "e2", "e2"])
    );
    assert_eq!(
      merged2_settings.includes,
      to_string_vec(vec!["i1", "i1", "i2", "i2"])
    );
    assert_eq!(
      merged2_settings.directories,
      to_string_vec(vec!["d1", "d1", "d2", "d2"])
    );
    assert_eq!(merged2_settings.hostname, settings_2.hostname);
    assert_eq!(merged2_settings.force, settings_1.force);
    assert_eq!(merged2_settings.down, settings_1.down);
    assert_eq!(merged2_settings.dry_run, settings_1.dry_run);
  }
}
