use nom::{IResult, combinator::map, branch::alt, bytes::complete::tag, sequence::tuple};

use super::{tagged::{Tagged, Tags}, Tree, TreeType, TreeNode, noun};
use super::TreeNode::NotLeaf;

pub fn vi(i: Tagged) -> IResult<Tagged, Tree> {
    Ok(map(
        alt((
            tag(&Tags::from(vec!["VB"])),
            tag(&Tags::from(vec!["VBD"])),
            tag(&Tags::from(vec!["VBP"]))
        )),
        |x| Tree { 
            tree_type: TreeType::Vi as u64, 
            tree_nodes: Vec::<TreeNode>::from(x)
        }
    )(i)?)
}

pub fn vt(i: Tagged) -> IResult<Tagged, Tree> {
    Ok(map(
        tuple((
            vi,
            noun::noun
        )),
        |(vi, noun)| Tree {
            tree_type: TreeType::Vt as u64,
            tree_nodes: vec![NotLeaf(vi), NotLeaf(noun)]
        }
    )(i)?)
}

pub fn vj(i: Tagged) -> IResult<Tagged, Tree> {
    Ok(map(
        tuple((
            vi,
            tag(&Tags::from(vec!["JJ"])),
        )),
        |(vi, jj)| Tree {
            tree_type: TreeType::Vt as u64,
            tree_nodes: vec![NotLeaf(vi), TreeNode::from(jj)]
        }
    )(i)?)
}

//verb present simple
pub fn vi_ps(i: Tagged) -> IResult<Tagged, Tree> {
    Ok(map(
        alt((
            tag(&Tags::from(vec!["VB"])),
        )),
        |x| Tree { 
            tree_type: TreeType::Vi as u64, 
            tree_nodes: Vec::<TreeNode>::from(x)
        }
    )(i)?)
}

pub fn vt_ps(i: Tagged) -> IResult<Tagged, Tree> {
    Ok(map(
        tuple((
            vi_ps,
            noun::noun
        )),
        |(vi, noun)| Tree {
            tree_type: TreeType::Vt as u64,
            tree_nodes: vec![NotLeaf(vi), NotLeaf(noun)]
        }
    )(i)?)
}

pub fn vj_ps(i: Tagged) -> IResult<Tagged, Tree> {
    Ok(map(
        tuple((
            vi_ps,
            tag(&Tags::from(vec!["JJ"])),
        )),
        |(vi, jj)| Tree {
            tree_type: TreeType::Vt as u64,
            tree_nodes: vec![NotLeaf(vi), TreeNode::from(jj)]
        }
    )(i)?)
}
#[cfg(test)] 
mod tests {
    use super::*;

    #[test] 
    fn test1() {
        let tagged = vec![("played", "VBD"), ("games", "NNS")];
        let tagged = Tagged::from(tagged);
        let (x, vt) = vt(tagged).unwrap();
        assert!(x.start == x.end);
        assert_eq!(2, vt.tree_nodes.len());
    }
}