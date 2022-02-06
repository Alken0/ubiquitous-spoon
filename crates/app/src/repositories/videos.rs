use crate::entities::file;
use file::File;
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, Order, QueryFilter, QueryOrder,
};

#[derive(Clone)]
pub struct Videos {
    db: DatabaseConnection,
}

impl Videos {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_all(&self) -> Result<Vec<File>, String> {
        file::Entity::find()
            .filter(visible_movies())
            .order_by(file::Column::Name, Order::Asc)
            .all(&self.db)
            .await
            .map(|e| e.into_iter().map(File::from).collect())
            .map_err(|e| e.to_string())
    }

    pub async fn find_by_id(&self, id: u64) -> Result<Option<File>, String> {
        file::Entity::find()
            .filter(
                Condition::all()
                    .add(visible_movies())
                    .add(file::Column::Id.eq(id)),
            )
            .one(&self.db)
            .await
            .map(|e| e.map(File::from))
            .map_err(|e| e.to_string())
    }
}

fn visible_movies() -> Condition {
    Condition::all()
        .add(
            Condition::any()
                .add(file::Column::Mime.like("video/%"))
                .add(file::Column::Mime.like("application/x-mpegURL"))
                .add(file::Column::Mime.like("vnd.apple.mpegURL")),
        )
        .add(
            Condition::all()
                .add(file::Column::Mime.not_like("video/MP2T"))
                .add(file::Column::Mime.not_like("video/vnd.dlna.mpeg-tts")),
        )
}
