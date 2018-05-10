use std::slice;
use std::os::raw::c_void;
use ndarray::ArrayView2;
use ndarray::ShapeBuilder;
use libc::c_int;
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
    pub fn decode(data: &[u8], scale: bool) -> Result<Self, DecodeError> {
        Self::decode_with_metadata(data, &DEFAULT_CAMERA_METADATA, scale)
    }

    pub fn decode_with_metadata(data: &[u8], camera_meta: &CameraMetadata, scale: bool) -> Result<Self, DecodeError> {
        let obj_ptr = unsafe {
            ffi_call_fallible!(
                ffi::rawspeed_rawimage_decode,
                DecodeError,
                data.as_ptr(),
                data.len(),
                camera_meta.as_ptr(),
                scale as c_int)
        };
        let info = unsafe { ffi::rawspeed_rawimage_info(obj_ptr) };
        Ok(RawImage {
            obj_ptr,
            info,
        })
    }

    #[inline]
    pub fn data(&self) -> &[u16] {
        let data_ptr = self.info.cropped_data as *mut u16;
        let data_len = self.info.pitch as usize * self.info.cropped_height as usize / 2;
        unsafe { slice::from_raw_parts(data_ptr, data_len) }
    }

    pub fn view(&self) -> ArrayView2<u16> {
        let h = self.info.cropped_height as usize;
        let w = self.info.cropped_width as usize;
        let pitch = self.info.pitch as usize / 2;
        ArrayView2::from_shape((h, w).strides((pitch, 1)), self.data()).unwrap()
    }

    #[inline]
    pub fn data_uncropped(&self) -> &[u16] {
        let data_ptr = self.info.data as *mut u16;
        let data_len = self.info.pitch as usize * self.info.height as usize / 2;
        unsafe { slice::from_raw_parts(data_ptr, data_len) }
    }

    pub fn view_uncropped(&self) -> ArrayView2<u16> {
        let h = self.info.height as usize;
        let w = self.info.width as usize;
        let pitch = self.info.pitch as usize / 2;
        ArrayView2::from_shape((h, w).strides((pitch, 1)), self.data_uncropped()).unwrap()
    }
}

impl Drop for RawImage {
    fn drop(&mut self) {
        unsafe {
            ffi::rawspeed_rawimage_free(self.obj_ptr);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn err() {
        let res = RawImage::decode(&[], false);
        assert!(res.is_err());
    }

    fn test_data() -> Vec<u8> {
        let mut file = File::open("test_data/test.cr2").unwrap();
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();
        data
    }

    #[test]
    fn view_uncropped() {
        let img = RawImage::decode(&test_data(), false).unwrap();
        let res = img.view_uncropped();
        assert_eq!(res.shape()[0], 3708);
        assert_eq!(res.shape()[1], 5568);
        let expected_points = [2049, 2059, 2055, 2058];
        let actual_points = [
            res[[0, 0]],
            res[[100, 1]],
            res[[1, 200]],
            res[[3665, 5493]]
        ];
        assert_eq!(actual_points, expected_points);
    }

    #[test]
    fn view() {
        let img = RawImage::decode(&test_data(), false).unwrap();
        let v = img.view();
        assert_eq!(v.shape()[0], 3666);
        assert_eq!(v.shape()[1], 5494);
        assert_eq!([
                v[[0, 0]],
                v[[100, 1]],
                v[[1, 200]],
                v[[3665, 5493]],
            ],
            [
                2076,
                2156,
                2169,
                2057,
            ]);
    }
}
