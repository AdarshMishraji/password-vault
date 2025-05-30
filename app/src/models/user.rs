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
    pub encrypted_dek: String,

    #[sea_orm(created_at)]
    pub created_at: DateTime<Utc>,
    #[sea_orm(updated_at)]
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::password::Entity")]
    Password,
    #[sea_orm(has_many = "super::recovery_code::Entity")]
    RecoveryCode,
}

impl Related<super::password::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Password.def()
    }
}

impl Related<super::recovery_code::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RecoveryCode.def()
    }
}
impl ActiveModelBehavior for ActiveModel {}
