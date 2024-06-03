use dpn_proto::user_balance::{ProtoBalanceChange, ProtoRefreshBalances, ProtoUserBalance};
use prost::Message;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshBalances {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BalanceChange {
    UserBalance(UserBalance),
    RefreshBalances(RefreshBalances),
}

impl BalanceChange {
    pub fn to_vec(&self) -> Vec<u8> {
        let proto: ProtoBalanceChange = self.clone().into();
        let binding = ::prost::Message::encode_to_vec(&proto);
        binding.as_slice().to_owned()
    }

    pub fn from_bytes(bz: &[u8]) -> Self {
        let proto = ProtoBalanceChange::decode(bz).expect("decode proto stream payload failed");
        proto.into()
    }
}

impl Into<ProtoBalanceChange> for BalanceChange {
    fn into(self) -> ProtoBalanceChange {
        match self {
            BalanceChange::UserBalance(bal) => ProtoBalanceChange {
                payload: Some(
                    dpn_proto::user_balance::proto_balance_change::Payload::UserBalance(
                        ProtoUserBalance {
                            user_addr: bal.user_addr,
                            balance: bal.balance,
                        },
                    ),
                ),
            },
            BalanceChange::RefreshBalances(_) => ProtoBalanceChange {
                payload: Some(
                    dpn_proto::user_balance::proto_balance_change::Payload::RefreshBalances(
                        ProtoRefreshBalances {},
                    ),
                ),
            },
        }
    }
}

impl Into<BalanceChange> for ProtoBalanceChange {
    fn into(self) -> BalanceChange {
        match self.payload.unwrap() {
            dpn_proto::user_balance::proto_balance_change::Payload::UserBalance(b) => {
                BalanceChange::UserBalance(UserBalance {
                    user_addr: b.user_addr,
                    balance: b.balance,
                })
            }
            dpn_proto::user_balance::proto_balance_change::Payload::RefreshBalances(_) => {
                BalanceChange::RefreshBalances(RefreshBalances {})
            }
        }
    }
}

#[cfg(test)]
mod tests {}
