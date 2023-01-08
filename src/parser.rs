use std::{fmt, collections::HashMap};

use crate::class::{get_class, Class};

fn parse(i: &str) -> Memory {
    let mut memory = Memory {classes: HashMap::new()};
    let mut i = i;
    loop {
        let result = get_class(i);
        if let Err(_) = result {
            break;
        }
        let (remaining_input, class) = result.unwrap();
        if memory.classes.contains_key(&class.id.clone()) {
            panic!("Already have this id")
        } 
        memory.classes.insert(class.id.clone(), class);
        i = remaining_input.trim_start();
    }
    memory
}

struct Memory {
    classes: HashMap<String, Class>,
}

impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        for (_, class) in &self.classes {
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