use num_bigint::BigUint;
use num_traits::{
	One,
	Zero,
};
use secp256k1::{
	constants::CURVE_ORDER,
	rand::rngs::OsRng,
	All,
	PublicKey,
	Secp256k1,
	SecretKey,
};
use tiny_keccak::{
	Hasher,
	Keccak,
};

pub fn encrypt(pub_key: &PublicKey, message: Vec<u8>) -> String {
	let encrypted: Vec<u8> = ecies::encrypt(&pub_key.serialize(), &message).unwrap();

	hex::encode(encrypted)
}

pub fn decrypt(priv_key: &SecretKey, message: Vec<u8>) -> String {
	let decrypted: Vec<u8> = ecies::decrypt(&priv_key.secret_bytes(), &message).unwrap();

	hex::encode(decrypted)
}

pub fn generate_keypair() -> (SecretKey, PublicKey) {
	let secp: Secp256k1<All> = Secp256k1::new();
	secp.generate_keypair(&mut OsRng)
}

pub fn pub_key_to_str(key: &PublicKey) -> String { hex::encode(key.serialize()) }

pub fn priv_key_to_str(key: &SecretKey) -> String { hex::encode(key.secret_bytes()) }

pub fn str_to_pub_key(key: &str) -> PublicKey {
	PublicKey::from_slice(&hex::decode(key).unwrap()[..]).unwrap()
}

pub fn str_to_priv_key(key: &str) -> SecretKey {
	SecretKey::from_slice(&hex::decode(key).unwrap()[..]).unwrap()
}

pub fn hex_to_big_num(hex: String) -> BigUint {
	BigUint::from_bytes_be(&hex::decode(hex).unwrap()[..])
}

pub fn big_num_to_hex(big: &BigUint) -> String { hex::encode(big.to_bytes_be()) }

pub fn get_addr_from_pub_key(public_key: &str) -> String {
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

pub fn get_addr_from_priv_key(private_key: &str) -> String {
	let formatted_private_key = &private_key[2..];
	let private_key_bytes = hex::decode(formatted_private_key).expect("Invalid hex");

	let secp: Secp256k1<All> = Secp256k1::new();
	let secret_key = SecretKey::from_slice(&private_key_bytes).unwrap();
	let public_key = PublicKey::from_secret_key(&secp, &secret_key);

	get_addr_from_pub_key(&pub_key_to_str(&public_key))
}

pub fn create_keccak256(data: &[u8]) -> String {
	let mut hasher = Keccak::v256();
	let mut output = [0u8; 32];
	hasher.update(data);
	hasher.finalize(&mut output);
	format!("0x{}", hex::encode(output))
}

pub fn interpolate(
	shares: &Vec<BigUint>, node_indices: &Vec<u32>, x_point: u32,
) -> Option<BigUint> {
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

		let mut delta = &upper
			* (&lower.modpow(&BigUint::from(usize::MAX), &n_secp256k1))
				.modpow(&BigUint::from(usize::MAX), &n_secp256k1);

		delta = (delta * &shares[i]) % &n_secp256k1;
		result = (result + delta) % &n_secp256k1;
	}

	Some(result)
}
