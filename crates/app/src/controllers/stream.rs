use crate::repositories::Files;
use axum::{
    body::{Bytes, Full},
    extract::{Extension, Path},
    http::{HeaderMap, HeaderValue, Response, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use regex::Regex;
use std::{cmp::max, convert::Infallible};

const DEFAULT_RANGE: u64 = 1048576;

pub fn setup() -> Router {
    Router::new().route("/:id", get(stream))
}

async fn stream(
    Extension(files): Extension<Files>,
    Path(id): Path<u64>,
    headers: HeaderMap,
) -> Result<Chunk, String> {
    let file = files
        .find_by_id(id)
        .await?
        .ok_or_else(|| "id not found".to_string())?;

    let size = file.size;
    let fs_file = fs::File::new(file.path.to_owned(), size);

    let range = headers.get("Range").map(Range::from);
    let response = match range {
        Some(range) => Chunk::new(&fs_file, &range).await?,
        None => Chunk::new(&fs_file, &Range::default()).await?,
    };
    return Ok(response);
}

#[derive(Default)]
pub struct Range(fs::Range);

impl Range {
    pub fn start(&self) -> u64 {
        self.0.start()
    }

    /// returns start + offset or None if the number is too big
    pub fn end(&self) -> Option<u64> {
        let offset = self.0.offset().checked_sub(1).unwrap_or(0);
        self.0.start().checked_add(offset)
    }

    pub fn range(&self) -> fs::Range {
        self.0
    }

    pub fn apply_file_size(&self, file_size: u64) -> Range {
        Range(self.0.apply_filesize(file_size))
    }
}

impl From<&HeaderValue> for Range {
    fn from(value: &HeaderValue) -> Self {
        let value = value.to_str().unwrap_or_default();
        let re = Regex::new(r"[0-9]+").unwrap();
        let mut reversed_numbers: Vec<u64> = re
            .find_iter(value)
            .map(|e| e.as_str())
            .map(|e| e.parse::<u64>().expect("invalid regex"))
            .collect();
        reversed_numbers.reverse();

        let start = reversed_numbers.pop().unwrap_or(0);
        let end = max(
            start,
            reversed_numbers.pop().unwrap_or(start + DEFAULT_RANGE),
        );
        return Self(fs::Range::new(start, end - start));
    }
}

pub struct Chunk {
    start: u64,
    end: u64,
    file_size: u64,
    mime: String,
    content: Vec<u8>,
}

impl Chunk {
    pub async fn new(file: &fs::File, range: &Range) -> Result<Self, String> {
        let range = range.apply_file_size(file.size());
        Ok(Self {
            start: range.start(),
            end: range.end().unwrap_or(0),
            file_size: file.size(),
            mime: file.mime().unwrap(),
            content: file.chunk(&range.range()).await.unwrap(),
        })
    }
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
