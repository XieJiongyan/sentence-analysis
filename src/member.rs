use std::fmt;

use nom::{IResult, sequence::tuple};
use nom::multi::many0;
use nom::bytes::complete::{tag};
use crate::name::parse_name;
use nom::sequence::{preceded, delimited, terminated};
use nom::character::complete::{multispace1, multispace0};


///member in class, not member variables in variable
#[derive(PartialEq, Debug)]
pub struct VarMember {
    pub name: String,
    pub inherits: Vec<(String, String)>, //There are (class.id, class.name) in the vec 
    //TODO make it ClassId
}

impl fmt::Display for VarMember {
    fn fmt(&self, f:  &mut fmt::Formatter) -> fmt::Result {
        write!(f, "var {}", self.name)?;
        for inherit in &self.inherits {
            write!(f, " :{}", inherit.1)?;
        }
        Ok(())
    }
}

fn parse_var_member(i: &str) -> IResult<&str, VarMember> {
    let (
        remaining_input,
        (_, _, name, inherits)
    ) = tuple((
        tag("var"),
        multispace1,
        parse_name,
        many0 (preceded (
            delimited(multispace0, tag(":"), multispace0),
            parse_name
        )),
    ))(i)?;
    let inherits = inherits
        .iter()
        .map(move |s| (s.to_owned(), s.to_owned()))
        .collect();
    let var_member = VarMember{name, inherits};
    Ok((remaining_input, var_member))
}

pub fn parse_vars(i: &str) -> IResult<&str, Vec<VarMember>> {
    Ok(many0(terminated(parse_var_member, multispace0))(i)?)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let name = "startPlace".to_owned();
        let inherit = ("Place".to_owned(), "Place".to_owned());
        let inherits = vec![inherit];
        let var_member = VarMember {name, inherits};
        assert_eq!(format!("{}", var_member), "var startPlace :Place");
    }

    #[test]
    fn test2() {
        let file = "var startPlace: Place\n var endPlace: Place";
        let result = parse_vars(file).unwrap();
        let vars = result.1;
        assert_eq!(vars.len(), 2);
        let var_member = VarMember{
            name: "startPlace".to_owned(),
            inherits: vec![("Place".to_owned(), "Place".to_owned())],
        };
        assert_eq!(vars.get(0).unwrap(), &var_member);
    }
}