use std::fs;

pub const N_NODES: u8 = fs::read_dir("./../config/node_info")
    .unwrap()
    .filter_map(Result::ok)
    .filter(|entry| entry.file_type().map(|ft| ft.is_file()).unwrap_or(false))
    .count();

pub const THRESHOLD: u8 = N_NODES / 2 + 1;