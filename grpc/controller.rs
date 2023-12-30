use crate::{
    services::{
        shared_key_service,
        wallet_service,
    },
    common::crypto,
    grpc::{
        service,
        p2p::{
            *,
            p2p_server::{
                P2p,
                P2pServer
            }
        }
    }
};
use tonic::{
    Request,
    Response,
    Status
};

#[derive(Debug, Default)]
pub struct P2PController {}

#[tonic::async_trait]
impl P2p for P2PController {
    async fn init_secret(
        &self,
        data: Request<InitSecretRequest>
    ) -> Result<Response<InitSecretResponse>, tonic::Status> {
        let data: InitSecretRequest = data.into_inner();

        match shared_key_service::create(&data.owner).await {
            Ok(pub_key) => Ok(
                Response::new(InitSecretResponse {
                    pub_key: crypto::pub_key_to_string(&pub_key)
                })
            ),
            Err(error) => Err(
                Status::internal(error.get_message())
            ),
        }
    }

    async fn generate_shares(
        &self,
        data: Request<GenerateSharesRequest>
    ) -> Result<Response<GenerateSharesResponse>, Status> {
        let data: GenerateSharesRequest = data.into_inner();
        
        match service::generate_shares(&data.owner).await {
            Ok(status) => Ok(
                Response::new(GenerateSharesResponse { status: true })
            ),
            Err(error) => Err(
                Status::internal(error.get_message())
            ),
        }
    }

    async fn check_wallet(
        &self,
        data: Request<CheckWalletRequest>
    ) -> Result<Response<CheckWalletResponse>, Status> {
        let data: CheckWalletRequest = data.into_inner();

        match wallet_service::find_by_owner(&data.email).await {
            Ok(wallet) => Ok(
                Response::new(CheckWalletResponse { 
                    pub_key: wallet.pub_key,
                    address: wallet.address,
                 })
            ),
            Err(error) => Err(
                Status::internal(error.get_message())
            ),
        }
    }

    async fn broadcast_assign_key(
        &self,
        data: Request<BroadcastAssignKeyRequest>
    ) -> Result<Response<BroadcastAssignKeyResponse>, Status> {
        let data: BroadcastAssignKeyRequest = data.into_inner();
        
        Ok(
            Response::new(BroadcastAssignKeyResponse {
                id: data.id,
                name: "".to_string(),
            })
        )
    }

    async fn add_received_share(
        &self,
        data: Request<AddReceivedShareRequest>
    ) -> Result<Response<AddReceivedShareResponse>, Status> {
        let data: AddReceivedShareRequest = data.into_inner();
        
        match shared_key_service::add_received_share(&data.owner, &data.received_share).await {
            Ok(_) => Ok(
                Response::new(AddReceivedShareResponse { status: true })
            ),
            Err(error) => Err(
                Status::internal(error.get_message())
            ),
        }
    }

    async fn derive_shared_secret(
        &self,
        data: Request<DeriveSharedSecretRequest>
    ) -> Result<Response<DeriveSharedSecretResponse>, Status> {
        let data: DeriveSharedSecretRequest = data.into_inner();
        
        match shared_key_service::derive_shared_secret(&data.owner).await {
            Ok(_) => Ok(
                Response::new(DeriveSharedSecretResponse { status: true })
            ),
            Err(error) => Err(
                Status::internal(error.get_message())
            ),
        }

    }

    async fn store_wallet_info(
        &self,
        data: Request<StoreWalletInfoRequest>
    ) -> Result<Response<StoreWalletInfoResponse>, Status> {
        let data: StoreWalletInfoRequest = data.into_inner();
        
        match wallet_service::create(
            data.owner,
            data.pub_key,
            data.address
        ).await {
            Ok(_) => Ok(
                Response::new(StoreWalletInfoResponse { status: true })
            ),
            Err(error) => Err(
                Status::internal(error.get_message())
            ),
        }
    }
}

impl P2PController {
    pub fn new() -> P2pServer<P2PController> {
        P2pServer::new(P2PController::default())
    }
}