use serde::{Deserialize, Serialize};
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
#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct Config {
    pub version: Version,
    #[serde(default = "default_references")]
    pub references: Vec<Reference>,
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
