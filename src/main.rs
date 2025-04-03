use dotenvy::dotenv;
use crate::db::manager::establish_connection;

mod routes;
mod db;

fn main() {
    dotenv().ok();
    establish_connection();
    println!("Hello, world!");
}
