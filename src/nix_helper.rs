use nix::unistd::{gethostname, Uid, User};

pub fn remove_last_slash(entry: String) -> String {
  let mut chars = entry.chars();
  if chars.clone().last().unwrap_or_default() == '/' {
    chars.next_back();
    chars.as_str().to_string()
  } else {
    entry
  }
}

pub fn get_hostname() -> String {
  let mut buf = [0u8; 64];
  let hostname_cstr = gethostname(&mut buf).expect("Failed getting hostname");
  hostname_cstr
    .to_str()
    .expect("Hostname wasn't valid UTF-8")
    .to_string()
}

pub fn get_user_home() -> String {
  User::from_uid(Uid::current())
    .expect("Could't find current user")
    .expect("Could't find current user")
    .dir
    .into_os_string()
    .into_string()
    .expect("Could't find user home")
}

pub fn get_default_config_files() -> Vec<String> {
  let home_dir = get_user_home();
  let first_config = format!("{}{}", home_dir, "/.parorc");
  let second_config = format!("{}{}", home_dir, "/.config/paro/parorc");
  let third_config = format!("{}{}", home_dir, "/.dotfiles/parorc");
  let fourth_config =
    format!("{}{}", home_dir, "/.dotfiles/config/paro/parorc");
  vec![first_config, second_config, third_config, fourth_config]
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_remove_last_slash() {
    assert_eq!("", remove_last_slash("".to_string()));
    assert_eq!("folda_name", remove_last_slash("folda_name".to_string()));
    assert_eq!("folda_name", remove_last_slash("folda_name/".to_string()));
  }
}
