use std::io::Result;
use std::process::Command;

fn main() -> Result<()> {
    // Export the js library and its bindings to the source directory.
    wasmql_build::export_js_library("src/wasmql.js", false)?;
    wasmql_build::export_ts_bindings("src/wasmql.d.ts")?;

    // Build frontend.
    Command::new("npm")
            .args(["run", "build"])
            .status()?;

    // Compile wasm binary to dist directory.
    wasmql_build::compile_wasm_codec("codec", "dist/codec.wasm")?;

    Ok(())
}
