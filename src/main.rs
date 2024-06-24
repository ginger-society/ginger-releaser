use clap::Parser;
use clap::Subcommand;

mod release_notes;
mod utils;

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initialize a releaser config in a project
    Init,
    /// Creates a release by bumping the version, creating a git tag and generating CHANGELOG.md file
    Release,
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

    let config = utils::read_config(file_path).unwrap();
    println!("{}", config.version.formatted());

    for reference in config.references {
        println!(
            "File: {}, Type: {}, Variable: {}, FileType : {}",
            reference.file_name, reference.output_type, reference.variable, reference.file_type
        );
    }

    match args.command {
        Commands::Init => {}
        Commands::Release => {
            match release_notes::generate_release_notes() {
                Err(e) => {
                    println!("Unable to generate {:?}", e);
                }
                Ok(_) => {
                    println!("Generated release notes successfully")
                }
            };
        }
    }
}
