use std::fmt;

use nom::{IResult, sequence::tuple};
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{multispace1, multispace0};
use nom::sequence::{preceded, delimited};
use nom::multi::many0;

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
        (_, _, class_name, inherits)
    ) = tuple((
        tag("class"),
        multispace1,
        class_name,
        many0 (preceded (
            delimited(multispace0, tag(":"), multispace0),
            class_name
        ))
    ))(i)?;

    let class_name = class_name.to_string();
    let id = class_name.clone();
    let inherits = inherits.iter().map(|c| c.to_string()).collect();
    Ok((remaining_input, Class{name: class_name, id, inherits}))
}

//FIXME Now cannot suport number
fn class_name(i: &str) -> IResult<&str, &str> {
    take_while1(|s: char| { s >= 'a' && s <= 'z' || s >= 'A' && s <= 'Z'})(i)
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
        let result = get_class("class NonCntr ").unwrap();
        let class = result.1;
        assert_eq!(format!("{}", class), "class NonCntr");
        assert_eq!(result.0, " ");
    }

    #[test]
    fn test3() {
        let file = "class ObjectInWorld: NonCntr";
        let result = get_class(file).unwrap();
        let class = result.1;
        assert_eq!(format!("{}", class), "class ObjectInWorld :NonCntr");
    }
}