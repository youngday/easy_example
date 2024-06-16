// please uncomment for grpc
use std::io::Result;
use std::env;
use std::path::PathBuf;

use poem_grpc_build::Config;

fn main() -> Result<()> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    Config::new()
        .codec("::poem_grpc::codec::JsonCodec")
        .type_attribute(".", "#[derive(serde::Deserialize, serde::Serialize)]")
        .file_descriptor_set_path(out_dir.join("helloworld_descriptor.bin"))
        .compile(&["./proto/helloworld.proto"], &["./proto"])
}



