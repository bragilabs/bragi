pub use sea_orm_migration::prelude::*;

mod m20250318_133718_create_artist_table;
mod m20250319_153237_create_album_table;
mod m20250319_153542_create_user_table;
mod m20250319_153613_create_playlist_table;
mod m20250320_162211_create_track_table;
mod m20250510_091903_add_artist_checksum;
mod m20250527_113751_alter_track_number;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250318_133718_create_artist_table::Migration),
            Box::new(m20250319_153237_create_album_table::Migration),
            Box::new(m20250319_153542_create_user_table::Migration),
            Box::new(m20250319_153613_create_playlist_table::Migration),
            Box::new(m20250320_162211_create_track_table::Migration),
            Box::new(m20250510_091903_add_artist_checksum::Migration),
            Box::new(m20250527_113751_alter_track_number::Migration),
        ]
    }
}
