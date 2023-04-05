use std::fmt;

use nom::branch::{permutation, alt};
use nom::{IResult, sequence::tuple};
use nom::bytes::complete::tag;
use nom::character::complete::{multispace1, multispace0};
use nom::sequence::{preceded, delimited, terminated};
use nom::multi::many0;
use nom::combinator::{opt, map};

use crate::cls::{Cls, parse_cls};
use crate::var::{Var, parse_vars, parse_var_member};
use crate::name::parse_name;
use crate::utils::package_name::PackageName;

pub struct Class {
    pub name: String,
    pub inherits: Vec<PackageName>, 
    pub vars: Vec<Var>,
    pub cls_iz : Vec<PackageName>,
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "class {}", self.name)?;
        for inherit in &self.inherits {
            write!(f, " :{}", inherit.get_name())?;
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

///在这一步中, cls 设置好 super_cid
pub fn parse_class(i: &str) -> IResult<&str, (Class, Vec<Cls>)> {
    let (
        remaining_input,
        (_, _, class_name, inherits, members)
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
            parse_class_member, 
            tuple((multispace0, tag("}"), multispace0)),
        )),
    ))(i)?;

    let inherits = inherits
        .iter()
        .map(|c| PackageName::from(c.to_string())) //FIXME get right class ID
        .collect();
    let vars = vars.unwrap_or(vec![]);
    let class = Class{name: class_name, inherits, vars};
    Ok((remaining_input, class))
}

enum ClassMember {
    ClsMember(Cls),
    VarMember(Var),
}
fn parse_a_class_member(i: &str) -> IResult<&str, ClassMember> {
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
fn parse_class_member(i: &str) -> IResult<&str, (Vec<Var>, Vec<Cls>)> {
    let (remaining_output, class_members) = many0(parse_a_class_member)(i)?;

    let mut var_vec = vec![];
    let mut cls_vec = vec![];

    for cls_member in class_members {
        match cls_member {
            ClassMember::ClsMember(cls) => {cls_vec.push(cls);},
            ClassMember::VarMember(var) => {var_vec.push(var);},
        }
    }
    Ok((remaining_output, (var_vec, cls_vec)))
}
impl Class {
    /// 预计是 package.name 格式
    /// 
    pub fn get_package_name(&self) -> PackageName {
        return PackageName::from(format!("base.{}", self.name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let name = String::from("ObjectInWord");
        let inherits = vec!["NonCntr"];
        let inherits = inherits
            .iter()
            .map(|c| PackageName::from(c.to_string()))
            .collect();
        let vars = vec![];
        let class = Class{ name, inherits, vars };
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
        let inherits = vec![PackageName::from("PeopleAction")];
        let vars = vec![
            Var{
                name: "startPlace".to_owned(),
                inherits: vec![PackageName::from("common.place.Place")]
            },
            Var{
                name: "endPlace".to_owned(),
                inherits: vec![PackageName::from("common.place.Place")]
            },
            Var{
                name: "vehicle".to_owned(),
                inherits: vec![PackageName::from("common.traffic.Vehicle")]
            },
        ];
        let class = Class{name, inherits, vars};
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