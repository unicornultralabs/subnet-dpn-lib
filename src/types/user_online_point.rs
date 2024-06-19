use dpn_proto::user_online_point::ProtoUserOnlinePoint;
use prost::Message;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct UserOnlinePoint {
    pub user_addr: String,
    pub login_session_id: String,
    pub masternode_id: String,
    pub poll_at: u64,
    pub last_poll_at: u64,
}

impl UserOnlinePoint {
    pub fn new(
        user_addr: String,
        login_session_id: String,
        masternode_id: String,
        poll_at: u64,
        last_poll_at: u64,
    ) -> Self {
        Self {
            user_addr,
            login_session_id,
            masternode_id,
            poll_at,
            last_poll_at,
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let proto: ProtoUserOnlinePoint = self.clone().into();
        let binding = ::prost::Message::encode_to_vec(&proto);
        binding.as_slice().to_owned()
    }

    pub fn from_bytes(bz: &[u8]) -> Self {
        let proto = ProtoUserOnlinePoint::decode(bz).expect("decode proto user online point failed");
        proto.into()
    }
}

impl Into<ProtoUserOnlinePoint> for UserOnlinePoint {
    fn into(self) -> ProtoUserOnlinePoint {
        ProtoUserOnlinePoint {
            user_addr: self.user_addr,
            login_session_id: self.login_session_id,
            masternode_id: self.masternode_id,
            poll_at: self.poll_at,
            last_poll_at: self.last_poll_at,
        }
    }
}

impl Into<UserOnlinePoint> for ProtoUserOnlinePoint {
    fn into(self) -> UserOnlinePoint {
        UserOnlinePoint {
            user_addr: self.user_addr,
            login_session_id: self.login_session_id,
            masternode_id: self.masternode_id,
            poll_at: self.poll_at,
            last_poll_at: self.last_poll_at,
        }
    }
}