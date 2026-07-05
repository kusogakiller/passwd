use rand::RngCore;
use sha2::{Sha256, Digest};
use blake3;
use hex;

pub fn generate(size: u32) -> String {
    match size {
        32 => generate_32(),
        64 => generate_64(),
        _ => "invalid size".to_string(),
    }
}

fn generate_32() -> String {
    let mut buf = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut buf);

    let hash = blake3::hash(&buf);
    hex::encode(&hash.as_bytes()[..16])
}

fn generate_64() -> String {
    let mut buf = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut buf);

    let hash = Sha256::digest(&buf);
    hex::encode(hash)
}
