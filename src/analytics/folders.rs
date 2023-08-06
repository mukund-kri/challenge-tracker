use std::collections::HashSet;

extern crate termion;
use termion::{color, style};

use crate::config::Config;
use crate::fileops::list_directories;

pub fn run(config: &Config) {
    let (missing_dirs, extra_dirs) = analyze(config);
    report(missing_dirs, extra_dirs);
}

fn analyze(config: &Config) -> (HashSet<String>, HashSet<String>) {
    let existing_dirs = list_directories(config.root_dir.as_str());

    let needed_dirs = config.computed_chapter_dirs();

    let missing_dirs = needed_dirs
        .difference(&existing_dirs)
        .map(|s| s.to_string())
        .collect::<HashSet<_>>();
    let extra_dirs = existing_dirs
        .difference(&needed_dirs)
        .map(|s| s.to_string())
        .collect::<HashSet<_>>();

    println!("Missing dirs: {:?}", missing_dirs);
    println!("Extra dirs: {:?}", extra_dirs);

    (missing_dirs, extra_dirs)
}

fn report(missing_dirs: HashSet<String>, extra_dirs: HashSet<String>) {
    if missing_dirs.is_empty() && extra_dirs.is_empty() {
        println!(
            "{}{}All good!{}",
            color::Fg(color::Green),
            style::Bold,
            style::Reset
        );
    } else {
        if !missing_dirs.is_empty() {
            println!(
                "{}{}Missing directories: {:?}{}",
                color::Fg(color::Red),
                style::Bold,
                missing_dirs,
                style::Reset
            );
        }
        if !extra_dirs.is_empty() {
            println!(
                "{}{}Extra directories: {:?}{}",
                color::Fg(color::Red),
                style::Bold,
                extra_dirs,
                style::Reset
            );
        }
    }
}

// Tests
mod tests {
    use super::*;

    #[test]
    fn test_analyze() {
        let mut config = Config::from_file("tests/challenges/c1/basic.yaml").unwrap();

        config.root_dir = "tests/challenges/c1".to_string();

        let (missing_dirs, extra_dirs) = analyze(&config);

        let expected_missing_dirs = HashSet::new();
        let expected_extra_dirs = HashSet::new();

        assert_eq!(expected_missing_dirs, missing_dirs);
        assert_eq!(expected_extra_dirs, extra_dirs);
    }
}
