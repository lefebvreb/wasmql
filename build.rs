use std::fs::{read, File};
use std::io::{Result, Write};

use minify_js::{minify, Session, TopLevelMode};

const JS_LIB_PATH: &str = "js/wasmql.js";
const JS_MIN_PATH: &str = "js/wasmql.min.js";

fn main() -> Result<()> {
    // Minify js lib on change using the `minify-js` crate.
    println!("cargo:rerun-if-changed={JS_LIB_PATH}");

    let mut out = Vec::new();

    minify(&Session::new(), TopLevelMode::Module, &read(JS_LIB_PATH)?, &mut out)
        .expect("syntax error");

    File::create(JS_MIN_PATH)?
        .write_all(&out)?;

    Ok(())
}