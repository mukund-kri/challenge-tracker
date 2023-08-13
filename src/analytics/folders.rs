use std::collections::HashSet;

use crate::config::{Config, ReportType};
use crate::fileops::list_directories;
use crate::reports::cli::folder_status_report;
use crate::reports::orgmode::todo_do_chapter;

/// Entry point for the folders analytics. I matches the folders in the project with the
/// chapters in the config file and reports out the missing and extra folders.
///
/// There are two report types: `cli` and `org` ...
///
/// 1 :: The `cli` report type is the default and prints out the missing and extra folders to the
/// console.
///
/// 2 :: The `org` report type prints out the missing folders in the org-mode format as TODOs.
///
/// # Arguments
///
/// * `config` - The Config struct.
pub fn run(config: &Config) {
    let (missing_dirs, extra_dirs) = analyze(config);

    if config.report_type == ReportType::OrgMode {
        todo_do_chapter(&missing_dirs, config).unwrap();
    } else {
        folder_status_report(missing_dirs, extra_dirs);
    }
}

/// Compute the missing and extra folders.
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

    (missing_dirs, extra_dirs)
}

// Tests
#[allow(unused_imports)]
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
