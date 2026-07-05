use rand::RngCore;
use blake3;
use hex;

pub fn generate(size: usize) -> String {
    let mut raw = vec![0u8; size];
    rand::thread_rng().fill_bytes(&mut raw);

    let hash = blake3::hash(&raw);
    let hex = hash.to_hex().to_string();

    match size {
        32 => hex[..32].to_string(),
        64 => hex[..64].to_string(),
        _ => hex[..32].to_string(),
    }
}
