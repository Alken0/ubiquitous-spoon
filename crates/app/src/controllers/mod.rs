use askama::Template;
use axum::{
    error_handling::HandleErrorExt,
    http::{StatusCode, Uri},
    response::Redirect,
    routing::{get, service_method_routing as service},
    Router,
};
use tower_http::services::ServeDir;

mod audios;
mod files;
mod settings;
mod stream;
mod videos;

const UPDATE_PATH: &'static str = "./";
const STATIC_FILE_DIR: &'static str = "./crates/app/static";

pub fn setup(router: Router) -> Router {
    router
        .nest(
            "/static",
            service::get(ServeDir::new(STATIC_FILE_DIR))
                .handle_error(|e| (StatusCode::NOT_FOUND, format!("{}", e))),
        )
        .nest("/videos", videos::setup())
        .nest("/audios", audios::setup())
        .nest("/files", files::setup())
        .nest("/stream", stream::setup())
        .nest("/settings", settings::setup())
        .route("/", get(index))
}

async fn index() -> Redirect {
    return Redirect::to(Uri::from_static("/videos"));
}

fn render(template: impl Template) -> Result<String, String> {
    template.render().map_err(|e| e.to_string())
}
