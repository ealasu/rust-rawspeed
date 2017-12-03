extern crate libc;
#[macro_use] extern crate failure;
extern crate rawspeed_sys;

use std::ffi::CString;
use std::path::Path;
use std::os::raw::c_void;
use rawspeed_sys::*;
use failure::Error;

#[derive(Debug, Fail)]
pub enum RawspeedError {
    #[fail(display = "failed to init camera metadata")]
    CameraMetadataInit
}

pub struct CameraMetadata(*const c_void);

impl CameraMetadata {
    pub fn init(filename: &str) -> Result<Self, RawspeedError> {
        let filename = CString::new(filename).unwrap();
        let ptr = unsafe { rawspeed_metadata_init(filename.as_ptr()) };
        if ptr.is_null() {
            return Err(RawspeedError::CameraMetadataInit);
        }
        Ok(CameraMetadata(ptr))
    }
}
