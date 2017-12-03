extern crate libc;
#[macro_use] extern crate failure;
extern crate tempdir;
extern crate rawspeed_sys;

#[macro_use] mod macros;
pub mod camera_metadata;
pub mod raw_image;

pub use camera_metadata::CameraMetadata;
pub use raw_image::RawImage;
