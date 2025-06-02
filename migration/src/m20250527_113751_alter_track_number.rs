use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager.alter_table(
            Table::alter()
                .table(Track::Table)
                .modify_column(
                    ColumnDef::new(Track::TrackNumber)
                        .integer()
                        .null()
                ).to_owned(),
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.alter_table(
            Table::alter()
                .table(Track::Table)
                .modify_column(
                    ColumnDef::new(Track::TrackNumber)
                        .integer()
                        .not_null()
                ).to_owned(),
        ).await
    }
}

#[derive(DeriveIden)]
enum Track {
    Table,
    TrackNumber
}
