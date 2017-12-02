use libc::{c_int, size_t, c_void, c_char};

extern "C" {
    pub fn rawspeed_metadata_init(filename: *const c_char) -> *const c_void;
    pub fn rawspeed_metadata_delete(ptr: *const c_void);
    pub fn rawspeed_rawimage_decode(data: *const u8, size: size_t, metadata: *const c_void) -> *const c_void;
    pub fn rawspeed_rawimage_delete(ptr: *const c_void);
    pub fn rawspeed_rawimage_data(ptr: *const c_void) -> *const u8;
    pub fn rawspeed_rawimage_width(ptr: *const c_void) -> c_int;
    pub fn rawspeed_rawimage_height(ptr: *const c_void) -> c_int;
    pub fn rawspeed_rawimage_pitch(ptr: *const c_void) -> c_int;
}
