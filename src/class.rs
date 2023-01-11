use std::fmt;

use nom::{IResult, sequence::tuple};
use nom::bytes::complete::{tag, take_while1, is_a, take_while_m_n, take_while};
use nom::character::complete::{multispace1, multispace0};
use nom::sequence::{preceded, delimited};
use nom::multi::many0;
use nom::combinator::{opt};

pub struct Class {
    pub name: String,
    pub id  : String,
    pub inherits: Vec<String>,
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "class {}", self.name)?;
        for inherit in &self.inherits {
            write!(f, " :{}", inherit)?;
        }
        Ok(())
    }
}

pub fn get_class(i: &str) -> IResult<&str, Class> {
    let (
        remaining_input,
        (_, _, class_name, inherits, _)
    ) = tuple((
        tag("class"),
        multispace1,
        class_name,
        many0 (preceded (
            delimited(multispace0, tag(":"), multispace0),
            class_name
        )),
        opt (delimited(
            delimited(multispace0, tag("{"), multispace0), 
            multispace0,
            delimited(multispace0, tag("}"), multispace0),
        )),
    ))(i)?;

    let id = class_name.clone(); //FIXME
    let inherits = inherits.iter().map(|c| c.to_string()).collect();
    Ok((remaining_input, Class{name: class_name, id, inherits}))
}

fn class_name(i: &str) -> IResult<&str, String> {
    let cond1 = |s: char| {
        s >= 'a' && s <= 'z' || s >= 'A' && s <= 'Z' || s == '_'
    };
    let (i, s0) = take_while_m_n(1, 1, cond1)(i)?;

    let cond2 = |s: char| {cond1(s) || s >= '0' && s <= '9'};
    let (remaining_input, s1) = take_while(cond2)(i)?;

    Ok((remaining_input, format!("{}{}", s0, s1)))
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
        let class = Class{ name, id, inherits };
        assert_eq!(format!("{}", class), "class ObjectInWord :NonCntr");
    }

    #[test]
    fn test2() {
        let result = get_class("class NonCntr1 ").unwrap();
        let class = result.1;
        assert_eq!(format!("{}", class), "class NonCntr1");
        assert_eq!(result.0, " ");
    }

    #[test]
    fn test3() {
        let file = "class ObjectInWorld: NonCntr {}";
        let result = get_class(file).unwrap();
        let class = result.1;
        assert_eq!(format!("{}", class), "class ObjectInWorld :NonCntr");
    }
}