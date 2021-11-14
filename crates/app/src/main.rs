use askama::Template;
use axum::{routing::get, AddExtensionLayer, Router, Server};
use sea_orm::Database;
use std::net::SocketAddr;
use std::str::FromStr;

const ADDRESS: &str = "127.0.0.1:8080";
const DATABASE_URL: &str = "sqlite://db.sqlite";

#[tokio::main]
async fn main() {
    let address = SocketAddr::from_str(ADDRESS).expect("invalid ip:port");
    let database = Database::connect(DATABASE_URL)
        .await
        .expect("Database connection failed");

    let app = Router::new()
        .route("/", get(hello_world))
        .layer(AddExtensionLayer::new(database));

    Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .expect("could not start server...");
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}

async fn hello_world() -> String {
    let template = HelloTemplate { name: "name" };
    template.render().unwrap()
}
