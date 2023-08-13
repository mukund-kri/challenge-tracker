use std::collections::HashSet;

extern crate termion;
use termion::{color, style};

/// Report the missing and extra folders to the console in a human readable format with colors.
///
/// # Arguments
///
/// * `missing_dirs` - (HashSet<String>) The missing directories.
/// * `extra_dirs`   - (HashSet<String>) The extra directories.
pub fn folder_status_report(missing_dirs: HashSet<String>, extra_dirs: HashSet<String>) {
    if missing_dirs.is_empty() && extra_dirs.is_empty() {
        println!(
            "{}{}All good!{}",
            color::Fg(color::Green),
            style::Bold,
            style::Reset
        );
    } else {
        if !missing_dirs.is_empty() {
            let no_missing_dirs = missing_dirs.len();

            println!(
                "\n{}{}Error: Found {} Missing directories{}",
                color::Fg(color::Red),
                style::Bold,
                no_missing_dirs,
                style::Reset
            );

            // List the missing directories
            for missing_dir in &missing_dirs {
                println!("{}", missing_dir);
            }
        }

        if !extra_dirs.is_empty() {
            let no_extra_dirs = extra_dirs.len();

            println!(
                "\n{}{}Found {} Extra directories{}",
                color::Fg(color::Red),
                style::Bold,
                no_extra_dirs,
                style::Reset
            );

            // List the extra directories
            for extra_dir in extra_dirs {
                println!(
                    "{}{}{}{}",
                    color::Fg(color::Red),
                    style::Bold,
                    extra_dir,
                    style::Reset
                );
            }
        }
    }
}
