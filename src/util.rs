use libc::c_char;
use std::{ffi,str};


pub fn build_string(c_buf:*const c_char) -> String {
  let buf: &[u8] = unsafe { ffi::c_str_to_bytes(&c_buf) };
  let str_slice: &str = str::from_utf8(buf).unwrap();
  let mut s = String::new();
  s.push_str(str_slice);
  s
}
