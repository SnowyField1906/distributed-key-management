pub const N_NODES: usize = 3;

pub const GRPC_URLS: [&'static str; N_NODES] = ["[::1]:4001", "[::1]:4002", "[::1]:4003"];

pub const THRESHOLD: usize = N_NODES / 2 + 1;

pub const TOKEN_INFO_GOOGLE_API: &str = "https://www.googleapis.com/oauth2/v3/tokeninfo";

pub const VERIFIERS: [&'static str; 1] = ["google"];
