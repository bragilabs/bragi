use std::sync::Arc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, ModelTrait, NotSet, Set};
use entities::album::Model as AlbumModel;
use entities::track::*;

pub struct TrackService {
    db: Arc<DatabaseConnection>
}

pub struct TrackCreate {
    pub title: String,
    pub path: String,
    pub track_number: i32,
    pub duration: i32,
    pub album_id: i32
}

impl TrackService {
    
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
    
    pub async fn get_all(&self) -> Result<Vec<Model>, DbErr> {
        Entity::find().all(self.db.as_ref()).await
    }

    pub async fn get_all_by_album(&self, album: AlbumModel) -> Result<Vec<Model>, DbErr> {
        album.find_related(Entity).all(self.db.as_ref()).await
    }
    
    pub async fn create(&self, create_body: TrackCreate) -> Result<Model, DbErr> {
        let track = ActiveModel {
            id: NotSet,
            title: Set(create_body.title),
            path: Set(create_body.path),
            track_number: Set(create_body.track_number),
            duration: Set(create_body.duration),
            album_id: Set(create_body.album_id)
        };
        
        let track = track.insert(self.db.as_ref()).await?;
        
        Ok(track)
    }
}