use crate::Vemcache;
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::WriteHalf;

pub async fn handle_ping(writer: &mut WriteHalf<'_>) {
    if let Err(_) = writer.write_all(b"pong\n").await {
        println!("Error sending response to client");
    }
}

pub async fn handle_insert(db: &mut Vemcache, values: Vec<f32>, writer: &mut WriteHalf<'_>) {
    db.insert_with_uuid(values);
    if let Err(_) = writer.write_all(b"OK\n").await {
        println!("Error sending response to client");
    }
}

pub async fn handle_named_insert(
    db: &mut Vemcache,
    key: String,
    values: Vec<f32>,
    writer: &mut WriteHalf<'_>,
) {
    db.insert_with_key(key, values);
    if let Err(_) = writer.write_all(b"OK\n").await {
        println!("Error sending response to client");
    }
}

pub async fn handle_get(db: &mut Vemcache, key: String, writer: &mut WriteHalf<'_>) {
    if let Some(values) = db.get(key) {
        let response = format!("{:?}\n", values);
        if let Err(_) = writer.write_all(response.as_bytes()).await {
            println!("Error sending response to client");
        }
    } else {
        if let Err(_) = writer.write_all(b"null\n").await {
            println!("Error sending response to client");
        }
    }
}

pub async fn handle_remove(db: &mut Vemcache, key: String, writer: &mut WriteHalf<'_>) {
    db.remove(key);
    if let Err(_) = writer.write_all(b"OK\n").await {
        println!("Error sending response to client");
    }
}

pub async fn handle_k_nearest_neighbors(
    db: &mut Vemcache,
    key: String,
    k: usize,
    writer: &mut WriteHalf<'_>,
) {
    match db.get(key) {
        Some(query_vector) => {
            let neighbors = db.k_nearest_neighbors(query_vector, k);
            let response = neighbors
                .into_iter()
                .map(|(id, vector)| format!("ID: {}, Vector: {:?}", id, vector))
                .collect::<Vec<String>>()
                .join("\n");
            if let Err(_) = writer.write_all(response.as_bytes()).await {
                println!("Error sending response to client");
            }
        }
        None => {
            let response = "Key not found\n";
            if let Err(_) = writer.write_all(response.as_bytes()).await {
                println!("Error sending response to client");
            }
        }
    }
}

pub async fn handle_vector_addition(
    db: &mut Vemcache,
    key1: String,
    key2: String,
    writer: &mut WriteHalf<'_>,
) {
    match (db.get(key1.clone()), db.get(key2.clone())) {
        (Some(_vector1), Some(_vector2)) => match db.vector_addition(&key1, &key2) {
            Some(result) => {
                let response = format!("Result: {:?}\n", result);
                if let Err(_) = writer.write_all(response.as_bytes()).await {
                    println!("Error sending response to client");
                }
            }
            None => {
                let response = "Vectors are not compatible for addition\n";
                if let Err(_) = writer.write_all(response.as_bytes()).await {
                    println!("Error sending response to client");
                }
            }
        },
        _ => {
            let response = "One or both keys not found\n";
            if let Err(_) = writer.write_all(response.as_bytes()).await {
                println!("Error sending response to client");
            }
        }
    }
}

pub async fn handle_vector_subtraction(
    db: &mut Vemcache,
    key1: String,
    key2: String,
    writer: &mut WriteHalf<'_>,
) {
    match (db.get(key1.clone()), db.get(key2.clone())) {
        (Some(_vector1), Some(_vector2)) => match db.vector_subtraction(&key1, &key2) {
            Some(result) => {
                let response = format!("Result: {:?}\n", result);
                if let Err(_) = writer.write_all(response.as_bytes()).await {
                    println!("Error sending response to client");
                }
            }
            None => {
                let response = "Vectors are not compatible for subtraction\n";
                if let Err(_) = writer.write_all(response.as_bytes()).await {
                    println!("Error sending response to client");
                }
            }
        },
        _ => {
            let response = "One or both keys not found\n";
            if let Err(_) = writer.write_all(response.as_bytes()).await {
                println!("Error sending response to client");
            }
        }
    }
}

pub async fn handle_vector_scaling(
    db: &mut Vemcache,
    key: String,
    scalar: f32,
    writer: &mut WriteHalf<'_>,
) {
    match db.get(key.clone()) {
        Some(_vector) => {
            // Perform vector scaling using the retrieved key and the provided scalar
            match db.vector_scaling(&key, scalar) {
                Some(result) => {
                    // Format and send the result to the client
                    let response = format!("Result: {:?}\n", result);
                    if let Err(_) = writer.write_all(response.as_bytes()).await {
                        println!("Error sending response to client");
                    }
                }
                None => {
                    // Handle the case where vector scaling failed (e.g., due to invalid scalar)
                    let response = format!("Error: Vector scaling failed\n");
                    if let Err(_) = writer.write_all(response.as_bytes()).await {
                        println!("Error sending response to client");
                    }
                }
            }
        }
        None => {
            let response = "Key not found\n";
            if let Err(_) = writer.write_all(response.as_bytes()).await {
                println!("Error sending response to client");
            }
        }
    }
}

pub async fn handle_cosine_similarity(
    db: &mut Vemcache,
    key1: String,
    key2: String,
    writer: &mut WriteHalf<'_>,
) {
    match (db.get(key1.clone()), db.get(key2.clone())) {
        (Some(vector1), Some(vector2)) => match db.cosine_similarity(&vector1, &vector2) {
            Some(similarity) => {
                let response = format!("Cosine Similarity: {:.4}\n", similarity);
                if let Err(_) = writer.write_all(response.as_bytes()).await {
                    println!("Error sending response to client");
                }
            }
            None => {
                let response = "Vectors are not compatible for cosine similarity\n";
                if let Err(_) = writer.write_all(response.as_bytes()).await {
                    println!("Error sending response to client");
                }
            }
        },
        _ => {
            let response = "One or both keys not found\n";
            if let Err(_) = writer.write_all(response.as_bytes()).await {
                println!("Error sending response to client");
            }
        }
    }
}

pub async fn handle_error(error_msg: &str, writer: &mut WriteHalf<'_>) {
    let response = format!("Error: {}\n", error_msg);
    if let Err(_) = writer.write_all(response.as_bytes()).await {
        println!("Error sending response to client");
    }
}

pub async fn handle_dump(db: &mut Vemcache, file_path: String, writer: &mut WriteHalf<'_>) {
    match db.dump(&file_path) {
        Ok(_) => {
            let response = format!("Database dump successful: {}\n", file_path);
            if let Err(_) = writer.write_all(response.as_bytes()).await {
                println!("Error sending response to client");
            }
        }
        Err(err) => {
            let response = format!("Error creating database dump: {}\n", err);
            if let Err(_) = writer.write_all(response.as_bytes()).await {
                println!("Error sending response to client");
            }
        }
    }
}
