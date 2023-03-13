use rand::{Rng, rngs::OsRng};
use hex::encode;

pub fn get_random_hash() -> String {
    let mut rng = OsRng;
    let bytes: [u8; 12] = rng.gen();
    encode(bytes)
}

#[test]
fn test_random_hash_length() {
    let hash = get_random_hash();
    assert_eq!(hash.len(), 24);
}

#[test]
fn test_random_hash_hex() {
    let hash = get_random_hash();
    let is_hex = hash.chars().all(|c| c.is_ascii_hexdigit());
    assert!(is_hex, "Hash should only contain hexadecimal characters");
}