#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(all(windows, target_pointer_width = "32"))]
const LIBRARY_NAME: &str = "atiadlxx";
#[cfg(all(windows, target_pointer_width = "64"))]
const LIBRARY_NAME: &str = "atiadlxy";
#[cfg(unix)]
const LIBRARY_NAME: &str = "libatiadlxx.so";

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};

#[cfg(unix)]
use libloading::os::unix::*;
#[cfg(windows)]
use libloading::os::windows::*;

#[macro_use]
mod macros;

static LIB_ADDR: AtomicUsize = AtomicUsize::new(0);
static IS_LIB_INIT: AtomicBool = AtomicBool::new(false);
static FN_PTR_ADDR: AtomicUsize = AtomicUsize::new(0);

#[no_mangle]
unsafe extern "C" fn ADL_Main_Memory_Alloc(size: i32) -> *mut std::ffi::c_void {
    use std::convert::TryInto as _;
    libc::malloc(size.try_into().unwrap())
}

#[no_mangle]
unsafe extern "C" fn ADL_Main_Memory_Free(buffer: *mut *mut std::ffi::c_void) {
    if !(*buffer).is_null() {
        libc::free(*buffer);
        *buffer = std::ptr::null_mut();
    }
}

pub(crate) unsafe fn get_adl_fn(fn_name: &str) -> Result<usize, libloading::Error> {
    // FIXME: That's meh
    let lib_addr = LIB_ADDR.load(Ordering::Relaxed);
    let lib_addr = if lib_addr == 0 {
        let lib = Library::new(LIBRARY_NAME)?;
        if !IS_LIB_INIT.load(Ordering::Relaxed) {
            let init_fn: Symbol<unsafe extern "C" fn(callback: ADL_MAIN_MALLOC_CALLBACK, iEnumConnectedAdapters: i32) -> u32> = lib.get(b"ADL_Main_Control_Create\0")?;
            if init_fn(Some(ADL_Main_Memory_Alloc), 0) != ADL_OK {
                panic!("Could not init ADL!");
            }

            IS_LIB_INIT.store(true, Ordering::Relaxed);
        }
        let ptr = lib.into_raw() as usize;
        LIB_ADDR.store(ptr, Ordering::Relaxed);
        ptr
    } else {
        lib_addr
    };



    Ok(0)
}
