use std::fmt;

use nom::{IResult, sequence::tuple};
use nom::bytes::complete::tag;
use nom::character::complete::{multispace1, multispace0};
use nom::sequence::{preceded, delimited};
use nom::multi::many0;
use nom::combinator::opt;

use crate::member::{VarMember, parse_vars};
use crate::name::parse_name;

pub struct Class {
    pub name: String,
    pub id  : String,
    pub inherits: Vec<String>, //TODO make it ClassId
    pub vars: Vec<VarMember>
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "class {}", self.name)?;
        for inherit in &self.inherits {
            write!(f, " :{}", inherit)?;
        }
        if self.vars.len() > 0 {
            write!(f, " {{\n")?;
            for var_member in &self.vars {
                write!(f, "  {}\n", var_member)?;
            }
            write!(f, "}}")?;
        }
        Ok(())
    }
}

pub fn parse_class(i: &str) -> IResult<&str, Class> {
    let (
        remaining_input,
        (_, _, class_name, inherits, vars)
    ) = tuple((
        tag("class"),
        multispace1,
        parse_name,
        many0 (preceded (
            delimited(multispace0, tag(":"), multispace0),
            parse_name
        )),
        opt (delimited(
            tuple((multispace0, tag("{"), multispace0)), 
            parse_vars, 
            tuple((multispace0, tag("}"), multispace0)),
        )),
    ))(i)?;

    let id = class_name.clone(); //FIXME get right id
    let inherits = inherits.iter().map(|c| c.to_string()).collect();
    let vars = vars.unwrap_or(vec![]);
    let class = Class{name: class_name, id, inherits, vars};
    Ok((remaining_input, class))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let name = String::from("ObjectInWord");
        let id = String::from("basic::ObjectInWord");
        let inherits = vec!["NonCntr"];
        let inherits = inherits.iter().map(|c| c.to_string()).collect();
        let vars = vec![];
        let class = Class{ name, id, inherits, vars };
        assert_eq!(format!("{}", class), "class ObjectInWord :NonCntr");
    }

    #[test]
    fn test2() {
        let result = parse_class("class NonCntr1 ").unwrap();
        let class = result.1;
        assert_eq!(format!("{}", class), "class NonCntr1");
        assert_eq!(result.0, " ");
    }

    #[test]
    fn test3() {
        let file = "class ObjectInWorld: NonCntr {}";
        let result = parse_class(file).unwrap();
        let class = result.1;
        assert_eq!(format!("{}", class), "class ObjectInWorld :NonCntr");
    }

    /// Test display for vars
    #[test]
    fn test4() {
        let name = "TakeTraffic".to_owned();
        let id = "common.traffic.TakeTraffic".to_owned();
        let inherits = vec!["PeopleAction".to_owned()];
        let vars = vec![
            VarMember{
                name: "startPlace".to_owned(),
                inherits: vec![("common.place.Place".to_owned(), "Place".to_owned())]
            },
            VarMember{
                name: "endPlace".to_owned(),
                inherits: vec![("common.place.Place".to_owned(), "Place".to_owned())]
            },
            VarMember{
                name: "vehicle".to_owned(),
                inherits: vec![("common.traffic.Vehicle".to_owned(), "Vehicle".to_owned())]
            },
        ];
        let class = Class{name, id, inherits, vars};
        let expected = "
class TakeTraffic :PeopleAction {
  var startPlace :Place
  var endPlace :Place
  var vehicle :Vehicle
}
        ".trim();
        let output = format!("{}", class);
        let output = output.trim();
        assert_eq!(expected, output);
    }

    #[test]
    fn test5() {
        let file = "
class TakeTraffic :PeopleAction {
  var startPlace :Place
  var endPlace :Place
  var vehicle :Vehicle
}
        ".trim();
        let result = parse_class(file).unwrap();
        let class = result.1;
        let output = format!("{}", class);
        let output = output.trim();
        assert_eq!(output, file);
    }
}