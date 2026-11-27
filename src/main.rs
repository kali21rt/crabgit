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
