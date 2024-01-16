use std::{
	env,
	sync::Arc,
};

use actix_web::web::Data;
use lazy_static::lazy_static;
use num_bigint::BigUint;
use secp256k1::{
	constants::CURVE_ORDER,
	All,
	PublicKey,
	Secp256k1,
	SecretKey,
};
use tokio::sync::RwLockWriteGuard;
use tonic::{
	transport::Channel,
	Request,
};

use crate::{
	common::{
		constants,
		crypto,
		messages,
	},
	config::microservice::{
		p2p::{
			p2p_client::P2pClient,
			*,
		},
		GrpcPool,
	},
	schemas::{
		shared_key_schema::SharedKey,
		wallet_schema::Wallet,
	},
	services::shared_key_service,
};

lazy_static! {
	pub static ref INDEX: usize = env::args().nth(1).unwrap().parse().unwrap();
}

pub async fn broadcast_all(grpc_pool: Data<Arc<GrpcPool>>) -> Result<(), messages::Error> {
	for node in 0..constants::N_NODES {
		if node == *INDEX {
			continue;
		}

		let mut p2p = grpc_pool.get_client_mut(node).await;

		match p2p
			.broadcast_assign_key(Request::new(BroadcastAssignKeyRequest { id: *INDEX as u32 }))
			.await
		{
			Ok(_) => {}
			Err(error) => {
				return Err(messages::Error::new(format!(
					"Error when broadcast_assign_key in {}\n{}",
					node, error
				)));
			}
		}
	}

	Ok(())
}

pub async fn generate_shared_secret(
	grpc_pool: &mut Data<Arc<GrpcPool>>, owner: &str,
) -> Result<Wallet, messages::Error> {
	let mut p2p: [_; constants::N_NODES] = [
		grpc_pool.get_client_mut(0).await,
		grpc_pool.get_client_mut(1).await,
		grpc_pool.get_client_mut(2).await,
	];

	let mut keys: Vec<BigUint> = Vec::new();

	// step 1: init secret
	for node in 0..constants::N_NODES {
		match p2p[node]
			.init_secret(Request::new(InitSecretRequest {
				owner: owner.to_string(),
			}))
			.await
		{
			Ok(data) => {
				keys.push(crypto::hex_to_big_num(data.into_inner().pub_key));
			}
			Err(error) => {
				return Err(messages::Error::new(format!(
					"Error when init_secret in {}\n{}",
					node, error
				)));
			}
		}
	}

	// step 2: generate shares
	generate_shares(&mut p2p, owner).await?;

	// step 3: derive shared secret key
	for node in 0..constants::N_NODES {
		match p2p[node]
			.derive_shared_secret(Request::new(DeriveSharedSecretRequest {
				owner: owner.to_string(),
			}))
			.await
		{
			Ok(_) => {}
			Err(error) => {
				return Err(messages::Error::new(format!(
					"Error when derive_shared_secret in {}\n{}",
					node, error
				)));
			}
		}
	}

	let mut shared_secret: BigUint = BigUint::default();
	let n_secp256k1: BigUint = BigUint::from_bytes_be(&CURVE_ORDER);

	for pub_key in keys {
		shared_secret = (shared_secret + pub_key) % &n_secp256k1;
	}

	let secp: Secp256k1<All> = Secp256k1::new();
	let priv_key: SecretKey = SecretKey::from_slice(&shared_secret.to_bytes_be()).unwrap();
	let pub_key: PublicKey = priv_key.public_key(&secp);

	let address: String = crypto::get_addr_from_pub_key(&crypto::big_num_to_hex(&shared_secret));

	for node in 0..constants::N_NODES {
		match p2p[node]
			.store_wallet_info(Request::new(StoreWalletInfoRequest {
				owner: owner.to_string(),
				pub_key: crypto::pub_key_to_str(&pub_key),
				address: address.clone(),
			}))
			.await
		{
			Ok(_) => {}
			Err(error) => {
				return Err(messages::Error::new(format!(
					"Error when store_wallet_info in {}\n{}",
					node, error
				)));
			}
		}
	}

	Ok(Wallet {
		id: None,
		owner: owner.to_string(),
		pub_key: crypto::pub_key_to_str(&pub_key),
		address: address.clone(),
	})
}

pub async fn generate_shares(
	p2p: &mut [RwLockWriteGuard<'_, P2pClient<Channel>>; constants::N_NODES], owner: &str,
) -> Result<bool, messages::Error> {
	let shared_key: SharedKey = match shared_key_service::find_by_owner(owner).await {
		Ok(shared_key) => shared_key,
		Err(_) => return Ok(false),
	};

	let mut indices: Vec<u32> = [0].to_vec();
	let mut shares: Vec<BigUint> = [crypto::hex_to_big_num(shared_key.secret)].to_vec();

	for node in 0..constants::N_NODES {
		if shares.len() < constants::THRESHOLD {
			let (random_share, _): (SecretKey, _) = crypto::generate_keypair();

			match p2p[node]
				.add_received_share(Request::new(AddReceivedShareRequest {
					owner: owner.to_string(),
					received_share: crypto::priv_key_to_str(&random_share),
				}))
				.await
			{
				Ok(_) => {}
				Err(error) => {
					Err(messages::Error::new(format!(
						"Error when add_received_share in {}\n{}",
						node, error
					)))?
				}
			}

			shares.push(crypto::hex_to_big_num(crypto::priv_key_to_str(&random_share)));
			indices.push(node as u32);
		} else {
			let point = crypto::interpolate(&shares, &indices, node as u32);

			p2p[node]
				.add_received_share(Request::new(AddReceivedShareRequest {
					owner: owner.to_string(),
					received_share: crypto::big_num_to_hex(&point.unwrap()),
				}))
				.await
				.unwrap();
		}
	}

	Ok(true)
}
