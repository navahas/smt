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

#[inline]
pub fn keccak(data: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(data);
    hasher.finalize().into()
}

pub fn hash_pair(left: [u8; 32], right: [u8; 32]) -> [u8; 32] {
    let mut h_pair = Keccak256::new();
    h_pair.update(left);
    h_pair.update(right);
    h_pair.finalize().into()
}

pub fn precompute_zero_hashes(max_levels: usize) -> Vec<[u8; 32]> {
    let mut zero = Vec::new();
    zero.push(keccak(&[]));
    for lvl in 0..max_levels {
        let z = zero[lvl];
        zero.push(hash_pair(z, z));
    }
    zero
}

pub fn hash_to_string(hash: &[u8]) -> String {
    to_hex(&hash)
}
