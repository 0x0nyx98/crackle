use std::process::Command;
use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let required_asm = ["boot", "trap"];

    for s in required_asm {
        if !(Command::new("as").args(&["-o", &(out_dir.clone() + format!("{s}.o").as_str()),
            format!("src/asm/{s}.s").as_str()])
            .status().unwrap().success() &&
            Command::new("ar").args(&["-crus",
                &(out_dir.clone() + format!("/lib{s}.a").as_str()),
                &(out_dir.clone() + format!("/{s}.o").as_str())])
            .status().unwrap().success()) {
            panic!("failed");
        }
            println!("cargo:rustc-link-search=native={}", out_dir);
            println!("cargo:rustc-link-lib=static={s}");
    }
}
