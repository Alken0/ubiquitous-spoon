use axum::Router;
use axum::{
    body::Body,
    http::{Method, Request},
};
use tower::util::Oneshot;
use tower::ServiceExt;

pub async fn init_app() -> Router {
    app::app("sqlite::memory:").await.unwrap()
}

fn get_request(uri: &str) -> Request<Body> {
    Request::builder()
        .method(Method::GET)
        .uri(uri)
        .body(Body::empty())
        .unwrap()
}

pub fn get(app: Router, uri: &str) -> Oneshot<Router, Request<Body>> {
    let request = get_request(uri);
    app.oneshot(request)
}
