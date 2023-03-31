use std::fmt;

pub struct Cls {
    pub id :i128,
    pub name :String,
    pub inherit_ciz :Vec<i128>, //inherit_class_ids
    pub parameter_ciz :Vec<i128>,
}

impl fmt::Display for Cls {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}