use core::fmt;

use nom::{IResult, branch::alt, combinator::map, sequence::terminated, character::complete::multispace0};

use crate::{cls::{Cls, parse_cls}, var::{Var, parse_var_member}};

#[derive(Clone)]
pub enum ClassMember {
    ClsMember(Cls),
    VarMember(Var),
}
pub fn parse_class_member(i: &str) -> IResult<&str, ClassMember> {
    alt((
        map(
            terminated(parse_var_member, multispace0),
            |e| {ClassMember::VarMember(e)}
        ),
        map(
            terminated(parse_cls, multispace0),
            |e| {ClassMember::ClsMember(e)}
        ),
    ))(i)
}

impl fmt::Display for ClassMember {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClassMember::ClsMember(cls) => write!(f, "{cls}"),
            ClassMember::VarMember(var) => write!(f, "{var}"),
        }
    }
}