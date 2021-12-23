use axum::Server;
use std::net::SocketAddr;
use std::str::FromStr;

const ADDRESS: &str = "127.0.0.1:8080";
const DATABASE_URL: &str = "sqlite://db.sqlite";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let address = SocketAddr::from_str(ADDRESS)?;
    let app = app::app(DATABASE_URL).await?;

    Server::bind(&address)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
