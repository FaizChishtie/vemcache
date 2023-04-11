use std::collections::HashMap;
use uuid::Uuid;

type VectorId = String;
type Vector = Vec<f32>;

use serde::Serialize;
use std::fs::File;
use std::io::Result as IoResult;

#[derive(Serialize)]
pub struct Vemcache {
    storage: HashMap<VectorId, Vector>,
}

impl Vemcache {
    /// Creates a new instance of the Vemcache database.
    ///
    /// The Vemcache database is an in-memory vector database that allows
    /// storing and retrieving vectors by keys. It also provides various
    /// vector operations such as vector addition, subtraction, scaling,
    /// and similarity calculations.
    ///
    /// # Example
    ///
    /// ```
    /// use vemcache::Vemcache;
    ///
    /// // Create a new Vemcache database instance
    /// let db = Vemcache::new();
    /// ```
    ///
    /// # Returns
    ///
    /// A new instance of the Vemcache database with an empty storage.
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    /// Inserts a vector into the Vemcache database with a specified key.
    ///
    /// The key is provided by the user and must be unique. If a vector with the
    /// same key already exists in the database, it will be overwritten.
    ///
    /// # Arguments
    ///
    /// * `key` - A unique string identifier for the vector.
    /// * `vector` - The vector to be inserted into the database.
    ///
    /// # Example
    ///
    /// ```
    /// use vemcache::Vemcache;
    ///
    /// // Create a new Vemcache database instance
    /// let mut db = Vemcache::new();
    ///
    /// // Insert a vector with a specified key
    /// db.insert_with_key("vector1".to_string(), vec![1.0, 2.0, 3.0]);
    /// ```
    pub fn insert_with_key(&mut self, key: String, vector: Vec<f32>) {
        self.storage.insert(key, vector);
    }

    /// Inserts a vector into the Vemcache database and generates a unique UUID as the key.
    ///
    /// The key is automatically generated as a UUID (Universally Unique Identifier) and
    /// returned to the user. The UUID is guaranteed to be unique within the database.
    ///
    /// # Arguments
    ///
    /// * `vector` - The vector to be inserted into the database.
    ///
    /// # Returns
    ///
    /// A string representation of the UUID that was generated as the key for the vector.
    ///
    /// # Example
    ///
    /// ```
    /// use vemcache::Vemcache;
    ///
    /// // Create a new Vemcache database instance
    /// let mut db = Vemcache::new();
    ///
    /// // Insert a vector and receive the generated UUID key
    /// let key = db.insert_with_uuid(vec![1.0, 2.0, 3.0]);
    /// ```
    pub fn insert_with_uuid(&mut self, vector: Vec<f32>) -> String {
        let id = Uuid::new_v4().to_string();
        self.storage.insert(id.clone(), vector);
        id
    }

    /// Removes a vector from the Vemcache database by its key (ID).
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier (key) of the vector to be removed.
    ///
    /// # Returns
    ///
    /// An `Option` containing the removed vector, if it existed in the database.
    /// Returns `None` if the vector with the specified key was not found.
    ///
    /// # Example
    ///
    /// ```
    /// use vemcache::Vemcache;
    ///
    /// // Create a new Vemcache database instance
    /// let mut db = Vemcache::new();
    ///
    /// // Insert a vector with a specified key
    /// db.insert_with_key("vector1".to_string(), vec![1.0, 2.0, 3.0]);
    ///
    /// // Remove the vector by its key
    /// let removed_vector = db.remove("vector1".to_string());
    /// assert_eq!(removed_vector, Some(vec![1.0, 2.0, 3.0]));
    /// ```
    pub fn remove(&mut self, id: VectorId) -> Option<Vector> {
        self.storage.remove(&id)
    }

