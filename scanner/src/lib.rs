use std::sync::Arc;
use sea_orm::Iden;
use walkdir::{DirEntry, WalkDir};
use service::artist::{ArtistService, ArtistAlter, ArtistCreate};

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn hash_artist_folder(path: &str) -> String {
    let mut hasher = blake3::Hasher::new();
    
    for entry in WalkDir::new(path).min_depth(1).into_iter().filter_map(|e| e.ok()) {
        if is_hidden(&entry) {
            continue;
        }
        if entry.file_type().is_file() {
            let file_name = entry.file_name().to_str().unwrap();
            hasher.update(file_name.as_bytes());
        }
    }
    
    hasher.finalize().to_hex().to_string()
}

/// Scans the library directory for artists
pub async fn scan_library(path: &str, artist_service: Arc<ArtistService>) {
    println!("Scanning {}", path);
    println!("-------------------");
    for entry in WalkDir::new(path).min_depth(1).max_depth(1).into_iter().filter_map(|e| e.ok()) {
        if is_hidden(&entry) {
            continue;
        }
        if entry.file_type().is_dir() {
            let file_name = entry.file_name().to_str().unwrap();
            let file_path = entry.path().to_str().unwrap();
            println!("Found Artist directory: {}", entry.path().display());
            let artist_hash = hash_artist_folder(entry.path().to_str().unwrap());
            if artist_service.exists(file_name).await.unwrap() {
                println!("Artist already exists in the database: {}", file_name);
                let artist = artist_service.get_by_name(file_name).await.unwrap().unwrap();
                if artist.checksum.is_none() {
                    println!("Artist checksum not found, updating...");

                }
                if artist.checksum.unwrap().to_string() != artist_hash {
                    println!("Artist checksum does not match, updating...");
                    artist_service.alter(artist.id, ArtistAlter {
                        checksum: Some(artist_hash.clone()),
                        name: None,
                        path: None
                    }).await.expect("TODO: panic message");
                    println!("Updated artist checksum in the database: {}", artist_hash);
                } else {
                    println!("Artist checksum matches, no update needed.");
                }
            } else {
                println!("Artist does not exist in the database: {}", file_name);

                let artist = ArtistCreate {
                    name: file_name.to_string(),
                    path: path.to_string(),
                    checksum: Some(artist_hash),
                };

                artist_service.create(artist).await.unwrap();
                println!("Created new artist in the database: {}", file_name);
            }
            println!("Scanning directory: {}", entry.path().display());
            continue;
        }
    }
    println!("-------------------");
}

/// Scans the artist directory for albums
async fn scan_artist(path: &str) {
    for entry in WalkDir::new(path).min_depth(1).into_iter().filter_map(|e| e.ok()) {
        if is_hidden(&entry) {
            continue;
        }
        if entry.file_type().is_dir() {

        }
    }
}

/// Scans the album folder for tracks
async fn scan_albums(path: &str) {
    for entry in WalkDir::new(path).min_depth(1).max_depth(1).into_iter().filter_map(|e| e.ok()) {

    }
}