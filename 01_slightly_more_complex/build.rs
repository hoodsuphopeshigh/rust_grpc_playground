fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=protos/");

    tonic_build::compile_protos("protos/slightlymorecomplex.proto")?;
    Ok(())
}
