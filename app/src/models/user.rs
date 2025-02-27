use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub email: String,
    pub master_password_hash: String,
    pub master_password_salt: String,
    pub encrypted_dek: String,
    pub salt: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::password::Entity")]
    Password,
    #[sea_orm(has_many = "super::session::Entity")]
    Session,
    #[sea_orm(has_many = "super::recovery_code::Entity")]
    RecoveryCode,
}

impl Related<super::password::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Password.def()
    }
}

impl Related<super::session::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Session.def()
    }
}

impl Related<super::recovery_code::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RecoveryCode.def()
    }
}
impl ActiveModelBehavior for ActiveModel {}
