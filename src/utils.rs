use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{error::Error, fmt, fs, path::Path};

#[derive(Debug)]
pub enum FileType {
    Py,
    Toml,
    Json,
    Unknown,
}

impl FileType {
    fn from_extension(ext: Option<&str>) -> FileType {
        match ext {
            Some("py") => FileType::Py,
            Some("toml") => FileType::Toml,
            Some("json") => FileType::Json,
            _ => FileType::Unknown,
        }
    }
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileType::Py => write!(f, "Py"),
            FileType::Toml => write!(f, "Toml"),
            FileType::Json => write!(f, "Json"),
            FileType::Unknown => write!(f, "Unknown"),
        }
    }
}
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq)]
pub enum Channel {
    Final,
    Nighly, // Also known as Dev branch
    Alpha,
    Beta,
}
impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Channel::Nighly => write!(f, "nighly"),
            Channel::Final => write!(f, "final"),
            Channel::Alpha => write!(f, "alpha"),
            Channel::Beta => write!(f, "beta"),
        }
    }
}
impl Channel {
    pub fn all() -> Vec<Channel> {
        vec![
            Channel::Final,
            Channel::Nighly,
            Channel::Alpha,
            Channel::Beta,
        ]
    }
    pub fn index_of(&self) -> usize {
        Channel::all()
            .iter()
            .position(|channel| channel == self)
            .unwrap_or(0)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct Version {
    pub channel: Channel,
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub revision: u32,
}

impl Version {
    pub fn formatted(&self) -> String {
        match &self.channel {
            Channel::Final => {
                format!("{}.{}.{}", self.major, self.minor, self.patch)
            }
            _ => {
                format!(
                    "{}.{}.{}-{}.{}",
                    self.major, self.minor, self.patch, self.channel, self.revision
                )
            }
        }
    }
    pub fn tuple(&self) -> String {
        format!(
            "({}, {}, {}, \"{}\", {})",
            self.major, self.minor, self.patch, self.channel, self.revision
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OutputType {
    String,
    Tuple,
}

impl fmt::Display for OutputType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutputType::String => write!(f, "String"),
            OutputType::Tuple => write!(f, "Tuple"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Reference {
    pub file_name: String,
    #[serde(default = "default_output_type")] // Use a default value function
    pub output_type: OutputType, // `type` is a reserved keyword in Rust
    pub variable: String,
    #[serde(skip, default = "default_file_type")] // This field is not in the TOML file
    pub file_type: FileType,
}

fn default_file_type() -> FileType {
    FileType::Unknown
}

fn default_output_type() -> OutputType {
    OutputType::String // Default value is "string"
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    pub git_url_prefix: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub version: Version,
    #[serde(default = "default_references")]
    pub references: Vec<Reference>,
    pub settings: Settings,
}

fn default_references() -> Vec<Reference> {
    vec![]
}

pub fn read_config(file_path: &str) -> Result<Config, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    let mut config: Config = toml::from_str(&contents)?;

    for reference in &mut config.references {
        reference.file_type = FileType::from_extension(
            Path::new(&reference.file_name)
                .extension()
                .and_then(|s| s.to_str()),
        );
    }

    Ok(config)
}

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
