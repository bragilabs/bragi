use std::path::PathBuf;
use axum::{extract::{Path, State}, Json};
use std::sync::Arc;
use axum::http::{header, StatusCode};
use axum::response::Response;
use serde::Serialize;
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use entities::{album, artist, track};
use service::artist::ArtistService;
use service::album::AlbumService;
use service::track::TrackService;
use scanner::Scanner;

#[derive(Clone)]
pub struct AppState {
    pub artist_service: Arc<ArtistService>,
    pub album_service: Arc<AlbumService>,
    pub track_service: Arc<TrackService>,
    pub scanner: Arc<Scanner>
}
#[derive(Serialize)]
pub struct TrackDTO {
    id: i32,
    title: String,
    duration: i32,
    track_number: i32,
}

impl From<track::Model> for TrackDTO {
    fn from(track: track::Model) -> Self {
        TrackDTO {
            id: track.id,
            title: track.title,
            duration: track.duration,
            track_number: track.track_number
        }
    }
}

#[derive(Serialize)]
pub struct AlbumDTO {
    id: i32,
    title: String,
    year: i32,
}

impl From<album::Model> for AlbumDTO {
    fn from(album: album::Model) -> Self {
        AlbumDTO {
            id: album.id,
            title: album.title,
            year: album.release_year
        }
    }
}

#[derive(Serialize)]
pub struct ArtistDTO {
    id: i32,
    name: String
}

impl From<artist::Model> for ArtistDTO {
    fn from(artist: artist::Model) -> Self {
        ArtistDTO {
            id: artist.id,
            name: artist.name
        }
    }
}

pub async fn get_all_artists(
    State(state): State<AppState>
) -> Json<Vec<ArtistDTO>> {
    let artists = state.artist_service.get_all().await.unwrap_or_default();
    let artists: Vec<ArtistDTO> = artists.into_iter().map(ArtistDTO::from).collect();
    Json(artists)
}

pub async fn get_artist_by_id(
    Path(artist_id): Path<i32>,
    State(state): State<AppState>
) -> Json<Option<ArtistDTO>> {
    let artist = state.artist_service.get_by_id(artist_id).await.unwrap();
    let artist = match artist {
        Some(artist) => ArtistDTO::from(artist),
        None => return Json(None),
    };
    Json(Some(artist))
}

pub async fn get_albums_by_artist(
    Path(artist_id): Path<i32>,
    State(state): State<AppState>
) -> Json<Vec<AlbumDTO>> {
    let albums = state.album_service.get_by_artist_id(artist_id).await.unwrap();
    let albums = match albums {
        Some(albums) => albums,
        None => return Json(vec![]),
    };
    let albums: Vec<AlbumDTO> = albums.into_iter().map(AlbumDTO::from).collect();
    Json(albums)
}

pub async fn get_tracks_by_album(
    Path(album_id): Path<i32>,
    State(state): State<AppState>
) -> Json<Vec<TrackDTO>> {
    let tracks = state.track_service.get_by_album_id(album_id).await.unwrap();
    let tracks = match tracks {
        Some(tracks) => tracks,
        None => return Json(vec![]),
    };
    let tracks: Vec<TrackDTO> = tracks.into_iter().map(TrackDTO::from).collect();
    Json(tracks)
}

pub async fn rescan_library(State(state): State<AppState>) -> StatusCode {

    let scanner = state.scanner.clone();
    tokio::spawn(async move {
        scanner.scan_library().await;
    });

    StatusCode::OK

}


pub async fn stream_track(
    Path(track_id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Response, StatusCode> {
    let track = state.track_service.get_by_id(track_id).await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let track = match track {
        Some(track) => track,
        None => return Err(StatusCode::NOT_FOUND),
    };

    let path: PathBuf = track.path.into();
    let file = File::open(&path).await.map_err(|_| StatusCode::NOT_FOUND)?;
    let stream = ReaderStream::new(file);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "audio/flac")
        .body(axum::body::Body::from_stream(stream))
        .unwrap())
}
