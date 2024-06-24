use std::fs;
use std::process::exit;

use clap::Parser;
use clap::Subcommand;
use utils::update_json;
use utils::update_py;
use utils::update_toml;
use utils::FileType;
use utils::OutputType;

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
    println!("{}", config.version.tuple());

    for reference in config.references {
        println!(
            "File: {}, Type: {}, Variable: {}, FileType : {}",
            reference.file_name, reference.output_type, reference.variable, reference.file_type
        );

        let mut contents = fs::read_to_string(&reference.file_name).unwrap();
        let var_name = reference.variable;
        let updated_content = match reference.file_type {
            FileType::Py => update_py(
                &mut contents,
                &config.version,
                &var_name,
                &reference.output_type,
            )
            .unwrap(),
            FileType::Toml => update_toml(&mut contents, &config.version, &var_name).unwrap(),
            FileType::Json => update_json(&mut contents, &config.version, &var_name).unwrap(),
            FileType::Unknown => {
                println!(
                    "Unknown file type encountered {}, Can not proceed. Exiting!.",
                    reference.file_type
                );
                exit(1)
            }
        };

        println!("{}", updated_content);

        fs::write(&reference.file_name, updated_content).unwrap();
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
