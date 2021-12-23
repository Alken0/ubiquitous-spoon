use super::render;
use crate::repositories::Videos;
use askama::Template;
use axum::{
    extract::{Extension, Path},
    response::Html,
    routing::get,
    Router,
};

pub fn setup() -> Router {
    Router::new()
        .route("/:id", get(player))
        .route("/", get(list_all))
}

#[derive(Template)]
#[template(path = "views/videos/list.html")]
struct VideosTemplate {
    videos: Vec<crate::entities::File>,
}

async fn list_all(Extension(videos): Extension<Videos>) -> Result<Html<String>, String> {
    let videos = videos.find_all().await?;
    let template = render(VideosTemplate { videos })?;
    Ok(Html::from(template))
}

#[derive(Template)]
#[template(path = "views/videos/player.html")]
struct PlayerTemplate {
    video: crate::entities::File,
}

pub async fn player(
    Extension(videos): Extension<Videos>,
    Path(id): Path<u64>,
) -> Result<Html<String>, String> {
    let video = videos
        .find_by_id(id)
        .await?
        .ok_or_else(|| "id not found".to_string())?;
    let template = render(PlayerTemplate { video })?;
    Ok(Html::from(template))
}
