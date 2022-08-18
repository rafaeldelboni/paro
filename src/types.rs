use crate::nix_helper::{get_hostname, get_user_home};
use serde::Deserialize;
use std::path::PathBuf;
use walkdir::DirEntry;

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Settings {
  pub tags: Vec<String>,
  pub excludes: Vec<String>,
  pub includes: Vec<String>,
  pub directories: Vec<String>,
  pub destination: String,
  pub hostname: String,
  pub force: bool,
  pub down: bool,
  pub dry_run: bool,
}

impl Settings {
  pub fn defaults() -> Settings {
    Settings {
      tags: Vec::<String>::new(),
      excludes: Vec::<String>::new(),
      includes: Vec::<String>::new(),
      directories: vec![get_user_home() + "/.dotfiles"],
      destination: get_user_home(),
      hostname: get_hostname(),
      force: false,
      down: false,
      dry_run: false,
    }
  }

  pub fn merge(self, other: Settings) -> Self {
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
      destination: if other.destination.is_empty() {
        self.destination
      } else {
        other.destination
      },
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

#[derive(Clone, Debug)]
pub struct PathBufPair(pub DirEntry, pub PathBuf);

#[cfg(test)]
mod tests {
  use super::*;

  fn to_string_vec(vector: Vec<&str>) -> Vec<String> {
    vector.into_iter().map(|e| e.to_string()).collect()
  }

  #[test]
  fn test_merge() {
    let settings_empty = Settings {
      tags: Vec::<String>::new(),
      excludes: Vec::<String>::new(),
      includes: Vec::<String>::new(),
      directories: Vec::<String>::new(),
      destination: "".to_string(),
      hostname: "".to_string(),
      force: false,
      down: false,
      dry_run: false,
    };
    let settings_1 = Settings {
      tags: to_string_vec(vec!["t1", "t1"]),
      excludes: to_string_vec(vec!["e1", "e1"]),
      includes: to_string_vec(vec!["i1", "i1"]),
      directories: to_string_vec(vec!["d1", "d1"]),
      destination: "dn1".to_string(),
      hostname: "h1".to_string(),
      force: true,
      down: true,
      dry_run: true,
    };
    let settings_2 = Settings {
      tags: to_string_vec(vec!["t2", "t2"]),
      excludes: to_string_vec(vec!["e2", "e2"]),
      includes: to_string_vec(vec!["i2", "i2"]),
      directories: to_string_vec(vec!["d2", "d2"]),
      destination: "dn2".to_string(),
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
    assert_eq!(merged_settings.destination, settings_1.destination);
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
    assert_eq!(merged2_settings.destination, settings_2.destination);
    assert_eq!(merged2_settings.hostname, settings_2.hostname);
    assert_eq!(merged2_settings.force, settings_1.force);
    assert_eq!(merged2_settings.down, settings_1.down);
    assert_eq!(merged2_settings.dry_run, settings_1.dry_run);
  }
}
