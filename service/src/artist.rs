use std::sync::Arc;
use sea_orm::*;
use entities::artist::{ActiveModel, Entity as Artist};

pub struct ArtistService {
    db: Arc<DatabaseConnection>,
}

pub struct ArtistCreate {
    pub name: String,
    pub path: String,
    pub checksum: Option<String>
}

pub struct ArtistAlter {
    pub name: Option<String>,
    pub path: Option<String>,
    pub checksum: Option<String>
}

impl ArtistService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        ArtistService { db }
    }
    pub async fn get_all(&self) -> Result<Vec<entities::artist::Model>, DbErr> {
        Artist::find().all(self.db.as_ref()).await
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Option<entities::artist::Model>, DbErr> {
        Artist::find_by_id(id).one(self.db.as_ref()).await
    }

    pub async fn create(&self, create_body: ArtistCreate) -> Result<entities::artist::Model, DbErr> {
        let artist = ActiveModel {
            id: NotSet,
            name: Set(create_body.name),
            path: Set(create_body.path),
            checksum: Set(create_body.checksum),
        };
        let artist = artist.insert(self.db.as_ref()).await?;
        Ok(artist)
    }

    pub async fn get_by_name(&self, name: &str) -> Result<Option<entities::artist::Model>, DbErr> {
        Artist::find()
            .filter(entities::artist::Column::Name.contains(name))
            .one(self.db.as_ref()).await
    }

    pub async fn exists(&self, name: &str) -> Result<bool, DbErr> {
        let count = Artist::find()
            .filter(entities::artist::Column::Name.contains(name))
            .count(self.db.as_ref()).await?;
        Ok(count > 0)
    }

    pub async fn alter(&self, id: i32, alter_body: ArtistAlter) -> Result<entities::artist::Model, DbErr> {
        let mut artist: ActiveModel = self.get_by_id(id).await?.unwrap().into();

        if let Some(name) = alter_body.name {
            artist.name = Set(name);
        }
        if let Some(path) = alter_body.path {
            artist.path = Set(path);
        }
        if let Some(checksum) = alter_body.checksum {
            artist.checksum = Set(Some(checksum));
        }

        let artist = artist.update(self.db.as_ref()).await?;
        Ok(artist)
    }
}