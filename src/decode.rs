use std::slice;
use std::os::raw::c_void;
use ndarray::Array2;
use rawspeed_sys as ffi;
use ::camera_metadata::{CameraMetadata, DEFAULT_CAMERA_METADATA};

#[derive(Debug, Fail)]
#[fail(display = "failed to decode raw image: {}", _0)]
pub struct DecodeError(String);

pub struct RawImage {
    obj_ptr: *mut c_void,
    pub info: ffi::RawspeedImageInfo,
}

impl RawImage {
    pub fn decode(data: &[u8], camera_meta: &CameraMetadata) -> Result<Self, DecodeError> {
        let obj_ptr = unsafe {
            ffi_call_fallible!(
                ffi::rawspeed_rawimage_decode,
                DecodeError,
                data.as_ptr(),
                data.len(),
                camera_meta.as_ptr())
        };
        let info = unsafe { ffi::rawspeed_rawimage_info(obj_ptr) };
        Ok(RawImage {
            obj_ptr,
            info,
        })
    }

    #[inline]
    pub fn data(&self) -> &[u16] {
        let data_ptr = self.info.data as *mut u16;
        let data_len = self.info.pitch as usize * self.info.height as usize / 2;
        unsafe { slice::from_raw_parts(data_ptr, data_len) }
    }
}

impl Drop for RawImage {
    fn drop(&mut self) {
        unsafe {
            ffi::rawspeed_rawimage_free(self.obj_ptr);
        }
    }
}

pub fn decode(data: &[u8]) -> Result<Array2<u16>, DecodeError> {
    decode_with_metadata(data, &DEFAULT_CAMERA_METADATA)
}

pub fn decode_with_metadata(data: &[u8], camera_meta: &CameraMetadata) -> Result<Array2<u16>, DecodeError> {
    let raw_image = RawImage::decode(data, camera_meta)?;
    let info = raw_image.info;
    let width = info.width as usize;
    let height = info.height as usize;
    let pitch = info.pitch as usize / 2;
    let data = raw_image.data();
    let mut pixels = Vec::with_capacity(width * height);
    for y in 0..height {
        let offset = y * pitch;
        pixels.extend(&data[offset..offset + width]);
    }
    Ok(Array2::from_shape_vec((height, width), pixels).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn test_err() {
        let res = decode(&[]);
        assert!(res.is_err());
    }

    #[test]
    fn test_ok() {
        let mut file = File::open("test_data/test.cr2").unwrap();
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();
        let res = decode(&data).unwrap();
        // height
        //assert_eq!(res.shape()[0], 3666);
        assert_eq!(res.shape()[0], 3708);
        // width
        //assert_eq!(res.shape()[1], 5494);
        assert_eq!(res.shape()[1], 5568);
        assert_eq!(res[[0, 0]], 2076);
        assert_eq!(res[[100, 1]], 2156);
        assert_eq!(res[[1, 200]], 2169);
        assert_eq!(res[[3665, 5493]], 2057);
    }
}
