use std::io::Result;

extern crate prost_build;
fn main() -> Result<()> {
    // tonic_build::compile_protos("src/protos/remote_worker.proto")?;
    // Ok(())
    // tonic_build::configure()
    //     .build_server(true)
    //     .build_client(true)
    //     .out_dir("protos")
    //     .compile(&["protos/voting.proto"], &["protos"])?; 
    // Ok(())

    prost_build::Config::new()
        .out_dir("src/protos")
        .compile_protos(&["src/protos/remote_worker.proto"], &["src/protos"])?;
    Ok(())
}
