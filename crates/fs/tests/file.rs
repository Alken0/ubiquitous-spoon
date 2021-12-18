use fs::{File, Range};
use tokio::fs::metadata;

#[tokio::test]
async fn len_of_chunk_equals_offset_of_range() {
    let path = "./tests/data/text.txt".to_owned();
    let meta = metadata(&path).await.unwrap();
    let file = File::new(path, meta.len());
    let range = Range::new(0, 100);

    let chunk = file.chunk(&range).await.expect("chunk is error");
    assert_eq!(chunk.len() as u64, range.offset());
}

#[tokio::test]
async fn invalid_path_is_error() {
    let path = "./tests/data/not_found.invalid".to_owned();
    let file = File::new(path, 100);
    let range = Range::new(0, 100);

    let chunk = file.chunk(&range).await;
    assert!(chunk.is_err());
}
