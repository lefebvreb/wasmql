use std::io::Result;

fn main() -> Result<()> {
    wasmql_build::export_minified_js("dist/wasmql.min.js")?;
    wasmql_build::compile_wasm_module("protocol", "dist/mod.wasm")?;
    Ok(())
}