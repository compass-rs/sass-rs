use libc::c_char;
use std::str;
use std::ffi::CStr;


pub fn build_string(c_buf:*const c_char) -> String {
      let buf: &[u8] = unsafe {CStr::from_ptr(c_buf).to_bytes() };
      let str_slice: &str = str::from_utf8(buf).unwrap();
      let mut s = String::new();
      s.push_str(str_slice);
      s
}
