use std::{fmt, collections::HashMap};

use crate::{class::{parse_class, Class}, utils::package_name::PackageName, cls::Cls};

fn _parse(i: &str) -> Memory {
    let class_list = HashMap::new();
    let cls_list = HashMap::new();
    let mut memory = Memory {
        classes: class_list, 
        cls_list,
    };
    let mut i = i;
    loop {
        let result = parse_class(i);
        if let Err(_) = result {
            break;
        }
        let (remaining_input, class) = result.unwrap();
        let class_id = class.get_package_name();
        if memory.classes.contains_key(&class_id) {
            panic!("Already have this id")
        } 
        memory.classes.insert(class_id, class);
        i = remaining_input.trim_start();
    }
    memory
}

struct Memory {
    classes: HashMap<PackageName, Class>, //"package.class" -> class
    cls_list :HashMap<PackageName, Cls>
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
    pub fn get_class(&self, class_id: &PackageName) -> Result<&Class, String> {
        Ok(self.classes.get(class_id).unwrap_or_else(|| {
            panic!("Error Get Class");
        }))
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