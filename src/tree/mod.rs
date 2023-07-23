mod adj;
mod tagged;
use crate::tree::TreeNode::{IsLeaf, NotLeaf};
pub enum TreeType {
    Noun = 1,
    Adj = 2,
    Sentence = 4,
    Adv = 8, //状语从句，e.t. in the morning
    HasAdv = 16,
    HasCC = 32, //有 and, to 等连词
}
#[derive(Debug)]
pub struct Tree {
    tree_type: u64,
    tree_nodes: Vec<TreeNode>,
}
impl Tree {
    pub fn words(&self) -> String {
        let mut ret = String::new();
        for tree_node in &self.tree_nodes {
            ret += match tree_node {
                NotLeaf(x) => {
                    x.words()
                },
                IsLeaf(x) => {
                    x.word.clone() + " "
                },
            }.as_str();
        }
        ret
    }
}
impl PartialEq for Tree {
    fn eq(&self, other: &Self) -> bool {
        if self.tree_nodes.len() != other.tree_nodes.len() {
            return false;
        }
        
        self.tree_type == other.tree_type && self.tree_nodes == other.tree_nodes
    }
}
#[derive(Debug, PartialEq)]
struct Leaf {
    word: String,
    tag: String,
}

impl From<(&str, &str)> for Leaf {
    fn from((word, tag): (&str, &str)) -> Self {
        Self { word: word.to_owned() , tag: tag.to_owned() }
    }
}
#[derive(Debug, PartialEq)]
enum TreeNode {
    NotLeaf(Tree),
    IsLeaf(Leaf),
}  

#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let leaf1 = Leaf{tag: String::from("VB"), word: String::from("Got")};
        let leaf2 = Leaf{tag: String::from("DT"), word: String::from("a")};
        let leaf3 = Leaf{tag: String::from("NN"), word: String::from("book")};

        let tree1 = Tree{
            tree_type: TreeType::Noun as u64,
            tree_nodes: vec![TreeNode::IsLeaf(leaf2), TreeNode::IsLeaf(leaf3)], 
        };
        let tree2 = Tree {
            tree_type: TreeType::Sentence as u64,
            tree_nodes: vec![TreeNode::IsLeaf(leaf1), TreeNode::NotLeaf(tree1)],
        };
        println!("{}", tree2.words());
        assert_eq!("Got a book ", tree2.words());
    }
}