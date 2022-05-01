use std::fs;
use std::path::Path;

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
    New { name: String },
}

pub fn run() {
    let args = Cli::parse();

    match args.cmd {
        Commands::New { name } => {
            generate_project(name);
        }
    }
}

fn generate_project(name: String) {
    let path = Path::new("generate");

    println!("Creating a new project: {}", name);

    let new_dir = Path::new(&name);
    create_dir(&new_dir);

    let entries = match fs::read_dir(&path) {
        Err(err) => panic!("Failed to read {:?}", err.kind()),
        Ok(entries) => entries,
    };

    for entry in entries {
        if let Ok(entry) = entry {
            let metadata = entry.metadata().unwrap();
            if metadata.is_dir() {
                let dir_path = new_dir.join(entry.file_name());
                create_dir(&dir_path);
            } else {
                println!("{:?}: {:?} ", entry.path(), new_dir.join(entry.file_name()));
                match fs::copy(entry.path(), new_dir.join(entry.file_name())) {
                    Ok(res) => {
                        println!("{:?} has been copied!: Result: {}", entry.file_name(), res)
                    }
                    Err(err) => println!("{:?} failed to copy", err.kind()),
                }
            }
        }
    }
}

fn create_dir(name: &Path) {
    match fs::create_dir(name) {
        Err(err) => println!("Failed to create directory: {:?}", err.kind()),
        Ok(()) => println!("Directory created: {:?}", name),
    }
}
