use std::io::Result;
use std::process::Command;

fn main() -> Result<()> {
    Command::new("npm")
        .args(["run", "build"])
        .status()?;

    wasmql_build::export_minified_js("dist/wasmql.min.js")?;
    wasmql_build::compile_wasm_codec("codec", "dist/codec.wasm")?;
    Ok(())
}
