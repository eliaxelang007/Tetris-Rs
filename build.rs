use bindgen::Builder;
use std::env::var;
use std::path::Path;

fn main() {
    let cargo_manifest_dir = var("CARGO_MANIFEST_DIR").unwrap();
    let current_directory = Path::new(&cargo_manifest_dir);
    println!(
        "cargo:rustc-link-search=native={}",
        current_directory.join("raylib_c\\lib").display()
    );

    println!("cargo:rustc-link-lib=raylibdll");
    println!("cargo:rustc-link-lib=raylib");

    let bindings = Builder::default()
        .header("raylib_c/include/wrapper.h")
        .rustified_enum(".*")
        .clang_arg(format!("--target={}", var("TARGET").unwrap()))
        .blocklist_type("__mingw_ldbl_type_t")
        .generate()
        .expect("Couldn't generate bindings.");

    let out_dir = var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir);

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
