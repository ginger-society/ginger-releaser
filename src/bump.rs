use clap::Subcommand;
use inquire::{InquireError, Select};

use crate::utils::{Channel, Version};

#[derive(Subcommand, Debug)]
pub enum BumpType {
    Major,
    Minor,
    Patch,
    Channel,
    Revision,
}

pub fn bump_version(bump_type: BumpType, version: &mut Version) -> &mut Version {
    match bump_type {
        BumpType::Channel => {
            let options = Channel::all();
            let ans: Result<Channel, InquireError> =
                Select::new("Please select the channel used in this project", options)
                    .with_starting_cursor(version.channel.index_of())
                    .prompt();

            version.channel = ans.unwrap()
        }
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
        BumpType::Revision => version.revision += 1,
    }

    version
}
