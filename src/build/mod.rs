use std::fs;
use std::path::Path;
use std::io::Result;
use std::process::Command;

pub const MINIFIED_JS: &str = include_str!(concat!(env!("OUT_DIR"), "/wasmql.min.js"));

pub fn export_minified_js<P: AsRef<Path>>(path: P) -> Result<()> {
    fs::write(path.as_ref(), MINIFIED_JS.as_bytes())
}

pub fn compile_wasm_module(crate_name: impl AsRef<str>, out_file: impl AsRef<Path>) -> Result<()>  {
    let crate_name = crate_name.as_ref();
    let out_file = out_file.as_ref();

    let out = Command::new("cargo")
        .args([
            "rustc", "-v",
            "--package", crate_name,
            "--target", "wasm32-unknown-unknown",
            "--crate-type", "cdylib",
            "--",
            "-C", "opt-level=z",
            "-C", "panic=abort",
            "-C", "strip=symbols",
        ])
        .output()?;

    fs::write("cargo.log", out.stderr)?;

    let wasm_output = Path::new("target/wasm32-unknown-unknown/release")
        .join(format!("{crate_name}.wasm"));

    fs::copy(wasm_output, out_file)?;

    Ok(())
}
