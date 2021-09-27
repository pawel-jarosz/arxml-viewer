use arxml_viewer::project_configuration as config;
use arxml_viewer::utilities::arxml_explorer as arxml_explorer;
use arxml_viewer::reader::tree_traversal::traverse_tree;
use xmltree::Element;
use std::fs;

fn get_arxml_list(configuration: &config::ProjectConfiguration) -> Vec<String> {
    let mut arxml_files = Vec::new();
    let is_excluded = |filename: &str| arxml_explorer::is_excluded_file(filename, configuration);
    arxml_explorer::find_arxml("/home/caedus/Repos/sample-applications/cm_provider_subscriber_scenario", &is_excluded, &mut arxml_files);
    arxml_files
}

fn main() {
    let config = config::ProjectConfiguration::new("dupa");
    let arxml_files = get_arxml_list(&config);
    for filename in &arxml_files {
        println!("Filename: {}", filename);
        let file_content = fs::read_to_string(&filename).expect("Error during reading a file");
        let tree = Element::parse(file_content.as_bytes()).expect("Invalid xml file");    
        traverse_tree(&tree, 0);
    }
}
