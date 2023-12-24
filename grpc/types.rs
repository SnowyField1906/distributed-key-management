pub trait P2PService {
  fn checkWallet(input: CheckWalletRequest) -> CheckWalletResponse;
  fn broadcastAssignKey(input: BroadcastAssignKeyRequest) -> BroadcastAssignKeyResponse;
  fn initSecret(input: InitSecretRequest) -> InitSecretResponse;
  fn addReceivedShare(input: AddReceivedShareRequest) -> AddReceivedShareResponse;
  fn generateShares(input: GenerateSharesRequest) -> GenerateSharesResponse;
  fn deriveSharedSecret(input: DeriveSharedSecretRequest) -> DeriveSharedSecretResponse;
  fn storeWalletInfo(input: StoreWalletInfoRequest) -> StoreWalletInfoResponse;
}

pub struct BroadcastAssignKeyRequest {
  id: u16
}

pub struct BroadcastAssignKeyResponse {
  id: u16,
  name: String
}

pub struct CheckWalletRequest {
  email: String
}

pub struct CheckWalletResponse {
  pub_key: String,
  address: String
}

pub struct InitSecretRequest {
  owner: String
}

pub struct InitSecretResponse {
  pub_key: String
}

pub struct AddReceivedShareRequest {
  owner: String,
  received_share: String
}

pub struct AddReceivedShareResponse {
    status: bool
}

pub struct GenerateSharesRequest {
  owner: String
}

pub struct GenerateSharesResponse {
    status: bool
}

pub struct DeriveSharedSecretRequest {
  owner: String
}

pub struct DeriveSharedSecretResponse {
  status: bool
}

pub struct StoreWalletInfoRequest {
  owner: String,
  pub_key: String,
  address: String
}

pub struct StoreWalletInfoResponse {
  status: bool
}