use dotenv::dotenv;
use std::net::SocketAddr;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::TcpListener;

mod commands;
mod handlers;
mod vemcache;

use handlers::*;
use vemcache::Vemcache;

async fn handle_client(mut stream: tokio::net::TcpStream, db: &mut Vemcache) {
    let (reader, mut writer) = stream.split();
    let mut reader = BufReader::new(reader);

    loop {
        let mut command = String::new();
        if let Err(_) = reader.read_line(&mut command).await {
            println!("Error reading from client");
            return;
        }
        command = command.trim().to_string();

        match commands::parse_command(&command) {
            Ok(commands::Command::Ping) => {
                handle_ping(&mut writer).await;
            }
            Ok(commands::Command::Insert(values)) => {
                handle_insert(db, values, &mut writer).await;
            }
            Ok(commands::Command::NamedInsert(key, values)) => {
                handle_named_insert(db, key, values, &mut writer).await;
            }
            Ok(commands::Command::Get(key)) => {
                handle_get(db, key, &mut writer).await;
            }
            Ok(commands::Command::Remove(key)) => {
                handle_remove(db, key, &mut writer).await;
            }
            Ok(commands::Command::KNearestNeighbors(key, k)) => {
                handle_k_nearest_neighbors(db, key, k, &mut writer).await;
            }
            Ok(commands::Command::VectorAddition(key1, key2)) => {
                handle_vector_addition(db, key1, key2, &mut writer).await;
            }
            Ok(commands::Command::VectorSubtraction(key1, key2)) => {
                handle_vector_subtraction(db, key1, key2, &mut writer).await;
            }
            Ok(commands::Command::VectorScaling(key, scalar)) => {
                handle_vector_scaling(db, key, scalar, &mut writer).await;
            }
            Ok(commands::Command::CosineSimilarity(key1, key2)) => {
                handle_cosine_similarity(db, key1, key2, &mut writer).await;
            }
            Ok(commands::Command::Dump(file_path)) => {
                handle_dump(db, file_path, &mut writer).await;
            }
            Err(error_msg) => {
                handle_error(error_msg, &mut writer).await;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    let host = dotenv::var("VEMCACHE_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = dotenv::var("VEMCACHE_PORT").unwrap_or_else(|_| "7070".to_string());

    let addr = format!("{}:{}", host, port).parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(addr).await.unwrap();

    let mut db = Vemcache::new();

    println!("Vemcache v{} listening on {}", VERSION, addr);

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        handle_client(stream, &mut db).await;
    }
}
