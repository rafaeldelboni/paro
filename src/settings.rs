use crate::nix_helper::{get_hostname, get_user_home};
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
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
}
