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

pub struct GrpcPool {
	clients: Vec<RwLock<P2pClient<Channel>>>,
}

impl GrpcPool {
	pub async fn new() -> Self {
		let mut clients = Vec::with_capacity(N_NODES);

		for _ in 0..N_NODES {
			let client = P2pClient::connect(GRPC_URLS[0].parse::<String>().unwrap())
				.await
				.unwrap();
			clients.push(RwLock::new(client));
		}

		GrpcPool { clients }
	}

	pub async fn get_client(&self, index: usize) -> RwLockReadGuard<'_, P2pClient<Channel>> {
		self.clients[index].read().await
	}

	pub async fn get_client_mut(&self, index: usize) -> RwLockWriteGuard<'_, P2pClient<Channel>> {
		self.clients[index].write().await
	}
}
