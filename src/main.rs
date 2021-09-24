use std::path::Path;
use std::fs;
use std::fmt;
use yaml_rust::YamlLoader;

struct ProjectConfiguration {
    root_dir: String,
    excluded_dirs: Vec<String>
}

impl ProjectConfiguration {

fn new(filename: &str) -> ProjectConfiguration {
    let s =
    "
    root_dir: /home/caedus/Repos/sample-applications/cm_provider_subscriber_scenario
    excluded_dirs:
        - radar/
    ";
    let parsed_config = &YamlLoader::load_from_str(s).unwrap()[0];
    let mut excluded_dirs = Vec::new();
    for item in parsed_config["excluded_dirs"].as_vec().expect("Invalid excluded dirs list") {
        excluded_dirs.push(String::from(item.as_str().expect("Invalid excluded path")));
    }
    ProjectConfiguration {
        root_dir: String::from(parsed_config["root_dir"].as_str().expect("Invalid root directory")),
        excluded_dirs: excluded_dirs
    }
}

}

fn is_arxml(filename: &str) -> bool {
    if filename.ends_with(".arxml") {
        return true;
    }
    false
}

fn is_excluded_file(filename: &str, configuration: &ProjectConfiguration) -> bool {
    for item in &configuration.excluded_dirs {
        if filename.starts_with(&format!("{}/{}", configuration.root_dir, item)) {
            return true;
        }
    }
    false
}


fn find_arxml(path: &Path, is_excluded: &impl Fn(&str) -> bool, result: &mut Vec<String>) {
    if !path.is_dir() {
        return;
    }
    let entry = fs::read_dir(path).expect("Invalid path");
    for item in entry {
        let path = item.expect("Invalid path").path();
        if path.as_path().is_dir() {
            find_arxml(path.as_path(), is_excluded, result);
        }
        else {
            let filename = path.to_str().expect("Invalid string");
            if is_arxml(filename) && !is_excluded(filename) {
                result.push(filename.to_string());
            }
        }
    }
}

fn main() {
    let config = ProjectConfiguration::new("dupa");
    let mut arxml_files = Vec::new();
    let is_excluded = |filename: &str| is_excluded_file(filename, &config);
    find_arxml(Path::new("/home/caedus/Repos/sample-applications/cm_provider_subscriber_scenario"), &is_excluded, &mut arxml_files);
    for item in arxml_files {
        println!("{}", item)
    }
}
