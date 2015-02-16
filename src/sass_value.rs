//! Wrap the Sass Values, see the libsass documentation:
//! https://github.com/sass/libsass/wiki/API-Sass-Value

use sass_sys;
use std::ffi;
use std::fmt;
use util;


/// Alias the sass_sys type to a prettier name.
pub type SassValueRaw = sass_sys::Union_Sass_Value;



/// Safe container of Union_Sass_Value
/// The allocation crosses the lines to libsass.
pub struct SassValue {
    value: * const SassValueRaw,
    owned: bool
}


impl SassValue {
  pub fn from_str(input:&str) -> SassValue {
    //! Create a Sass Value from a string.
    //! We will own and delete this value, although it is unclear
    //! if this is a real use case.
    SassValue {
      value: SassValue::raw_from_str(input),
      owned: true
    }
  }

  pub fn from_raw(raw: *const SassValueRaw) -> SassValue {
    //! Wrap a const Sass Value pointer coming from libsass.
    //! We won't need to delete this.
    SassValue {
      value: raw,
      owned: false
    }
  }

  pub fn raw_from_str(input:&str) -> *mut SassValueRaw {
    //! Create a raw Sass Value to return to libsass.
    let c_str = ffi::CString::from_slice(input.as_bytes());
    unsafe { sass_sys::sass_make_string(c_str.as_ptr()) }
  }
}

impl fmt::Display for SassValue {

  fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    fn fmt_list(value: * const SassValueRaw)  -> String {
      let len = unsafe { sass_sys::sass_list_get_length(value) };
      let mut out = String::new();
      out.push_str("[");
      for i in 0..len {
        let entry = unsafe {sass_sys::sass_list_get_value(value,i)};
        out.push_str(fmt_raw(entry).as_slice());
        out.push_str(" ");
      }
      out.push_str("]");
      out
    }

    fn fmt_raw(value: * const SassValueRaw) -> String {
      let sass_tag = unsafe {sass_sys::sass_value_get_tag(value)};
      match sass_tag {
        sass_sys::SASS_LIST =>  fmt_list(value),
        sass_sys::SASS_STRING => util::build_string(unsafe{sass_sys::sass_string_get_value(value)}),
        _ => format!("sass tag {}",sass_tag)

      }
    }

    fmt.pad_integral(true, "", fmt_raw(self.value).as_slice())




  }
}

impl Drop for SassValue {
  fn drop(&mut self) {
    if self.owned {
      unsafe {
        sass_sys::sass_delete_value(self.value as *mut SassValueRaw)
      }
    }
  }
}
