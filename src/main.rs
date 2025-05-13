use std::process::exit;
use std::sync::Arc;
use std::time::Duration;
use dotenvy::dotenv;
use scanner::scan_library;
use sea_orm::{DatabaseConnection, Database, ConnectOptions};
use service::artist::ArtistService;

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
    
    scan_library("/home/aron/Music/", artist_service.clone()).await;
    println!("Done.");
}
