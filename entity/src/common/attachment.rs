//! SeaORM Entity. Generated by sea-orm-codegen 0.6.0

use sea_orm::entity::prelude::*;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Deserialize, Serialize)]
#[sea_orm(rs_type = "String", db_type = "String(Some(5))", enum_name = "storage_type")]
pub enum StorageType {
    #[sea_orm(string_value = "IB")]
    InBuild,
    #[sea_orm(string_value = "s3")]
    S3,
    #[sea_orm(string_value = "GCS")]
    GCS,
}

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Deserialize, Serialize)]
#[sea_orm(rs_type = "String", db_type = "String(Some(5))", enum_name = "storage_category")]
pub enum StorageCategory {
    #[sea_orm(string_value = "Evidence")]
    Evidence,
    #[sea_orm(string_value = "MockData")]
    MockData,
    #[sea_orm(string_value = "GCS")]
    GCS,
}


#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "attachment")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub category: StorageCategory,
    pub reference_id: Option<Uuid>,
    pub path: String,
    pub name: String,
    pub desc: Option<String>,
    pub attachment: Option<Vec<u8>>
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}


impl ActiveModelBehavior for ActiveModel {}
