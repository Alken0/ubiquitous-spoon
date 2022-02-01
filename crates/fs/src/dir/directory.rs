use crate::path::Path;
use crate::Entry;
use crate::File;
use futures::future::join_all;
use tokio::fs::read_dir;
use tokio::io::Result;

#[derive(Debug, PartialEq)]
pub struct Directory {
    path: String,
}

impl Directory {
    pub fn new(path: String) -> Self {
        let path = path.replace("\\", "/");
        Self { path }
    }

    pub fn path(&self) -> String {
        self.path.to_owned()
    }

    pub async fn elements(&self) -> Result<Vec<Entry>> {
        let mut entries = read_dir(&self.path).await?;

        let mut paths: Vec<Path> = Vec::new();
        while let Ok(Some(entry)) = entries.next_entry().await {
            paths.push(entry.into());
        }

        let async_new_entries = paths.into_iter().map(String::from).map(Entry::new);
        let elements = join_all(async_new_entries)
            .await
            .into_iter()
            .filter_map(|f| f.ok())
            .collect();

        return Ok(elements);
    }

    // returns all files in this directory and all sub/subsub/... directories
    pub async fn files_recursively(&self) -> Vec<File> {
        let mut files_to_output = Vec::new();
        let mut dirs_to_check = vec![Directory::new(self.path.to_string())];

        while !dirs_to_check.is_empty() {
            let async_checks = dirs_to_check.iter().map(|e| e.elements());
            let entries: Vec<Entry> = join_all(async_checks)
                .await
                .into_iter()
                .filter_map(|e| e.ok())
                .flatten()
                .collect();

            entries.into_iter().for_each(|f| match f {
                Entry::File(f) => files_to_output.push(f),
                Entry::Directory(d) => dirs_to_check.push(d),
            });
        }

        return files_to_output;
    }
}
