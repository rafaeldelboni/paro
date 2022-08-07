use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct ParoSettings {
  pub tags: Vec<String>,
  pub excludes: Vec<String>,
  pub includes: Vec<String>,
  pub directories: Vec<String>,
}
