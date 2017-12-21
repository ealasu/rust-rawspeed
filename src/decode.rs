use std::slice;
use ndarray::Array2;
use rawspeed_sys as ffi;
use ::camera_metadata::{CameraMetadata, DEFAULT_CAMERA_METADATA};

#[derive(Debug, Fail)]
#[fail(display = "failed to decode raw image: {}", _0)]
pub struct DecodeError(String);

pub fn decode(data: &[u8]) -> Result<Array2<u16>, DecodeError> {
    decode_with_metadata(data, &DEFAULT_CAMERA_METADATA)
}

pub fn decode_with_metadata(data: &[u8], camera_meta: &CameraMetadata) -> Result<Array2<u16>, DecodeError> {
    let obj_ptr = unsafe {
        ffi_call_fallible!(
            ffi::rawspeed_rawimage_decode,
            DecodeError,
            data.as_ptr(),
            data.len(),
            camera_meta.as_ptr())
    };
    let width = unsafe { ffi::rawspeed_rawimage_width(obj_ptr) } as usize;
    let height = unsafe { ffi::rawspeed_rawimage_height(obj_ptr) } as usize;
    let pitch = unsafe { ffi::rawspeed_rawimage_pitch(obj_ptr) } as usize / 2;
    let data_ptr = unsafe { ffi::rawspeed_rawimage_data(obj_ptr) } as *mut u16;
    let data_len = pitch * height;
    let data = unsafe { slice::from_raw_parts(data_ptr, data_len) };
    let mut pixels = Vec::with_capacity(width * height);
    for y in 0..height {
        let offset = y * pitch;
        pixels.extend(&data[offset..offset + width]);
    }
    unsafe {
        ffi::rawspeed_rawimage_free(obj_ptr);
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
        assert_eq!(res.shape()[0], 3666);
        // width
        assert_eq!(res.shape()[1], 5494);
        assert_eq!(res[[0, 0]], 140);
        assert_eq!(res[[100, 1]], 544);
        assert_eq!(res[[1, 200]], 611);
        assert_eq!(res[[3665, 5493]], 43);
    }
}
