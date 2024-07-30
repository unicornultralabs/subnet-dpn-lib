use ethers::types::Address;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct VpnUser {
    pub email: Option<String>,
    pub deposit_addr: Address,
    pub withdrawal_addr: Option<Address>,
    pub created_at: i64,
    pub last_login: i64,
}

impl User {
    pub fn new(
        email: Option<String>,
        deposit_addr: Address,
        withdrawal_addr: Option<Address>,
        created_at: i64,
        last_login: i64,
    ) -> Self {
        Self {
            email,
            deposit_addr,
            withdrawal_addr,
            created_at,
            last_login,
        }
    }
}
