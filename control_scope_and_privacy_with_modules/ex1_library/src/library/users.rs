// Modulo  users
// - Debe contener struc User: name: String, id: u32
pub struct User {
    name: String,
    id: u32,
}
impl User {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn id(&self) -> u32 {
        self.id
    }
}

pub fn create_user(name: &str, id: u32) -> User {
    User {
        name: String::from(name),
        id,
    }
}