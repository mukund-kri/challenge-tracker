/// All filesystem related operations go here.
use std::collections::HashSet;

/// The directory that are not chapters like .git, .github, etc.
fn is_not_chapter(dir_name: &str) -> bool {
    !dir_name.starts_with('.')
}

pub fn list_directories(path: &str) -> HashSet<String> {
    let mut dirs = HashSet::new();
    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            let dir_name = path.file_name().unwrap().to_str().unwrap().to_string();

            // remove dot folders. they are typically hidden folders
            if is_not_chapter(&dir_name) {
                dirs.insert(dir_name);
            }
        }
    }
    dirs
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_directories() {
        let actual = list_directories("tests/challenges/c1");
        let expected = HashSet::from(["01.basics".to_string(), "02.loops".to_string()]);
        assert_eq!(expected, actual);
    }
}
