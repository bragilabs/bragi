use std::process::exit;
use std::sync::Arc;
use std::time::Duration;
use axum::routing::get;
use dotenvy::dotenv;
use scanner::{Scanner};
use sea_orm::{DatabaseConnection, Database, ConnectOptions};
use api::AppState;
use service::album::AlbumService;
use service::artist::ArtistService;
use service::track::TrackService;


async fn init_db() -> DatabaseConnection {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut opt = ConnectOptions::new(db_url);
    opt.acquire_timeout(Duration::from_secs(5));

    match Database::connect(opt).await {
        Ok(db) => {
            println!("Connected to database");
            db
        }
        Err(e) => {
            if e.to_string().contains("Connection Error") {
                println!("Error connecting to database: {}", e);
                println!("HINT: Check if the database is running, and that the connection string is correct.");
            } else {
                println!("Error: {}", e);
            }
            eprintln!("Failed to establish a connection to the database. Bailing...");
            exit(1)
        }
    }
}

#[tokio::main]
async fn main() {
    println!("Bragi is starting up!");
    let db = init_db();
    let db = Arc::new(db.await);
    
    let artist_service = Arc::new(ArtistService::new(db.clone()));
    let album_service = Arc::new(AlbumService::new(db.clone()));
    let track_service = Arc::new(TrackService::new(db.clone()));
    let scanner = Scanner::new(artist_service.clone(), album_service.clone(), track_service.clone());
    let library = std::env::var("LIBRARY_PATH").expect("LIBRARY_PATH must be set");
    scanner.scan_library(library.as_str()).await;
    println!("Done scanning library.");

    let state = AppState {
        artist_service: artist_service.clone(),
        album_service: album_service.clone(),
        track_service: track_service.clone(),
    };

    let app = axum::Router::new()
        .route("/api/artists", get(api::get_all_artists))
        .route("/api/artists/{id}/albums", get(api::get_albums_by_artist))
        .route("/api/albums/{id}/tracks", get(api::get_tracks_by_album))
        .with_state(state);

    println!("Bragi is ready to serve requests!");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