    /// Retrieves a vector from the Vemcache database by its key (ID).
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier (key) of the vector to be retrieved.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the retrieved vector, if it exists in the database.
    /// Returns `None` if the vector with the specified key was not found.
    ///
    /// # Example
    ///
    /// ```
    /// use vemcache::Vemcache;
    ///
    /// // Create a new Vemcache database instance
    /// let mut db = Vemcache::new();
    ///
    /// // Insert a vector with a specified key
    /// db.insert_with_key("vector1".to_string(), vec![1.0, 2.0, 3.0]);
    ///
    /// // Retrieve the vector by its key
    /// let vector = db.get("vector1".to_string());
    /// assert_eq!(vector, Some(&vec![1.0, 2.0, 3.0]));
    /// ```
    pub fn get(&self, id: VectorId) -> Option<&Vector> {
        self.storage.get(&id)
    }

    /// Calculates the Euclidean distance between two vectors.
    ///
    /// The Euclidean distance is the square root of the sum of the squared differences
    /// between corresponding elements of the two vectors. The vectors must have the same
    /// number of dimensions.
    ///
    /// # Arguments
    ///
    /// * `v1` - The first vector.
    /// * `v2` - The second vector.
    ///
    /// # Returns
    ///
    /// The Euclidean distance between the two vectors as a floating-point value.
    ///
    /// # Example
    ///
    /// ```
    /// use vemcache::Vemcache;
    ///
    /// // Create a new Vemcache database instance
    /// let db = Vemcache::new();
    ///
    /// // Define two vectors
    /// let vector1 = vec![1.0, 2.0, 3.0];
    /// let vector2 = vec![4.0, 5.0, 6.0];
    ///
    /// // Calculate the Euclidean distance between the vectors
    /// let distance = db.euclidean_distance(&vector1, &vector2);
    /// assert_eq!(distance, (27.0 as f32).sqrt());
    /// ```
    pub fn euclidean_distance(v1: &Vector, v2: &Vector) -> f32 {
        v1.iter()
            .zip(v2.iter())
            .map(|(x, y)| (x - y).powi(2))
            .sum::<f32>()
            .sqrt()
    }

    /// Finds the k-nearest neighbors to a given query vector in the Vemcache database.
    ///
    /// The k-nearest neighbors are determined based on the Euclidean distance between
    /// the query vector and the vectors stored in the database. The function returns
    /// a vector of tuples, where each tuple contains the key (ID) and a reference to
    /// one of the k-nearest neighbor vectors.
    ///
    /// # Arguments
    ///
    /// * `query` - The query vector for which the k-nearest neighbors are to be found.
    /// * `k` - The number of nearest neighbors to retrieve.
    ///
    /// # Returns
    ///
    /// A vector of tuples containing the keys (IDs) and references to the k-nearest
    /// neighbor vectors. If there are fewer than k vectors in the database, the function
    /// returns all available vectors.
    ///
    /// # Example
    ///
    /// ```
    /// use vemcache::Vemcache;
    ///
    /// // Create a new Vemcache database instance
    /// let mut db = Vemcache::new();
    ///
    /// // Insert vectors into the database
    /// db.insert_with_key("vector1".to_string(), vec![1.0, 2.0, 3.0]);
    /// db.insert_with_key("vector2".to_string(), vec![4.0, 5.0, 6.0]);
    /// db.insert_with_key("vector3".to_string(), vec![7.0, 8.0, 9.0]);
    ///
    /// // Define a query vector
    /// let query_vector = vec![2.0, 3.0, 4.0];
    ///
    /// // Find the 2 nearest neighbors to the query vector
    /// let nearest_neighbors = db.k_nearest_neighbors(&query_vector, 2);
    /// assert_eq!(nearest_neighbors, vec![
    ///     ("vector1".to_string(), &vec![1.0, 2.0, 3.0]),
    ///     ("vector2".to_string(), &vec![4.0, 5.0, 6.0])
    /// ]);
    /// ```
    pub fn k_nearest_neighbors(&self, query: &Vec<f32>, k: usize) -> Vec<(String, &Vec<f32>)> {
        let mut neighbors = self
            .storage
            .iter()
            .map(|(id, vector)| (id.clone(), Vemcache::euclidean_distance(query, vector)))
            .collect::<Vec<_>>();
        neighbors.sort_by(|(_, dist1), (_, dist2)| dist1.partial_cmp(dist2).unwrap());
        neighbors
            .into_iter()
            .take(k)
            .map(|(id, _)| (id.clone(), self.storage.get(&id).unwrap()))
            .collect()
    }

