use clap::{arg, command};

use challenge_checker::config_v1::Config;

fn main() {
    // clap parser with one argument the file name, with challenges.yaml as default
    let matches = command!()
        .arg(arg!(-f --filename <FOLDER> "Challenge config file").default_value("challenges.yaml"))
        .get_matches();

    // Extract the project path from cli args
    let config_filename = matches.get_one::<String>("filename").unwrap();

    // Read the v1 config file
    let config = Config::from_file(&config_filename).unwrap();

    // Convert to v1
    let config = config.to_v2();

    // serialize new config to yaml
    let yaml = serde_yaml::to_string(&config).unwrap();

    println!("{}", yaml);
}
