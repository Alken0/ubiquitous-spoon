use crate::entities::File;
use axum::{
    body::{Bytes, Full},
    http::{Response, StatusCode},
    response::IntoResponse,
};
use std::convert::Infallible;

pub struct Whole {
    mime: String,
    content: Vec<u8>,
}

impl Whole {
    pub async fn new(file: &File) -> Result<Self, String> {
        let fs_file = fs::File::new(file.path.to_owned(), file.size);

        Ok(Self {
            mime: file.mime.to_string(),
            content: fs_file
                .chunk(&fs::Range::new(0, file.size))
                .await
                .map_err(|e| e.to_string())?,
        })
    }
}

impl IntoResponse for Whole {
    type Body = Full<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> Response<Self::Body> {
        Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", self.mime)
            .body(Full::from(self.content))
            .expect("valid streaming body")
    }
}
