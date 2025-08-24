use sha3::{Digest, Keccak256};

pub fn to_hex(bytes: &[u8]) -> String {
    // 02x to ensure a consistent 64 characters length for 256-bit hash (u8)
    bytes
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>()
}

pub fn to_hex_prefixed(bytes: &[u8]) -> String {
    let hex = bytes
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();

    format!("0x{}", hex)
}

pub fn hash(data: &[u8]) -> Vec<u8> {
    let mut hasher = Keccak256::new();
    hasher.update(data);
    let result: Vec<u8> = hasher.finalize().to_vec();
    result
}

pub fn hash_to_string(data: &[u8]) -> String {
    let mut hasher = Keccak256::new();
    hasher.update(data);
    let result: [u8; 32] = hasher.finalize().into();
    to_hex(&result)
}
