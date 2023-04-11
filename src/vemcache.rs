use uuid::Uuid;
use std::collections::HashMap;

type VectorId = String;
type Vector = Vec<f32>;

pub struct Vemcache {
    storage: HashMap<VectorId, Vector>,
}

impl Vemcache {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    pub fn insert_with_key(&mut self, key: String, vector: Vec<f32>) {
        self.storage.insert(key, vector);
    }

    pub fn insert_with_uuid(&mut self, vector: Vec<f32>) -> String {
        let id = Uuid::new_v4().to_string();
        self.storage.insert(id.clone(), vector);
        id
    }

    pub fn remove(&mut self, id: VectorId) -> Option<Vector> {
        self.storage.remove(&id)
    }

    pub fn get(&self, id: VectorId) -> Option<&Vector> {
        self.storage.get(&id)
    }

    pub fn euclidean_distance(v1: &Vector, v2: &Vector) -> f32 {
        v1.iter()
            .zip(v2.iter())
            .map(|(x, y)| (x - y).powi(2))
            .sum::<f32>()
            .sqrt()
    }

    pub fn nearest_neighbor(&self, query: &Vec<f32>) -> Option<(String, &Vec<f32>)> {
        self.storage
            .iter()
            .map(|(id, vector)| (id.clone(), Vemcache::euclidean_distance(query, vector)))
            .min_by(|(_, dist1), (_, dist2)| dist1.partial_cmp(dist2).unwrap())
            .map(|(id, _)| (id.clone(), self.storage.get(&id).unwrap()))
    }
    
    // Add the following functions to the VectorDB implementation

    // K-Nearest Neighbors
    pub fn k_nearest_neighbors(&self, query: &Vec<f32>, k: usize) -> Vec<(String, &Vec<f32>)> {
        let mut neighbors = self.storage
            .iter()
            .map(|(id, vector)| (id.clone(), Vemcache::euclidean_distance(query, vector)))
            .collect::<Vec<_>>();
        neighbors.sort_by(|(_, dist1), (_, dist2)| dist1.partial_cmp(dist2).unwrap());
        neighbors.into_iter()
            .take(k)
            .map(|(id, _)| (id.clone(), self.storage.get(&id).unwrap()))
            .collect()
    }

    // Vector Addition
    pub fn vector_addition(&self, key1: &str, key2: &str) -> Option<Vec<f32>> {
        let v1 = self.storage.get(key1)?;
        let v2 = self.storage.get(key2)?;
        if v1.len() != v2.len() {
            return None;
        }
        Some(v1.iter().zip(v2.iter()).map(|(x, y)| x + y).collect())
    }

    // Vector Subtraction
    pub fn vector_subtraction(&self, key1: &str, key2: &str) -> Option<Vec<f32>> {
        let v1 = self.storage.get(key1)?;
        let v2 = self.storage.get(key2)?;
        if v1.len() != v2.len() {
            return None;
        }
        Some(v1.iter().zip(v2.iter()).map(|(x, y)| x - y).collect())
    }

    // Vector Scaling
    pub fn vector_scaling(&self, key: &str, scalar: f32) -> Option<Vec<f32>> {
        let v = self.storage.get(key)?;
        Some(v.iter().map(|x| x * scalar).collect())
    }

    // Cosine Similarity
    pub fn cosine_similarity(v1: &Vec<f32>, v2: &Vec<f32>) -> Option<f32> {
        if v1.len() != v2.len() {
            return None;
        }
        let dot_product = v1.iter().zip(v2.iter()).map(|(x, y)| x * y).sum::<f32>();
        let magnitude_v1 = (v1.iter().map(|x| x.powi(2)).sum::<f32>()).sqrt();
        let magnitude_v2 = (v2.iter().map(|x| x.powi(2)).sum::<f32>()).sqrt();
        Some(dot_product / (magnitude_v1 * magnitude_v2))
    }
}