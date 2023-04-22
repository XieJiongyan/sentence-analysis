use std::{fmt, collections::HashMap};

use crate::{class::{parse_class, Class}, utils::class_id::ClassId, cls::Cls};

struct Memory {
    classes: HashMap<ClassId, Class>, //"package.class" -> class
    cls_ids: Vec<ClassId>, //""
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

impl Memory {
    pub fn get_class(&self, class_id: &ClassId) -> Result<&Class, String> {
        Ok(self.classes.get(class_id).unwrap_or_else(|| {
            panic!("Error Get Class");
        }))
    }

    pub fn new() -> Memory {
        Memory { classes: HashMap::new(), cls_ids: vec![] }
    }

    fn _parse(&mut self, i: &str) {
        let mut i = i;
        loop {
            let result = parse_class(i);
            if let Err(_) = result {
                break;
            }
            let (remaining_input, (class, cls_ids)) = result.unwrap();
            let class_id = class.get_cid();
            if self.classes.contains_key(&class_id) {
                panic!("Already have this id")
            } 
            self.classes.insert(class_id, class);
            i = remaining_input.trim_start();


        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut memory = Memory::new();
        memory._parse("class NonCntr class ObjectInWorld");
        assert_eq!(format!("{}", memory), "class NonCntr\nclass ObjectInWorld\n");
    }
}