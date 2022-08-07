use crate::settings::ParoSettings;
use clap::{App, Arg, ArgAction, Command};

pub struct ClapParser {
  clap: App<'static>,
}

impl<'help> ClapParser {
  pub fn new() -> Self {
    let app = Command::new("paro")
      .version("0.0.1")
      .about("Tool for managing dotfiles directories.")
      .after_help(
        "Note: `paro -h` prints a short and concise overview while \
                `paro --help` gives all details.",
      )
      .arg(
        Arg::new("excludes")
          .short('x')
          .long("excludes")
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

  pub fn to_settings(self, manual_args: Vec<&str>) -> ParoSettings {
    let matches = if manual_args.is_empty() {
      self.clap.get_matches().clone()
    } else {
      self.clap.get_matches_from(manual_args).clone()
    };
    ParoSettings {
      excludes: matches
        .get_many::<String>("excludes")
        .unwrap_or_default()
        .map(|v| v.to_string())
        .collect(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_clap_excludes() {
    let settings = ClapParser::new().to_settings(vec![
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
