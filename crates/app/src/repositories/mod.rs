mod audios;
mod files;
mod videos;

pub use audios::Audios;
pub use files::Files;
pub use files::InsertFile;
pub use videos::Videos;

use axum::{AddExtensionLayer, Router};
use sea_orm::DatabaseConnection;
pub fn setup(router: Router, db: &DatabaseConnection) -> Router {
    router
        .layer(AddExtensionLayer::new(Files::new(db.clone())))
        .layer(AddExtensionLayer::new(Audios::new(db.clone())))
        .layer(AddExtensionLayer::new(Videos::new(db.clone())))
}
