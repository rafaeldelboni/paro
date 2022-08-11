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
      )
      .arg(
        Arg::new("includes")
          .short('i')
          .long("include")
          .value_name("file-pattern")
          .help("Install files that match <file-pattern>.")
          .long_help(
            "Install files that match <file-pattern>. \
             Despite being excluded by the -x flag or a setting in the config.
             This can be repeated with additional patterns.",
          )
          .takes_value(true)
          .action(ArgAction::Append),
      )
      .arg(
        Arg::new("directories")
          .short('a')
          .long("add-dir")
          .value_name("folder-pattern")
          .help("Install dotfiles directories from the <folder-pattern>.")
          .long_help(
            "Install dotfiles directories from the <folder-pattern>. \
             This can be repeated with additional patterns.",
          )
          .takes_value(true)
          .action(ArgAction::Append),
      )
      .arg(
        Arg::new("hostname")
          .short('B')
          .long("hostname")
          .value_name("name")
          .help("Override the computer hostname by <name>.")
          .long_help(
            "Override the computer hostname by <name>. \
             Shall return the standard host name for the current machine.",
          )
          .takes_value(true)
          .action(ArgAction::Set),
      )
      .arg(
        Arg::new("force")
          .short('f')
          .long("force")
          .help("Override if the file already exists in your home directory.")
          .long_help(
            "Override if the file already exists in your home directory, \
             does not prompt for how to handle it.",
          )
          .action(ArgAction::SetTrue),
      )
      .arg(
        Arg::new("down")
          .short('d')
          .long("down")
          .help("Remove all the rc files that the paro suite knows about.")
          .long_help(
            "Remove all the rc files that the paro suite knows about, \
             This can be further controlled with the -t, -B and -a flags.",
          )
          .action(ArgAction::SetTrue),
      )
      .arg(
        Arg::new("dry-run")
          .short('D')
          .long("dry-run")
          .help("Shows what paro would do without causing the effects.")
          .long_help(
            "Shows what paro would do without causing the effects. \
             A simulated or practice performance; rehearsal.",
          )
          .action(ArgAction::SetTrue),
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
      includes: to_vec_string(&matches, "includes"),
      directories: to_vec_string(&matches, "directories"),
      hostname: matches
        .get_one::<String>("hostname")
        .unwrap_or(&"".to_string())
        .to_string(),
      force: matches.get_one::<bool>("force").copied().unwrap(),
      down: matches.get_one::<bool>("down").copied().unwrap(),
      dry_run: matches.get_one::<bool>("dry-run").copied().unwrap(),
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
    assert_eq!(settings.includes, Vec::<String>::new());
    assert_eq!(settings.directories, Vec::<String>::new());
    assert_eq!(settings.hostname, String::new());
    assert_eq!(settings.force, false);
    assert_eq!(settings.down, false);
    assert_eq!(settings.dry_run, false);
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

  #[test]
  fn test_clap_includes() {
    let settings = ClapParser::new().into_settings(vec![
      "paro",
      "-i",
      "file.txt",
      "-i",
      "file2.txt",
      "-i",
      "file3.txt",
    ]);
    assert_eq!(settings.includes, ["file.txt", "file2.txt", "file3.txt"]);
  }

  #[test]
  fn test_clap_directories() {
    let settings = ClapParser::new().into_settings(vec![
      "paro", "-a", "home/", "-a", "dome/", "-a", "pombe/",
    ]);
    assert_eq!(settings.directories, ["home/", "dome/", "pombe/"]);
  }

  #[test]
  fn test_clap_hostname() {
    let settings = ClapParser::new().into_settings(vec![
      "paro",
      "-B",
      "my-machine",
      "-B",
      "will-override-my-machine",
    ]);
    assert_eq!(settings.hostname, "will-override-my-machine");
  }
}
