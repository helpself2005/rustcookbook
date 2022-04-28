
use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub age: i32
}

impl User {
    pub fn new(name: String, age: i32) -> Self {
        let uuid = Uuid::new_v4();
        User { id: uuid.to_string(), name, age }
    }
}