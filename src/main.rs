mod actions;
mod file_actions;
mod files;
mod nix_helper;
mod parsers;
mod settings;
mod terminal;

use crate::{
  file_actions::FileActions, parsers::clap::ClapParser,
  parsers::config::ConfigParser,
};

fn main() {
  let config_files = nix_helper::get_default_config_files();
  let config = ConfigParser::new(&config_files).into_settings();
  let clap = ClapParser::new().into_settings(vec![]);
  let settings = config.merge(clap).with_defaults();
  let files_actions: FileActions = FileActions::new(settings).build();
  actions::execute(files_actions);
}
