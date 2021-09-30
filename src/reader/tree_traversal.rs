use xmltree::Element;

fn print_header(level: u32, prompt: char, end: char) {
    for _ in 0..level {
        print!("{}", prompt);
    }
    print!("{}", end);
}

fn traverse_root(tree: &Element, level: u32) {
    println!("AUTOSAR");
    for item in &tree.children {
        traverse_tree(item.as_element().expect("Cannot be treated as element"), level + 2);
    }
}

fn traverse_packages(tree: &Element, level: u32) {
    for item in &tree.children {
        traverse_tree(item.as_element().expect("Invalid element"), level + 2);
    }
}

fn traverse_package(tree: &Element, level: u32) {
    print_header(level, '=', '>');
    println!(" AR-PACKAGE: {:?}", tree.get_child("SHORT-NAME").expect("Invalid element").get_text().expect("Does not have a text"));
    for item in &tree.children {
        if item.as_element().expect("Invalid element").name == "SHORT-NAME" {
            continue;
        }
        traverse_tree(item.as_element().expect("Invalid element"), level + 2);
    }
}

fn traverse_elements(tree: &Element, level: u32) {
    print_header(level - 2, ' ', ' ');
    println!(" ELEMENTS:");
    for item in &tree.children {
        traverse_tree(item.as_element().expect("Invalid element"), level);
    }
}

fn traverse_generics(tree: &Element, level: u32) {
    print_header(level + 2, ' ', ' ');
    print!("{}: ", tree.name);
    if let Some(value) = tree.get_text() {
        println!("{}", value);
        return;
    }
    if let Some(value) = tree.get_child("SHORT-NAME") {
        print!("{}", value.get_text().expect("Invalid value"));
    }
    if tree.attributes.len() != 0 {
        for (key, value) in &tree.attributes {
            print!(" | {} => {} |", key, value);
        }
        print!(":")
    }
    println!();
    for item in &tree.children {
        if let Some(subtree) = item.as_element() {
            match subtree.name.as_str() {
                "SHORT-NAME" => { continue; }
                _ => { traverse_generics(subtree, level + 4); }
            }
        }
    }
    //println!("{:?}", tree);
}

pub fn traverse_tree(tree: &Element, level: u32) {
    match tree.name.as_str() {
        "AUTOSAR" => {
            traverse_root(tree, level);
        }
        "AR-PACKAGES" => {
            traverse_packages(tree, level);
        }
        "AR-PACKAGE" => {
            traverse_package(tree, level);
        }
        "ELEMENTS" => {
            traverse_elements(tree, level);
        }
        _ => {
            traverse_generics(tree, level);
        }
    }
}
