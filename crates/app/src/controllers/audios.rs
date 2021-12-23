use super::render;
use crate::repositories::Audios;
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
#[template(path = "views/audios/list.html")]
struct AudiosTemplate {
    audios: Vec<crate::entities::File>,
}

async fn list_all(Extension(audios): Extension<Audios>) -> Result<Html<String>, String> {
    let audios = audios.find_all().await?;
    let template = render(AudiosTemplate { audios })?;
    Ok(Html::from(template))
}

#[derive(Template)]
#[template(path = "views/audios/player.html")]
struct PlayerTemplate {
    audio: crate::entities::File,
}

pub async fn player(
    Extension(audios): Extension<Audios>,
    Path(id): Path<u64>,
) -> Result<Html<String>, String> {
    let audio = audios
        .find_by_id(id)
        .await?
        .ok_or_else(|| "id not found".to_string())?;
    let template = render(PlayerTemplate { audio })?;
    Ok(Html::from(template))
}
