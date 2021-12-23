use axum::Router;
use sea_orm::{DbConn, DbErr};

pub mod file;
pub use file::File;

pub async fn setup(router: Router, db: &DbConn) -> Result<Router, DbErr> {
    file::setup(db).await?;
    Ok(router)
}
