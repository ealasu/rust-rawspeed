extern crate libc;
#[macro_use] extern crate failure;
#[macro_use] extern crate lazy_static;
extern crate tempdir;
extern crate image;
extern crate rawspeed_sys;

#[macro_use] mod macros;
pub mod camera_metadata;
pub mod decode;

pub use camera_metadata::*;
pub use decode::*;
