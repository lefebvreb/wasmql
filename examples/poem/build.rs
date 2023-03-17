use std::io::Result;

fn main() -> Result<()> {
    wasmql::build::export_minified_js("dist/wasmql.min.js")?;
    wasmql::build::compile_wasm_module("protocol", "dist/mod.wasm")?;
    Ok(())
}