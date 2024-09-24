use crate::{references::update_references, release_notes};
use ginger_shared_rs::{OutputType, ReleaserConfig, Version};
use inquire::{
    ui::{Color, RenderConfig, Styled},
    Editor,
};
use regex::Regex;
use serde_json::Value;
use std::{error::Error, process::Command};

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

fn description_render_config() -> RenderConfig {
    RenderConfig::default()
        .with_canceled_prompt_indicator(Styled::new("<skipped>").with_fg(Color::DarkYellow))
}

pub fn update_project_source(config: &ReleaserConfig, require_long_msg: bool) {
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

            let mut notes = String::from("");

            if require_long_msg {
                notes = Editor::new("Description:")
                    .with_formatter(&|submission| {
                        let char_count = submission.chars().count();
                        if char_count == 0 {
                            String::from("<skipped>")
                        } else if char_count <= 20 {
                            submission.into()
                        } else {
                            let mut substr: String = submission.chars().take(17).collect();
                            substr.push_str("...");
                            substr
                        }
                    })
                    .with_render_config(description_render_config())
                    .prompt()
                    .unwrap();
            }

            let commit_message = format!(
                "chore: version bump to {}\n\n {}",
                config.version.formatted(),
                notes
            );

            let add_status = Command::new("git")
                .arg("add")
                .arg(".")
                .status()
                .expect("Failed to add files");

            if !add_status.success() {
                println!("Failed to add files");
                return;
            }

            let status = Command::new("git")
                .arg("commit")
                .arg("-m")
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
