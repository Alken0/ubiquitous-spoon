use self::{range::Range, response::FileResponse};
use crate::repositories::FileRepository;
use axum::{
    extract::{Extension, Path},
    routing::get,
    Router,
};

mod chunk;
mod range;
mod response;
mod whole;

const DEFAULT_RANGE: u64 = 1048576;

pub fn setup() -> Router {
    Router::new().route("/:group_id/:group_member_name", get(stream))
}

async fn stream(
    Extension(files): Extension<FileRepository>,
    Path((group_id, group_member_name)): Path<(String, String)>,
    range: Option<Range>,
) -> Result<FileResponse, String> {
    let file = files
        .find_by_group(&group_id, &group_member_name)
        .await?
        .ok_or_else(|| "not found".to_string())?;

    let response = FileResponse::new(&file, &range).await?;
    return Ok(response);
}
