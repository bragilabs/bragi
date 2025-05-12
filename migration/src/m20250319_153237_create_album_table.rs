use sea_orm_migration::{prelude::*, schema::*};
use crate::m20250318_133718_create_artist_table::Artist;

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
                        integer(Album::ReleaseYear)
                            .not_null()
                            .unsigned()
                    )
                    
                    .col(
                        integer(Album::ArtistID)
                            .unsigned()
                    )
                    
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_Album_Artist")
                            .from(Album::Table, Album::ArtistID)
                            .to(Artist::Table, Artist::ID)
                            .on_delete(ForeignKeyAction::Cascade)
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
pub enum Album {
    Table,
    ID,
    Title,
    Path,
    ReleaseYear,
    ArtistID
}
