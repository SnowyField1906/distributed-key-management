pub struct NodeEnpoint {
    ip: &'static str,
    port: u16
}

impl NodeEnpoint {
    pub fn get_endpoint(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}

pub const NODES: [NodeEnpoint; 3] = [
    NodeEnpoint {
        ip: "127.0.0.1",
        port: 4001
    },
    NodeEnpoint {
        ip: "127.0.0.1",
        port: 4002
    },
    NodeEnpoint {
        ip: "127.0.0.1",
        port: 4003
    },
];

pub const N_NODES: u8 = NODES.len() as u8;

pub const THRESHOLD: u8 = N_NODES / 2 + 1;