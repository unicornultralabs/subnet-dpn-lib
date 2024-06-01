use dpn_proto::user_balance::ProtoUserBalance;
use prost::Message;

#[derive(Debug, Clone)]
pub struct UserBalance {
    pub user_addr: String,
    pub balance: i64,
}

impl UserBalance {
    pub fn to_vec(&self) -> Vec<u8> {
        let proto: ProtoUserBalance = self.clone().into();
        let binding = ::prost::Message::encode_to_vec(&proto);
        binding.as_slice().to_owned()
    }

    pub fn from_bytes(bz: &[u8]) -> Self {
        let proto = ProtoUserBalance::decode(bz).expect("decode proto stream payload failed");
        proto.into()
    }
}

impl Into<ProtoUserBalance> for UserBalance {
    fn into(self) -> ProtoUserBalance {
        ProtoUserBalance {
            user_addr: self.user_addr,
            balance: self.balance,
        }
    }
}

impl Into<UserBalance> for ProtoUserBalance {
    fn into(self) -> UserBalance {
        UserBalance {
            user_addr: self.user_addr,
            balance: self.balance,
        }
    }
}

#[cfg(test)]
mod tests {}
