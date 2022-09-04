use crate::nix_helper::{get_hostname, get_user_home};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
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
  pub verbose: u8,
}

impl Settings {
  pub fn with_defaults(self) -> Self {
    Self {
      tags: if self.tags.is_empty() {
        Vec::<String>::new()
      } else {
        self.tags
      },
      excludes: if self.excludes.is_empty() {
        Vec::<String>::new()
      } else {
        self.excludes
      },
      includes: if self.includes.is_empty() {
        Vec::<String>::new()
      } else {
        self.includes
      },
      directories: if self.directories.is_empty() {
        vec![get_user_home() + "/.dotfiles"]
      } else {
        self.directories
      },
      destination: if self.destination.is_empty() {
        get_user_home()
      } else {
        self.destination
      },
      hostname: if self.hostname.is_empty() {
        get_hostname()
      } else {
        self.hostname
      },
      force: self.force,
      down: self.down,
      dry_run: self.dry_run,
      verbose: self.verbose,
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
      force: self.force || other.force,
      down: self.down || other.down,
      dry_run: self.dry_run || other.dry_run,
      verbose: self.verbose | other.verbose,
    }
  }

  pub fn special_folder_vec(&self) -> Vec<String> {
    let mut special_folders = self
      .tags
      .clone()
      .into_iter()
      .map(|t| "tag-".to_string() + &t)
      .collect::<Vec<String>>();

    if !self.hostname.is_empty() {
      special_folders.push("host-".to_string() + &self.hostname);
    }

    special_folders
  }
}

impl Default for Settings {
  fn default() -> Self {
    Settings {
      tags: Vec::<String>::new(),
      excludes: Vec::<String>::new(),
      includes: Vec::<String>::new(),
      directories: Vec::<String>::new(),
      destination: "".to_string(),
      hostname: "".to_string(),
      force: false,
      down: false,
      dry_run: false,
      verbose: 0,
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
  fn test_with_defaults() {
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
      verbose: 2,
    };

    let settings_with_defaults = settings_1.clone().with_defaults();
    assert_eq!(settings_with_defaults.tags, settings_1.tags);
    assert_eq!(settings_with_defaults.excludes, settings_1.excludes);
    assert_eq!(settings_with_defaults.includes, settings_1.includes);
    assert_eq!(settings_with_defaults.directories, settings_1.directories);
    assert_eq!(settings_with_defaults.destination, settings_1.destination);
    assert_eq!(settings_with_defaults.hostname, settings_1.hostname);
    assert_eq!(settings_with_defaults.force, settings_1.force);
    assert_eq!(settings_with_defaults.down, settings_1.down);
    assert_eq!(settings_with_defaults.dry_run, settings_1.dry_run);
    assert_eq!(settings_with_defaults.verbose, settings_1.verbose);
  }

  #[test]
  fn test_merge() {
    let settings_empty: Settings = Default::default();
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
      verbose: 2,
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
      verbose: 0,
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
    assert_eq!(merged_settings.verbose, settings_1.verbose);

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
    assert_eq!(merged2_settings.verbose, settings_1.verbose);
  }
}
