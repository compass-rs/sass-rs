#![crate_type = "lib"]
#![feature(std_misc)]
#![feature(libc)]
#![feature(collections)]


extern crate "sass-sys" as sass_sys;
extern crate libc;
mod util;

pub mod sass_context;
pub mod sass_importer;



pub fn libsass_version() -> String {
  let c_buf = unsafe { sass_sys::libsass_version() };
  util::build_string(c_buf)
}

pub fn sass2scss_version() -> String {
  let c_buf = unsafe { sass_sys::sass2scss_version() };
  util::build_string(c_buf)
}
