use std::fmt::format;
use std::fs::{self, ReadDir};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(name = "rn-gen")]
#[clap(
    author = "Alan Hughes",
    version = "1.0.0",
    about = "Generates a new React native project"
)]
struct Cli {
    #[clap(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(arg_required_else_help = true)]
    #[clap(about = "The name of your new project")]
    New {
        name: String,
        #[clap(long = "expo")]
        expo: bool,
    },
}

pub fn run() {
    let args = Cli::parse();

    match args.cmd {
        Commands::New { name, expo } => {
            if expo {
                generate_expo(name);
            } else {
                generate_project(name);
            }
        }
    }
}

fn generate_project(name: String) {
    println!("Creating a new project: {}", name);
    let path = Path::new("generate");
    let target = Path::new(&name);
    create_dir(&target.to_path_buf(), &path.to_path_buf());
}

fn generate_expo(name: String) {
    let expo = Command::new("npx")
        .arg("expo-cli")
        .arg("init")
        .arg(name)
        .arg("--template")
        .arg("blank")
        .arg("--non-interactive")
        .output()
        .expect("Expo not available");

    io::stdout().write_all(&expo.stdout).unwrap();
    io::stderr().write_all(&expo.stderr).unwrap();
}

fn copy_file(from: PathBuf, to: PathBuf) {
    fs::copy(from, to).expect("Failed to copy file");
}

fn create_dir(new_dir: &Path, parent: &Path) {
    fs::create_dir(&new_dir).expect("Failed to create directory");

    let entries = get_dir_entries(parent);

    for entry in entries {
        if let Ok(entry) = entry {
            let metadata = entry.metadata().expect("Failed to read file metadata");
            if metadata.is_dir() {
                let dir_path = new_dir.join(entry.file_name());
                create_dir(&dir_path.to_path_buf(), &entry.path());
            } else {
                copy_file(entry.path(), new_dir.join(entry.file_name()));
            }
        }
    }
}

fn get_dir_entries(path: &Path) -> ReadDir {
    fs::read_dir(path).expect("Failed to read directory")
}
