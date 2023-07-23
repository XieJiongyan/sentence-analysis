use std::fs;
use crate::pylib;
use crate::tree::Tree;

fn get_tree_from_file(file_name: String) -> Vec<Tree> {
    let contents = fs::read_to_string(file_name).expect("no such file");
    let sentence_with_tag = pylib::get_tag(&contents);
    vec![]
}