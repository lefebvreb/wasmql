//! Utilities for build scripts of projects using the WasmQL library.
//! 
//! Generally, you would want to include both the (minified) WasmQL javascript library 
//! to your static files folder, along with your compiled and up-to-date wasm codec(s).
//! 
//! You can use this crate to do just that. First, add this crate to the
//! build dependencies of your project, in `Cargo.toml`:
//! 
//! ```toml
//! [build-dependencies]
//! wasmql-build = "*"
//! ```
//! 
//! Then, put the following snippet in your project's `build.rs`:
//! 
//! ```no_run
//! use std::io::Result;
//!
//! fn main() -> Result<()> {
//!     // Destination path of the minified js lib.
//!     wasmql_build::export_minified_js("static/wasmql.min.js")?;
//!     // Name of the crate containing your codec, path to write the generated wasm binary to.
//!     wasmql_build::compile_wasm_codec("my-codec", "static/codec.wasm")?;
//!     Ok(())
//! }
//! ```

use std::{fs, env};
use std::path::Path;
use std::io::Result;
use std::process::Command;
use std::str;

/// Minified version of the WasmQL javascript library.
/// 
/// While WasmQL codecs are wasm binaries specific to each endpoint, they
/// still require a javascript glue library to operate. This library is tiny (~1kB) and common
/// to all codecs.
/// 
/// This here constant contains the code of this library as a `&'static str`. 
/// Use the [`export_minified_js`] if you want to export this string to a file.
pub const MINIFIED_JS: &str = include_str!(concat!(env!("OUT_DIR"), "/wasmql.min.js"));

pub fn export_minified_js<P: AsRef<Path>>(dest: P) -> Result<()> {
    fs::write(dest.as_ref(), MINIFIED_JS)
}

pub fn compile_wasm_codec<S: AsRef<str>, P: AsRef<Path>>(crate_name: S, dest: P) -> Result<()> {
    let crate_name = crate_name.as_ref();

    let target_dir = Path::new(&env::var("OUT_DIR").unwrap())
        .join("target");

    fs::create_dir_all(&target_dir)?;

    Command::new("cargo")
        .arg("rustc").arg("-v")
        .arg("--release")
        .arg("--package").arg(crate_name)
        .arg("--target").arg("wasm32-unknown-unknown")
        .arg("--crate-type").arg("cdylib")
        .arg("--target-dir").arg(&target_dir)
        .arg("--color").arg("always")
        .arg("--")
        .arg("--cfg").arg("wasmql_frontend")
        .arg("-Copt-level=z")
        .arg("-Cpanic=abort")
        .arg("-Cstrip=symbols")
        .status()?;

    let wasm_file = target_dir
        .join("wasm32-unknown-unknown")
        .join("release")
        .join(Path::new(crate_name).with_extension("wasm"));

    fs::copy(wasm_file, dest.as_ref())?;

    Ok(())
}
