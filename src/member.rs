use std::fmt;


///member in class, not member variables in variable
pub struct Var_member {
    pub name: String,
    pub inherits: Vec<(String, String)>, //There are (class.id, class.name) in the vec
}

impl fmt::Display for Var_member {
    fn fmt(&self, f:  &mut fmt::Formatter) -> fmt::Result {
        write!(f, "var {}", self.name)?;
        for inherit in &self.inherits {
            write!(f, " :{}", inherit.1)?;
        }
        Ok(())
    }
}
/**
 * //TODO: Add a parser for `var var_name: ${class}`
 * We can do it by add a new file name.rs
 * //TODO: Not a keyword
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let name = "startPlace".to_owned();
        let inherit = ("Place".to_owned(), "Place".to_owned());
        let inherits = vec![inherit];
        let var_member = Var_member {name, inherits};
        assert_eq!(format!("{}", var_member), "var startPlace :Place");
    }
}