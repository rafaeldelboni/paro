use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct ParoSettings {
  pub tags: Vec<String>,
  pub excludes: Vec<String>,
}
