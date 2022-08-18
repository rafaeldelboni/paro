use crate::types::PathBufPair;
use regex::RegexSet;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

// TODO unit test
pub fn change_root_dir(
  origin_path: &Path,
  current: &String,
  new: &String,
) -> PathBuf {
  let new_path = PathBuf::from(new);
  new_path.join(
    origin_path
      .strip_prefix(Path::new(&current))
      .unwrap_or(Path::new("")),
  )
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
          paths.push(PathBufPair(
            t.clone(),
            change_root_dir(t.path().clone(), &dir, &destination),
          ));
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

    assert_eq!(str_files.len(), 5);
    assert_eq!(
      str_files,
      vec![
        "tests/example-dotfiles/folder",
        "tests/example-dotfiles/folder/something.txt",
        "tests/example-dotfiles/tag-um",
        "tests/example-dotfiles/tag-um/.file.txt",
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
}
