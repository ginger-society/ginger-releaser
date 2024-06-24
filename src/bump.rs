use inquire::{InquireError, Select};

use crate::utils::{Channel, Version};

pub enum BumpType {
    Major,
    Minor,
    Patch,
    Channel,
    Revision,
}

pub fn BumpVersion(bump_type: BumpType, version: &mut Version) -> &mut Version {
    match bump_type {
        BumpType::Channel => {
            let options = Channel::all();
            let ans: Result<Channel, InquireError> =
                Select::new("Please select the language used in this project", options)
                    .with_starting_cursor(version.channel.index_of())
                    .prompt();

            version.channel = ans.unwrap()
        }
        BumpType::Major => version.major += 1,
        BumpType::Minor => version.minor += 1,
        BumpType::Patch => version.patch += 1,
        BumpType::Revision => version.revision += 1,
    }

    version
}
