use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name="urls")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub id: i32,
    pub url: String,
    pub shorten: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("Not implemented")
    }
}

impl ActiveModelBehavior for ActiveModel {}

