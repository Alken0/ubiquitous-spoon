use crate::services::UpdateService;
use askama::Template;
use axum::{
    extract::Extension,
    http::Uri,
    response::{Html, Redirect},
    routing::{get, post},
    Router,
};

const UPDATE_PATH: &'static str = "./";
const REFRESH_REDIRECT_PATH: &'static str = "/";

pub fn setup() -> Router {
    Router::new()
        .route("/refresh", post(refresh))
        .route("/shutdown", post(shutdown))
        .route("/", get(settings))
}

#[derive(Template)]
#[template(path = "views/settings.html")]
struct SettingsTemplate {}

async fn settings() -> Result<Html<String>, String> {
    let template = SettingsTemplate {}.render().map_err(|e| e.to_string())?;
    Ok(Html::from(template))
}

async fn refresh(Extension(updater): Extension<UpdateService>) -> Result<Redirect, String> {
    updater.run(UPDATE_PATH.to_string()).await?;
    let redirect = Redirect::to(Uri::from_static(REFRESH_REDIRECT_PATH));
    return Ok(redirect);
}

async fn shutdown() {
    std::process::exit(0)
}
