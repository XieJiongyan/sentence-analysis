use nom::{IResult, branch::alt, bytes::complete::tag, sequence::tuple, combinator::opt};

use super::{Tree, tagged::{Tagged, Tags}, TreeType, adj, TreeNode};
use crate::tree::TreeNode::NotLeaf;

pub fn noun(i: Tagged) -> IResult<Tagged, Tree> {
    Ok(alt((
        noun_not_single,
        noun_single
    ))(i)?)
}

fn noun_not_single(i: Tagged) -> IResult<Tagged, Tree> {
    let (remaining_output, x) = tuple((
        dt,
        opt(adj::adj),
        alt((
            tag(&Tags::from(vec!["CD"])),
            tag(&Tags::from(vec!["NN"])),
            tag(&Tags::from(vec!["NNS"])),            
        ))
    ))(i)?;
    assert!(!(x.0.get_start().tag == "CD" && x.2.get_start().tag == "CD"));
    let mut tree_nodes = Vec::<TreeNode>::from(x.0);
    if let Some(tree) = x.1 {
        tree_nodes.push(NotLeaf(tree));
    }
    tree_nodes.append(&mut Vec::<TreeNode>::from(x.2));
    Ok((remaining_output, Tree {tree_type: TreeType::Noun as u64, tree_nodes}))
}

fn dt(i: Tagged) -> IResult<Tagged, Tagged> {
    Ok(alt((
        tag(&Tags::from(vec!["DT"])),
        tag(&Tags::from(vec!["CD"])),
        tag(&Tags::from(vec!["PRP$"])),
    ))(i)?)
}

fn noun_single(i: Tagged) -> IResult<Tagged, Tree> {
    let (remaining_output, noun_single) = alt((
        tag(&Tags::from(vec!["DT"])),
        tag(&Tags::from(vec!["CD"])),
        tag(&Tags::from(vec!["PRP"])),
        tag(&Tags::from(vec!["NN"])),
        tag(&Tags::from(vec!["NNS"])),
    ))(i)?;
    Ok((
        remaining_output, 
        Tree {tree_type: TreeType::Noun as u64, tree_nodes: Vec::<TreeNode>::from(noun_single)}
    ))
}

#[cfg(test)]
mod tests {
    

    use super::*;

    #[test]
    fn test1() {
        let tagged = vec![("the", "DT"), ("dogs", "NNS")];
        let tagged = Tagged::from(tagged);
        let (_, noun) = noun(tagged).unwrap();
        assert_eq!(TreeType::Noun as u64, noun.tree_type);
        assert_eq!(2, noun.tree_nodes.len());
    }

    #[test]
    fn test2() {
        let tagged = vec![("the", "DT"), ("beautyful", "JJ"), ("dog", "NNS"), ("runs", "VBZ")];
        let tagged = Tagged::from(tagged);
        let (tagged, noun) = noun(tagged).unwrap();
        assert_eq!(TreeType::Noun as u64, noun.tree_type);
        assert_eq!(3, noun.tree_nodes.len());
        assert_eq!(3, tagged.start)
    }
}