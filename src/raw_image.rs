use std::os::raw::c_void;
use std::slice;
use rawspeed_sys as ffi;
use ::camera_metadata::CameraMetadata;

pub struct RawImage(*mut c_void);

#[derive(Debug, Fail)]
#[fail(display = "failed to decode raw image: {}", _0)]
pub struct DecodeError(String);

impl RawImage {
    pub fn decode(data: &[u8], camera_meta: &CameraMetadata) -> Result<Self, DecodeError> {
        let ptr = unsafe {
            ffi_call_fallible!(
                ffi::rawspeed_rawimage_decode,
                DecodeError,
                data.as_ptr(),
                data.len(),
                camera_meta.as_ptr())
        };
        Ok(RawImage(ptr))
    }

    pub fn data(&self) -> &[u8] {
        unsafe {
            let ptr = ffi::rawspeed_rawimage_data(self.0);
            slice::from_raw_parts(ptr, self.pitch() as usize * self.height() as usize)
        }
    }

    pub fn height(&self) -> u32 {
        unsafe { ffi::rawspeed_rawimage_height(self.0) as u32 }
    }

    pub fn width(&self) -> u32 {
        unsafe { ffi::rawspeed_rawimage_width(self.0) as u32 }
    }

    pub fn pitch(&self) -> u32 {
        unsafe { ffi::rawspeed_rawimage_pitch(self.0) as u32 }
    }
}

impl Drop for RawImage {
    fn drop(&mut self) {
        unsafe {
            ffi::rawspeed_rawimage_free(self.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;
    use ::camera_metadata::CameraMetadata;

    #[test]
    fn test_err() {
        let meta = CameraMetadata::default().unwrap();
        let res = RawImage::decode(&[], &meta);
        assert!(res.is_err());
    }

    #[test]
    fn test_ok() {
        let meta = CameraMetadata::default().unwrap();
        let mut file = File::open("test_data/test.cr2").unwrap();
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();
        let res = RawImage::decode(&data, &meta).unwrap();
        assert_eq!(res.width(), 5494);
        assert_eq!(res.height(), 3666);
        assert_eq!(res.pitch(), 11136);
        assert_eq!(res.data().len(), 40824576);
    }
}