    /// Performs element-wise addition of two vectors stored in the Vemcache database.
    ///
    /// The vectors are identified by their keys (IDs). The function returns the result
    /// of the addition as a new vector. The vectors must have the same number of dimensions.
    ///
    /// # Arguments
    ///
    /// * `key1` - The key (ID) of the first vector to be added.
    /// * `key2` - The key (ID) of the second vector to be added.
    ///
    /// # Returns
    ///
    /// An `Option` containing the result of the vector addition as a new vector.
    /// Returns `None` if either of the keys is not found in the database, or if the
    /// vectors have different dimensions.
    ///
    /// # Example
    ///
    /// ```
    /// use vemcache::Vemcache;
    ///
    /// // Create a new Vemcache database instance
    /// let mut db = Vemcache::new();
    ///
    /// // Insert vectors into the database
    /// db.insert_with_key("vector1".to_string(), vec![1.0, 2.0, 3.0]);
    /// db.insert_with_key("vector2".to_string(), vec![4.0, 5.0, 6.0]);
    ///
    /// // Perform vector addition
    /// let result = db.vector_addition("vector1", "vector2");
    /// assert_eq!(result, Some(vec![5.0, 7.0, 9.0]));
    /// ```
    pub fn vector_addition(&self, key1: &str, key2: &str) -> Option<Vec<f32>> {
        let v1 = self.storage.get(key1)?;
        let v2 = self.storage.get(key2)?;
        if v1.len() != v2.len() {
            return None;
        }
        Some(v1.iter().zip(v2.iter()).map(|(x, y)| x + y).collect())
    }

    /// Performs element-wise subtraction of two vectors stored in the Vemcache database.
    ///
    /// The vectors are identified by their keys (IDs). The function returns the result
    /// of the subtraction as a new vector. The vectors must have the same number of dimensions.
    ///
    /// # Arguments
    ///
    /// * `key1` - The key (ID) of the minuend vector.
    /// * `key2` - The key (ID) of the subtrahend vector.
    ///
    /// # Returns
    ///
    /// An `Option` containing the result of the vector subtraction as a new vector.
    /// Returns `None` if either of the keys is not found in the database, or if the
    /// vectors have different dimensions.
    ///
    /// # Example
    ///
    /// ```
    /// use vemcache::Vemcache;
    ///
    /// // Create a new Vemcache database instance
    /// let mut db = Vemcache::new();
    ///
    /// // Insert vectors into the database
    /// db.insert_with_key("vector1".to_string(), vec![1.0, 2.0, 3.0]);
    /// db.insert_with_key("vector2".to_string(), vec![4.0, 5.0, 6.0]);
    ///
    /// // Perform vector subtraction
    /// let result = db.vector_subtraction("vector1", "vector2");
    /// assert_eq!(result, Some(vec![-3.0, -3.0, -3.0]));
    /// ```
    pub fn vector_subtraction(&self, key1: &str, key2: &str) -> Option<Vec<f32>> {
        let v1 = self.storage.get(key1)?;
        let v2 = self.storage.get(key2)?;
        if v1.len() != v2.len() {
            return None;
        }
        Some(v1.iter().zip(v2.iter()).map(|(x, y)| x - y).collect())
    }

