use sea_orm::{entity::prelude::*, ConnectionTrait, DbConn, DbErr, ExecResult, Schema};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "files")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i64,
    pub name: String,
    pub path: String,
    pub mime: String,
    pub size: i64,
    pub group_id: String,
    pub group_member_name: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub(super) async fn setup(db: &DbConn) -> Result<ExecResult, DbErr> {
    let statement = Schema::create_table_from_entity(Entity)
        .if_not_exists()
        .to_owned();
    let builder = db.get_database_backend();
    db.execute(builder.build(&statement)).await
}

// helper because u64 is not supported by sqlite
#[derive(Serialize)]
pub struct File {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub mime: String,
    pub size: u64,
    pub group_id: String,
    pub group_member_name: String,
}

impl TryFrom<File> for Model {
    type Error = String;

    fn try_from(value: File) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value
                .id
                .try_into()
                .map_err(|_| "file.id should not be negative or too big".to_string())?,
            mime: value.mime,
            name: value.name,
            path: value.path,
            size: value
                .size
                .try_into()
                .map_err(|_| "file.size should not be negative or too big".to_string())?,
            group_id: value.group_id,
            group_member_name: value.group_member_name,
        })
    }
}

impl From<Model> for File {
    fn from(value: Model) -> Self {
        File {
            id: value.id.try_into().expect("should never be negative"),
            name: value.name,
            path: value.path,
            mime: value.mime,
            size: value.size.try_into().expect("should never be negative"),
            group_id: value.group_id.try_into().expect("should never be negative"),
            group_member_name: value.group_member_name,
        }
    }
}
