#![crate_type = "lib"]

extern crate "sass-sys" as sass_sys;

pub fn one() -> *const sass_sys::Union_Sass_Value {
  unsafe {
    sass_sys::sass_make_null()    
  }
}
