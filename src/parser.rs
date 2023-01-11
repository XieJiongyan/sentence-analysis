use std::{fmt, collections::HashMap};

use crate::class::{get_class, Class};

fn _parse(i: &str) -> Memory {
    let mut memory = Memory {classes: vec![], class_ids: HashMap::new()};
    let mut i = i;
    loop {
        let result = get_class(i);
        if let Err(_) = result {
            break;
        }
        let (remaining_input, class) = result.unwrap();
        let class_id = class.id.clone();
        if memory.class_ids.contains_key(&class_id) {
            panic!("Already have this id")
        } 
        memory.class_ids.insert(class_id, memory.classes.len());
        memory.classes.push(class);
        i = remaining_input.trim_start();
    }
    memory
}

struct Memory {
    classes: Vec<Class>,
    class_ids: HashMap<String, usize>,
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

impl Memory {
    pub fn get_class(&self, class_id: &str) -> Result<&Class, String> {
        let class_id = self.class_ids.get(class_id).unwrap_or_else(|| {
            panic!("Error Get Class");
        });
        Ok(&self.classes[*class_id])
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let memory = _parse("class NonCntr class ObjectInWorld");
        assert_eq!(format!("{}", memory), "class NonCntr\nclass ObjectInWorld\n");
    }
}