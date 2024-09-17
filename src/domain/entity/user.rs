pub struct User {
    id: String,
    user_name: String,
    password_hash: String,
}

impl User {
    pub fn new(id: String, user_name: String, password_hash: String) -> Self {
        Self {
            id,
            user_name,
            password_hash,
        }
    }

    pub fn veryfi_password(&self, password: &str) {
        // Todo
    }
}
