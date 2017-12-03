#[macro_export]
macro_rules! ffi_call_fallible {
    ($func:path, $err:path, $($args:expr),+) => {
        {
            use std::ptr;
            use std::ffi::CStr;
            use libc::{self, free};

            let mut error_msg = ptr::null_mut();
            let res = $func($($args),+, &mut error_msg);
            if res.is_null() {
                let what = if error_msg.is_null() {
                    "unknown error".to_string()
                } else {
                    let s = CStr::from_ptr(error_msg).to_string_lossy().to_string();
                    free(error_msg as *mut libc::c_void);
                    s
                };
                return Err($err(what));
            }
            res
        }
    }
}
