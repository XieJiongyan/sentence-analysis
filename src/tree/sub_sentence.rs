use nom::{IResult, combinator::map, branch::alt, sequence::tuple, bytes::complete::tag};

use super::{Tree, tagged::{Tagged, Tags}, verb::{vi_ps, vj_ps, vt_ps}, TreeType, TreeNode, noun};
use super::TreeNode::NotLeaf;

pub fn to_do(i: Tagged) -> IResult<Tagged, Tree> {
    Ok(map(
        tuple((
            tag(&Tags::from(vec!["TO"])),
            alt((
                vi_ps,
                vt_ps,
                vj_ps,
            )),
        )),
        |(to, v_ps)| Tree {
            tree_type: TreeType::ToDo as u64,
            tree_nodes: vec![TreeNode::from(to), NotLeaf(v_ps)],
        }
    )(i)?)
}

pub fn in_noun(i: Tagged) -> IResult<Tagged, Tree> {
    Ok(map(
        tuple((
            tag(&Tags::from(vec!["IN"])),
            noun::noun,
        )),
        |(_in, noun)| Tree {
            tree_type: TreeType::InNoun as u64,
            tree_nodes: vec![TreeNode::from(_in), NotLeaf(noun)]
        } 
    )(i)?)
}