use libc::c_char;
use std::ffi::CStr;


pub fn to_string(c_buf: *const c_char) -> String {
    unsafe { CStr::from_ptr(c_buf) }.to_string_lossy().into_owned()
}
