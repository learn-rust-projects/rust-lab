use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("src/pb")?;
    let builder = tonic_prost_build::configure();
    builder
    .out_dir("src/pb")
    .compile_protos(&["proto/route_guide.proto"], &["proto"])
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
    Ok(())
}