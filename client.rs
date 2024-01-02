mod common;
mod config;
mod dtos;
mod schemas;

use actix_web::web::Json;
use secp256k1::{
    Secp256k1,
    SecretKey,
    PublicKey,
    All,
    rand::rngs::OsRng,
    constants::CURVE_ORDER,
};
use reqwest::{
    get,
    Client,
    Error,
};
use std::{
    env,
    path,
};
use num_bigint::BigUint;
use std::{
    fmt::Debug,
    collections::HashMap,
};
use crate::{
    common::{
        constants::{
            N_NODES,
            THRESHOLD,
        },
        messages,
        crypto,
    },
    dtos::{
        lookup_shared_secret_dto::LookupSharedSecretDto,
        create_commitment_dto::CreateCommitmentDto,
        node_commitment_dto::NodeCommitmentDto,
        node_shared_secret_dto::NodeSharedSecretDto,
    },
    schemas::wallet_schema::Wallet,
};

fn threshold_same<T: Eq + Clone + Debug>(arr: Vec<T>, t: usize) -> Option<T> {
    let mut hash_map = HashMap::new();

    for element in arr {
        let str = format!("{:?}", element);
        *hash_map.entry(str).or_insert(0) += 1;

        if hash_map[&str] == t {
            return Some(element);
        }
    }

    None
}

fn k_combinations<T: Clone>(s: &[T], k: usize) -> Vec<Vec<T>> {
    if k > s.len() || k == 0 {
        return Vec::new();
    }

    if k == s.len() {
        return vec![s.to_vec()];
    }

    if k == 1 {
        return s.iter().map(|&x| vec![x.clone()]).collect();
    }

    let mut combs = Vec::new();
    let mut tail_combs = Vec::new();

    for i in 0..=(s.len() - k) {
        tail_combs = k_combinations(&s[i + 1..], k - 1);
        for j in 0..tail_combs.len() {
            let mut temp = vec![s[i].clone()];
            temp.extend_from_slice(&tail_combs[j]);
            combs.push(temp);
        }
    }
    combs
}

fn get_node_enpoint(node: usize) -> String {
    let env_path: path::PathBuf = env::current_dir()
        .and_then(|a| Ok(a
            .join("config")
            .join("node_info")
            .join(format!("node{}.env", node))
        ))
        .unwrap();
    dotenv::from_path(env_path.as_path()).ok();

    let host: String = dotenv::var("HOST").unwrap();
    let http_port: String = dotenv::var("HTTP_PORT").unwrap();
    let endpoint: String = format!("http://{}:{}", host, http_port);

    endpoint
}

async fn get_address(owner: &str) -> Wallet {
    for node in 0..N_NODES {
        match get(format!("{}/api/wallet/{}", get_node_enpoint(node), owner))
            .await {
                Ok(response) => {
                    let wallet: Wallet = response.json().await.unwrap();
                    return wallet;
                },
                Err(_) => {}
            }
    }

    panic!("No node responded to the request");
}

async fn get_priv_key(token_id: String, owner: String, verifier: String) -> String {
    let address: Wallet = get_address(&owner).await;

    let (temp_priv, temp_pub): (SecretKey, PublicKey) = crypto::generate_keypair();

    let token_commitment = crypto::create_keccak256(token_id.as_bytes());
    let mut signatures: Vec<NodeCommitmentDto> = Vec::new();
    
    let client = Client::new();

    for node in 0..N_NODES {
        match client
            .post(format!("{}/api/commitment", get_node_enpoint(node)))
            .json(&CreateCommitmentDto {
                commitment: token_commitment.clone(),
                temp_pub: crypto::pub_key_to_str(&temp_pub),
            })
            .send()
            .await {
                Ok(response) => {
                    signatures.push(response.json().await.unwrap());
                },
                Err(_) => {}
            }
    }

    if signatures.len() < (N_NODES / 4) * 3 + 1 {
        panic!("Not enough nodes responded to the request");
    }

    let mut shares: Vec<NodeSharedSecretDto> = Vec::new();

    for node in 0..N_NODES {
        match client
            .post(format!("{}/api/shared-key", get_node_enpoint(node)))
            .json(&LookupSharedSecretDto {
                owner: owner.clone(),
                token_id: token_id.clone(),
                temp_pub_key: crypto::pub_key_to_str(&temp_pub),
                node_signatures: signatures.clone(),
            })
            .send()
            .await {
                Ok(response) => {
                    shares.push(response.json().await.unwrap());
                },
                Err(_) => {}
            }
    }
    
    let threshold_pub_key = threshold_same(
        shares
            .iter()
            .map(|share| share.pub_key.clone())
            .collect(),
        THRESHOLD
    ).unwrap();

    if shares.len() >= THRESHOLD {
        let mut decrypted_shares: Vec<String> = Vec::new();

        for share in shares {
            let decrypted_share = crypto::decrypt(&temp_priv, hex::decode(share.share).unwrap());
            decrypted_shares.push(hex::encode(decrypted_share));
        }

        let all_combinations: Vec<Vec<String>> = k_combinations(&decrypted_shares, THRESHOLD);

        let mut priv_key: String = "1".to_string();

        for combination in all_combinations {
            let shares: Vec<String> = decrypted_shares
                .iter()
                .filter(|share| combination.contains(share))
                .map(|share| share.to_string())
                .collect();

            
            
        }
    }

    priv_key
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/stargazers",
        owner = "rust-lang-nursery",
        repo = "rust-cookbook");
    println!("{}", request_url);
    let response = reqwest::get(&request_url).await?;

    let users: Vec<User> = response.json().await?;
    println!("{:?}", users);
    Ok(())
}
