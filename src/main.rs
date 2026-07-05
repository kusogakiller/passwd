use clap::{Parser, Subcommand};
use rand::RngCore;
use sha2::{Sha256, Digest};
use blake3;
use hex;

#[derive(Parser)]
#[command(name = "rustcrypt")]
#[command(version = "0.1.0")]
#[command(about = "Safe. Fast. Cryptographic.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Generate {
        size: usize,
    },

    Version,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate { size } => {
            match size {
                32 => {
                    let mut buf = [0u8; 16];
                    rand::thread_rng().fill_bytes(&mut buf);
                
                    let out = blake3::hash(&buf).as_bytes()[..16].to_vec();
                
                    println!("{}", hex::encode(out));
                }

                64 => {
                    let mut buf = [0u8; 32];
                    rand::thread_rng().fill_bytes(&mut buf);

                    let hash = Sha256::digest(&buf);
                    println!("{}", hex::encode(hash));
                }

                _ => {
                    eprintln!("Invalid size. Use 32 or 64.");
                }
            }
        }

        Commands::Version => {
            println!("rustcrypt 0.1.0");
        }
    }
}
