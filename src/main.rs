mod core;

use std::env;
use std::fs::File;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: missing arguments");
        std::process::exit(1);
    }

    if args[1] == "version" {
        println!("rustcrypt 0.1.0");
        println!("Safe, Fast, Cryptographic.");
        return;
    }

    let size: usize = match args[1].parse() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Error: size must be a i32");
            std::process::exit(1);
        }
    };

    let format = args.get(2).map(|s| s.as_str());

    let output = core::generate(size);

    if format == Some("json") {
        let json = serde_json::json!({
            "output": output,
            "length": size
        });

        let mut file = File::create("output.json").unwrap();
        file.write_all(json.to_string().as_bytes()).unwrap();

        println!("{}", json);
    } else {
        println!("size: {}", size);
        println!("output: {}", output);
    }
}
