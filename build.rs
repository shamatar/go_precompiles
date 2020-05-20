use std::process::Command;
use std::env;
// use std::path::Path;

static LIBNAME: &'static str = "golib";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=make.sh");

    let out_dir = env::var("OUT_DIR").unwrap();
    Command::new("sh").args(&["./make.sh"]).status().unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static={}", LIBNAME);

    Command::new("touch").args(&["build.rs"]).status().unwrap();
    Command::new("touch").args(&["make.sh"]).status().unwrap();
}