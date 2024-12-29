use risc0_zkvm::guest::env;
use sha2::{Digest, Sha256};
fn main() {
    // TODO: Implement your guest code here
    let input: String = env::read();
    // Compute SHA-256 hash of the input
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let output = hasher.finalize();
    let hash_string = hex::encode(output);
    env::commit(&hash_string); 
}
