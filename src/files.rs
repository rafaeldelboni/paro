use std::ffi::OsStr;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

pub fn change_root_dir(
  origin_path: &Path,
  current: &String,
  new: &String,
  hide: bool,
) -> PathBuf {
  match origin_path.strip_prefix(Path::new(&current)) {
    Ok(t) => {
      if hide {
        PathBuf::from(new).join(format!("{}{}", ".", t.to_string_lossy()))
      } else {
        PathBuf::from(new).join(t)
      }
    }
    Err(_) => origin_path.to_path_buf(),
  }
}

pub fn is_hidden(entry: &OsStr) -> bool {
  entry.to_str().map(|s| s.starts_with('.')).unwrap_or(false)
}

pub fn is_same_file(
  origin_file: &Path,
  destiny_file: &Path,
) -> Result<bool, std::io::Error> {
  // TODO consider links and follow links to get metadata

  if !origin_file.exists() || !destiny_file.exists() {
    return Ok(false);
  }

  let m1 = origin_file.metadata()?;
  let m2 = destiny_file.metadata()?;

  if m1.ino() != m2.ino() {
    return Ok(false);
  }

  if m1.dev() != m2.dev() {
    return Ok(false);
  }

  Ok(true)
}

#[cfg(test)]
mod tests {
  use super::*;
  use walkdir::WalkDir;

  #[test]
  fn test_is_same_file() {
    let path_1 = Path::new("tests/example-dotfiles/tag-um/file.txt");
    let path_2 = Path::new("tests/example-dotfiles/tag-um/file.txt");

    assert_eq!(is_same_file(path_1, path_2).unwrap(), true);

    let path_1 = Path::new("tests/example-dotfiles/tag-um/file.txt");
    let path_2 = Path::new("tests/example-dotfiles/tag-um/.file.txt");

    assert_eq!(is_same_file(path_1, path_2).unwrap(), false);

    let path_1 = Path::new("non-existing.txt");
    let path_2 = Path::new("tests/example-dotfiles/tag-um/file.txt");

    assert_eq!(is_same_file(path_1, path_2).unwrap(), false);

    let path_1 = Path::new("tests/example-dotfiles/tag-um/file.txt");
    let path_2 = Path::new("non-existing.txt");

    assert_eq!(is_same_file(path_1, path_2).unwrap(), false);
  }

  #[test]
  fn test_change_root_dir() {
    let path = Path::new("/test/file.txt");
    assert_eq!(
      change_root_dir(path, &"/test".to_string(), &"/new".to_string(), false)
        .to_string_lossy(),
      "/new/file.txt"
    );

    // should ignore if root is not in the current path
    let path2 = Path::new("/test/file.txt");
    assert_eq!(
      change_root_dir(
        path2,
        &"/non-root".to_string(),
        &"/new".to_string(),
        false
      )
      .to_string_lossy(),
      "/test/file.txt"
    );

    let path3 = Path::new("/test/file.txt");
    assert_eq!(
      change_root_dir(path3, &"/test".to_string(), &"/new".to_string(), true)
        .to_string_lossy(),
      "/new/.file.txt"
    );
  }

  #[test]
  fn test_is_hidden() {
    let mut files = WalkDir::new("tests/example-dotfiles/tag-um/")
      .sort_by_file_name()
      .into_iter();
    assert_eq!(
      is_hidden(&files.next().unwrap().unwrap().file_name()),
      false
    );
    assert_eq!(is_hidden(&files.next().unwrap().unwrap().file_name()), true);
  }
}
