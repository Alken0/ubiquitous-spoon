use crate::dir::directory::Directory;
use crate::error;
use crate::file::File;
use std::io::Result;
use tokio::fs::metadata;

#[derive(Debug, PartialEq)]
pub enum Entry {
    Directory(Directory),
    File(File),
}

impl Entry {
    pub async fn new(path: String) -> Result<Self> {
        let meta = metadata(&path).await?;

        if meta.is_file() {
            let file = File::new(path, meta.len());
            return Ok(Entry::File(file));
        }

        if meta.is_dir() {
            let dir = Directory::new(path);
            return Ok(Entry::Directory(dir));
        }

        error::invalid_input(format!("path is neither file nor dir: '{}'", path))
    }
}
