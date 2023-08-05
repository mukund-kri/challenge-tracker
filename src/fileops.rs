/// All filesystem related operations go here.
use std::collections::HashSet;

pub fn list_directories(path: &str) -> HashSet<String> {
    let mut dirs = HashSet::new();
    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            let dir_name = path.file_name().unwrap().to_str().unwrap().to_string();
            dirs.insert(dir_name);
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
