use derivative::Derivative;
use serde::Deserialize;
/// All code related to reading the `challenges.yaml` congiguration file.
use std::error::Error;

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
    pub fn computed_chapter_dirs(&self) -> std::collections::HashSet<String> {
        let mut dirs = std::collections::HashSet::new();
        for chapter in &self.chapters {
            let dir_name = format!("{:02}.{}", chapter.number, chapter.name);
            dirs.insert(dir_name);
        }
        dirs
    }
}

mod tests {
    use std::collections::HashSet;

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
            language: "rust".to_string(),
            chapters: vec![basics, loops],
            dirs_cached: false,
            root_dir: ".".to_string(),
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
