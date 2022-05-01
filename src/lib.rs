use std::fs::{self, DirEntry, File};
use std::io::{self, Write};
use std::path::Path;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(name = "rngen")]
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
    match fs::create_dir(&name) {
        Err(err) => println!("{:?}", err.kind()),
        Ok(dir) => dir,
    };

    let entries = match fs::read_dir(&path) {
        Err(err) => panic!("Failed to read {:?}", err.kind()),
        Ok(entries) => entries,
    };

    for entry in entries {
        if let Ok(entry) = entry {
            let result = match copy_dir_all(&entry) {
                Ok(()) => println!("Created"),
                Err(err) => panic!("{}", err.kind()),
            };

            println!("{:?}", result);
        }
    }
}

fn copy_dir_all(entry: &DirEntry) -> io::Result<()> {
    for entry in fs::read_dir(entry)? {
        let entry = entry?;
        let ty = entry.file_type()?;

        if ty.is_dir() {
            copy_dir_all(&entry)?;
        } else {
            fs::copy(entry.path(), path.join(entry.file_name()))?;
        }
    }
    Ok(())
}
