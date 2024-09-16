use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::Path;

use ginger_shared_rs::{Channel, ReleaserConfig, ReleaserSettings, Version};

pub fn init(config_path: &str) -> Result<(), Box<dyn Error>> {
    if !Path::new(config_path).exists() {
        let initial_version = Version {
            channel: Channel::Nightly,
            major: 0,
            minor: 0,
            patch: 0,
            revision: 0,
        };

        let initial_config = ReleaserConfig {
            version: initial_version.clone(),
            references: vec![],
            settings: ReleaserSettings {
                git_url_prefix: Some(String::from("")),
            },
        };

        let toml_string = toml::to_string(&initial_config)?;
        let mut file = fs::File::create(config_path)?;
        file.write_all(toml_string.as_bytes())?;

        println!(
            "Created {} with version {}",
            config_path,
            initial_version.formatted()
        );
    } else {
        println!("{} already exists, skipping initialization.", config_path);
    }

    Ok(())
}
