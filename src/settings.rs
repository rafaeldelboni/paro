use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct ParoSettings {
  pub excludes: Vec<String>,
}
