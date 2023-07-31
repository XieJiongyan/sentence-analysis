use nom::{IResult, bytes::complete::tag, combinator::opt};

use super::{tagged::{Tagged, Tags}, Tree, TreeType, TreeNode, Leaf};
use super::TreeNode::IsLeaf;

pub fn adj(i: Tagged) -> IResult<Tagged, Tree> {
    let (remainging_input, adj) = tag(&Tags::from(vec!["JJ"]))(i)?;
    let adj = Tree{
        tree_type: TreeType::Adj as u64, 
        tree_nodes: Vec::<TreeNode>::from(adj),
    };
    Ok((remainging_input, adj))
}

#[cfg(test)]
mod tests {
    use crate::tree::tagged;

    use super::*;

    #[test]
    fn test1() {
        let tagged = vec![("better", "JJ"), ("one", "CD")];
        let tagged = Tagged::from(tagged);
        let (remaining_input, adj) = adj(tagged).unwrap();

        let expected_input = Tagged::from(vec![("one", "CD")]);
        assert_eq!(expected_input, remaining_input);

        let expected_tree = Tree{
            tree_type: TreeType::Adj as u64, 
            tree_nodes: vec![IsLeaf(Leaf::from(("better", "JJ")))],
        };
        assert_eq!(expected_tree, adj);
    }
    #[test]
    fn test2() {
        let tagged = vec![("my", "PRP$"), ("better", "JJ"), ("one", "CD")];
        let mut tagged = Tagged::from(tagged);
        tagged.start = 1;
        let (remaining_input, adj) = adj(tagged).unwrap();

        let expected_input = Tagged::from(vec![("one", "CD")]);
        assert_eq!(expected_input, remaining_input);

        let expected_tree = Tree{
            tree_type: TreeType::Adj as u64, 
            tree_nodes: vec![IsLeaf(Leaf::from(("better", "JJ")))],
        };
        assert_eq!(expected_tree, adj);
    }
}