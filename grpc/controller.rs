use std::sync::Arc;

use actix_web::web::Data;
use tokio::sync::RwLockWriteGuard;
use tonic::{
	transport::Channel,
	Request,
	Response,
	Status,
};

use crate::{
	common::{
		constants,
		crypto,
		messages,
	},
	config::{
		database::DatabasePool,
		microservice::{
			p2p::{
				p2p_client::P2pClient,
				p2p_server::{
					P2p,
					P2pServer,
				},
				*,
			},
			GrpcPool,
		},
	},
	grpc::service,
	services::{
		shared_key_service,
		wallet_service,
	},
};

#[derive(Debug, Default)]
pub struct P2PController {
	database_pool: Data<Arc<DatabasePool>>,
	grpc_pool: Data<Arc<GrpcPool>>,
}

#[tonic::async_trait]
impl P2p for P2PController {
	async fn init_secret(
		&self, data: Request<InitSecretRequest>,
	) -> Result<Response<InitSecretResponse>, tonic::Status> {
		let data: InitSecretRequest = data.into_inner();

		match shared_key_service::create(&self.database_pool, &data.owner).await {
			Ok(pub_key) => {
				Ok(Response::new(InitSecretResponse {
					pub_key: crypto::pub_key_to_str(&pub_key),
				}))
			}
			Err(error) => Err(Status::internal(error.get_message())),
		}
	}

	async fn broadcast_all(
		&self, data: Request<BroadcastAllRequest>,
	) -> Result<Response<BroadcastAllResponse>, tonic::Status> {
		match service::broadcast_all(&self.grpc_pool).await {
			Ok(_) => Ok(Response::new(BroadcastAllResponse {})),
			Err(error) => Err(Status::internal(error.get_message())),
		}
	}

	async fn generate_shares(
		&self, data: Request<GenerateSharesRequest>,
	) -> Result<Response<GenerateSharesResponse>, Status> {
		let data: GenerateSharesRequest = data.into_inner();

		match service::generate_shares(&self.database_pool, &self.grpc_pool, &data.owner).await {
			Ok(_) => Ok(Response::new(GenerateSharesResponse { status: true })),
			Err(error) => Err(Status::internal(error.get_message())),
		}
	}

	async fn check_wallet(
		&self, data: Request<CheckWalletRequest>,
	) -> Result<Response<CheckWalletResponse>, Status> {
		let data: CheckWalletRequest = data.into_inner();

		match wallet_service::find_by_owner(&self.database_pool, &data.email).await {
			Ok(wallet) => {
				Ok(Response::new(CheckWalletResponse {
					pub_key: wallet.pub_key,
					address: wallet.address,
				}))
			}
			Err(error) => Err(Status::internal(error.get_message())),
		}
	}

	async fn broadcast_assign_key(
		&self, data: Request<BroadcastAssignKeyRequest>,
	) -> Result<Response<BroadcastAssignKeyResponse>, Status> {
		let data: BroadcastAssignKeyRequest = data.into_inner();

		Ok(Response::new(BroadcastAssignKeyResponse {
			id: data.id,
			name: "".to_string(),
		}))
	}

	async fn add_received_share(
		&self, data: Request<AddReceivedShareRequest>,
	) -> Result<Response<AddReceivedShareResponse>, Status> {
		let data: AddReceivedShareRequest = data.into_inner();

		match shared_key_service::add_received_share(
			&self.database_pool,
			&data.owner,
			&data.received_share,
		)
		.await
		{
			Ok(_) => Ok(Response::new(AddReceivedShareResponse { status: true })),
			Err(error) => Err(Status::internal(error.get_message())),
		}
	}

	async fn derive_shared_secret(
		&self, data: Request<DeriveSharedSecretRequest>,
	) -> Result<Response<DeriveSharedSecretResponse>, Status> {
		let data: DeriveSharedSecretRequest = data.into_inner();

		match shared_key_service::derive_shared_secret(&self.database_pool, &data.owner).await {
			Ok(_) => Ok(Response::new(DeriveSharedSecretResponse { status: true })),
			Err(error) => Err(Status::internal(error.get_message())),
		}
	}

	async fn store_wallet_info(
		&self, data: Request<StoreWalletInfoRequest>,
	) -> Result<Response<StoreWalletInfoResponse>, Status> {
		let data: StoreWalletInfoRequest = data.into_inner();

		match wallet_service::create(&self.database_pool, data.owner, data.pub_key, data.address)
			.await
		{
			Ok(_) => Ok(Response::new(StoreWalletInfoResponse { status: true })),
			Err(error) => Err(Status::internal(error.get_message())),
		}
	}
}

impl P2PController {
	pub async fn new(
		database_pool: Data<Arc<DatabasePool>>, grpc_pool: Data<Arc<GrpcPool>>,
	) -> P2pServer<P2PController> {
		P2pServer::new(P2PController {
			database_pool,
			grpc_pool,
		})
	}
}
