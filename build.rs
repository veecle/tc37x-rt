use std::{env, error::Error, fs::File, io::Write, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    // build directory for this crate
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // extend the library search path
    println!("cargo:rustc-link-search={}", out_dir.display());

    // put linker scripts in the build directory
    File::create(out_dir.join("tc37xA_memory.ld"))?
        .write_all(include_bytes!("ld/tc37xA_memory.ld"))?;
    File::create(out_dir.join("tc37x_bsp_example_llvm.ld"))?
        .write_all(include_bytes!("ld/tc37x_bsp_example_llvm.ld"))?;

    Ok(())
}
