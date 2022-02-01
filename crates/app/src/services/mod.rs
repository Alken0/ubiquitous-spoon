mod updater;
pub use updater::UpdateService;

use axum::{AddExtensionLayer, Router};
use sea_orm::DatabaseConnection;

use crate::repositories::FileRepository;
pub fn setup(router: Router, db: &DatabaseConnection) -> Router {
    router.layer(AddExtensionLayer::new(UpdateService::new(
        FileRepository::new(db.clone()),
    )))
}
