#[derive(PartialEq)]
pub struct ClassId {
    pub id: String
}

impl ClassId {
    pub fn get_name(&self) -> &str {
        self.id.split(".").last().unwrap()
    }
}

impl From<&str> for ClassId {
    fn from(a: &str) -> Self {
        ClassId { id: a.to_owned() }
    }
}