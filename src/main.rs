use sha1::{Digest, Sha1};
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("usage: gitlet <command>");
        return;
    }

    match args[1].as_str() {
        "init" => init(),
        "hash-object" => {
            if args.len() < 3 {
                eprintln!("usage: gitlet hash-object <file>");
                return;
            }
            hash_object(&args[2]);
        }
        cmd => eprintln!("unknown command: {}", cmd),
    }
}

fn init() {
    if Path::new(".gitlet").exists() {
        eprintln!("gitlet repository already exists in .gitlet/");
        return;
    }

    fs::create_dir(".gitlet").unwrap();
    fs::create_dir(".gitlet/objects").unwrap();
    fs::create_dir_all(".gitlet/refs/heads").unwrap();
    fs::write(".gitlet/HEAD", "ref: refs/heads/main\n").unwrap();
    println!("Initialized empty gitlet repository in .gitlet/");
}

fn hash_object(path: &str) {
    let content = match fs::read(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("fatal: could not open '{}' for reading: {}", path, e);
            return;
        }
    };

    let header = format!("blob {}\0", content.len());

    let mut data = Vec::new();
    data.extend_from_slice(header.as_bytes());
    data.extend_from_slice(&content);

    let hash = Sha1::digest(&data);
    let hex: String = hash.iter().map(|b| format!("{:02x}", b)).collect();

    println!("{}", hex);
}
