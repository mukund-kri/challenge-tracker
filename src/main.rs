use clap::{arg, command, Parser};

use challenge_checker::analytics::folders::run;
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
        .arg(arg!(-p --path <FOLDER> "Path to the challenge folder").default_value("."))
        .arg(
            arg!(-r --report <MODE> "Report on the challenges")
                .value_parser(["cli", "org"])
                .default_value("cli"),
        )
        .get_matches();

    // Extract the project path from cli args
    let path = matches.get_one::<String>("path").unwrap();

    // Extract the report mode from cli args
    let report_mode = matches.get_one::<String>("report").unwrap();
    println!("Report mode: {}", report_mode);

    let mut config = Config::from_path(&path.as_str()).unwrap();
    config.report_type = ReportType::new(&report_mode);
    run(&config);
}
