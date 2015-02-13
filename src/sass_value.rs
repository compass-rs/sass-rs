use sass_sys;
use std::ffi;


#[derive(Debug)]
pub struct SassValue {
    value: * mut sass_sys::Union_Sass_Value
}


impl SassValue {
  pub fn from_string(input:&str) -> SassValue {
    let c_str = ffi::CString::from_slice(input.as_bytes());
    let value = unsafe { sass_sys::sass_make_string(c_str.as_ptr()) };
    SassValue {
      value: value
    }
  }
}

impl Drop for SassValue {
  fn drop(&mut self) {
    unsafe {
      sass_sys::sass_delete_value(self.value)
    }
  }
}
