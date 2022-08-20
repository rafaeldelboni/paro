use crate::settings::Settings;
use regex::RegexSet;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

#[derive(Clone, Debug)]
pub struct PathActions(pub DirEntry, pub PathBuf);

fn change_root_dir(
  origin_path: &Path,
  current: &String,
  new: &String,
) -> PathBuf {
  match origin_path.strip_prefix(Path::new(&current)) {
    Ok(t) => PathBuf::from(new).join(t),
    Err(_) => origin_path.to_path_buf(),
  }
}

fn is_hidden(entry: &DirEntry) -> bool {
  entry
    .file_name()
    .to_str()
    .map(|s| s.starts_with('.'))
    .unwrap_or(false)
}

pub fn select_files(files: &mut Vec<PathActions>, settings: &Settings) {
  for dir in &settings.directories {
    let mut entries = WalkDir::new(&dir).into_iter();
    loop {
      match entries.next() {
        None => break,
        Some(Ok(entry)) => {
          if is_hidden(&entry) {
            if entry.file_type().is_dir() && entry.depth() > 0 {
              entries.skip_current_dir();
            }
            continue;
          }
          files.push(PathActions(
            entry.clone(),
            change_root_dir(entry.path(), dir, &settings.destination),
          ));
        }
        Some(Err(err)) => println!("ERROR: {}", err),
      };
    }
  }
}

pub fn exclude_files(files: &mut Vec<PathActions>, settings: &Settings) {
  let set = RegexSet::new(settings.excludes.clone()).unwrap();
  files.retain(|x| !set.is_match(x.0.path().to_str().unwrap()));
}

pub fn include_files(files: &mut Vec<PathActions>, settings: &Settings) {
  for file in &settings.includes {
    let mut entries = WalkDir::new(&file).into_iter();
    loop {
      match entries.next() {
        None => break,
        Some(Ok(entry)) => {
          let mut path = PathBuf::from(file);
          path.pop();
          files.push(PathActions(
            entry.clone(),
            change_root_dir(
              entry.path(),
              &path.to_str().unwrap_or("").to_owned(),
              &settings.destination,
            ),
          ));
        }
        Some(Err(err)) => println!("ERROR: {}", err),
      };
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_select_files() {
    let mut settings = Settings::default();
    let mut files: Vec<PathActions> = Vec::<PathActions>::new();
    settings.directories = vec![
      "tests/example-dotfiles/folder".to_string(),
      "tests/example-dotfiles/tag-um".to_string(),
    ];
    settings.destination = "/destiny".to_string();
    select_files(&mut files, &settings);

    let mut str_dest_files: Vec<String> = files
      .clone()
      .into_iter()
      .map(|e| e.1.to_str().unwrap().to_string())
      .collect();
    str_dest_files.sort();

    assert_eq!(str_dest_files.len(), 4);
    assert_eq!(
      str_dest_files,
      vec![
        "/destiny/",
        "/destiny/",
        "/destiny/file.txt",
        "/destiny/something.txt",
      ]
    );
  }

  #[test]
  fn test_exclude_files() {
    let mut settings = Settings::default();
    let mut files: Vec<PathActions> = Vec::<PathActions>::new();
    settings.directories = vec![
      "tests/example-dotfiles/folder".to_string(),
      "tests/example-dotfiles/tag-um".to_string(),
    ];
    settings.excludes = vec![
      "tests/example-dotfiles/folder*".to_string(),
      "tests/example-dotfiles/tag-um/.file.txt".to_string(),
    ];
    settings.destination = "/destiny".to_string();
    select_files(&mut files, &settings);
    exclude_files(&mut files, &settings);

    let mut str_dest_files: Vec<String> = files
      .clone()
      .into_iter()
      .map(|e| e.1.to_str().unwrap().to_string())
      .collect();
    str_dest_files.sort();

    assert_eq!(str_dest_files.len(), 2);
    assert_eq!(str_dest_files, vec!["/destiny/", "/destiny/file.txt",]);
  }

  #[test]
  fn test_include_files() {
    let mut settings = Settings::default();
    let mut files: Vec<PathActions> = Vec::<PathActions>::new();
    settings.directories = vec![
      "tests/example-dotfiles/folder".to_string(),
      "tests/example-dotfiles/tag-um".to_string(),
    ];
    settings.includes =
      vec!["tests/example-dotfiles/.ignored-file".to_string()];
    settings.destination = "/destiny".to_string();
    select_files(&mut files, &settings);
    include_files(&mut files, &settings);

    let mut str_dest_files: Vec<String> = files
      .clone()
      .into_iter()
      .map(|e| e.1.to_str().unwrap().to_string())
      .collect();
    str_dest_files.sort();

    assert_eq!(str_dest_files.len(), 5);
    assert_eq!(
      str_dest_files,
      vec![
        "/destiny/",
        "/destiny/",
        "/destiny/.ignored-file",
        "/destiny/file.txt",
        "/destiny/something.txt",
      ]
    );
  }

  #[test]
  fn test_change_root_dir() {
    let path = Path::new("/test/file.txt");
    assert_eq!(
      change_root_dir(path, &"/test".to_string(), &"/new".to_string())
        .to_string_lossy(),
      "/new/file.txt"
    );

    // should ignore if root is not in the current path
    let path2 = Path::new("/test/file.txt");
    assert_eq!(
      change_root_dir(path2, &"/non-root".to_string(), &"/new".to_string())
        .to_string_lossy(),
      "/test/file.txt"
    );
  }

  #[test]
  fn test_is_hidden() {
    let mut files = WalkDir::new("tests/example-dotfiles/tag-um/")
      .sort_by_file_name()
      .into_iter();
    assert_eq!(is_hidden(&files.next().unwrap().unwrap()), false);
    assert_eq!(is_hidden(&files.next().unwrap().unwrap()), true);
  }
}
