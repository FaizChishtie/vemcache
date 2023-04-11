pub enum Command {
    Ping,
    Insert(Vec<f32>),
    NamedInsert(String, Vec<f32>),
    Get(String),
    Remove(String),
    KNearestNeighbors(String, usize), // KNN with key and k value
    VectorAddition(String, String),   // VAdd with two keys
    VectorSubtraction(String, String),// VSub with two keys
    VectorScaling(String, f32),       // VScale with key and scalar
    CosineSimilarity(String, String), // VCosine with two keys
}

pub fn parse_command(input: &str) -> Result<Command, &str> {
    let tokens: Vec<&str> = input.split_whitespace().collect();

    if tokens.is_empty() {
        return Err("Empty command");
    }

    match tokens[0].to_lowercase().as_str() {
        "ping" => Ok(Command::Ping),
        "insert" => {
            if tokens.len() < 2 {
                return Err("Invalid INSERT command");
            }
            let values = tokens[1..]
                .iter()
                .filter_map(|s| s.parse::<f32>().ok())
                .collect();

            Ok(Command::Insert(values))
        }
        "named_insert" => {
            if tokens.len() < 3 {
                return Err("Invalid NAMED_INSERT command");
            }
            let key = tokens[1].to_string();
            let values = tokens[2..]
                .iter()
                .filter_map(|s| s.parse::<f32>().ok())
                .collect();

            Ok(Command::NamedInsert(key, values))
        }
        "get" => {
            if tokens.len() != 2 {
                return Err("Invalid GET command");
            }
            let key = tokens[1].to_string();
            Ok(Command::Get(key))
        }
        "remove" => {
            if tokens.len() != 2 {
                return Err("Invalid REMOVE command");
            }
            let key = tokens[1].to_string();
            Ok(Command::Remove(key))
        }
        "knn" => {
            let key = tokens.get(1).ok_or("Missing key")?.to_string();
            let k = tokens
                .get(2)
                .ok_or("Missing k")?
                .parse::<usize>()
                .map_err(|_| "Invalid k value")?;
            Ok(Command::KNearestNeighbors(key, k))
        }
        "vadd" => {
            let key1 = tokens.get(1).ok_or("Missing key1")?.to_string();
            let key2 = tokens.get(2).ok_or("Missing key2")?.to_string();
            Ok(Command::VectorAddition(key1, key2))
        }
        "vsub" => {
            let key1 = tokens.get(1).ok_or("Missing key1")?.to_string();
            let key2 = tokens.get(2).ok_or("Missing key2")?.to_string();
            Ok(Command::VectorSubtraction(key1, key2))
        }
        "vscale" => {
            let key = tokens.get(1).ok_or("Missing key")?.to_string();
            let scalar = tokens
                .get(2)
                .ok_or("Missing scalar")?
                .parse::<f32>()
                .map_err(|_| "Invalid scalar value")?;
            Ok(Command::VectorScaling(key, scalar))
        }
        "vcosine" => {
            let key1 = tokens.get(1).ok_or("Missing key1")?.to_string();
            let key2 = tokens.get(2).ok_or("Missing key2")?.to_string();
            Ok(Command::CosineSimilarity(key1, key2))
        }
        _ => Err("Unknown command"),
    }
}
