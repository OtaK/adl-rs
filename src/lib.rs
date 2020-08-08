#[cfg(all(windows, target_pointer_width = "32"))]
pub const LIBRARY_NAME: &str = "atiadlxx.dll";
#[cfg(all(windows, target_pointer_width = "64"))]
pub const LIBRARY_NAME: &str = "atiadlxy.dll";
#[cfg(unix)]
pub const LIBRARY_NAME: &str = "libatiadlxx.so";

mod error;
pub use self::error::*;

#[derive(Debug)]
pub struct AmdDisplayLibrary {
    handle: adl_sys::ADL_CONTEXT_HANDLE,
    lib: libloading::Library
}

impl AmdDisplayLibrary {
    pub fn new() -> AdlResult<Self> {
        let lib = libloading::Library::new(LIBRARY_NAME)?;
        Ok(Self { lib })
    }
}
