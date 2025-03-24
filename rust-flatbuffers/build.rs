use std::process::Command;

fn main() {
    let flatc = "flatc";

    let out_dir = std::env::var("OUT_DIR").unwrap();

    let schema = "./schemes/small.fbs";

    let status = Command::new(flatc)
        .args(&["-o", &out_dir, "--rust", schema])
        .status()
        .expect("Failed to compile schema");

    if !status.success() {
        panic!("Failed to compile schema");
    }
    println!("cargo:rerun-if-changed={}", schema);
}
