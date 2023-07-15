use std::fmt;

use nom::branch::alt;
use nom::{IResult, sequence::tuple};
use nom::bytes::complete::tag;
use nom::character::complete::{multispace1, multispace0};
use nom::sequence::{preceded, delimited, terminated};
use nom::multi::many0;
use nom::combinator::{opt, map};

use crate::class_member::{parse_class_member, ClassMember};
use crate::cls::Cls;
use crate::name::parse_name;
use crate::utils::class_id::ClassId;

pub struct Class {
    pub name: String,
    pub inherits: Vec<ClassId>, 
    pub members: Vec<ClassMember>, //使用时可以直接用, 
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "class {}", self.name)?;
        for inherit in &self.inherits {
            write!(f, " :{}", inherit.get_name())?;
        }
        if self.members.len() > 0 {
            write!(f, " {{\n")?;
            for member in &self.members {
                write!(f, "  {}\n", member)?;
            }
            write!(f, "}}")?;
        }
        Ok(())
    }
}

impl Class {
    pub fn get_cid(&self) -> ClassId {
        ClassId::from(&self.name)
    }
}

/// 在这一步中, cls 设置好 super_cid
/// 返回 Class 和他所有的 Cls
pub fn parse_class(i: &str) -> IResult<&str, (Class, Vec<ClassId>)> {
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
            parse_class_members, 
            tuple((multispace0, tag("}"), multispace0)),
        )),
    ))(i)?;

    let inherits = inherits
        .iter()
        .map(|c| ClassId::from(c)) //FIXME get right class ID
        .collect();
    let members = members.unwrap_or(vec![]);
    let mut cls_iz: Vec<ClassId> = vec![];

    let members: Vec<ClassMember> = members
        .iter()
        .map(|c| match c {
            ClassMember::ClsMember(cls) => {
                let mut cls = cls.clone();
                cls.super_cid = Some(ClassId::from(&class_name));
                cls_iz.push(cls.get_id());
                ClassMember::ClsMember(cls)
            }
            x => x.clone(),
        })
        .collect();
    let class = Class{name: class_name, inherits, members};
    Ok((remaining_input, (class, cls_iz)))
}

fn parse_class_members(i: &str) -> IResult<&str, Vec<ClassMember>> {
    many0(parse_class_member)(i)
}
impl Class {
    /// 预计是 package.name 格式
    /// 
    pub fn get_package_name(&self) -> ClassId {
        return ClassId::from(&format!("base.{}", self.name))
    }
}

#[cfg(test)]
mod tests {
    use crate::var::Var;

    use super::*;

    #[test]
    fn test1() {
        let name = String::from("ObjectInWord");
        let inherits = vec!["NonCntr"];
        let inherits = inherits
            .iter()
            .map(|c| ClassId::from(*c))
            .collect();
        let members = vec![];
        let class = Class{ name, inherits, members,};
        assert_eq!(format!("{}", class), "class ObjectInWord :NonCntr");
    }

    #[test]
    fn test2() {
        let result = parse_class("class NonCntr1 ").unwrap();
        let class = result.1.0;
        assert_eq!(format!("{}", class), "class NonCntr1");
        assert_eq!(result.0, " ");
    }

    #[test]
    fn test3() {
        let file = "class ObjectInWorld: NonCntr {}";
        let result = parse_class(file).unwrap();
        let class = result.1.0;
        assert_eq!(format!("{}", class), "class ObjectInWorld :NonCntr");
    }

    /// Test display for vars
    #[test]
    fn test4() {
        let name = "TakeTraffic".to_owned();
        let inherits = vec![ClassId::from("PeopleAction")];
        let vars = vec![
            Var{
                name: "startPlace".to_owned(),
                inherits: vec![ClassId::from("common.place.Place")]
            },
            Var{
                name: "endPlace".to_owned(),
                inherits: vec![ClassId::from("common.place.Place")]
            },
            Var{
                name: "vehicle".to_owned(),
                inherits: vec![ClassId::from("common.traffic.Vehicle")]
            },
        ];
        let members = vars.iter().map(|var| ClassMember::VarMember(var.clone())).collect();
        let class = Class{name, inherits, members};
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
        let class = result.1.0;
        let output = format!("{}", class);
        let output = output.trim();
        assert_eq!(output, file);
    }
}