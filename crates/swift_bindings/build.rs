use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    if cfg!(target_os = "macos") {
        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
        let swift_file = PathBuf::from("Bindings.swift");
        let dylib_path = out_dir.join("libaxbridge.dylib");
        let status = Command::new("swiftc")
            .args(&[
                swift_file.to_str().unwrap(),
                "-emit-library",
                "-o",
                dylib_path.to_str().unwrap(),
                "-target",
                "arm64-apple-macos14.0",
            ])
            .status()
            .expect("failed to invoke swiftc");

        if !status.success() {
            panic!("swiftc failed with status: {:?}", status);
        }

        println!("cargo:rustc-link-search=native={}", out_dir.display());
        println!("cargo:rustc-link-lib=dylib=axbridge");
        println!("cargo:rerun-if-changed={}", swift_file.display());
    }
}
