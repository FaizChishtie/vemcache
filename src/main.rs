use std::net::SocketAddr;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

mod commands;
mod vemcache;

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

        // Process the command
        match commands::parse_command(&command) {
            Ok(commands::Command::Ping) => {
                if let Err(_) = writer.write_all(b"pong\n").await {
                    println!("Error sending response to client");
                    return;
                }
            }
            Ok(commands::Command::Insert(values)) => {
                db.insert_with_uuid(values);
                if let Err(_) = writer.write_all(b"OK\n").await {
                    println!("Error sending response to client");
                    return;
                }
            }
            Ok(commands::Command::NamedInsert(key, values)) => {
                db.insert_with_key(key, values);
                if let Err(_) = writer.write_all(b"OK\n").await {
                    println!("Error sending response to client");
                    return;
                }
            }
            Ok(commands::Command::Get(key)) => {
                if let Some(values) = db.get(key) {
                    let response = format!("{:?}\n", values);
                    if let Err(_) = writer.write_all(response.as_bytes()).await {
                        println!("Error sending response to client");
                        return;
                    }
                } else {
                    if let Err(_) = writer.write_all(b"null\n").await {
                        println!("Error sending response to client");
                        return;
                    }
                }
            }
            Ok(commands::Command::Remove(key)) => {
                db.remove(key);
                if let Err(_) = writer.write_all(b"OK\n").await {
                    println!("Error sending response to client");
                    return;
                }
            }
            Ok(commands::Command::KNearestNeighbors(key, k)) => {
                // Handle KNearestNeighbors command
                match db.get(key) {
                    Some(query_vector) => {
                        // Find the k nearest neighbors
                        let neighbors = db.k_nearest_neighbors(query_vector, k);
            
                        // Format the response
                        let response = neighbors
                            .into_iter()
                            .map(|(id, vector)| {
                                format!("ID: {}, Vector: {:?}", id, vector)
                            })
                            .collect::<Vec<String>>()
                            .join("\n");
            
                        // Send response with the nearest neighbors
                        if let Err(_) = writer.write_all(response.as_bytes()).await {
                            println!("Error sending response to client");
                            return;
                        }
                    }
                    None => {
                        // Key not found in the database
                        let response = "Key not found\n";
                        if let Err(_) = writer.write_all(response.as_bytes()).await {
                            println!("Error sending response to client");
                            return;
                        }
                    }
                }
            }            
            Ok(commands::Command::VectorAddition(key1, key2)) => {
                // Handle VectorAddition command
                match (db.get(key1), db.get(key2)) {
                    (Some(vector1), Some(vector2)) => {
                        // Perform vector addition
                        match db.vector_addition(&vector1, &vector2) {
                            Some(result) => {
                                // Format the response
                                let response = format!("Result: {:?}\n", result);
            
                                // Send response with the result of vector addition
                                if let Err(_) = writer.write_all(response.as_bytes()).await {
                                    println!("Error sending response to client");
                                    return;
                                }
                            }
                            None => {
                                // Vectors are not compatible for addition (e.g., different dimensions)
                                let response = "Vectors are not compatible for addition\n";
                                if let Err(_) = writer.write_all(response.as_bytes()).await {
                                    println!("Error sending response to client");
                                    return;
                                }
                            }
                        }
                    }
                    _ => {
                        // One or both keys not found in the database
                        let response = "One or both keys not found\n";
                        if let Err(_) = writer.write_all(response.as_bytes()).await {
                            println!("Error sending response to client");
                            return;
                        }
                    }
                }
            }            
            Ok(commands::Command::VectorSubtraction(key1, key2)) => {
                // Handle VectorSubtraction command
                match (db.get(key1), db.get(key2)) {
                    (Some(vector1), Some(vector2)) => {
                        // Perform vector subtraction
                        match db.vector_subtraction(&vector1, &vector2) {
                            Some(result) => {
                                // Format the response
                                let response = format!("Result: {:?}\n", result);
            
                                // Send response with the result of vector subtraction
                                if let Err(_) = writer.write_all(response.as_bytes()).await {
                                    println!("Error sending response to client");
                                    return;
                                }
                            }
                            None => {
                                // Vectors are not compatible for subtraction (e.g., different dimensions)
                                let response = "Vectors are not compatible for subtraction\n";
                                if let Err(_) = writer.write_all(response.as_bytes()).await {
                                    println!("Error sending response to client");
                                    return;
                                }
                            }
                        }
                    }
                    _ => {
                        // One or both keys not found in the database
                        let response = "One or both keys not found\n";
                        if let Err(_) = writer.write_all(response.as_bytes()).await {
                            println!("Error sending response to client");
                            return;
                        }
                    }
                }
            }            
            Ok(commands::Command::VectorScaling(key, scalar)) => {
                // Handle VectorScaling command
                match db.get(key) {
                    Some(vector) => {
                        // Perform vector scaling
                        let result = db.vector_scaling(&vector, scalar);
            
                        // Format the response
                        let response = format!("Result: {:?}\n", result);
            
                        // Send response with the result of vector scaling
                        if let Err(_) = writer.write_all(response.as_bytes()).await {
                            println!("Error sending response to client");
                            return;
                        }
                    }
                    None => {
                        // Key not found in the database
                        let response = "Key not found\n";
                        if let Err(_) = writer.write_all(response.as_bytes()).await {
                            println!("Error sending response to client");
                            return;
                        }
                    }
                }
            }            
            Ok(commands::Command::CosineSimilarity(key1, key2)) => {
                // Handle CosineSimilarity command
                match (db.get(key1), db.get(key2)) {
                    (Some(vector1), Some(vector2)) => {
                        // Calculate cosine similarity
                        match db.cosine_similarity(&vector1, &vector2) {
                            Some(similarity) => {
                                // Format the response
                                let response = format!("Cosine Similarity: {:.4}\n", similarity);
            
                                // Send response with the cosine similarity value
                                if let Err(_) = writer.write_all(response.as_bytes()).await {
                                    println!("Error sending response to client");
                                    return;
                                }
                            }
                            None => {
                                // Vectors are not compatible for cosine similarity (e.g., different dimensions)
                                let response = "Vectors are not compatible for cosine similarity\n";
                                if let Err(_) = writer.write_all(response.as_bytes()).await {
                                    println!("Error sending response to client");
                                    return;
                                }
                            }
                        }
                    }
                    _ => {
                        // One or both keys not found in the database
                        let response = "One or both keys not found\n";
                        if let Err(_) = writer.write_all(response.as_bytes()).await {
                            println!("Error sending response to client");
                            return;
                        }
                    }
                }
            }            
            Err(error_msg) => {
                let response = format!("Error: {}\n", error_msg);
                if let Err(_) = writer.write_all(response.as_bytes()).await {
                    println!("Error sending response to client");
                    return;
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 7070));
    let listener = TcpListener::bind(addr).await.unwrap();

    let mut db = Vemcache::new();

    println!("Vemcache listening on {}", addr);

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        handle_client(stream, &mut db).await;
    }
}