use flate2::Compression;
use flate2::write::ZlibEncoder;
use sha1::{Digest, Sha1};
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("usage: crabgit <command>");
        return;
    }

    match args[1].as_str() {
        "init" => init(),
        "hash-object" => {
            if args.len() < 3 {
                eprintln!("usage: crabgit hash-object <file>");
                return;
            }
            hash_object(&args[2]);
        }
        cmd => eprintln!("unknown command: {}", cmd),
    }
}

fn init() {
    if Path::new(".crabgit").exists() {
        eprintln!("crabgit repository already exists in .crabgit/");
        return;
    }

    fs::create_dir(".crabgit").unwrap();
    fs::create_dir(".crabgit/objects").unwrap();
    fs::create_dir_all(".crabgit/refs/heads").unwrap();
    fs::write(".crabgit/HEAD", "ref: refs/heads/main\n").unwrap();
    println!("Initialized empty crabgit repository in .crabgit/");
}

fn hash_object(path: &str) {
    if !Path::new(".crabgit").exists() {
        eprintln!("fatal: not a crabgit repository");
        return;
    }

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

    // --- BARU: simpan objek ---
    let dir = format!(".crabgit/objects/{}", &hex[..2]);
    let file = format!("{}/{}", dir, &hex[2..]);

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&data).unwrap();
    let compressed = encoder.finish().unwrap();

    fs::create_dir_all(&dir).unwrap();
    fs::write(&file, compressed).unwrap();
    // --- akhir bagian baru ---

    println!("{}", hex);
}
