use std::path::PathBuf;

use fs::{Directory, Entry};
use futures::future::join_all;
use testing::{error::Result, functions::assert_vec_equal};
use tokio::fs::create_dir;

#[tokio::test]
async fn elements() -> Result<()> {
    // git does not save empty dirs
    if !PathBuf::from("./tests/data/dir").exists() {
        create_dir("./tests/data/dir").await?;
    }

    let path = "./tests/data".to_owned();
    let dir = Directory::new(path).elements().await?;

    let expected = join_all(vec![
        new_entry("./tests/data/text.txt"),
        new_entry("./tests/data/dir"),
    ])
    .await;

    assert_vec_equal(&dir, &expected, "vecs are not equal")?;

    Ok(())
}

async fn new_entry(path: &str) -> Entry {
    Entry::new(path.to_owned()).await.unwrap()
}

#[tokio::test]
async fn invalid_path_is_error() {
    let path = "./tests/not_found".to_owned();
    let elements = Directory::new(path).elements().await;

    assert!(elements.is_err());
}
