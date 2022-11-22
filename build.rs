use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_dir = "src/proto";
    fs::create_dir_all(Path::new(proto_dir))?;

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir(proto_dir)
        .compile(&["proto/acorn.proto"], &["proto"])?;
    Ok(())
}
