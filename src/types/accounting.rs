use dpn_proto::user_balance::ProtoUserBalance;

#[derive(Debug, Clone)]
pub struct UserBalance {
    pub user_addr: String,
    pub balance: i64,
}

impl UserBalance {}

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
