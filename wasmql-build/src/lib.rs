//! Utilities for build scripts of projects using the WasmQL library.
//!
//! You need to include the WasmQL javascript library in your
//! frontend, along with your compiled and up-to-date wasm codec(s).
//!
//! You can use this crate to do just that, by creating a build file
//! that has, for example, the following code in it:
//!
//! ```no_run
//! use std::io::Result;
//!
//! fn main() -> Result<()> {
//!     // Destination path of the js lib.
//!     wasmql_build::export_js_library("src/wasmql.js", false)?;
//!     // Destination path of the ts bindings.
//!     wasmql_build::export_ts_bindings("src/wasmql.d.ts")?;
//!     // Name of the crate containing your codec, path to write the generated wasm binary to.
//!     wasmql_build::compile_wasm_codec("my-codec-crate", "dist/codec.wasm")?;
//!     Ok(())
//! }
//! ```

use std::io::Result;
use std::path::Path;
use std::process::Command;
use std::str;
use std::{env, fs};

/// Source of the WasmQL JavaScript library.
///
/// While WasmQL codecs are wasm binaries specific to each endpoint, they
/// still require a JavaScript glue library to operate. This library is tiny once
/// minified (~1kB) and common to all codecs.
///
/// This here constant contains the code of this library as a `&'static str`.
/// Use the [`export_js_library`] function if you want to export this string to a file.
pub const JS_LIB: &str = include_str!("wasmql.js");

/// Minified version of the WasmQL JavaScript library.
/// 
/// See also [`JS_LIB`].
pub const MINIFIED_JS_LIB: &str = include_str!(concat!(env!("OUT_DIR"), "/wasmql.min.js"));

/// Typescript bindings to the WasmQL JavaScript library.
/// 
/// This constant contains the TypeScripy bindings to the WasmQL library,
/// allowing its usage with TypeScript projects.
/// 
/// You can use the [`export_ts_bindings`] function to export this file wherever you want.
/// 
/// See also [`JS_LIB`].
pub const TS_BINDINGS: &str = include_str!("wasmql.d.ts");

/// Exports the JavaScript WasmQL library to the specified `path`.
///
/// Can be used to always keep the correct version of this library in your frontend sources.
/// 
/// The `minified` flag controls wether or not to export the minified version instead of the
/// source one.
///
/// # Examples
///
/// ```no_run
/// wasmql_build::export_js_library("dist/wasmql.min.js", true)?;
/// ```
pub fn export_js_library<P: AsRef<Path>>(dest: P, minified: bool) -> Result<()> {
    let dest = dest.as_ref();
    println!("cargo:rerun-if-changed={}", dest.display());
    fs::write(dest, if minified { MINIFIED_JS_LIB } else { JS_LIB })
}

/// Exports the TypeScript bindings to the WasmQL library to the specified `path`.
///
/// Can be used to always keep the correct version of this library in your frontend sources.
///
/// # Examples
///
/// ```no_run
/// wasmql_build::export_js_library("src/wasmql.js", false)?;
/// wasmql_build::export_ts_bindings("src/wasmql.d.ts")?;
/// ```
pub fn export_ts_bindings<P: AsRef<Path>>(dest: P) -> Result<()> {
    let dest = dest.as_ref();
    println!("cargo:rerun-if-changed={}", dest.display());
    fs::write(dest, TS_BINDINGS)
}

/// Compiles your WasmQL codec and exports the resulting `.wasm` binary.
///
/// `crate_path` is the path to the directory containing your WasmQL codec crate,
/// and `dest` is the path to export the `.wasm` file to.
///
/// # Examples
///
/// ```no_run
/// wasmql_build::compile_wasm_codec("my-codec-crate", "dist/codec.wasm")?;
/// ```
pub fn compile_wasm_codec<P1: AsRef<Path>, P2: AsRef<Path>>(
    crate_path: P1,
    dest: P2,
) -> Result<()> {
    let crate_path = crate_path.as_ref();
    let dest = dest.as_ref();

    println!("cargo:rerun-if-changed={}", crate_path.display());
    println!("cargo:rerun-if-changed={}", dest.display());

    let target_dir = Path::new(&env::var("OUT_DIR").unwrap()).join("target");

    fs::create_dir_all(&target_dir)?;

    Command::new("cargo")
        .current_dir(crate_path)
        .arg("rustc")
        .arg("--color")
        .arg("always")
        .arg("--release")
        .arg("--target-dir")
        .arg(&target_dir)
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .arg("--config")
        .arg("profile.release.lto='thin'")
        .arg("--config")
        .arg("profile.release.opt-level='z'")
        .arg("--config")
        .arg("profile.release.strip='symbols'")
        .arg("--crate-type")
        .arg("cdylib")
        .status()?;

    let wasm_file = target_dir
        .join("wasm32-unknown-unknown")
        .join("release")
        .join(crate_path.with_extension("wasm"));

    fs::copy(wasm_file, dest)?;

    Ok(())
}
