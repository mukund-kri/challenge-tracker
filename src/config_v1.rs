/// A copy of v1 version of Configuration structs. Its only purpose now is for the transformer
/// program to migrate from v1 to v2
///
use derivative::Derivative;
use serde::Deserialize;
/// All code related to reading the `challenges.yaml` configuration file.
use std::{collections::BTreeMap, error::Error};

use crate::config::{Chapter as ChapterV2, Config as ConfigV2};

/// The struct that represents a chapter in this app. A chapter may be a chapter in a book, tutorial
/// etc. A chapter can have 1 or more topics. The topic is a concept to master.
/// On the filesystem, a chapter is a directory the same name as the chapter name with the
/// index prepended to it. For example, the chapter `basics` with index `1` will be `01.basics`.
#[derive(Debug, PartialEq, Deserialize)]
pub struct Chapter {
    pub number: i32,
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

    /// Conversion to v2
    pub fn to_v2(&self) -> ConfigV2 {
        let mut chapters = BTreeMap::new();
        for chapter in &self.chapters {
            let new_chapter = ChapterV2 {
                name: chapter.name.clone(),
                topics: chapter.topics.clone(),
            };
            let index = format!("{:02}", chapter.number);
            chapters.insert(index, new_chapter);
        }

        ConfigV2 {
            language: self.language.clone(),
            project: self.project.clone(),
            chapters,
            dirs_cached: self.dirs_cached,
            root_dir: self.root_dir.clone(),
            report_type: crate::config::ReportType::CLI,
        }
    }
}

#[allow(unused_imports)]
mod tests {

    use std::vec;

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
        };

        let actual = Config::from_file("tests/challenges/c1/challenges_v1.yaml").unwrap();
        assert_eq!(config, actual);
    }
}
