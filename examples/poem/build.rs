use std::io::Result;

fn main() -> Result<()> {
    // Hack.
    std::env::set_var("REBUILD", format!("{:?}", std::time::Instant::now()));
    println!("cargo:rerun-if-env-changed=REBUILD");

    wasmql_build::export_minified_js("dist/wasmql.min.js")?;
    wasmql_build::compile_wasm_codec("codec", "dist/codec.wasm")?;
    Ok(())
}
