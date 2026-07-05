use rand::RngCore;
use sha2::{Sha256, Digest};
use base64::{engine::general_purpose, Engine as _};
use ripemd::Ripemd128;
use std::fs::File;
use std::io::Write;

fn main() {
    let mut raw = [0u8; 256];
    rand::thread_rng().fill_bytes(&mut raw);

    let mut hasher = Sha256::new();

    hasher.update(&raw);
    let first = hasher.finalize_reset();

    hasher.update(first);
    let second = hasher.finalize();

    let b64 = general_purpose::STANDARD.encode(second);

    let mut ripemd = Ripemd128::new();
    ripemd.update(b64.as_bytes());
    let result = ripemd.finalize();

    let hex = result
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();

    let mut file = File::create("output.json").unwrap();

    write!(
        file,
        r#"{{
  "base64": "{}",
  "ripemd128": "{}"
}}"#,
        b64, hex
    ).unwrap();
}
