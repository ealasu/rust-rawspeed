use std::ffi::CString;
use std::os::raw::c_void;
use std::fs::File;
use std::io::prelude::*;
use rawspeed_sys as ffi;
use failure::Error;
use tempdir::TempDir;

pub struct CameraMetadata(*mut c_void);

#[derive(Debug, Fail)]
#[fail(display = "failed to init camera metadata: {}", _0)]
pub struct CameraMetadataInitError(String);

impl CameraMetadata {
    pub fn init(filename: &str) -> Result<Self, CameraMetadataInitError> {
        let filename = CString::new(filename).unwrap();
        unsafe {
            let ptr = ffi_call_fallible!(
                ffi::rawspeed_metadata_init,
                CameraMetadataInitError,
                filename.as_ptr());
            Ok(CameraMetadata(ptr))
        }
    }

    pub fn default() -> Result<Self, Error> {
        let tmp_dir = TempDir::new("rawspeed")?;
        let file_path = tmp_dir.path().join("cameras.xml");
        let mut tmp_file = File::create(&file_path)?;
        tmp_file.write_all(ffi::data::CAMERAS_XML.as_bytes())?;
        Ok(Self::init(file_path.to_str().unwrap())?)
    }

    pub fn as_ptr(&self) -> *const c_void {
        self.0
    }
}

impl Drop for CameraMetadata {
    fn drop(&mut self) {
        unsafe {
            ffi::rawspeed_metadata_free(self.0);
        }
    }
}

unsafe impl Sync for CameraMetadata {}

lazy_static! {
    pub static ref DEFAULT_CAMERA_METADATA: CameraMetadata =
        CameraMetadata::default().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_err() {
        let res = CameraMetadata::init("");
        assert!(res.is_err());
    }

    #[test]
    fn test_success() {
        CameraMetadata::default().unwrap().as_ptr();
        DEFAULT_CAMERA_METADATA.as_ptr();
    }
}
