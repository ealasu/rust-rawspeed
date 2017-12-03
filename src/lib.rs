extern crate libc;
extern crate rawspeed_sys;

use std::ffi::CString;
use std::path::Path;
use std::os::raw::c_void;
use rawspeed_sys::*;

pub struct CameraMetadata(*const c_void);

impl CameraMetadata {
    pub fn init(filename: &str) -> Self {
        let filename = CString::new(filename).unwrap();
        let ptr = unsafe { rawspeed_metadata_init(filename.as_ptr()) };
        if ptr.is_null() {
            None
        }
        CameraMetadata(ptr)
    }
}
