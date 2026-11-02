use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Compile Swift file into a static object block
    let status = Command::new("swiftc")
        .args([
            "-emit-object",
            "-module-name",
            "McWalkCore",
            "-parse-as-library",
            "src/McWalkCore.swift",
            "-o",
            out_dir.join("McWalkCore.o").to_str().unwrap(),
        ])
        .status()
        .expect("Failed to execute swiftc compiler");

    assert!(status.success(), "Swift compilation failed");

    // Package the object file into a static library (.a)
    let status = Command::new("ar")
        .args([
            "crs",
            out_dir.join("libMcWalkCore.a").to_str().unwrap(),
            out_dir.join("McWalkCore.o").to_str().unwrap(),
        ])
        .status()
        .expect("Failed to execute ar archive manager");

    assert!(status.success(), "Archiving Swift object into static library failed");

    // Link directives for Rust compiler
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=McWalkCore");

    // Core Foundation & Swift runtime links required on macOS execution
    println!("cargo:rustc-link-lib=framework=Foundation");
    println!("cargo:rustc-link-search=framework=/System/Library/Frameworks");
}
