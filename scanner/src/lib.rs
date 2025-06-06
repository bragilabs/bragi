use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use walkdir::{DirEntry, WalkDir};
use service::artist::{ArtistService, ArtistAlter, ArtistCreate};
use service::album::{AlbumCreate, AlbumService};
use regex::Regex;
use service::track::{TrackCreate, TrackService};
use once_cell::sync::Lazy;

struct TrackInfo {
    title: String,
    track_number: i32,
}

pub struct Scanner {
    artist_service: Arc<ArtistService>,
    album_service: Arc<AlbumService>,
    track_service: Arc<TrackService>,
    library_path: String,
}

impl Scanner {

    pub fn new(artist_service: Arc<ArtistService>, album_service: Arc<AlbumService>, track_service: Arc<TrackService>, library_path: String) -> Self {
        Scanner { artist_service, album_service, track_service, library_path }
    }
    pub async fn scan_library(&self) {
        println!("Scanning {}", self.library_path);
        println!("-------------------");
        let all_artists = self.artist_service.get_all().await.unwrap();
        let artist_map: HashMap<String, (i32, Option<String>)> = all_artists
            .into_iter()
            .map(|artist| (artist.name.clone(), (artist.id.clone(), artist.checksum.clone())))
            .collect();
        for entry in WalkDir::new(self.library_path.clone()).min_depth(1).max_depth(1).into_iter().filter_map(|e| e.ok()) {
            if is_hidden(&entry) {
                continue;
            }
            if entry.file_type().is_dir() {
                let artist_name = entry.file_name().to_str().unwrap();
                println!("Found Artist directory: {}", entry.path().display());
                let current_hash = hash_artist_folder(entry.path().to_str().unwrap());
                if let Some((artist_id, stored_hash)) = artist_map.get(&artist_name.to_string()) {
                    println!("Artist already exists in the database: {}", artist_name);
                    if stored_hash.is_none() {
                        println!("Artist checksum not found, updating...");
                        self.artist_service.alter(artist_id.clone(), ArtistAlter {
                            checksum: Some(current_hash.clone()),
                            name: None,
                            path: None
                        }).await.expect("TODO: panic message");
                        Box::pin(self.scan_artist(entry.path(), artist_id.clone())).await;
                        continue

                    }
                    if stored_hash.clone().unwrap() != current_hash {
                        println!("Artist checksum does not match, updating...");
                        self.artist_service.alter(artist_id.clone(), ArtistAlter {
                            checksum: Some(current_hash.clone()),
                            name: None,
                            path: None
                        }).await.expect("TODO: panic message");
                        println!("Updated artist checksum in the database: {}", current_hash);
                        Box::pin(self.scan_artist(entry.path(), artist_id.clone())).await;
                    } else {
                        println!("Artist checksum matches, no update needed.");
                        continue;
                    }
                } else {
                    println!("Artist does not exist in the database: {}", artist_name);

                    let artist = ArtistCreate {
                        name: artist_name.to_string(),
                        path: self.library_path.to_string(),
                        checksum: Some(current_hash.clone()),
                    };

                    let artist = self.artist_service.create(artist).await.unwrap();
                    println!("Created new artist in the database: {}", artist.name);
                    Box::pin(self.scan_artist(entry.path(), artist.id)).await;
                }
            }
        }
        println!("-------------------");
    }

    /// Scans the artist directory for albums
    async fn scan_artist(&self, path: &Path, artist_id: i32) {
        let artist_albums = self.album_service.get_all().await.unwrap();
        let albums_map: HashMap<String, i32> = artist_albums
            .into_iter()
            .map(|album| (album.title.clone(), album.id.clone()))
            .collect();
        for entry in WalkDir::new(path).min_depth(1).max_depth(1).into_iter().filter_map(|e| e.ok()) {
            if is_hidden(&entry) {
                continue;
            }
            if entry.file_type().is_dir() {
                let release_year = extract_release_year(&entry);
                let album_name = extract_album_name(&entry);
                if release_year.is_none() || album_name.is_none() {
                    println!("Unable to parse album: {}", entry.path().display());
                    println!("Check the readme.md for the naming convention");
                    continue;
                }
                println!("Found album: {}, which came out in {}", album_name.clone().unwrap(), release_year.clone().unwrap());
                if let Some(album_id) = albums_map.get(&album_name.clone().unwrap()) {
                    println!("Album already exists in database, scanning for new tracks...");
                    Box::pin(self.scan_album(entry.path(), artist_id, album_id.clone())).await;
                } else {
                    println!("Album does not exist in the database, adding it and scanning for new tracks...");
                    let album = AlbumCreate {
                        title: album_name.clone().unwrap(),
                        path: path.to_str().unwrap().to_string(),
                        release_year: release_year.unwrap().parse().unwrap(),
                        artist_id,
                    };
                    let album = self.album_service.create(album).await.unwrap();
                    println!("Added album: {}, to the database", album.title);
                    Box::pin(self.scan_album(entry.path(), artist_id, album.id)).await;
                }
            }
        }
    }

