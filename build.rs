use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/helloworld.proto")?;
    Ok(())

    // let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    // println!(">>> out_dir: {:?}", out_dir);
    // tonic_build::configure()
    //     .file_descriptor_set_path(out_dir.join("helloworld_descriptor.bin"))
    //     .compile_protos(&["proto/helloworld.proto"], &["proto"])
    //     .unwrap();
}