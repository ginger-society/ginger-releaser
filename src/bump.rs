use std::process::exit;

use clap::Subcommand;
use ginger_shared_rs::{Channel, Version};

#[derive(Subcommand, Debug)]
pub enum BumpType {
    Major,
    Minor,
    Patch,
    Revision,
}

pub fn bump_version(bump_type: BumpType, version: &mut Version) -> &mut Version {
    match bump_type {
        BumpType::Major => {
            version.major += 1;
            version.minor = 0;
            version.patch = 0;
            version.revision = 0;
        }
        BumpType::Minor => {
            version.minor += 1;
            version.patch = 0;
            version.revision = 0;
        }
        BumpType::Patch => {
            version.patch += 1;
            version.revision = 0;
        }
        BumpType::Revision => match version.channel {
            Channel::Final => {
                println!("You can't change revision once a project is in Final stage. Existing");
                exit(1)
            }
            _ => version.revision += 1,
        },
    }

    version
}

pub fn bump_channel(version: &mut Version) -> &mut Version {
    match version.channel {
        Channel::Nightly => version.channel = Channel::Alpha,
        Channel::Final => {
            println!("Since the project is already out of Beta stage, please use major/minor/patch releaes.");
            exit(1)
        }
        Channel::Alpha => version.channel = Channel::Beta,
        Channel::Beta => {
            version.channel = Channel::Final;
            version.major += 1;
            version.minor = 0;
            version.patch = 0;
            version.revision = 0;
        }
    }
    version
}
