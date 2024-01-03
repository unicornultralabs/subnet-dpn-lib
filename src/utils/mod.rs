pub mod hash;

use ethers::utils::{format_units, parse_units};
use hex::encode;
pub use web3::types::{
    Address, Bytes, Log, TransactionRequest, H128, H160, H2048, H256, U128, U256, U64,
};

/// Converts `U256` into the corresponding `BigUint` value.

fn ensure_chunkable(bytes: &[u8]) {
    assert!(
        bytes.len() % 32 == 0,
        "Bytes must be divisible by 32 to split into chunks"
    );
}

pub fn h256_to_u256(num: H256) -> U256 {
    U256::from_big_endian(num.as_bytes())
}

pub fn address_to_h256(address: &Address) -> H256 {
    let mut buffer = [0u8; 32];
    buffer[12..].copy_from_slice(address.as_bytes());
    H256(buffer)
}

pub fn address_to_u256(address: &Address) -> U256 {
    h256_to_u256(address_to_h256(address))
}

pub fn bytes_to_hex_string(bz: &[u8]) -> String {
    format!("0x{}", String::from(encode(bz)))
}

pub fn bytes_to_chunks(bytes: &[u8]) -> Vec<[u8; 32]> {
    ensure_chunkable(bytes);
    bytes
        .chunks(32)
        .map(|el| {
            let mut chunk = [0u8; 32];
            chunk.copy_from_slice(el);
            chunk
        })
        .collect()
}

pub fn be_chunks_to_h256_words(chunks: Vec<[u8; 32]>) -> Vec<H256> {
    chunks.into_iter().map(|el| H256::from_slice(&el)).collect()
}

pub fn bytes_to_be_words(vec: Vec<u8>) -> Vec<U256> {
    ensure_chunkable(&vec);
    vec.chunks(32).map(U256::from_big_endian).collect()
}

pub fn be_words_to_bytes(words: &[U256]) -> Vec<u8> {
    words
        .iter()
        .flat_map(|w| {
            let mut bytes = [0u8; 32];
            w.to_big_endian(&mut bytes);
            bytes
        })
        .collect()
}

pub fn u256_to_h256(num: U256) -> H256 {
    let mut bytes = [0u8; 32];
    num.to_big_endian(&mut bytes);
    H256::from_slice(&bytes)
}

/// Converts `U256` value into the Address
pub fn u256_to_account_address(value: &U256) -> Address {
    let mut bytes = [0u8; 32];
    value.to_big_endian(&mut bytes);

    Address::from_slice(&bytes[12..])
}

/// Converts `H256` value into the Address
pub fn h256_to_account_address(value: &H256) -> Address {
    Address::from_slice(&value.as_bytes()[12..])
}

pub fn be_bytes_to_safe_address(bytes: &[u8]) -> Option<Address> {
    if bytes.len() < 20 {
        return None;
    }

    let (zero_bytes, address_bytes) = bytes.split_at(bytes.len() - 20);

    if zero_bytes.iter().any(|b| *b != 0) {
        None
    } else {
        Some(Address::from_slice(address_bytes))
    }
}

/// Converts `h256` value as BE into the u32
pub fn h256_to_u32(value: H256) -> u32 {
    let be_u32_bytes: [u8; 4] = value[28..].try_into().unwrap();
    u32::from_be_bytes(be_u32_bytes)
}

/// Converts u32 into the h256 as BE bytes
pub fn u32_to_h256(value: u32) -> H256 {
    let mut result = [0u8; 32];
    result[28..].copy_from_slice(&value.to_be_bytes());
    H256(result)
}

/// Converts `U256` value into bytes array
pub fn u256_to_bytes_be(value: &U256) -> Vec<u8> {
    let mut bytes = vec![0u8; 32];
    value.to_big_endian(bytes.as_mut_slice());
    bytes
}

pub fn u256_to_szabo(value: U256) -> i64 {
    format_units(value, "szabo")
        .unwrap()
        .parse::<f64>()
        .unwrap() as i64
}

pub fn szabo_to_u256(value: i64) -> U256 {
    U256::from(parse_units(value, "szabo").unwrap())
}
