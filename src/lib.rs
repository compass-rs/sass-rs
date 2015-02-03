#![crate_type = "lib"]
#![feature(std_misc)]
#![feature(libc)]

extern crate "sass-sys" as sass_sys;
extern crate libc;

use libc::c_char;
use std::{ffi,str};

fn build_string(c_buf:*const c_char) -> String {
  let buf: &[u8] = unsafe { ffi::c_str_to_bytes(&c_buf) };
  let str_slice: &str = str::from_utf8(buf).unwrap();
  let mut s = String::new();
  s.push_str(str_slice);
  s
}

pub fn libsass_version() -> String {
  let c_buf = unsafe { sass_sys::libsass_version() };
  build_string(c_buf)
}

pub fn sass2scss_version() -> String {
  let c_buf = unsafe { sass_sys::sass2scss_version() };
  build_string(c_buf)
}
