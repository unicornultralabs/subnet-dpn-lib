#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Notification {
    pub id: String,
    pub header: String,
    pub content: String,
    pub created_at: i64,
}

impl Notification {
    pub fn new(
        id: String,
        header: String,
        content: String,
        created_at: i64,
    ) -> Self {
        Self {
            id,
            header,
            content,
            created_at,
        }
    }
}