    /// Scans the album folder for tracks
    /// Check track durations if it exists in db, since users may change files with the same filename.
    async fn scan_album(&self, path: &Path, artist_id: i32, album_id: i32) {
        let album = self.album_service.get_by_id(album_id).await.unwrap().unwrap();
        let album_tracks = self.track_service.get_all_by_album(album).await.unwrap();
        let stored_tracks: Vec<String> = album_tracks.iter().map(|e| e.title.clone()).collect();
        for entry in WalkDir::new(path).min_depth(1).max_depth(1).into_iter().filter_map(|e| e.ok()) {
            if is_hidden(&entry) {
                continue;
            }
            if entry.file_type().is_file() && is_audio_file(&entry.path()) {
                let track_data = extract_track_info(&entry).unwrap();
                println!("Found Track: {}", entry.path().display());
                if stored_tracks.contains(&track_data.title.to_string()) {
                    println!("Track already exists in the database");
                } else {
                    println!("Track does not exist in database");
                    let track = TrackCreate {
                        title: track_data.title,
                        duration: 1337,
                        album_id,
                        path: entry.path().to_str().unwrap().to_string(),
                        track_number: track_data.track_number,
                    };

                    let track = self.track_service.create(track).await.unwrap();
                    println!("Created new track in database: {}", track.title);
                }
            }
        }
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn extract_release_year(entry: &DirEntry) -> Option<String> {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\((\d{4})\)").unwrap());
    let Some(result) = RE.captures(entry.file_name().to_str().unwrap()) else {
        return None
    };

    result.get(1).map(|x| x.as_str().to_string())
}

fn extract_album_name(entry: &DirEntry) -> Option<String> {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(.*)\s\(\d{4}\)").unwrap());
    let Some(result) = RE.captures(entry.file_name().to_str().unwrap()) else {
        return None
    };

    result.get(1).map(|x| x.as_str().to_string())
}

// TODO: Refactor function to NOT return None if it fails to parse unneeded data, causing the program to panic
fn extract_track_info(entry: &DirEntry) -> Option<TrackInfo> {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(?P<track_number>\d+)\s*[.-]\s*(?:(?P<artist>.+?)\s*-\s*)?(?P<title>.+)").unwrap());

    let file_stem = get_filename_stem(entry.path())?;
    
    let Some(result) = RE.captures(&file_stem) else {
        println!("Unable to parse track info from track: {}", file_stem);
        return None
    };

    let Some(track_number) = result.name("track_number").map(|x| x.as_str().to_string().parse::<i32>().unwrap()) else {
        println!("Unable to parse track number, defaulting to null");
        return None
    };
    let Some(track_title) = result.name("title").map(|x| x.as_str().to_string()) else {
        println!("Unable to parse track title");
        return None
    };

    Some(TrackInfo {
        title: track_title.to_string(),
        track_number
    })
}

fn is_audio_file(path: &Path) -> bool {
    const AUDIO_EXTENSIONS: [&str; 13] = ["mp3", "flac", "wav", "ogg", "m4a", "aac", "alac", "aiff", "dsd", "opus", "wma", "amr", "ape", ];
    let file_ext: &str = path.extension().and_then(|s| s.to_str()).unwrap_or("");
    AUDIO_EXTENSIONS.iter().any(|&s| s == file_ext)
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

fn get_filename_stem(path: &Path) -> Option<String> {
    path.file_stem()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
}