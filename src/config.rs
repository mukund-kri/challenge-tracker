use derivative::Derivative;
use serde::Deserialize;
/// All code related to reading the `challenges.yaml` congiguration file.
use std::error::Error;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Chapter {
    pub number: u32,
    pub name: String,
    pub topics: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Deserialize, Derivative)]
pub struct Config {
    pub language: String,
    pub chapters: Vec<Chapter>,

    #[derivative(Default(value = "false"))]
    #[serde(skip_deserializing)]
    pub dirs_cached: bool,
}

pub fn read_config(config_filename: &str) -> Result<Config, Box<dyn Error>> {
    let config_file = std::fs::read_to_string(config_filename)?;
    let config: Config = serde_yaml::from_str(&config_file)?;

    Ok(config)
}

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
            language: "rust".to_string(),
            chapters: vec![basics, loops],
            dirs_cached: false,
        };

        let actual = read_config("tests/challenges/c1/basic.yaml").unwrap();
        assert_eq!(config, actual);
    }
}
