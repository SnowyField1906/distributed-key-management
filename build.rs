use std::error::Error;
use tonic_build;

fn main() -> Result<(), Box<dyn Error>> {
    tonic_build::compile_protos("grpc/protos/p2p.proto")?;
    Ok(())
}