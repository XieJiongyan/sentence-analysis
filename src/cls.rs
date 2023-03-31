use std::fmt;

use crate::utils::package_name::PackageName;

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