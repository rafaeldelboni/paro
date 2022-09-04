use crate::files;
use crate::settings::Settings;
use regex::RegexSet;
use std::collections::BTreeMap;
use std::fs::FileType;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FileEntry {
  pub path: PathBuf,
  pub file_type: FileType,
  pub depth: isize,
}

type Actions = BTreeMap<PathBuf, FileEntry>;

#[derive(Clone, Debug)]
pub struct FileActions {
  pub settings: Settings,
  pub actions: Actions,
}

fn in_special_folder(
  entry: &DirEntry,
  special_folders: &[String],
  set: &RegexSet,
) -> Option<String> {
  let in_special_folder: Vec<_> = set
    .matches(entry.path().to_str().unwrap())
    .into_iter()
    .collect();
  if !in_special_folder.is_empty() {
    Some(special_folders[in_special_folder[0]].clone())
  } else {
    None
  }
}

fn to_file_entry(entry: DirEntry, depth_adjust: isize) -> FileEntry {
  FileEntry {
    path: entry.path().to_path_buf(),
    file_type: entry.file_type(),
    depth: (entry.depth() as isize) + depth_adjust,
  }
}

impl FileActions {
  pub fn new(settings: Settings) -> Self {
    Self {
      settings,
      actions: Actions::new(),
    }
  }

  pub fn select_files(&mut self) {
    let special_folders = &self.settings.special_folder_vec();
    let set = RegexSet::new(special_folders).unwrap();

    for dir in &self.settings.directories {
      let mut entries = WalkDir::new(&dir).into_iter();
      loop {
        match entries.next() {
          None => break,
          Some(Ok(entry)) => {
            if files::is_hidden(entry.file_name()) {
              if entry.file_type().is_dir() && entry.depth() > 0 {
                entries.skip_current_dir();
              }
              continue;
            }

            if let Some(special_folder) =
              in_special_folder(&entry, special_folders, &set)
            {
              self.actions.insert(
                files::change_root_dir(
                  entry.path(),
                  &format!("{}/{}", dir, special_folder),
                  &self.settings.destination,
                  false,
                ),
                to_file_entry(entry, -1),
              );
              continue;
            }

            self.actions.insert(
              files::change_root_dir(
                entry.path(),
                &dir,
                &self.settings.destination,
                false,
              ),
              to_file_entry(entry, 0),
            );
          }
          Some(Err(err)) => print!("ERROR: {}\r\n", err),
        };
      }
    }
  }

  pub fn exclude_files(&mut self) {
    let set = RegexSet::new(self.settings.excludes.clone()).unwrap();
    self
      .actions
      .retain(|_k, v| !set.is_match(v.path.to_str().unwrap()));
  }

  pub fn include_files(&mut self) {
    for file in &self.settings.includes {
      WalkDir::new(&file).into_iter().for_each(|e| match e {
        Ok(entry) => {
          let mut path = PathBuf::from(file);
          path.pop();
          self.actions.insert(
            files::change_root_dir(
              entry.path(),
              &path.to_str().unwrap_or("").to_owned(),
              &self.settings.destination,
              false,
            ),
            to_file_entry(entry, 0),
          );
        }
        Err(err) => print!("ERROR: {}\r\n", err),
      });
    }
  }

  pub fn cleanup_special_folders(&mut self) {
    let dir = self.settings.destination.clone();
    let set =
      RegexSet::new(vec![dir.clone() + "/tag-", dir + "/host-"]).unwrap();
    self
      .actions
      .retain(|k, _v| !set.is_match(k.to_str().unwrap()));
  }

  pub fn hide_files(&mut self) {
    let mut new_actions = Actions::new();
    for (key, value) in self.actions.clone() {
      if value.depth > 0
        && !files::is_hidden(key.file_name().unwrap_or(key.as_os_str()))
      {
        new_actions.insert(
          files::change_root_dir(
            &key,
            &self.settings.destination,
            &self.settings.destination,
            true,
          ),
          value,
        );
      } else {
        new_actions.insert(key, value);
      }
    }
    self.actions = new_actions
  }

