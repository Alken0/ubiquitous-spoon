use crate::entities::file;
use file::File;
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, Order, QueryFilter, QueryOrder, Set,
    Unset,
};

#[derive(Clone)]
pub struct FileRepository {
    db: DatabaseConnection,
}

impl FileRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_by_group(
        &self,
        group_id: &str,
        group_member_name: &str,
    ) -> Result<Option<File>, String> {
        file::Entity::find()
            .filter(
                Condition::all()
                    .add(file::Column::GroupId.like(group_id))
                    .add(file::Column::GroupMemberName.like(group_member_name)),
            )
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

    /// silently throws away files that are too large to be saved in database (size > i64::MAX)
    pub async fn insert_all(&self, files: Vec<InsertFile>) -> Result<(), String> {
        // SQLITE-Limitation: throws error "Execution Error: error returned from database: too many SQL variables" if there are too many inserts at once
        let mut inserts = files
            .into_iter()
            .filter_map(|e| e.try_into().ok())
            .peekable();

        while inserts.peek().is_some() {
            let chunk: Vec<file::ActiveModel> = inserts.by_ref().take(1024).collect();
            let result = file::Entity::insert_many(chunk)
                .exec(&self.db)
                .await
                .map(|_| ())
                .map_err(|e| e.to_string());

            if result.is_err() {
                println!("{}", result.as_ref().err().unwrap());
                return result;
            }
        }

        Ok(())
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
    pub name: String,
    pub path: String,
    pub mime: String,
    pub size: u64,
    pub group_id: String,
    pub group_member_name: String,
}

impl TryInto<file::ActiveModel> for InsertFile {
    type Error = String;

    fn try_into(self) -> Result<file::ActiveModel, Self::Error> {
        Ok(file::ActiveModel {
            id: Unset(None),
            name: Set(self.name),
            path: Set(self.path),
            mime: Set(self.mime),
            size: Set(self
                .size
                .try_into()
                .map_err(|_| "file size is too big: max is i64::MAX".to_string())?),
            group_id: Set(self.group_id),
            group_member_name: Set(self.group_member_name),
        })
    }
}
