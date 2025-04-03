use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(Playlist::Table)
                    .if_not_exists()

                    .col(
                        pk_auto(Playlist::ID)
                            .unsigned()
                    )

                    .col(
                        string(Playlist::Name)
                            .string_len(255)
                            .not_null()
                    )

                    .col(
                        integer(Playlist::UserID)
                            .not_null()
                            .unsigned()
                    )

                    .col(
                        array(Playlist::Tracks, ColumnType::Integer)
                            .not_null()
                            .default("ARRAY[]::integer[]")
                    )

                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(Playlist::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Playlist {
    Table,
    ID,
    Name,
    UserID,
    Tracks,
}
