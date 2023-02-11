use std::fmt;

use nom::{IResult, sequence::tuple};
use nom::multi::many0;
use nom::bytes::complete::tag;
use crate::name::parse_name;
use crate::utils::class_id::ClassId;
use nom::sequence::{preceded, delimited, terminated};
use nom::character::complete::{multispace1, multispace0};


///member in class, not member variables in variable
#[derive(PartialEq)]
pub struct Var {
    pub name: String,
    pub inherits: Vec<ClassId>, //There are (class.id, class.name) in the vec 
    //TODO add parameter
}

impl fmt::Display for Var {
    fn fmt(&self, f:  &mut fmt::Formatter) -> fmt::Result {
        write!(f, "var {}", self.name)?;
        for inherit in &self.inherits {
            write!(f, " :{}", inherit.get_name())?;
        }
        Ok(())
    }
}

fn parse_var_member(i: &str) -> IResult<&str, Var> {
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
        .map(move |s| ClassId{id: s.to_owned()})
        .collect();
    let var_member = Var{name, inherits};
    Ok((remaining_input, var_member))
}

pub fn parse_vars(i: &str) -> IResult<&str, Vec<Var>> {
    Ok(many0(terminated(parse_var_member, multispace0))(i)?)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let name = "startPlace".to_owned();
        let inherit = ClassId{id: "Place".to_owned()};
        let inherits = vec![inherit];
        let var_member = Var {name, inherits};
        assert_eq!(format!("{}", var_member), "var startPlace :Place");
    }

    #[test]
    fn test2() {
        let file = "var startPlace: Place\n var endPlace: Place";
        let result = parse_vars(file).unwrap();
        let vars = result.1;
        assert_eq!(vars.len(), 2);
        let var_member = Var{
            name: "startPlace".to_owned(),
            inherits: vec![ClassId::from("Place")],
        };
        assert!(vars.get(0).unwrap() == &var_member);
    }
}