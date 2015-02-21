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
      value: sass_string_from_str(input),
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

}

pub fn sass_string_from_str(input:&str) -> *mut SassValueRaw {
  //! Create a raw Sass Value to return to libsass.
  let c_str = ffi::CString::new(input).unwrap();
  unsafe { sass_sys::sass_make_string(c_str.as_ptr()) }
}

pub fn sass_error_from_str(input:&str) -> *mut SassValueRaw {
  //! Create a raw Sass Value to return to libsass.
  let c_str = ffi::CString::new(input).unwrap();
  unsafe { sass_sys::sass_make_error(c_str.as_ptr()) }
}

  pub fn sass_value_to_string(input: * const SassValueRaw) -> Option<String> {
  if unsafe{sass_sys::sass_value_is_string(input)} != 0 {
    Some(util::build_string(unsafe{sass_sys::sass_string_get_value(input)}))
  } else {
    None
  }
}

impl fmt::Display for SassValue {

  fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    //! Format arbitrary Sass Values

    fn fmt_list(value: * const SassValueRaw)  -> String {
      let len = unsafe { sass_sys::sass_list_get_length(value) };
      let mut out = String::new();
      out.push_str("[");
      for i in 0..len {
        let entry = unsafe {sass_sys::sass_list_get_value(value,i)};
        if i>0 {
          out.push_str(", ");
        }
        out.push_str(fmt_raw(entry).as_slice());
      }
      out.push_str("]");
      out
    }

    fn fmt_raw(value: * const SassValueRaw) -> String {
      let sass_tag = unsafe {sass_sys::sass_value_get_tag(value)};
      match sass_tag {
        sass_sys::SASS_LIST =>  fmt_list(value),
        sass_sys::SASS_STRING => util::build_string(
          unsafe{sass_sys::sass_string_get_value(value)}),
        sass_sys::SASS_BOOLEAN => {
          let v = unsafe{ sass_sys::sass_boolean_get_value(value) };
          if v != 0 {
            String::from_str("true")
          } else {
            String::from_str("false")
          }
        },
        sass_sys::SASS_NUMBER => {
          let v = unsafe { sass_sys::sass_number_get_value(value)};
          format!("{}",v)
        },
        sass_sys::SASS_COLOR => {String::from_str("color(?,?,?,?)")},
        sass_sys::SASS_MAP => {String::from_str("{?,?}")},
        sass_sys::SASS_NULL => String::from_str("(null)"),
        sass_sys::SASS_ERROR => util::build_string(
          unsafe {sass_sys::sass_error_get_message(value)}
          ),
        sass_sys::SASS_WARNING => util::build_string(
          unsafe {sass_sys::sass_error_get_message(value)}
          ),
        _ => format!("bad sass tag {}", sass_tag)
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
