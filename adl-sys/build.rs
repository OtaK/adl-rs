use std::env;
use std::path::PathBuf;

fn main() {
    #[cfg(target_arch = "x86")]
    println!("cargo:rustc-link-lib=atiadlxx");
    #[cfg(target_arch = "x86_64")]
    println!("cargo:rustc-link-lib=atiadlxy");

    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
