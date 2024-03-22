#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Notification {
    pub id: String,
    pub header: String,
    pub content: String,
}

impl Notification {
    pub fn new(id: String, header: String, content: String) -> Self {
        Self {
            id,
            header,
            content,
        }
    }
}
