use crate::utils::hash::hash;
use dpn_proto::notification::ProtoNotification;
use ethers::types::H256;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Notification {
    pub id: H256,
    pub header: String,
    pub content: String,
    pub created_at: i64,
}

impl Notification {
    pub fn new(header: String, content: String, created_at: i64) -> Self {
        let mut _self = Self {
            id: H256::zero(),
            header,
            content,
            created_at,
        };
        let proto: ProtoNotification = _self.clone().into();
        let binding = ::prost::Message::encode_to_vec(&proto);
        let bz = binding.as_slice();
        let id = hash(bz);
        _self.id = id;

        return _self;
    }
}

impl Into<ProtoNotification> for Notification {
    fn into(self) -> ProtoNotification {
        ProtoNotification {
            header: self.header,
            content: self.content,
            created_at: self.created_at,
        }
    }
}
