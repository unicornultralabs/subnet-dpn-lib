pub struct ClientAccount {
    pub user_id: String,
    pub username: String,
    pub salt: String,
    pub hashed_password: String,
}

impl ClientAccount {
    pub fn new(
        user_id: String, 
        username: String, 
        salt: String,
        hashed_password: String,
    ) -> Self {
        Self {
            user_id,
            username,
            salt,
            hashed_password,
        }
    }
}
