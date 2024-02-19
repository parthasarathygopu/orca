// //! SeaORM Entity. Generated by sea-orm-codegen 0.6.0
//
// use sea_orm::entity::prelude::*;
// use sea_orm::EntityTrait;
// use serde::{Deserialize, Serialize};
//
//
// #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
// #[sea_orm(table_name = "object_repository")]
// pub struct Model {
//     #[serde(skip_deserializing)]
//     #[sea_orm(primary_key)]
//     pub id: i32,
//     pub path: String,
//     pub name: String,
//     pub endpoint: String,
//     pub endpoint_name: String,
//     pub description: String,
// }
//
// #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
// pub enum Relation {
//     #[sea_orm(has_many = "super::action::Entity")]
//     Action,
//     #[sea_orm(
//     belongs_to = "crate::app::app::Entity",
//     from = "Column::AppId",
//     to = "crate::app::app::Column::Id"
//     )]
//     App,
// }
//
// // `Related` trait has to be implemented by hand
// impl Related<super::action::Entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::Action.def()
//     }
// }
//
// impl Related<crate::app::app::Entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::App.def()
//     }
// }
//
// impl ActiveModelBehavior for ActiveModel {}