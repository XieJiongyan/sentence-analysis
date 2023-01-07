use std::fmt;

use crate::class::{get_class, Class};

fn parse(i: &str) -> Memory {
    let mut memory = Memory {classes: vec![]};
    let mut i = i;
    loop {
        let result = get_class(i);
        if let Err(_) = result {
            break;
        }
        let (remaining_input, class) = result.unwrap();
        memory.classes.push(class);
        i = remaining_input.trim_start();
    }
    memory
}

struct Memory {
    classes: Vec<Class>,
}

impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        for class in &self.classes {
            result += &format!("{}\n", class);
        }
        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let memory = parse("class NonCntr class ObjectInWorld");
        assert_eq!(format!("{}", memory), "class NonCntr\nclass ObjectInWorld\n");
    }
}