    /// Performs scalar multiplication of a vector stored in the Vemcache database.
    ///
    /// The vector is identified by its key (ID). The function returns the result
    /// of the scalar multiplication as a new vector.
    ///
    /// # Arguments
    ///
    /// * `key` - The key (ID) of the vector to be scaled.
    /// * `scalar` - The scalar value by which the vector is to be multiplied.
    ///
    /// # Returns
    ///
    /// An `Option` containing the result of the vector scaling as a new vector.
    /// Returns `None` if the key is not found in the database.
    ///
    /// # Example
    ///
    /// ```
    /// use vemcache::Vemcache;
    ///
    /// // Create a new Vemcache database instance
    /// let mut db = Vemcache::new();
    ///
    /// // Insert a vector into the database
    /// db.insert_with_key("vector1".to_string(), vec![1.0, 2.0, 3.0]);
    ///
    /// // Perform vector scaling
    /// let result = db.vector_scaling("vector1", 2.0);
    /// assert_eq!(result, Some(vec![2.0, 4.0, 6.0]));
    /// ```
    pub fn vector_scaling(&self, key: &str, scalar: f32) -> Option<Vec<f32>> {
        let v = self.storage.get(key)?;
        Some(v.iter().map(|x| x * scalar).collect())
    }

    /// Calculates the cosine similarity between two vectors.
    ///
    /// The cosine similarity is a measure of similarity between two vectors in a
    /// multi-dimensional space. It is defined as the cosine of the angle between
    /// the vectors. The vectors must have the same number of dimensions.
    ///
    /// # Arguments
    ///
    /// * `v1` - The first vector.
    /// * `v2` - The second vector.
    ///
    /// # Returns
    ///
    /// An `Option` containing the cosine similarity between the two vectors as a floating-point value.
    /// Returns `None` if the vectors have different dimensions.
    ///
    /// # Example
    ///
    /// ```
    /// use vemcache::Vemcache;
    ///
    /// // Define two vectors
    /// let vector1 = vec![1.0, 2.0, 3.0];
    /// let vector2 = vec![4.0, 5.0, 6.0];
    ///
    /// // Calculate the cosine similarity between the vectors
    /// let similarity = Vemcache::cosine_similarity(&vector1, &vector2).unwrap();
    /// assert_eq!(similarity, 0.9746318461970762);
    /// ```
    pub fn cosine_similarity(&self, v1: &Vec<f32>, v2: &Vec<f32>) -> Option<f32> {
        if v1.len() != v2.len() {
            return None;
        }
        let dot_product = v1.iter().zip(v2.iter()).map(|(x, y)| x * y).sum::<f32>();
        let magnitude_v1 = (v1.iter().map(|x| x.powi(2)).sum::<f32>()).sqrt();
        let magnitude_v2 = (v2.iter().map(|x| x.powi(2)).sum::<f32>()).sqrt();
        Some(dot_product / (magnitude_v1 * magnitude_v2))
    }

    /// Dumps the contents of the Vemcache database to a JSON file.
    ///
    /// This function serializes the entire contents of the database (i.e., the `storage` field)
    /// into a JSON file specified by the `file_path` argument.
    ///
    /// # Arguments
    ///
    /// * `file_path` - The path to the file where the database dump will be written.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vemcache::Vemcache;
    /// let mut db = Vemcache::new();
    ///
    /// // Insert some vectors into the database
    /// db.insert_with_key("vector1".to_string(), vec![1.0, 2.0, 3.0]);
    /// db.insert_with_key("vector2".to_string(), vec![4.0, 5.0, 6.0]);
    ///
    /// // Dump the database to a file
    /// match db.dump("vemcache_dump.json") {
    ///     Ok(_) => println!("Database dump successful."),
    ///     Err(err) => eprintln!("Error creating database dump: {}", err),
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an `IoResult<()>` to indicate whether the operation was successful or if there was an I/O error.
    pub fn dump(&self, file_path: &str) -> IoResult<()> {
        // Open the file for writing
        let file = File::create(file_path)?;

        // Serialize the storage field into JSON format and write it to the file
        serde_json::to_writer(file, &self.storage)?;

        Ok(())
    }
}
