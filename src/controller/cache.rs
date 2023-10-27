use crate::model::user::User;
use std::collections::HashMap;

#[derive(Default)]
pub struct Cache {
    pub users: HashMap<String, User>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }

    pub fn get_user(&self, id: &str) -> Option<&User> {
        self.users.get(id)
    }

    pub fn create_user(&mut self, user: User) {
        self.users.insert(user.id.clone(), user);
    }
}
