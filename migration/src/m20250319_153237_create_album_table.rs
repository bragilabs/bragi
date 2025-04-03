use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(Album::Table)
                    .if_not_exists()
                    .col(
                        pk_auto(Album::ID)
                            .unsigned()
                    )

                    .col(
                        string(Album::Title)
                            .not_null()
                    )

                    .col(
                        string(Album::Path)
                            .not_null()
                    )

                    .col(
                        integer(Album::ArtistID)
                            .unsigned()
                    )

                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(Album::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Album {
    Table,
    ID,
    Title,
    Path,
    ArtistID
}
