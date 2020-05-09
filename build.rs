extern crate bindgen;

use std::env;
use std::path::{PathBuf, Path};
use std::process::Command;

fn build_lightning(prefix: &str) {
    Command::new("./build-lightning.sh")
        .arg(prefix)
        .output().unwrap();
}

fn build_c(prefix: &str) {
    Command::new("make")
        .env("PREFIX", prefix)
        .arg("-C")
        .arg("C/")
        .output().unwrap();
}

fn lightning_built(prefix: &Path) -> bool {
    prefix.exists()
}

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let prefix = out_path.join("lightning-prefix");
    let libdir = prefix.join("lib");
    let incdir = prefix.join("include");

    if !lightning_built(&prefix) {
        build_lightning(prefix.to_str().unwrap());
    }
    build_c(prefix.to_str().unwrap());

    println!("cargo:rustc-link-search=native={}", libdir.to_str().unwrap());

    println!("cargo:rustc-link-lib=static=lightning");
    println!("cargo:rustc-link-lib=static=lightningsys");

    // Tell cargo to rerun the build if these files changed.
    println!("cargo:rerun-if-changed=C/lightning-sys.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        // Tell bindgen to regenerate bindings if the wrapper.h's contents or transitively
        // included files change.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .rustfmt_bindings(true)
        .clang_arg(format!("-I{}", incdir.to_str().unwrap()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
