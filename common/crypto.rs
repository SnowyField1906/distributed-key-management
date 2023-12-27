use secp256k1::{
    Secp256k1,
    SecretKey,
    PublicKey,
    All,
    rand::rngs::OsRng,
    constants::CURVE_ORDER,
};
use num_bigint::BigUint;
use num_traits::{
    Zero,
    One
};
use tiny_keccak::{
    Keccak,
    Hasher
};

pub fn generate_keypair() -> (SecretKey, PublicKey) {
    let secp: Secp256k1<All> = Secp256k1::new();
    secp.generate_keypair(&mut OsRng)
}

pub fn pub_key_to_string(key: &PublicKey) -> String {
    hex::encode(key.serialize())
}

pub fn priv_key_to_string(key: &SecretKey) -> String {
    hex::encode(key.secret_bytes())
}

pub fn string_to_pub_key(key: &str) -> PublicKey {
    PublicKey::from_slice(&hex::decode(key).unwrap()[..]).unwrap()
}

pub fn string_to_priv_key(key: &str) -> SecretKey {
    SecretKey::from_slice(&hex::decode(key).unwrap()[..]).unwrap()
}

pub fn hex_to_biguint(hex: String) -> BigUint {
    BigUint::from_bytes_be(&hex::decode(hex).unwrap()[..])
}

pub fn biguint_to_hex(biguint: &BigUint) -> String {
    hex::encode(biguint.to_bytes_be())
}

pub fn get_address(public_key: &str) -> String {
    let formatted_public_key = &public_key[2..];
    let public_key_bytes = hex::decode(formatted_public_key).expect("Invalid hex");

    let hash = create_keccak256(&public_key_bytes);
    let address = &hash[hash.len() - 40..];

    let hash_address = &create_keccak256(address.as_bytes())[2..];

    let mut checksum_address = String::from("0x");

    for (i, &byte) in address.as_bytes().iter().enumerate() {
        let hash_byte = u8::from_str_radix(&hash_address[i..=i], 16).unwrap_or_else(|_| byte);
        if hash_byte >= 8 {
            checksum_address.push(byte as char);
        } else {
            checksum_address.push(byte.to_ascii_lowercase() as char);
        }
    }

    checksum_address
}

pub fn create_keccak256(data: &[u8]) -> String {
    let mut hasher = Keccak::v256();
    let mut output = [0u8; 32];
    hasher.update(data);
    hasher.finalize(&mut output);
    format!("0x{}", hex::encode(output))
}

pub fn interpolate(shares: &Vec<BigUint>, node_indices: &Vec<u32>, x_point: u32) -> Option<BigUint> {
    let mut result = BigUint::zero();
    if shares.len() != node_indices.len() {
        return None;
    }

    let n_secp256k1: BigUint = BigUint::from_bytes_be(&CURVE_ORDER);

    for i in 0..shares.len() {
        let mut upper = BigUint::one();
        let mut lower = BigUint::one();

        for j in 0..shares.len() {
            if j != i {
                upper *= &x_point - &node_indices[j];
                upper %= &n_secp256k1;

                let temp = &node_indices[i] - &node_indices[j];
                lower *= &temp % &n_secp256k1;
                lower %= &n_secp256k1;
            }
        }

        let mut delta = &upper * (&lower.modpow(&BigUint::from(usize::MAX), &n_secp256k1)).modpow(&BigUint::from(usize::MAX), &n_secp256k1);
        
        delta = (delta * &shares[i]) % &n_secp256k1;
        result = (result + delta) % &n_secp256k1;
    }

    Some(result)
}