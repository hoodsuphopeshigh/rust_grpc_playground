fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=protos/helloworld.proto");

    tonic_build::compile_protos("protos/helloworld.proto")?;
    Ok(())
}
