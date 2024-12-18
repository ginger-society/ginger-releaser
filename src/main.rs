use bump::{bump_channel, bump_version, BumpType};
use clap::{Parser, Subcommand};
use ginger_shared_rs::{read_releaser_config_file, write_releaser_config_file};
use init::init;
use snapshot::generate_snapshot;
use utils::update_project_source;

mod bump;
mod init;
mod references;
mod release_notes;
mod snapshot;
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

#[tokio::main]
async fn main() {
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
            let mut config = read_releaser_config_file(file_path).unwrap();
            bump_channel(&mut config.version);
            write_releaser_config_file(file_path, &config).unwrap();
            update_project_source(&config, false)
        }
        Commands::Release { bump_type } => {
            let mut config = read_releaser_config_file(file_path).unwrap();

            bump_version(bump_type.clone(), &mut config.version);
            write_releaser_config_file(file_path, &config).unwrap();
            if config.settings.take_snapshots && bump_type == BumpType::Minor {
                generate_snapshot(&config).await
            }
            update_project_source(&config, bump_type == BumpType::Minor);
        }
    }
}
