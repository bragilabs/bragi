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
pub async fn scan(path: &str, artist_service: Arc<ArtistService>) {
    println!("Scanning {}", path);
    println!("-------------------");
    for entry in WalkDir::new(path).min_depth(1).into_iter().filter_map(|e| e.ok()) {
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
                    artist_service.create(
                        file_name.to_string(),
                        entry.path().to_str().unwrap().to_string(),
                    ).await.unwrap();
                    println!("Updated artist in the database: {}", file_name);
                } else {
                    println!("Artist checksum matches, no update needed.");
                }
            } else {
                println!("Artist does not exist in the database: {}", file_name);

                let artist = ArtistCreate {
                    name: file_name.to_string(),
                    path: path.to_string(),
                    checksum: Some(artist_hash),
                }

                artist_service.create(
                    file_name.to_string(),
                    entry.path().to_str().unwrap().to_string(),
                ).await.unwrap();
                println!("Created new artist in the database: {}", file_name);
            }
            println!("Scanning directory: {}", entry.path().display());
            continue;
        }
    }
    println!("-------------------");
}
