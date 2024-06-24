use bump::{bump_channel, bump_version, BumpType};
use clap::{Parser, Subcommand};
use init::init;
use references::update_references;
use utils::{read_config, update_project_source, write_config};

use std::process::Command;

mod bump;
mod init;
mod references;
mod release_notes;
mod utils;

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initialize a releaser config in a project
    Init,
    /// Creates a release by bumping the version, creating a git tag and generating CHANGELOG.md file
    Release {
        #[command(subcommand)]
        bump_type: BumpType,
    },
    /// Bumps channel in the order nighly < alpha < beta < final
    Bump,
}

#[derive(Parser, Debug)]
#[command(name = "Ginger-Releaser")]
#[command(about = "A release management CLI tool", long_about = None)]
#[command(version, long_about = None)]
struct Args {
    /// name of the command to run
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let args = Args::parse();

    let file_path = "releaser.toml"; // Update the path to your TOML file

    match args.command {
        Commands::Init => match init(&file_path) {
            Err(e) => {
                println!("{:?}", e)
            }
            Ok(_) => {}
        },
        Commands::Bump => {
            let mut config = read_config(file_path).unwrap();
            bump_channel(&mut config.version);
            write_config(file_path, &config).unwrap();
            update_project_source(&config)
        }
        Commands::Release { bump_type } => {
            let mut config = read_config(file_path).unwrap();
            bump_version(bump_type, &mut config.version);
            write_config(file_path, &config).unwrap();
            update_project_source(&config)
        }
    }
}
