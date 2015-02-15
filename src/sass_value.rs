//! Wrap the Sass Values, see the libsass documentation:
//! https://github.com/sass/libsass/wiki/API-Sass-Value

use sass_sys;
use std::ffi;


/// Alias the sass_sys type to a prettier name.
pub type SassValueRaw = sass_sys::Union_Sass_Value;



/// Safe container of Union_Sass_Value
/// This may not be of much value since the allocation
/// crosses the lines to libsass.
pub struct SassValue {
    value: * mut SassValueRaw
}


impl SassValue {
  pub fn from_str(input:&str) -> SassValue {
    SassValue {
      value: SassValue::raw_from_str(input)
    }
  }

  pub fn raw_from_str(input:&str) -> *mut SassValueRaw {
    let c_str = ffi::CString::from_slice(input.as_bytes());
    unsafe { sass_sys::sass_make_string(c_str.as_ptr()) }
  }
}

impl Drop for SassValue {
  fn drop(&mut self) {
    unsafe {
      sass_sys::sass_delete_value(self.value)
    }
  }
}
