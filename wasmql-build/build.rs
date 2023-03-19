use std::env;
use std::fs::{read, File};
use std::io::{Result, Write};
use std::path::Path;

use minify_js::{Session, TopLevelMode};

const JS_LIB_PATH: &str = "../src/wasmql.js";

/// Minifies the js lib, that is then included in the source code of
/// the `wasmql-build` crate.
fn main() -> Result<()> {
    println!("cargo:rerun-if-changed={JS_LIB_PATH}");

    let mut out = Vec::new();

    minify_js::minify(&Session::new(), TopLevelMode::Module, &read(JS_LIB_PATH)?, &mut out)
        .expect("js syntax error");

    let out_file = Path::new(&env::var("OUT_DIR").unwrap())
        .join("wasmql.min.js");

    File::create(out_file)?
        .write_all(&out)?;

    Ok(())
}