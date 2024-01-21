use std::env;

use tokio::sync::{
	RwLock,
	RwLockReadGuard,
	RwLockWriteGuard,
};
use tonic::transport::Channel;

use crate::common::constants::{
	GRPC_URLS,
	N_NODES,
};
pub mod p2p {
	tonic::include_proto!("p2p");
}

use p2p::p2p_client::P2pClient;

#[derive(Debug, Default)]
pub struct GrpcPool {
	clients: Vec<RwLock<P2pClient<Channel>>>,
	index: usize,
}

impl GrpcPool {
	pub async fn new() -> Self {
		let mut clients = Vec::with_capacity(N_NODES);

		for node in 0..N_NODES {
			let client = P2pClient::connect(format!("http://{}", GRPC_URLS[node]))
				.await
				.unwrap();
			clients.push(RwLock::new(client));
		}

		let index: usize = env::args().nth(1).unwrap().parse().unwrap();

		GrpcPool { clients, index }
	}

	pub async fn get_client(
		&self, index: Option<usize>,
	) -> RwLockReadGuard<'_, P2pClient<Channel>> {
		self.clients[index.unwrap_or(self.index)].read().await
	}

	pub async fn get_client_mut(
		&self, index: Option<usize>,
	) -> RwLockWriteGuard<'_, P2pClient<Channel>> {
		self.clients[index.unwrap_or(self.index)].write().await
	}
}
