use super::render;
use crate::repositories::FileRepository;
use askama::Template;
use axum::{extract::Extension, response::Html, routing::get, Router};

pub fn setup() -> Router {
    Router::new().route("/", get(list_all))
}

#[derive(Template)]
#[template(path = "views/files.html")]
struct ListAllTemplate {
    files: Vec<crate::entities::File>,
}

async fn list_all(Extension(files): Extension<FileRepository>) -> Result<Html<String>, String> {
    let files = files.find_all().await?;
    let template = render(ListAllTemplate { files })?;
    Ok(Html::from(template))
}
