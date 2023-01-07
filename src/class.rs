use std::fmt;

use nom::{IResult, sequence::tuple};
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::multispace1;
pub struct Class {
    pub name: String,
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "class {}", self.name)?;
        Ok(())
    }
}

pub fn get_class(i: &str) -> IResult<&str, Class> {
    let (
        remaining_input,
        (_, _, class_name)
    ) = tuple((
        tag("class"),
        multispace1,
        class_name,
    ))(i)?;

    Ok((remaining_input, Class{name: class_name.to_string()}))
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
        let name = String::from("NonCntr");
        let class = Class{ name };
        assert_eq!(format!("{}", class), "class NonCntr");
    }

    #[test]
    fn test2() {
        let result = get_class("class NonCntr ").unwrap();
        let class = result.1;
        assert_eq!(format!("{}", class), "class NonCntr");
        assert_eq!(result.0, " ");
    }
}