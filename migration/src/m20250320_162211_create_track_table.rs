use std::convert::identity;
use std::ops::DerefMut;
use std::os::raw::c_uint;
use sea_orm_migration::{prelude::*, schema::*};

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
                            .unsigned()
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
                        integer(Track::AlbumID)
                            .unsigned()
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
    AlbumID
}
