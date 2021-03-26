use std::env;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    Command::new("cargo")
        .arg("install")
        .arg("diesel_cli")
        .arg("--no-default-features")
        .arg("--features").arg("postgres")
        .arg("--target-dir").arg(&out_dir)
        .arg("--root").arg(&out_dir)
        .status()
        .unwrap();

    println!("cargo:rustc-env=DIESEL_WRAPPER_EXE={}/bin/diesel", out_dir);
}
