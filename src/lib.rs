extern crate libc;
#[macro_use] extern crate failure;
extern crate rawspeed_sys;

use std::ffi::{CStr, CString};
use std::path::Path;
use std::os::raw::c_void;
use std::ptr;
use rawspeed_sys::*;
use failure::Error;

#[derive(Debug, Fail)]
pub enum RawspeedError {
    #[fail(display = "failed to init camera metadata: {}", what)]
    CameraMetadataInit { what: String },
}

pub struct CameraMetadata(*const c_void);

impl CameraMetadata {
    pub fn init(filename: &str) -> Result<Self, RawspeedError> {
        let filename = CString::new(filename).unwrap();
        unsafe {
            let mut error_msg = ptr::null();
            let res = rawspeed_metadata_init(filename.as_ptr(), &mut error_msg);
            if res.is_null() {
                let what = if error_msg.is_null() {
                    "".to_string()
                } else {
                    CStr::from_ptr(error_msg).to_string_lossy().to_string()
                };
                return Err(RawspeedError::CameraMetadataInit { what });
            }
            Ok(CameraMetadata(res))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        CameraMetadata::init("").unwrap();
    }
}
