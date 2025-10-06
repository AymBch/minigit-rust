use rand;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::create_dir;
use std::fs::read_dir;
use std::fs::read_to_string as fs_read_to_string;
use std::fs::write;
use std::path::Path;
use std::process;

const CONTENT_JSON_FILE: &str = "commits.json";
const MINIGIT_DIR: &str = ".minigit";

#[derive(Serialize, Deserialize)]
struct Commit {
    id: String,
    message: String,
    timestamp: String,
    files: Vec<FileSnapshot>,
}

#[derive(Serialize, Deserialize)]
struct FileSnapshot {
    name: String,
    hash: String,
}

fn read_all_files() -> Vec<FileSnapshot> {
    let mut snapshots = Vec::new();
    read_dir_recursive(".", &mut snapshots);
    snapshots
}

fn read_dir_recursive<P: AsRef<Path>>(path: P, snapshots: &mut Vec<FileSnapshot>) {
    for entry in read_dir(path).unwrap() {
        let entiry = entry.unwrap();
        let path = entiry.path();

        if path.starts_with(MINIGIT_DIR) {
            continue;
        }

        if path.is_dir() {
            read_dir_recursive(&path, snapshots);
        } else if path.is_file() {
            let file_name = path.file_name().unwrap().to_string_lossy().to_string();
            let content = fs_read_to_string(path).unwrap_or_default();
            let hash = hash_content(&content);

            snapshots.push(FileSnapshot {
                name: file_name,
                hash,
            });
        }
    }
}

fn hash_content(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn current_timestamp() -> String {
    chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

fn init_repo() {
    let path = Path::new(".minigit");
    if path.exists() {
        println!("Repository already initialized.");
    } else {
        create_dir(path).expect("Failed to create .minigit directory");
        write(path.join(CONTENT_JSON_FILE), "[]").expect("Failed to create content.json");
        println!("initialized empty repository in .minigit");
    }
}

fn commit_changes(message: &str) {
    let repo_path = Path::new(MINIGIT_DIR);
    if !repo_path.exists() {
        eprintln!("Repository not initialized. Please run 'mini_git init' first.");
        process::exit(1);
    }
    println!("Committing changes with message: {}", message);
    // read existing commits
    let path = repo_path.join(CONTENT_JSON_FILE);
    let data = fs_read_to_string(&path).expect("Failed to read commits.json");
    let mut commits: Vec<Commit> =
        serde_json::from_str(&data).expect("Failed to parse commits.json");

    let snapshots = read_all_files();
    let new_commit = Commit {
        id: format!("{:x}", rand::random::<u64>()), // Simple random ID
        message: message.to_string(),
        timestamp: current_timestamp(),
        files: snapshots,
    };

    commits.push(new_commit);

    let updated_data = serde_json::to_string_pretty(&commits).expect("Failed to serialize commits");
    write(path, updated_data).expect("Failed to write commits.json");
    println!("Changes committed.");
}

fn show_log() {
    println!("Showing commit history....");
    let path = Path::new(MINIGIT_DIR).join(CONTENT_JSON_FILE);
    let data = fs_read_to_string(&path).expect("Failed to read commits.json");
    let commits: Vec<Commit> = serde_json::from_str(&data).expect("Failed to parse commits.json");
    for commit in commits.iter().rev() {
        println!("commit {}", commit.id);
        println!("Date: {}", commit.timestamp);
        println!("\n    {}\n", commit.message);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: mini_git <command> [<args>]");
        process::exit(1);
    }

    if args.len() == 2 && args[1] == "--version" || args[1] == "-v" {
        println!(
            "{} version {}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        );
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "init" => {
            init_repo();
        }
        "commit" => {
            if args.len() < 3 {
                eprintln!("Usage: mini_git commit <message>");
                process::exit(1);
            }
            let message = &args[2..].join(" ");
            commit_changes(message);
        }
        "log" => {
            show_log();
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            process::exit(1);
        }
    }
}
