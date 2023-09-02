use clap::{arg, command, Parser};

use challenge_checker::analytics::folders::run as run_folder_analytics;
use challenge_checker::analytics::readme::run as run_readme_analytics;
use challenge_checker::config::{Config, ReportType};

/// Run analytics on my coding challenges
#[derive(Debug, Parser)]
#[command(author, about, version)]
struct Args {
    /// Path to the challenge folder    
    #[clap(short, long, default_value = ".")]
    path: String,
}

fn main() {
    let matches = command!()
        .subcommand(command!("folders").about("Check if the folders are correct"))
        .subcommand(command!("readme").about("Check if the README is correct"))
        .arg(arg!(-p --path <FOLDER> "Path to the challenge folder").default_value("."))
        .arg(
            arg!(-r --report <MODE> "Report on the challenges")
                .value_parser(["cli", "org"])
                .default_value("cli"),
        )
        .get_matches();

    // Extract the report mode from cli args
    let report_mode = matches.get_one::<String>("report").unwrap();
    let path = matches.get_one::<String>("path").unwrap();

    // Read the config file
    let mut config = Config::from_path(&path.as_str()).unwrap();
    config.report_type = ReportType::new(&report_mode);
    println!("Report mode: {}", report_mode);

    // match with sub command
    match matches.subcommand() {
        Some(("folders", _)) => {
            run_folder_analytics(&config);
        }
        Some(("readme", _)) => {
            run_readme_analytics(&config);
        }
        _ => {
            panic!("No subcommand was used");
        }
    };
}
