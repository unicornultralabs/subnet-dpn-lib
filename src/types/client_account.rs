pub struct ClientAccount {
    pub user_id: String,
    pub username: String,
    pub salt: String,
    pub password_hashed: String,
}

impl ClientAccount {
    pub fn new(
        user_id: String, 
        username: String, 
        salt: String,
        password_hashed: String,
    ) -> Self {
        Self {
            user_id,
            username,
            salt,
            password_hashed,
        }
    }
}
