use super::entry::Entry;
use crate::path::Path;
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
}
