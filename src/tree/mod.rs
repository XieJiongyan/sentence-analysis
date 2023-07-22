
pub enum TreeType {
    Noun = 1,
    Adj = 2,
    Sentence = 4,
    Adv = 8, //状语从句，e.t. in the morning
}
pub struct Tree {
    tree_type: u64,
    tree_nodes: Vec<TreeNode>,
}

pub struct Leaf {
    tag: String,
    word: String,
}
pub enum TreeNode {
    Tree(Tree),
    Leaf(Leaf),
}  

impl Tree {
    pub fn words(&self) -> String {
        let mut ret = String::new();
        for tree_node in &self.tree_nodes {
            ret += match tree_node {
                TreeNode::Tree(x) => {
                    x.words()
                },
                TreeNode::Leaf(x) => {
                    x.word.clone() + " "
                },
            }.as_str();
        }
        ret
    }
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
            tree_nodes: vec![TreeNode::Leaf(leaf2), TreeNode::Leaf(leaf3)], 
        };
        let tree2 = Tree {
            tree_type: TreeType::Sentence as u64,
            tree_nodes: vec![TreeNode::Leaf(leaf1), TreeNode::Tree(tree1)],
        };
        println!("{}", tree2.words());
        assert_eq!("Got a book ", tree2.words());
    }
}