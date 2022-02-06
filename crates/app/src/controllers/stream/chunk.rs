use super::range::Range;
use crate::entities::File;
use axum::{
    body::{Bytes, Full},
    http::{Response, StatusCode},
    response::IntoResponse,
};
use std::convert::Infallible;

pub struct Chunk {
    start: u64,
    end: u64,
    file_size: u64,
    mime: String,
    content: Vec<u8>,
}

impl Chunk {
    pub async fn new(file: &File, range: &Option<Range>) -> Result<Self, String> {
        let range = default_range(file, range);
        let fs_file = fs::File::new(file.path.to_owned(), file.size);

        Ok(Self {
            start: range.start(),
            end: range.end().unwrap_or(0),
            file_size: file.size,
            mime: file.mime.to_string(),
            content: fs_file
                .chunk(&range.range())
                .await
                .map_err(|e| e.to_string())?,
        })
    }
}

fn default_range(file: &File, range: &Option<Range>) -> Range {
    let range = range.unwrap_or_default();
    let range = range.apply_file_size(file.size);
    return range;
}

impl IntoResponse for Chunk {
    type Body = Full<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> Response<Self::Body> {
        Response::builder()
            .status(StatusCode::PARTIAL_CONTENT)
            .header("Content-Type", self.mime)
            .header("Accept-Ranges", "bytes")
            .header(
                "Content-Range",
                format!("bytes {}-{}/{}", self.start, self.end, self.file_size),
            )
            .body(Full::from(self.content))
            .expect("valid streaming body")
    }
}
