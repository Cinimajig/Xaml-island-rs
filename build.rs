#![allow(dead_code)]

const VERSION: i32 = 0;

fn main() {
    println!("cargo:rerun-if-changed=program.manifest");
    println!("cargo:rerun-if-changed=resources.rc");
    windres::Build::new().compile("resources.rc").unwrap();
}
