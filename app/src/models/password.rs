use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "password")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub user_id: Uuid,
    #[sea_orm(nullable)]
    pub website_url: Option<String>,
    #[sea_orm(nullable)]
    pub app_name: Option<String>,
    #[sea_orm(nullable)]
    pub encrypted_username: Option<String>,
    #[sea_orm(nullable)]
    pub encrypted_email: Option<String>,
    pub encrypted_password: String,
    #[sea_orm(nullable)]
    pub is_deleted: bool,

    #[sea_orm(created_at)]
    pub created_at: DateTime<Utc>,
    #[sea_orm(updated_at)]
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
