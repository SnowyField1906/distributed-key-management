use crate::{
    grpc::{
        types::*,
        service
    },
    services::{
        shared_key_service,
        wallet_service,
    }
};
use tonic::{
    transport::Server,
    Request,
    Response,
    Status
};

pub struct GrpcController {
    pub wallet_service: WalletService,
    pub shared_key_service: SharedKeyService,
}
