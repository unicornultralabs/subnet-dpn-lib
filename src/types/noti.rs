use serde::Serialize;


#[derive(Debug, Clone, Serialize)]
pub struct RegisterNotification {
    pub user_addr: String,
    pub email: String,
    pub token: String,
    pub device_type: String,
}

#[derive(Debug, Clone, Serialize)]
pub enum Notification {
    RegisterNotification(RegisterNotification),
    NoLongerOnlinePeer(Vec<String>)
}