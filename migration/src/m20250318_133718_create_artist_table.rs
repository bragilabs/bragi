use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Artist::Table)
                    .if_not_exists()
                    .col(
                        pk_auto(Artist::ID)
                            .unsigned()
                    )

                    .col(
                        string(Artist::Name)
                            .not_null()
                    )

                    .col(
                        string(Artist::Path)
                            .not_null()
                    )

                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(Artist::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Artist {
    Table,
    ID,
    Name,
    Path
}