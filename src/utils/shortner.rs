use sha2::{Sha256, Digest};

pub fn generate_code(url: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(url);
    let result = hasher.finalize();
    hex::encode(result)[0..6].to_string()
}