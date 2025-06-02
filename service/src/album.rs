use std::sync::Arc;
use sea_orm::*;
use entities::album::*;
use entities::album::Column::{Title, ArtistId};
use entities::prelude::Album;

pub struct AlbumService {
    db: Arc<DatabaseConnection>
}

pub struct AlbumCreate {
    pub title: String,
    pub path: String,
    pub release_year: i32,
    pub artist_id: i32
}

impl AlbumService {
    
    pub fn new(db: Arc<DatabaseConnection>) -> Self { AlbumService { db } }
    
    pub async fn create(&self, create_body: AlbumCreate) -> Result<Model, DbErr> {
        let album = ActiveModel {
            id: NotSet,
            title: Set(create_body.title),
            path: Set(create_body.path),
            release_year: Set(create_body.release_year),
            artist_id: Set(create_body.artist_id)
        };
        
        let album = album.insert(self.db.as_ref()).await?;
        Ok(album)
    }

    pub async fn exists(&self, title: &str) -> Result<bool, DbErr> {
        let count = Entity::find()
            .filter(Title.contains(title))
            .count(self.db.as_ref()).await?;
        Ok(count > 0)
    }
    
    pub async fn get_all(&self) -> Result<Vec<Model>, DbErr> {
        Entity::find().all(self.db.as_ref()).await
    }
    
    pub async fn get_by_id(&self, album_id: i32) -> Result<Option<Model>, DbErr> {
        Entity::find_by_id(album_id).one(self.db.as_ref()).await
    }
    
    pub async fn get_by_artist_id(&self, artist_id: i32) -> Result<Option<Vec<Model>>, DbErr> {
        Entity::find()
            .filter(ArtistId.eq(artist_id))
            .all(self.db.as_ref())
            .await
            .map(|albums| if albums.is_empty() { None } else { Some(albums) })
    }
}