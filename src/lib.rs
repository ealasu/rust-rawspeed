extern crate libc;
#[macro_use] extern crate failure;
extern crate tempdir;
extern crate rawspeed_sys;

#[macro_use] mod macros;
pub mod camera_metadata;

pub use camera_metadata::CameraMetadata;

