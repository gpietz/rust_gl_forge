use std::fs::File;
use std::io::{Read, Result};
use std::iter::Map;
use std::path::Path;

use sha2::{Digest, Sha256};

pub fn calculate_hash<T: AsRef<[u8]>>(text: T) -> Vec<u8> {
    calculate_hash_from_bytes(text.as_ref())
}

pub fn calculate_hash_from_bytes(bytes: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher.finalize().to_vec()
}

pub fn calculate_file_hash<P: AsRef<Path>>(file_path: P) -> Result<Vec<u8>> {
    let mut file = File::open(file_path.as_ref())?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut hasher = Sha256::new();
    hasher.update(&buffer);
    Ok(hasher.finalize().to_vec())
}

pub fn hash_to_string(hash: &[u8]) -> String {
    let hash_string: String = Map::collect(hash.iter().map(|byte| format!("{:02X}", byte)));
    hash_string
}

pub fn compare_hashes<T: AsRef<[u8]>>(expected_hash: &T, actual_hash: &[u8]) -> bool {
    let actual_hash_str = hash_to_string(actual_hash);
    expected_hash.as_ref().eq_ignore_ascii_case(actual_hash_str.as_ref())
}
