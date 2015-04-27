#![crate_type = "lib"]



extern crate sass_sys;
extern crate libc;
mod util;
mod raw;

// copied from libcore for Unique<>
mod ptr;

pub mod dispatcher;
pub mod sass_context;
pub mod sass_importer;
pub mod sass_value;
pub mod sass_function;



pub fn libsass_version() -> String {
    let c_buf = unsafe { sass_sys::libsass_version() };
    util::build_string(c_buf)
}

pub fn sass2scss_version() -> String {
    let c_buf = unsafe { sass_sys::sass2scss_version() };
    util::build_string(c_buf)
}
