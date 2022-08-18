use crate::types::PathBufPair;
use regex::RegexSet;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

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

// TODO use paro Settings as argument
pub fn walk_directories(
  directories: Vec<String>,
  destination: String,
) -> Vec<PathBufPair> {
  let mut paths: Vec<PathBufPair> = Vec::<PathBufPair>::new();
  for dir in directories {
    for entry in WalkDir::new(&dir) {
      match entry {
        Ok(t) => {
          if !is_hidden(&t) {
            paths.push(PathBufPair(
              t.clone(),
              change_root_dir(t.path(), &dir, &destination),
            ));
          }
        }
        Err(e) => println!("Error: {}", e),
      }
    }
  }
  paths
}

pub fn remove_files(files: &mut Vec<PathBufPair>, excludes: Vec<String>) {
  println!("{:?}", excludes);
  let set = RegexSet::new(excludes).unwrap();
  files.retain(|x| !set.is_match(x.0.path().to_str().unwrap()));
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_walk_directories() {
    let files = walk_directories(
      vec![
        "tests/example-dotfiles/folder".to_string(),
        "tests/example-dotfiles/tag-um".to_string(),
      ],
      "/destiny".to_string(),
    );
    let mut str_files: Vec<String> = files
      .clone()
      .into_iter()
      .map(|e| e.0.path().to_str().unwrap().to_string())
      .collect();
    str_files.sort();

    assert_eq!(str_files.len(), 4);
    assert_eq!(
      str_files,
      vec![
        "tests/example-dotfiles/folder",
        "tests/example-dotfiles/folder/something.txt",
        "tests/example-dotfiles/tag-um",
        "tests/example-dotfiles/tag-um/file.txt",
      ]
    );
  }

  #[test]
  fn test_remove_files() {
    let mut files = walk_directories(
      vec![
        "tests/example-dotfiles/folder".to_string(),
        "tests/example-dotfiles/tag-um".to_string(),
      ],
      "/destiny".to_string(),
    );
    remove_files(
      &mut files,
      vec![
        "tests/example-dotfiles/folder*".to_string(),
        "tests/example-dotfiles/tag-um/.file.txt".to_string(),
      ],
    );

    let mut str_files: Vec<String> = files
      .clone()
      .into_iter()
      .map(|e| e.0.path().to_str().unwrap().to_string())
      .collect();
    str_files.sort();

    assert_eq!(str_files.len(), 2);
    assert_eq!(
      str_files,
      vec![
        "tests/example-dotfiles/tag-um",
        "tests/example-dotfiles/tag-um/file.txt",
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
