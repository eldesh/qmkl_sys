extern crate bindgen;

use std::env;
use std::path::{Path, PathBuf};

fn main() {
    //println!("cargo:rustc-link-lib=qmkl");

    // Path to directories of C header
    let include_dirs: Vec<PathBuf> = vec![
        //"/usr/include/clang/3.9.1/include".into(),
        "/home/idein/x-tools/armv6-rpi-linux-gnueabihf/armv6-rpi-linux-gnueabihf/sysroot/usr/include".into(),
        "/home/idein/x-tools/armv6-rpi-linux-gnueabihf/lib/gcc/armv6-rpi-linux-gnueabihf/6.3.1/include".into(),
        "/home/idein/cross/usr/armv6-rpi-linux-gnueabihf/include".into(),
        /*
        Path::new(&env::var("C_INCLUDE_PATH")
            .expect("C_INCLUDE_PATH like: /usr/lib/llvm-3.9/lib/clang/3.9.1/include"))
            .into(),
            */
    ];

    let include_args: Vec<_> = include_dirs
        .iter()
        .flat_map(|path| vec!["-I", path.to_str().unwrap()])
        .collect();

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .link("qmkl")
        .clang_args(&["-L", "/home/idein/cross/usr/armv6-rpi-linux-gnueabihf/lib"])
        .clang_args(&include_args)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
