use std::fmt;

use nom::{IResult, sequence::{tuple, terminated, delimited, preceded}, character::complete::{multispace1, multispace0}, multi::{many1, many0}, bytes::complete::tag};

use crate::{utils::class_id::ClassId, name::parse_name};

#[derive(Clone)]
pub struct Cls {
    pub name :String,
    pub super_cid : Option<ClassId>,
    pub inherit_ciz :Vec<ClassId>, //inherit_class_ids
    pub parameter_ciz :Vec<ClassId>, //parameter_class_ids
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

///
/// 经过 parse_cls 之后 super_cid 还为空
pub fn parse_cls(i: &str) -> IResult<&str, Cls> {
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

    let parameter_ciz = parameters
        .iter()
        .map(move |s| ClassId::from(s))
        .collect();
    let inherit_ciz = inherits
        .iter()
        .map(move |s| ClassId::from(s))
        .collect();

    let cls = Cls {
        name,
        super_cid : None,
        inherit_ciz,
        parameter_ciz
    };
    Ok((remaining_input, cls))
    
}

impl Cls {
    pub fn get_id(&self) -> ClassId {
        let mut package_name = ClassId::from(&self.name);
        package_name.super_cid = Some(self.super_cid.as_ref().unwrap().get_name().to_owned());
        package_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let name = "Deny".to_string();
        let inherit_ciz = vec![ClassId::from("PeopleAction")];
        let parameter_ciz = vec![ClassId::from("Proposition")];
        let super_cid = Some(ClassId::from("People"));
        let cls = Cls {
            name,
            super_cid,
            inherit_ciz,
            parameter_ciz,
        };

        let expected = "cls Deny(Proposition) :PeopleAction";

        assert_eq!(expected, format!("{}", cls))
    }

    #[test]
    fn test2() {
        let file = "cls Deny(Proposition)  :PeopleAction";
        let (remaining_input, cls) = parse_cls(file).unwrap();
        assert_eq!(remaining_input, "");
        assert_eq!(cls.name, "Deny");
        assert!(cls.super_cid == None);
        assert_eq!(cls.inherit_ciz.len(), 1);
        assert_eq!(cls.inherit_ciz[0].get_name(), "PeopleAction");
        assert_eq!(cls.parameter_ciz.len(), 1);
        assert_eq!(cls.parameter_ciz[0].get_name(), "Proposition");
    }
}