use std::{fs, env};
use std::path::Path;
use std::io::Result;
use std::process::Command;
use std::str;

pub const MINIFIED_JS: &str = include_str!(concat!(env!("OUT_DIR"), "/wasmql.min.js"));

pub fn export_minified_js<P: AsRef<Path>>(path: P) -> Result<()> {
    fs::write(path.as_ref(), MINIFIED_JS)
}

pub fn compile_wasm_module(crate_name: impl AsRef<str>, out_file: impl AsRef<Path>) -> Result<()>  {    
    let crate_name = crate_name.as_ref();

    let target_dir = Path::new(&env::var("OUT_DIR").unwrap())
        .join("target");

    fs::create_dir_all(&target_dir)?;

    Command::new("cargo")
        .arg("rustc")
        .arg("--release")
        .arg("--package").arg(crate_name)
        .arg("--target").arg("wasm32-unknown-unknown")
        .arg("--crate-type").arg("cdylib")
        .arg("--target-dir").arg(&target_dir)
        .arg("--")
        .arg("-Copt-level=z")
        .arg("-Cpanic=abort")
        .arg("-Cstrip=symbols")
        .status()?;

    let wasm_file = Path::new(&target_dir)
        .join("wasm32-unknown-unknown")
        .join("release")
        .join(Path::new(crate_name).with_extension("wasm"));

    fs::copy(wasm_file, out_file.as_ref())?;

    Ok(())
}
