pub struct ClassId {
    id: String
}

impl ClassId {
    pub fn get_name(&self) -> &str {
        self.id.split(".").last().unwrap()
    }
}