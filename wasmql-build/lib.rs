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
    let dest = dest.as_ref();
    println!("cargo:rerun-if-changed={}", dest.display());
    fs::write(dest, MINIFIED_JS)
}

pub fn compile_wasm_codec<P1: AsRef<Path>, P2: AsRef<Path>>(crate_path: P1, dest: P2) -> Result<()> {
    let crate_path = crate_path.as_ref();
    let dest = dest.as_ref();

    println!("cargo:rerun-if-changed={}", crate_path.display());
    println!("cargo:rerun-if-changed={}", dest.display());

    let target_dir = Path::new(&env::var("OUT_DIR").unwrap())
        .join("target");

    fs::create_dir_all(&target_dir)?;

    Command::new("cargo")
        .current_dir(crate_path)
        .arg("rustc")
        .arg("--color").arg("always")
        .arg("--release")
        .arg("--target-dir").arg(&target_dir)
        .arg("--target").arg("wasm32-unknown-unknown")
        .arg("--config").arg("profile.release.lto='thin'")
        .arg("--config").arg("profile.release.opt-level='z'")
        .arg("--config").arg("profile.release.strip='symbols'")
        .arg("--crate-type").arg("cdylib")
        .status()?;

    let wasm_file = target_dir
        .join("wasm32-unknown-unknown")
        .join("release")
        .join(crate_path.with_extension("wasm"));

    fs::copy(wasm_file, dest)?;

    Ok(())
}
