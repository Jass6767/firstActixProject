use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Urls::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Urls::Id).integer().primary_key().not_null().auto_increment())
                    .col(ColumnDef::new(Urls::Url).string().not_null())
                    .col(ColumnDef::new(Urls::Shorten).string().not_null()).to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Urls::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Urls {
    Table,
    Id,
    Url,
    Shorten,
}
