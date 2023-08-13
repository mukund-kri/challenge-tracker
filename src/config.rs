use std::collections::HashSet;

use derivative::Derivative;
use serde::Deserialize;
/// All code related to reading the `challenges.yaml` congiguration file.
use std::error::Error;

/// Enum that controls the output of the analytics.
#[derive(Debug, PartialEq)]
pub enum ReportType {
    /// Output the analytics in org-mode format.
    OrgMode,
    CLI,
}

impl Default for ReportType {
    fn default() -> Self {
        ReportType::CLI
    }
}

impl ReportType {
    pub fn new(type_str: &str) -> Self {
        match type_str {
            "org" => ReportType::OrgMode,
            _ => ReportType::CLI,
        }
    }
}

/// The struct that represents a chapter in this app. A chapter may be a chapter in a book, tutorial
/// etc. A chapter can have 1 or more topics. The topic is a concept to master.
/// On the filesystem, a chapter is a directory the same name as the chapter name with the
/// index prepended to it. For example, the chapter `basics` with index `1` will be `01.basics`.
#[derive(Debug, PartialEq, Deserialize)]
pub struct Chapter {
    pub number: u32,
    pub name: String,
    pub topics: Option<Vec<String>>,
}

/// Configuration for the app. This is read (typically) from the `challenges.yaml` file in the
/// root folder of the project.
#[derive(Debug, PartialEq, Deserialize, Derivative)]
pub struct Config {
    pub language: String,
    pub project: String,
    pub chapters: Vec<Chapter>,

    #[derivative(Default(value = "false"))]
    #[serde(skip_deserializing)]
    pub dirs_cached: bool,

    #[derivative(Default(value = "String::from(\".\")"))]
    #[serde(skip_deserializing)]
    pub root_dir: String,

    #[serde(skip_deserializing)]
    pub report_type: ReportType,
}

/// Implementation of the Config struct.
impl Config {
    /// Make the Config from the specified yaml file.
    ///
    /// # Arguments
    ///
    /// * `config_filename` - The name of the yaml file to read.
    ///
    /// # Returns
    ///
    /// * `Result<Config, Box<dyn Error>>` - The Config struct.
    pub fn from_file(config_filename: &str) -> Result<Config, Box<dyn Error>> {
        let config_file = std::fs::read_to_string(config_filename)?;
        let mut config: Config = serde_yaml::from_str(&config_file)?;

        config.root_dir = String::from(".");
        Ok(config)
    }

    pub fn from_path(project_path: &str) -> Result<Config, Box<dyn Error>> {
        let project_path = project_path.to_string();
        let config_path = format!("{}/challenges.yaml", project_path);

        let config_file = std::fs::read_to_string(config_path)?;
        let mut config: Config = serde_yaml::from_str(&config_file)?;

        config.root_dir = project_path;
        Ok(config)
    }

    /// Make read and make the Config, this time with the default filename - `challenges.yaml`.
    /// This is the most common use case.
    pub fn new() -> Result<Config, Box<dyn Error>> {
        Config::from_file("challenges.yaml")
    }

    /// Construct the desired directory structure for the challenges.
    ///
    /// # Returns
    ///
    /// * `Set<String>` - The set of directories corresponding to the challenges.
    pub fn computed_chapter_dirs(&self) -> HashSet<String> {
        let mut dirs = HashSet::new();
        for chapter in &self.chapters {
            let dir_name = format!("{:02}.{}", chapter.number, chapter.name);
            dirs.insert(dir_name);
        }
        dirs
    }
}

#[allow(unused_imports)]
mod tests {

    use super::*;

    // Check reading the config file, and deserializing it into a Config struct.
    #[test]
    fn test_read_config() {
        let basics = Chapter {
            number: 1,
            name: "basics".to_string(),
            topics: None,
        };
        let loops = Chapter {
            number: 2,
            name: "loops".to_string(),
            topics: None,
        };
        let config = Config {
            project: "Rust Language".to_string(),
            language: "rust".to_string(),
            chapters: vec![basics, loops],
            dirs_cached: false,
            root_dir: ".".to_string(),
            report_type: ReportType::CLI,
        };

        let actual = Config::from_file("tests/challenges/c1/basic.yaml").unwrap();
        assert_eq!(config, actual);
    }

    // Check if correct directories are computed from the config.
    #[test]
    fn test_computed_chapter_dirs() {
        let dirs = HashSet::from(["01.basics".to_string(), "02.loops".to_string()]);
        let config = Config::from_file("tests/challenges/c1/basic.yaml").unwrap();
        let actual = config.computed_chapter_dirs();

        assert_eq!(dirs, actual);
    }
}
