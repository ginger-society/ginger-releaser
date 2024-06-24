use clap::Subcommand;

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
        BumpType::Channel => match version.channel {
            Channel::Nightly => version.channel = Channel::Alpha,
            Channel::Final => version.channel = Channel::Alpha,
            Channel::Alpha => version.channel = Channel::Beta,
            Channel::Beta => version.channel = Channel::Final,
        },
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
