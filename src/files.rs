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
