use tonic::Request;
use lazy_static::lazy_static;
use std::env;
use num_bigint::BigUint;
use secp256k1::{
    Secp256k1,
    SecretKey,
    PublicKey,
    All,
    constants::CURVE_ORDER,
};
use crate::{
    common::{
        constants,
        crypto,
        messages,
    },
    grpc::{
        controller::P2PController,
        p2p::{
            *,
            p2p_server::P2p
        }
    },
    schemas::{
        wallet_schema::Wallet,
        shared_key_schema::SharedKey,
    },
    services::shared_key_service,
};

lazy_static! {
    pub static ref NODES: [P2PController; constants::N_NODES] = [
        P2PController::default(),
        P2PController::default(),
        P2PController::default(),
    ];
    pub static ref INDEX: usize = env::args().nth(1).unwrap().parse().unwrap();
}

pub async fn broadcast_all() -> Result<(), messages::Error> {
    for node in 0..constants::N_NODES {
        if node == *INDEX {
            continue;
        }
        
        match NODES[node].broadcast_assign_key(
            Request::new(BroadcastAssignKeyRequest {
                id: *INDEX as u32,
            })
        ).await {
            Ok(_) => {},
            Err(error) => {
                return Err(messages::Error::new(
                    format!("Error when broadcast_assign_key in {}\n{}", node, error).as_str(),
                ));
            }
        }
    }

    Ok(())
}

pub async fn generate_shared_secret(owner: &str) -> Result<Wallet, messages::Error> {
    let mut keys: Vec<BigUint> = Vec::new();

    // step 1: init secret
    for node in 0..constants::N_NODES {
        let p2p = &NODES[node];

        match p2p.init_secret(
            Request::new(InitSecretRequest {
                owner: owner.to_string(),
            })
        ).await {
            Ok(data) => {
                keys.push(crypto::hex_to_biguint(data.into_inner().pub_key));
            },
            Err(error) => {
                return Err(messages::Error::new(
                    format!("Error when init_secret in {}\n{}", node, error).as_str(),
                ));
            }
        }
    }

    // step 2: get shares
    for node in 0..constants::N_NODES {
        let p2p = &NODES[node];

        match p2p.generate_shares(
            Request::new(GenerateSharesRequest {
                owner: owner.to_string(),
            })
        ).await {
            Ok(_) => {},
            Err(error) => {
                return Err(messages::Error::new(
                    format!("Error when generate_shares in {}\n{}", node, error).as_str(),
                ));
            }
        }
    }

    // step 3: derive shared secret key
    for node in 0..constants::N_NODES {
        let p2p = &NODES[node];

        match p2p.derive_shared_secret(
            Request::new(DeriveSharedSecretRequest {
                owner: owner.to_string(),
            })
        ).await {
            Ok(_) => {},
            Err(error) => {
                return Err(messages::Error::new(
                    format!("Error when derive_shared_secret in {}\n{}", node, error).as_str(),
                ));
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

    let address: String = crypto::get_address(&crypto::biguint_to_hex(&shared_secret));

    for node in 0..constants::N_NODES {
        let p2p = &NODES[node];

        match p2p.store_wallet_info(
            Request::new(StoreWalletInfoRequest {
                owner: owner.to_string(),
                pub_key: crypto::pub_key_to_string(&pub_key),
                address: address.clone(),
            })
        ).await {
            Ok(_) => {},
            Err(error) => {
                return Err(messages::Error::new(
                    format!("Error when store_wallet_info in {}\n{}", node, error).as_str(),
                ));
            }
        }
    }

    Ok(Wallet {
        id: None,
        owner: owner.to_string(),
        pub_key: crypto::pub_key_to_string(&pub_key),
        address: address.clone(),
    })
}

pub async fn generate_shares(owner: &str) -> Result<bool, messages::Error> {
    let shared_key: SharedKey = match shared_key_service::find_by_owner(owner).await {
        Ok(shared_key) => shared_key,
        Err(_) => return Ok(false),
    };

    let mut indices: Vec<u32> = [0].to_vec();
    let mut shares: Vec<BigUint> = [
        crypto::hex_to_biguint(shared_key.secret),
    ].to_vec();

    for node in 0..constants::N_NODES {
        if shares.len() < constants::THRESHOLD {
            let (random_share, _): (SecretKey, _) = crypto::generate_keypair();

            match NODES[node].add_received_share(
                Request::new(AddReceivedShareRequest {
                    owner: owner.to_string(),
                    received_share: crypto::priv_key_to_string(&random_share),
                })
            ).await {
                Ok(_) => {},
                Err(error) => Err(messages::Error::new(
                    format!("Error when add_received_share in {}\n{}", node, error).as_str(),
                ))?,
            }

            shares.push(crypto::hex_to_biguint(crypto::priv_key_to_string(&random_share)));
            indices.push(node as u32);
        } else {
            let point = crypto::interpolate(&shares, &indices, node as u32);

            NODES[node].add_received_share(
                Request::new(AddReceivedShareRequest {
                    owner: owner.to_string(),
                    received_share: crypto::biguint_to_hex(&point.unwrap()),
                })
            ).await.unwrap();
        }
    }

    Ok(true)
}