use std::{fs, env};
use std::path::Path;
use std::io::Result;
use std::process::Command;

pub const MINIFIED_JS: &str = include_str!(concat!(env!("OUT_DIR"), "/wasmql.min.js"));

pub fn export_minified_js<P: AsRef<Path>>(path: P) -> Result<()> {
    fs::write(path.as_ref(), MINIFIED_JS)
}

pub fn compile_wasm_module(crate_name: impl AsRef<str>, out_file: impl AsRef<str>) -> Result<()>  {
    let crate_name = crate_name.as_ref();
    let out_file = out_file.as_ref();

    let target_dir = format!("{}/target", env::var("OUT_DIR").unwrap());
    fs::create_dir_all(&target_dir)?;

    let out = Command::new("cargo")
        .args([
            "rustc", "-v",
            "--release",
            "--package", crate_name,
            "--target", "wasm32-unknown-unknown",
            "--crate-type", "cdylib",
            "--target-dir", &format!("{}/target", env::var("OUT_DIR").unwrap()),
            "--",
            "-o", out_file,
            "-C", "opt-level=z",
            "-C", "panic=abort",
            "-C", "strip=symbols",
        ])
        .output()?;

    fs::write("cargo.log", out.stderr)?;

    // let wasm_output = Path::new("target/wasm32-unknown-unknown/release")
    //     .join(format!("{crate_name}.wasm"));

    // eprintln!("{:?}", wasm_output);

    // fs::copy(wasm_output, out_file)?;

    Ok(())
}
