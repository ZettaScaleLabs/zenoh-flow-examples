use anyhow::Result;
use std::path::PathBuf;
use prost_build::Config;
use glob::glob;

fn main() -> Result<()> {
    let include_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("protobuf");
    let protos: Vec<_> = glob(include_dir.join("*.proto").to_str().unwrap())?
        .into_iter()
        .filter_map(Result::ok)
        .collect();
    Config::new()
        .type_attribute(".", "#[derive(::serde::Serialize, ::serde::Deserialize, ::zenoh_flow::zenoh_flow_derive::ZFData)]")
        .compile_protos(&protos, &[include_dir])?;
    Ok(())
}
