use walkdir::{DirEntry, WalkDir};

pub fn walk_directories(directories: Vec<String>) -> Vec<DirEntry> {
  let mut paths: Vec<DirEntry> = Vec::<DirEntry>::new();
  for dir in directories {
    for entry in WalkDir::new(dir) {
      match entry {
        Ok(t) => paths.push(t),
        Err(e) => println!("Error: {}", e),
      }
    }
  }
  paths
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_walk_directories() {
    let files = walk_directories(vec![
      "tests/example-dotfiles/folder".to_string(),
      "tests/example-dotfiles/tag-um".to_string(),
    ]);
    let str_files: Vec<String> = files
      .clone()
      .into_iter()
      .map(|e| e.path().to_str().unwrap().to_string())
      .collect();

    assert_eq!(str_files.len(), 5);
    assert_eq!(
      str_files,
      vec![
        "tests/example-dotfiles/folder",
        "tests/example-dotfiles/folder/something.txt",
        "tests/example-dotfiles/tag-um",
        "tests/example-dotfiles/tag-um/file.txt",
        "tests/example-dotfiles/tag-um/.file.txt"
      ]
    );
  }
}
