use crate::entities::{file, File};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, Order, QueryFilter, QueryOrder};

#[derive(Clone)]
pub struct Audios {
    db: DatabaseConnection,
}

impl Audios {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_all(&self) -> Result<Vec<File>, String> {
        file::Entity::find()
            .filter(file::Column::Mime.like("audio/%"))
            .order_by(file::Column::Name, Order::Asc)
            .all(&self.db)
            .await
            .map(|e| e.into_iter().map(File::from).collect())
            .map_err(|e| e.to_string())
    }

    pub async fn find_by_id(&self, id: u64) -> Result<Option<File>, String> {
        file::Entity::find()
            .filter(file::Column::Id.eq(id))
            .filter(file::Column::Mime.like("audio/%"))
            .one(&self.db)
            .await
            .map(|e| e.map(File::from))
            .map_err(|e| e.to_string())
    }
}
