use ethers::types::H256;
use sha3::{Digest, Sha3_256};

pub fn hash(b: &[u8]) -> H256 {
    let mut hasher = Sha3_256::new();
    hasher.update(b);
    let rs = hasher.finalize();
    H256::from_slice(rs.as_slice())
}
