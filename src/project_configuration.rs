use yaml_rust::YamlLoader;

pub struct ProjectConfiguration {
    pub root_dir: String,
    pub excluded_dirs: Vec<String>
}

impl ProjectConfiguration {

pub fn new(filename: &str) -> ProjectConfiguration {
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
