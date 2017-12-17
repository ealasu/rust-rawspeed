use std::os::raw::c_void;
use std::slice;
use std::convert::AsRef;
use image::{Image, ImageMut, ImageDimensions, ImageSlice};
use rawspeed_sys as ffi;
use ::camera_metadata::{CameraMetadata, DEFAULT_CAMERA_METADATA};

pub struct RawImage {
    pub dimensions: ImageDimensions,
    data_ptr: *mut u16,
    data_len: usize,
    obj_ptr: *mut c_void,
}

impl<'a> AsRef<ImageSlice<'a, u16>> for RawImage {
    #[inline(always)]
    fn as_ref(&self) -> &ImageSlice<'a, u16> {
        unsafe {
            &*(self as *const RawImage as *const ImageSlice<u16>)
        }
    }
}

#[derive(Debug, Fail)]
#[fail(display = "failed to decode raw image: {}", _0)]
pub struct DecodeError(String);

impl RawImage {
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        Self::decode_with_metadata(data, &DEFAULT_CAMERA_METADATA)
    }

    pub fn decode_with_metadata(data: &[u8], camera_meta: &CameraMetadata) -> Result<Self, DecodeError> {
        let obj_ptr = unsafe {
            ffi_call_fallible!(
                ffi::rawspeed_rawimage_decode,
                DecodeError,
                data.as_ptr(),
                data.len(),
                camera_meta.as_ptr())
        };
        let dimensions = ImageDimensions {
            width: unsafe { ffi::rawspeed_rawimage_width(obj_ptr) as usize },
            height: unsafe { ffi::rawspeed_rawimage_height(obj_ptr) as usize },
            pitch: unsafe { ffi::rawspeed_rawimage_pitch(obj_ptr) as usize } / 2,
        };
        let data_ptr = unsafe { ffi::rawspeed_rawimage_data(obj_ptr) as *mut u16 };
        let data_len = dimensions.pitch * dimensions.height;
        Ok(RawImage {
            dimensions,
            data_ptr,
            data_len,
            obj_ptr,
        })
    }
}

impl Image for RawImage {
    type Pixel = u16;

    fn dimensions(&self) -> ImageDimensions { self.dimensions }

    fn pixels(&self) -> &[Self::Pixel] {
        unsafe { slice::from_raw_parts(self.data_ptr, self.data_len) }
    }
}

impl ImageMut for RawImage {
    fn pixels_mut(&mut self) -> &mut [Self::Pixel] {
        unsafe { slice::from_raw_parts_mut(self.data_ptr, self.data_len) }
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
    fn test_err() {
        let res = RawImage::decode(&[]);
        assert!(res.is_err());
    }

    #[test]
    fn test_ok() {
        let mut file = File::open("test_data/test.cr2").unwrap();
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();
        let res = RawImage::decode(&data).unwrap();
        assert_eq!(res.dimensions.width, 5494);
        assert_eq!(res.dimensions.height, 3666);
        assert_eq!(res.dimensions.pitch, 5568);
        assert_eq!(res.as_bytes().len(), 40824576);
        assert_eq!(res.dimensions(), res.dimensions);
        assert_eq!(res.as_ref().pixels.len(), res.pixels().len());
        assert_eq!(res.as_ref().pixels[0], res.pixels()[0]);
        let len = res.pixels().len();
        assert_eq!(res.as_ref().pixels[len-1], res.pixels()[len-1]);
    }
}
