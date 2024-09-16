use ginger_shared_rs::{OutputType, ReleaserConfig, Version};
use regex::Regex;
use serde_json::Value;
use std::{error::Error, fs, process::Command};

use crate::{references::update_references, release_notes};

pub fn update_py(
    contents: &mut String,
    version: &Version,
    variable: &String,
    output_type: &OutputType,
) -> Result<String, Box<dyn Error>> {
    let version_str = match output_type {
        OutputType::Tuple => version.tuple(),
        OutputType::String => format!("\"{}\"", version.formatted()),
    };
    // Regex to match the VERSION variable assignment
    let re_variable = Regex::new(&format!(r"(?m)^{} = .*", regex::escape(variable)))?;
    // Regex to match the __version__ variable assignment

    // Update VERSION variable
    *contents = re_variable
        .replace_all(contents, format!("{} = {}", variable, version_str).as_str())
        .to_string();

    Ok(contents.to_string())
}

pub fn update_toml(
    contents: &mut String,
    version: &Version,
    variable: &String,
) -> Result<String, Box<dyn Error>> {
    let version_str = version.formatted();
    // Regex to match the version variable assignment
    let re_version = Regex::new(&format!(r#"(?m)^{} = ".*""#, regex::escape(variable)))?;

    // Update version variable
    *contents = re_version
        .replace_all(
            contents,
            format!(r#"{} = "{}""#, variable, version_str).as_str(),
        )
        .to_string();

    Ok(contents.to_string())
}

pub fn update_json(
    contents: &mut String,
    version: &Version,
    variable: &String,
) -> Result<String, Box<dyn Error>> {
    let mut json_value: Value = serde_json::from_str(contents)?;

    if let Some(obj) = json_value.as_object_mut() {
        if let Some(_) = obj.get_mut(variable) {
            obj[variable] = Value::String(version.formatted());
        }
    }

    *contents = serde_json::to_string_pretty(&json_value)?;
    Ok(contents.to_string())
}

pub fn update_project_source(config: &ReleaserConfig) {
    update_references(&config);
    match release_notes::generate_release_notes(
        &config.settings.git_url_prefix.clone().unwrap(),
        config.version,
    ) {
        Err(e) => {
            println!("Unable to generate {:?}", e);
        }
        Ok(_) => {
            println!("Generated release notes successfully");

            let commit_message = format!("chore: version bump to {}", config.version.formatted());
            let status = Command::new("git")
                .arg("commit")
                .arg("-am")
                .arg(&commit_message)
                .status()
                .expect("Failed to commit version bump");
            if !status.success() {
                println!("Failed to create commit");
                return;
            }

            // Create a tag with the version
            let tag_name = config.version.formatted();
            let status = Command::new("git")
                .arg("tag")
                .arg(&tag_name)
                .status()
                .expect("Failed to create tag");

            if !status.success() {
                println!("Failed to create tag");
                return;
            }

            println!("Version bumped to {}, commit and tag created", tag_name);
        }
    };
}
