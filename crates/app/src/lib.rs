use axum::Router;
use sea_orm::Database;

mod controllers;
mod entities;
mod repositories;
mod services;

pub async fn app(database_url: &str) -> anyhow::Result<Router> {
    let database = Database::connect(database_url).await?;

    let mut app = Router::new();
    app = controllers::setup(app);
    app = repositories::setup(app, &database);
    app = services::setup(app, &database);
    app = entities::setup(app, &database).await?;

    Ok(app)
}
