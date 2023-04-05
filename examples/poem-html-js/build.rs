use std::io::Result;

fn main() -> Result<()> {
    wasmql_build::export_js_library("dist/wasmql.min.js", true)?;
    wasmql_build::compile_wasm_codec("codec", "dist/codec.wasm")?;
    Ok(())
}
