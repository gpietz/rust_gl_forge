use sha2::{Digest, Sha256};
use std::env;
use std::fs::File;
use std::io::{Read, Result};
use std::iter::Map;
use std::path::Path;

fn calculate_file_hash<P: AsRef<Path>>(file_path: P) -> Result<Vec<u8>> {
    let mut file = File::open(file_path.as_ref())?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut hasher = Sha256::new();
    hasher.update(&buffer);
    Ok(hasher.finalize().to_vec())
}

fn hash_to_string(hash: &[u8]) -> String {
    let hash_string: String = Map::collect(hash.iter().map(|byte| format!("{:02X}", byte)));
    hash_string
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: fhash.exe <filename>");
        return;
    }

    let file_path = &args[1];
    match calculate_file_hash(file_path) {
        Ok(hash) => println!("SHA256 hash: {}", hash_to_string(&hash)),
        Err(err) => eprintln!("Failed to Calculate SHA256 hash: {}", err),
    }
}