  pub fn build(&mut self) -> Self {
    self.select_files();
    self.exclude_files();
    self.include_files();
    self.cleanup_special_folders();
    self.hide_files();
    self.to_owned()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn to_str_dest_files(files: FileActions) -> Vec<String> {
    let mut str_dest_files: Vec<String> = files
      .actions
      .clone()
      .into_iter()
      .map(|(k, _v)| k.to_str().unwrap().to_string())
      .collect();
    str_dest_files.sort();
    str_dest_files
  }

  #[test]
  fn test_select_files() {
    let mut settings = Settings::default();
    settings.directories = vec!["tests/example-dotfiles/".to_string()];
    settings.destination = "/destiny".to_string();
    let mut files: FileActions = FileActions::new(settings);

    files.select_files();

    let str_dest_files: Vec<String> = to_str_dest_files(files);

    assert_eq!(str_dest_files.len(), 14);
    assert_eq!(
      str_dest_files,
      vec![
        "/destiny/",
        "/destiny/folder",
        "/destiny/folder/something.txt",
        "/destiny/host-dois",
        "/destiny/host-dois/file.txt",
        "/destiny/host-um",
        "/destiny/host-um/file.txt",
        "/destiny/normal-file.txt",
        "/destiny/tag-dois",
        "/destiny/tag-dois/file.txt",
        "/destiny/tag-dois/file2.txt",
        "/destiny/tag-um",
        "/destiny/tag-um/file.txt",
        "/destiny/tag-um/file1.txt",
      ]
    );
  }

  #[test]
  fn test_hide_files() {
    let mut settings = Settings::default();
    settings.directories = vec!["tests/example-dotfiles/".to_string()];
    settings.destination = "/destiny".to_string();
    let mut files: FileActions = FileActions::new(settings);

    files.select_files();
    files.hide_files();

    let str_dest_files: Vec<String> = to_str_dest_files(files);

    assert_eq!(str_dest_files.len(), 14);
    assert_eq!(
      str_dest_files,
      vec![
        "/destiny/",
        "/destiny/.folder",
        "/destiny/.folder/something.txt",
        "/destiny/.host-dois",
        "/destiny/.host-dois/file.txt",
        "/destiny/.host-um",
        "/destiny/.host-um/file.txt",
        "/destiny/.normal-file.txt",
        "/destiny/.tag-dois",
        "/destiny/.tag-dois/file.txt",
        "/destiny/.tag-dois/file2.txt",
        "/destiny/.tag-um",
        "/destiny/.tag-um/file.txt",
        "/destiny/.tag-um/file1.txt"
      ]
    );
  }

  #[test]
  fn test_select_files_with_tag_host() {
    let mut settings = Settings::default();
    settings.directories = vec!["tests/example-dotfiles/".to_string()];
    settings.destination = "/destiny".to_string();
    settings.tags = vec!["um".to_string()];
    settings.hostname = "dois".to_string();
    let mut files: FileActions = FileActions::new(settings);

    files.select_files();
    files.cleanup_special_folders();

    let mut str_dest_files: Vec<String> = files
      .actions
      .clone()
      .into_iter()
      .map(|(k, v)| {
        k.to_str().unwrap().to_string() + ":" + &v.depth.to_string()
      })
      .collect();
    str_dest_files.sort();

    assert_eq!(str_dest_files.len(), 6);
    assert_eq!(
      str_dest_files,
      vec![
        "/destiny/:0",
        "/destiny/file.txt:1",
        "/destiny/file1.txt:1",
        "/destiny/folder/something.txt:2",
        "/destiny/folder:1",
        "/destiny/normal-file.txt:1",
      ]
    );
  }

  #[test]
  fn test_exclude_files() {
    let mut settings = Settings::default();
    settings.directories = vec![
      "tests/example-dotfiles/folder".to_string(),
      "tests/example-dotfiles/tag-um".to_string(),
    ];
    settings.excludes = vec![
      "tests/example-dotfiles/folder*".to_string(),
      "tests/example-dotfiles/tag-um/.file.txt".to_string(),
      "tests/example-dotfiles/tag-um/.file1.txt".to_string(),
    ];
    settings.destination = "/destiny".to_string();
    let mut files: FileActions = FileActions::new(settings);

    files.select_files();
    files.exclude_files();

    let str_dest_files: Vec<String> = to_str_dest_files(files);

    assert_eq!(str_dest_files.len(), 3);
    assert_eq!(
      str_dest_files,
      vec!["/destiny/", "/destiny/file.txt", "/destiny/file1.txt"]
    );
  }

  #[test]
  fn test_include_files() {
    let mut settings = Settings::default();
    settings.directories = vec![
      "tests/example-dotfiles/folder".to_string(),
      "tests/example-dotfiles/tag-um".to_string(),
    ];
    settings.includes =
      vec!["tests/example-dotfiles/.ignored-file".to_string()];
    settings.destination = "/destiny".to_string();
    let mut files: FileActions = FileActions::new(settings);

    files.select_files();
    files.include_files();
    files.hide_files();

    let str_dest_files: Vec<String> = to_str_dest_files(files);

    assert_eq!(str_dest_files.len(), 5);
    assert_eq!(
      str_dest_files,
      vec![
        "/destiny/",
        "/destiny/.file.txt",
        "/destiny/.file1.txt",
        "/destiny/.ignored-file",
        "/destiny/.something.txt",
      ]
    );
  }

  #[test]
  fn test_to_file_entry() {
    let mut files = WalkDir::new("tests/example-dotfiles/tag-um/")
      .sort_by_file_name()
      .into_iter();

    let file_one = files.next().unwrap().unwrap();
    let file_two = files.next().unwrap().unwrap();

    assert_eq!(
      to_file_entry(file_one.clone(), 2),
      FileEntry {
        path: file_one.path().to_path_buf(),
        file_type: file_one.file_type(),
        depth: 2,
      }
    );

    assert_eq!(
      to_file_entry(file_two.clone(), 0),
      FileEntry {
        path: file_two.path().to_path_buf(),
        file_type: file_two.file_type(),
        depth: 1,
      }
    );
  }
}
