use crate::config::Config;
use crate::fileops::list_directories;

pub fn run(config: &Config) {
    println!("\n\nRunning analytics on the README");
    check_readmes(config);
}

/// Check if the READMEs are present in all the folders.
fn check_readmes(config: &Config) {
    println!("Checking READMEs");

    let existing_dirs = list_directories(config.root_dir.as_str());

    let mut existing_dirs: Vec<String> = existing_dirs.into_iter().collect();
    existing_dirs.sort();

    // iterate through the dirs and check if the READMEs are present
    for dir in existing_dirs {
        let readme_path = format!("{}/{}", config.root_dir, dir);
        let readme_path = format!("{}/README.md", readme_path);
        if !std::path::Path::new(&readme_path).exists() {
            println!("README missing in {}", dir);
        }
    }
}
