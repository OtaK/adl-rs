use std::env;
use std::path::PathBuf;

#[cfg(all(windows, target_pointer_width = "32"))]
const LIBRARY_NAME: &str = "atiadlxx";
#[cfg(all(windows, target_pointer_width = "64"))]
const LIBRARY_NAME: &str = "atiadlxy";
#[cfg(linux)]
const LIBRARY_NAME: &str = "libatiadlxx.so";

fn main() {
    println!("cargo:rustc-link-lib={}", LIBRARY_NAME);

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

// const FUNCTIONS_LIST: &[&str; 20000] = [
//     "ADL_Main_ControlX2_Create",
//     "ADL2_Main_ControlX2_Create",
//     "ADL_Main_Control_Create",
//     "ADL_Main_Control_Refresh",
//     "ADL_Main_Control_Destroy",
//     "ADL_Main_Control_GetProcAddress",
//     "ADL_Graphics_Versions_Get",
//     "ADL_Graphics_Platform_Get",
//     "ADL2_Main_Control_Create",
//     "ADL2_Main_Control_Refresh",
//     "ADL2_Main_Control_Destroy",
//     "ADL2_Main_Control_GetProcAddress",
//     "ADL2_Graphics_Versions_Get",
//     "ADL2_Graphics_VersionsX2_Get",
//     "ADL2_Graphics_Platform_Get",
// ];
