#![feature(let_chains)]

use std::{env, path::Path, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=src/boot.asm");

    let out_dir = env::var("OUT_DIR").unwrap();

    let boot_obj_path = Path::new(&out_dir).join("boot.o").display().to_string();
    let output = Command::new("/usr/bin/nasm")
        .args(["-f", "elf32"])
        .args(["-o", &boot_obj_path])
        .arg("src/boot.asm")
        .output()
        .expect("running nasm");

    if let Some(exit_code) = output.status.code() && exit_code != 0 {
        panic!("assembling boot.asm failed!\n{}", String::from_utf8(output.stderr).expect("converting nasm stderr to UTF-8"));
    };

    println!("cargo:rustc-link-arg=-Tlink.ld");
    println!("cargo:rustc-link-arg=-L{out_dir}");
}
