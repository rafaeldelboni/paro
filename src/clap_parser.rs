use crate::settings::ParoSettings;
use clap::{App, Arg, ArgAction, ArgMatches, Command};

pub struct ClapParser {
  clap: App<'static>,
}

fn to_vec_string(matches: &ArgMatches, id: &str) -> Vec<String> {
  matches
    .get_many::<String>(id)
    .unwrap_or_default()
    .map(|v| v.to_string())
    .collect()
}

impl ClapParser {
  pub fn new() -> Self {
    let app = Command::new("paro")
      .version("0.0.1")
      .about("Tool for managing dotfiles directories.")
      .after_help(
        "Note: `paro -h` prints a short and concise overview while \
                `paro --help` gives all details.",
      )
      .arg(
        Arg::new("tags")
          .short('t')
          .long("tag")
          .value_name("tag")
          .help("Install dotfiles according to <tag>.")
          .long_help(
            "Do not install files that match <file-pattern>. \
             Tagged files go in a directory named for the tag, \
             prefixed with tag-. Therefore, files under .dotfiles/tag-git \
             are only installed when installing using the git tag. \
             This can be repeated with additional patterns.",
          )
          .takes_value(true)
          .action(ArgAction::Append),
      )
      .arg(
        Arg::new("excludes")
          .short('x')
          .long("exclude")
          .value_name("file-pattern")
          .help("Do not install files that match <file-pattern>.")
          .long_help(
            "Do not install files that match <file-pattern>. \
             This can be repeated with additional patterns.",
          )
          .takes_value(true)
          .action(ArgAction::Append),
      );

    Self { clap: app }
  }

  pub fn into_settings(self, manual_args: Vec<&str>) -> ParoSettings {
    let matches = if manual_args.is_empty() {
      self.clap.get_matches()
    } else {
      self.clap.get_matches_from(manual_args)
    };
    ParoSettings {
      tags: to_vec_string(&matches, "tags"),
      excludes: to_vec_string(&matches, "excludes"),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_to_vec_string() {
    let matches = ClapParser::new().clap.get_matches_from(vec![
      "paro",
      "-x",
      "file.txt",
      "-x",
      "file2.txt",
      "-x",
      "file3.txt",
    ]);
    assert_eq!(
      to_vec_string(&matches, "excludes"),
      ["file.txt", "file2.txt", "file3.txt"]
    );
  }

  #[test]
  fn test_clap_defaults() {
    let settings = ClapParser::new().into_settings(vec!["paro"]);
    assert_eq!(settings.tags, Vec::<String>::new());
    assert_eq!(settings.excludes, Vec::<String>::new());
  }

  #[test]
  fn test_clap_tags() {
    let settings =
      ClapParser::new().into_settings(vec!["paro", "-t", "linux1"]);
    assert_eq!(settings.tags, ["linux1"]);
  }

  #[test]
  fn test_clap_excludes() {
    let settings = ClapParser::new().into_settings(vec![
      "paro",
      "-x",
      "file.txt",
      "-x",
      "file2.txt",
      "-x",
      "file3.txt",
    ]);
    assert_eq!(settings.excludes, ["file.txt", "file2.txt", "file3.txt"]);
  }
}
