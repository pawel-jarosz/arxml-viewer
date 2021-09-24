use std::path::Path;
use std::fs;

fn is_arxml(filename: &str) -> bool {
    if filename.ends_with(".arxml") {
        return true;
    }
    false
}

fn find_arxml(path: &Path, result: &mut Vec<String>) {
    if !path.is_dir() {
        return;
    }
    let entry = fs::read_dir(path).expect("Invalid path");
    for item in entry {
        let path = item.expect("Invalid path").path();
        if path.as_path().is_dir() {
            find_arxml(path.as_path(), result);
        }
        else {
            let filename = path.to_str().expect("Invalid string");
            if is_arxml(filename) {
                result.push(filename.to_string());
            }
        }
    }
}

fn main() {
    let mut arxml_files = Vec::new();
    find_arxml(Path::new("/home/caedus/Repos/sample-applications/cm_provider_subscriber_scenario"), &mut arxml_files);
    for item in arxml_files {
        println!("{}", item)
    }
}
