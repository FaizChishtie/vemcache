/// Represents the various commands that can be executed by the Vemcache server.
/// Each variant corresponds to a specific command and its associated parameters.
pub enum Command {
    /// The `Ping` command is used to check the connection to the Vemcache server.
    /// The server responds with "pong" when it receives this command.
    Ping,
    /// The `Insert` command is used to insert a vector into the database.
    /// The server generates a unique identifier (UUID) for the vector.
    /// Parameters: Vector to be inserted.
    Insert(Vec<f32>),
    /// The `NamedInsert` command is used to insert a vector into the database with a specified key.
    /// Parameters: Key (String) and Vector to be inserted.
    NamedInsert(String, Vec<f32>),
    /// The `Get` command is used to retrieve a vector from the database using its key.
    /// Parameters: Key (String) of the vector to be retrieved.
    Get(String),
    /// The `Remove` command is used to remove a vector from the database using its key.
    /// Parameters: Key (String) of the vector to be removed.
    Remove(String),
    /// The `KNearestNeighbors` command is used to find the k nearest neighbors of a vector.
    /// Parameters: Key (String) of the query vector and k value (usize) specifying the number of neighbors.
    KNearestNeighbors(String, usize),
    /// The `VectorAddition` command is used to perform element-wise addition of two vectors.
    /// Parameters: Keys (Strings) of the two vectors to be added.
    VectorAddition(String, String),
    /// The `VectorSubtraction` command is used to perform element-wise subtraction of two vectors.
    /// Parameters: Keys (Strings) of the two vectors to be subtracted.
    VectorSubtraction(String, String),
    /// The `VectorScaling` command is used to scale a vector by a scalar value.
    /// Parameters: Key (String) of the vector to be scaled and the scalar value (f32).
    VectorScaling(String, f32),
    /// The `CosineSimilarity` command is used to calculate the cosine similarity between two vectors.
    /// Parameters: Keys (Strings) of the two vectors to be compared.
    CosineSimilarity(String, String),
    /// The `Dump` command is used to create a JSON dump of the database.
    /// The server responds with a success or error message based on the result.
    Dump(String),
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
        "dump" => {
            if tokens.len() != 2 {
                return Err("Invalid DUMP command");
            }
            let file_path = tokens[1].to_string();
            Ok(Command::Dump(file_path))
        }
        _ => Err("Unknown command"),
    }
}
