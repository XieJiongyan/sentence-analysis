#[derive(PartialEq, Eq, Hash)]
pub struct PackageName {
    pub id: String
}

impl PackageName {
    pub fn get_name(&self) -> &str {
        self.id.split(".").last().unwrap()
    }
}

impl From<&str> for PackageName {
    fn from(a: &str) -> Self {
        PackageName { id: a.to_owned() }
    }
}

impl From<String> for PackageName {
    fn from(a: String) -> Self {
        PackageName { id: a.clone() }
    }
}