use super::common::{get, init_app};
use axum::http::StatusCode;

mod get {
    use super::*;

    #[tokio::test]
    async fn status_code() {
        let app = init_app().await;
        let response = get(app, "/stream/1234/114").await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
