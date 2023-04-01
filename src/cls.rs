use std::fmt;

use nom::{IResult, sequence::{tuple, terminated, delimited, preceded}, character::complete::{multispace1, multispace0}, multi::{many1, many0}, bytes::complete::tag};

use crate::{utils::package_name::PackageName, name::parse_name};

pub struct Cls {
    pub name :String,
    pub super_cid : PackageName,
    pub inherit_ciz :Vec<PackageName>, //inherit_class_ids
    pub parameter_ciz :Vec<PackageName>, //parameter_class_ids
}

impl fmt::Display for Cls {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "cls {}(", self.name);
        let parameters = "";
        for i in 0..self.parameter_ciz.len() {
            let comma = if i != self.parameter_ciz.len() - 1 {
                ", "
            } else {
                ""
            };
            write!(f, "{}{}", self.parameter_ciz[i].get_name(), comma);
        }
        write!(f, ")");
        for i in 0..self.inherit_ciz.len() {
            write!(f, ": {}", self.inherit_ciz[i].get_name());
        }
        Ok(())
    }
}

fn parse_cls(i: &str) -> IResult<&str, Cls> {
    let (
        remaining_input,
        (_, _, name, _, parameters, _, inherits)
    ) = tuple((
        tag("cls"),
        multispace1,
        parse_name,
        delimited(
            multispace0,
            tag("("),
            multispace0,
        ),
        many0 (
            terminated(
                parse_name, 
                delimited(multispace0, tag(","), multispace0)
            )
        ),
        tag(")"),
        many1(
            preceded(
                delimited(multispace0, tag(":"), multispace0),
                parse_name
            )
        )
    ))(i)?;

    let parameters = parameters
        .iter()
        .map(move |s| PackageName{id: s.to_owned()})
        .collect();
    let inherits = inherits
        .iter()
        .map(move |s| PackageName{id: s.to_owned()})
        .collect();
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let name = "Deny".to_string();
        let inherit_ciz = vec![PackageName::from("PeopleAction")];
        let parameter_ciz = vec![PackageName::from("Proposition")];
        let super_cid = PackageName::from("People");
        let cls = Cls {
            name,
            super_cid,
            inherit_ciz,
            parameter_ciz,
        };

        let expected = "cls Deny(Proposition) :PeopleAction";

        assert_eq!(expected, format!("{}", cls))
    }
}