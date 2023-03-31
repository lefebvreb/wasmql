use std::io::Result;
use std::process::Command;

fn main() -> Result<()> {
    // Build frontend.
    Command::new("npm")
            .args(["run", "build"])
            .status()?;

    // Compile the wasm codec and export the minified js lib.
    wasmql_build::export_minified_js("dist/wasmql.min.js")?;
    wasmql_build::compile_wasm_codec("codec", "dist/codec.wasm")?;

    Ok(())
}
