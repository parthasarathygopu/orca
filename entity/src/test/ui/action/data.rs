//! SeaORM Entity. Generated by sea-orm-codegen 0.6.0

use sea_orm::entity::prelude::*;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Deserialize, Serialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "action_data_kind")]
pub enum ATargetKind {
    #[sea_orm(string_value = "Runtime")]
    Runtime,
    #[sea_orm(string_value = "Static")]
    Static,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "action_data")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub kind: ATargetKind,
    pub value: String,
    pub action_id: Uuid
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::action::Entity",
        from = "Column::ActionId",
        to = "super::action::Column::Id"
    )]
    Action,

}

// `Related` trait has to be implemented by hand
impl Related<super::action::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Action.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
