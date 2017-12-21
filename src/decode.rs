use std::slice;
use image::{ImageDimensions, OwnedImage};
use rawspeed_sys as ffi;
use ::camera_metadata::{CameraMetadata, DEFAULT_CAMERA_METADATA};

#[derive(Debug, Fail)]
#[fail(display = "failed to decode raw image: {}", _0)]
pub struct DecodeError(String);

pub fn decode(data: &[u8]) -> Result<OwnedImage<u16>, DecodeError> {
    decode_with_metadata(data, &DEFAULT_CAMERA_METADATA)
}

pub fn decode_with_metadata(data: &[u8], camera_meta: &CameraMetadata) -> Result<OwnedImage<u16>, DecodeError> {
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
    let pixels_slice = unsafe { slice::from_raw_parts(data_ptr, data_len) };
    let pixels = pixels_slice.to_vec();
    unsafe {
        ffi::rawspeed_rawimage_free(obj_ptr);
    }
    Ok(OwnedImage {
        dimensions,
        pixels,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;
    use image::Image;

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
        assert_eq!(res.dimensions.width, 5494);
        assert_eq!(res.dimensions.height, 3666);
        assert_eq!(res.dimensions.pitch, 5568);
        assert_eq!(res.as_bytes().len(), 40824576);
        assert_eq!(res.pixels[0], 140);
        assert_eq!(*res.pixel_at(1, 100), 544);
        assert_eq!(*res.pixel_at(200, 1), 611);
        assert_eq!(res.pixels[res.pixels.len()-1], 2076);
    }
}
