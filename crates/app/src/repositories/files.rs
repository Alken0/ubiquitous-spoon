use crate::entities::file;
use file::File;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, Order, QueryFilter, QueryOrder, Set, Unset,
};

#[derive(Clone)]
pub struct Files {
    db: DatabaseConnection,
}

impl Files {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_by_id(&self, id: u64) -> Result<Option<File>, String> {
        let id = id.try_into().map_err(|_| "id is too big")?;
        file::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map(|f| f.map(File::from))
            .map_err(|e| e.to_string())
    }

    pub async fn find_all(&self) -> Result<Vec<File>, String> {
        file::Entity::find()
            .order_by(file::Column::Name, Order::Asc)
            .all(&self.db)
            .await
            .map(|f| f.into_iter().map(File::from).collect())
            .map_err(|e| e.to_string())
    }

    pub async fn insert_all(&self, files: Vec<InsertFile>) -> Result<(), String> {
        // Entity::insert_many returns sql-error if vector is empty
        if files.is_empty() {
            return Ok(());
        }

        let inserts: Vec<file::ActiveModel> = files.into_iter().map(|e| e.active_model).collect();

        return file::Entity::insert_many(inserts)
            .exec(&self.db)
            .await
            .map(|_| ())
            .map_err(|e| e.to_string());
    }

    pub async fn delete_by_path(&self, path: &str) -> Result<(), String> {
        file::Entity::delete_many()
            .filter(file::Column::Path.like(&format!("{}%", path)))
            .exec(&self.db)
            .await
            .map(|_| ())
            .map_err(|e| e.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct InsertFile {
    active_model: file::ActiveModel,
}

impl InsertFile {
    pub fn new(name: String, path: String, mime: String, size: u64) -> Result<Self, String> {
        Ok(Self {
            active_model: file::ActiveModel {
                id: Unset(None),
                name: Set(name),
                path: Set(path),
                mime: Set(mime),
                size: Set(size
                    .try_into()
                    .map_err(|_| "file size is too big: max is i64::MAX".to_string())?),
            },
        })
    }
}
