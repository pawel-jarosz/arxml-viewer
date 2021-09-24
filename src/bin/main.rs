use arxml_viewer::project_configuration as config;
use arxml_viewer::utilities::arxml_explorer as arxml_explorer;

fn main() {
    let config = config::ProjectConfiguration::new("dupa");
    let mut arxml_files = Vec::new();
    let is_excluded = |filename: &str| arxml_explorer::is_excluded_file(filename, &config);
    arxml_explorer::find_arxml("/home/caedus/Repos/sample-applications/cm_provider_subscriber_scenario", &is_excluded, &mut arxml_files);
    for item in arxml_files {
        println!("{}", item)
    }
}
