#[derive(PartialEq, Eq, Hash, Clone)]
pub struct ClassId {
    pub id: String,
    //TODO: TEST
    pub super_cid: Option<String>, //专门为 Cls 打造, 只有 Cls 非空 
}

impl ClassId {
    pub fn get_name(&self) -> &str {
        self.id.split(".").last().unwrap()
    }
}

impl From<&str> for ClassId {
    fn from(a: &str) -> Self {
        ClassId { id: a.to_owned(), super_cid: None }
    }
}

impl From<&String> for ClassId {
    fn from(a: &String) -> Self {
        ClassId { id: a.to_owned(), super_cid: None }
    }
}
