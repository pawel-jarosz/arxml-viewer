use std::path::Path;
use std::fs;
use std::fmt;

use crate::project_configuration::ProjectConfiguration;

fn is_arxml(filename: &str) -> bool {
    if filename.ends_with(".arxml") {
        return true;
    }
    false
}

pub fn is_excluded_file(filename: &str, configuration: &ProjectConfiguration) -> bool {
    for item in &configuration.excluded_dirs {
        if filename.starts_with(&format!("{}/{}", configuration.root_dir, item)) {
            return true;
        }
    }
    false
}

pub fn find_arxml(path: &str, is_excluded: &impl Fn(&str) -> bool, result: &mut Vec<String>) {
    let path = Path::new(path);
    if !path.is_dir() {
        return;
    }
    let entry = fs::read_dir(path).expect("Invalid path");
    for item in entry {
        let path = item.expect("Invalid path").path();
        if path.as_path().is_dir() {
            find_arxml(path.as_path().to_str().expect("Invalid characters"), is_excluded, result);
        }
        else {
            let filename = path.to_str().expect("Invalid string");
            if is_arxml(filename) && !is_excluded(filename) {
                result.push(filename.to_string());
            }
        }
    }
}
