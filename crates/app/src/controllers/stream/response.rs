use super::{chunk::Chunk, range::Range, whole::Whole};
use crate::entities::File;
use axum::{
    body::{Bytes, Full},
    http::Response,
    response::IntoResponse,
};
use std::convert::Infallible;

use super::DEFAULT_RANGE;

pub enum FileResponse {
    WHOLE(Whole),
    CHUNKED(Chunk),
}

impl FileResponse {
    pub async fn new(file: &File, range: &Option<Range>) -> Result<Self, String> {
        if file.size <= DEFAULT_RANGE {
            return Self::whole(file).await;
        }
        return Self::chunked(file, range).await;
    }

    async fn whole(file: &File) -> Result<Self, String> {
        let response = Self::WHOLE(Whole::new(file).await?);
        return Ok(response);
    }

    async fn chunked(file: &File, range: &Option<Range>) -> Result<Self, String> {
        let response = Self::CHUNKED(Chunk::new(file, range).await?);
        return Ok(response);
    }
}

impl IntoResponse for FileResponse {
    type Body = Full<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> Response<Self::Body> {
        match self {
            Self::WHOLE(r) => r.into_response(),
            Self::CHUNKED(r) => r.into_response(),
        }
    }
}
