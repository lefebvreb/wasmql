use std::fs::{read, File};
use std::io::{Result, Write};
use std::path::Path;

use minify_js::{Session, TopLevelMode};

const JS_LIB_PATH: &str = "../src/wasmql.js";
const JS_MIN_LIB_PATH: &str = "../src/wasmql.min.js";

fn minify(out_dir: &Path) -> Result<()> {
    let mut out = Vec::new();

    minify_js::minify(&Session::new(), TopLevelMode::Module, &read(JS_LIB_PATH)?, &mut out)
        .expect("js syntax error");

    File::create(out_dir.join(JS_MIN_LIB_PATH))?
        .write_all(&out)?;

    Ok(())
}

fn compile(ql_dir: &Path, out_dir: &Path) -> Result<()> {
    todo!()
}

pub fn dist(
    ql_dir: impl AsRef<Path>,
    out_dir: impl AsRef<Path>,
) -> Result<()> {
    let ql_dir = ql_dir.as_ref();
    let out_dir = out_dir.as_ref();
    minify(out_dir)?;
    compile(ql_dir, out_dir)?;
    Ok(())
}