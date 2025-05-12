use sea_orm_migration::{prelude::*, schema::*};
use crate::m20250319_153237_create_album_table::Album;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(Track::Table)
                    .if_not_exists()

                    .col(
                        pk_auto(Track::ID)
                    )

                    .col(
                        string(Track::Title)
                            .not_null()
                            .string_len(255)
                    )

                    .col(
                        string(Track::Path)
                            .not_null()
                    )
                    
                    .col(
                        integer(Track::TrackNumber)
                            .not_null()
                            .unsigned()
                    )
                    
                    .col(
                        integer(Track::Duration)
                            .not_null()
                            .unsigned()
                    )
                    
                    .col(
                        integer(Track::AlbumID)
                            .unsigned()
                            .not_null()
                    )
                    
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_Track_Album")
                            .from(Track::Table, Track::AlbumID)
                            .to(Album::Table, Album::ID)
                            .on_delete(ForeignKeyAction::Cascade)
                    )

                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(Track::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Track {
    Table,
    ID,
    Title,
    Path,
    TrackNumber,
    Duration,
    AlbumID
}
