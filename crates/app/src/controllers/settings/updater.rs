use crate::repositories::{Files, InsertFile};
use futures::future::join_all;
use tokio::task;

pub struct UpdateService {
    repository: Files,
    to_check: Vec<fs::Directory>,
}

impl UpdateService {
    pub fn new(repository: Files, to_check: &str) -> Self {
        Self {
            repository,
            to_check: vec![fs::Directory::new(to_check.to_owned())],
        }
    }

    pub async fn clean_run(self) -> Result<(), String> {
        self.clean().await?;
        self.run().await;
        Ok(())
    }

    async fn clean(&self) -> Result<(), String> {
        for dir in &self.to_check {
            self.repository.delete_by_path(&dir.path()).await?;
        }
        Ok(())
    }

    async fn run(mut self) {
        task::spawn(async move {
            while !self.to_check.is_empty() {
                let mut content = collect_content(&self.to_check).await;
                self.to_check.append(&mut content.dirs);
                let result = self
                    .repository
                    .insert_all(into_file_insert(content.files))
                    .await;

                // TODO: better error handling for async-background tasks?
                if result.is_err() {
                    println!(
                        "error has occured while inserting new files: {}",
                        result.err().unwrap()
                    )
                }
            }
        });
    }
}

#[derive(Default)]
struct DirContent {
    files: Vec<fs::File>,
    dirs: Vec<fs::Directory>,
}

fn into_file_insert(files: Vec<fs::File>) -> Vec<InsertFile> {
    files
        .into_iter()
        .filter_map(|f| f.try_into().ok())
        .collect()
}

impl TryInto<InsertFile> for fs::File {
    type Error = String;

    fn try_into(self) -> Result<InsertFile, Self::Error> {
        Ok(InsertFile::new(
            self.name(),
            self.path(),
            self.mime().map_err(|e| e.to_string())?,
            self.size(),
        )?)
    }
}

async fn collect_content(elements: &[fs::Directory]) -> DirContent {
    let async_checks = elements.iter().map(|e| e.elements());

    let entries: Vec<fs::Entry> = join_all(async_checks)
        .await
        .into_iter()
        .filter_map(|e| e.ok())
        .flatten()
        .collect();

    let mut output = DirContent::default();
    entries.into_iter().for_each(|f| match f {
        fs::Entry::File(f) => output.files.push(f),
        fs::Entry::Directory(d) => output.dirs.push(d),
    });

    return output;
}

#[cfg(test)]
mod test {
    use super::*;
    use testing::{
        error::{Error, Result},
        functions::assert_vec_equal,
    };

    #[tokio::test]
    async fn files_and_dirs() -> Result<()> {
        let result = collect_content(&vec![fs::Directory::new("./tests/data".to_owned())]).await;
        let expected_files: Vec<fs::File> = into_fs_file(&vec![
            "./tests/data/test-file.txt",
            "./tests/data/test-file.yml",
            "./tests/data/music.mp3",
            "./tests/data/toystory.mp4",
        ])
        .await?;
        let expected_dirs: Vec<fs::Directory> = into_fs_dir(&vec![
            "./tests/data/dir1",
            "./tests/data/dir2",
            "./tests/data/dir3",
        ])
        .await;

        assert_vec_equal(
            &result.files,
            &expected_files,
            "incorrect number of files found",
        )?;
        assert_vec_equal(
            &result.dirs,
            &expected_dirs,
            "incorrect number of dirs found",
        )?;
        Ok(())
    }

    #[tokio::test]
    async fn multiple_dirs_as_input() -> Result<()> {
        let input_dirs: Vec<fs::Directory> = into_fs_dir(&vec![
            "./tests/data/dir1",
            "./tests/data/dir2",
            "./tests/data/dir3",
        ])
        .await;
        let result = collect_content(&input_dirs).await;
        let expected_files: Vec<fs::File> = into_fs_file(&vec![
            "./tests/data/dir1/test1.txt",
            "./tests/data/dir2/test2.txt",
            "./tests/data/dir3/test3.txt",
        ])
        .await?;

        assert_vec_equal(&result.files, &expected_files, "did not find all files")?;
        assert_vec_equal(&result.dirs, &Vec::new(), "found non existing dirs")?;
        Ok(())
    }

    #[tokio::test]
    async fn non_existing_dir() {
        let dir = vec![fs::Directory::new("./tests/data/not_found".to_owned())];
        let result = collect_content(&dir).await;
        assert!(result.files.is_empty());
        assert!(result.dirs.is_empty());
    }

    #[tokio::test]
    async fn empty_input() -> Result<()> {
        let result = collect_content(&Vec::new()).await;
        if !result.files.is_empty() {
            return Err(Error::msg(format!("files not empty: {:?}", result.files)));
        }
        if !result.dirs.is_empty() {
            return Err(Error::msg(format!("dirs not empty: {:?}", result.dirs)));
        }
        Ok(())
    }

    async fn into_fs_file(paths: &[&str]) -> Result<Vec<fs::File>> {
        let mut output = Vec::new();
        for p in paths {
            let file = fs::File::new_from_path(p).await?;
            output.push(file);
        }
        return Ok(output);
    }

    async fn into_fs_dir(paths: &[&str]) -> Vec<fs::Directory> {
        let mut output = Vec::new();
        for p in paths {
            let dir = fs::Directory::new(p.to_string());
            output.push(dir);
        }
        return output;
    }
}
