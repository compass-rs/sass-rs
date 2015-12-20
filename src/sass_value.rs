//! Wrap the Sass Values, see the libsass documentation:
//! https://github.com/sass/libsass/wiki/API-Sass-Value

use sass_sys;
use std::ffi;
use std::fmt;
use util;
use raw::SassValueRaw;
use ptr::Unique;

/// Wrap a raw sass value.
#[derive(Debug)]
pub struct SassValue {
    raw: Unique<SassValueRaw>,
    is_const: bool
}

impl SassValue {
    // Wrap a read only Sass Value pointer coming from libsass.
    pub fn from_raw(raw: *const SassValueRaw) -> SassValue {
        SassValue {
            raw: unsafe {Unique::new(raw as *mut SassValueRaw)},
            is_const: true
        }
    }

    /// Create a raw SassValue containing a sass string.
    pub fn sass_string(input:&str) -> SassValue {
        let c_str = ffi::CString::new(input).unwrap();
        SassValue {
            raw: unsafe { Unique::new(sass_sys::sass_make_string(c_str.as_ptr())) },
            is_const: false
        }
    }

    /// Create a raw SassValue containing a sass string.
    pub fn sass_error(input:&str) -> SassValue {
        let c_str = ffi::CString::new(input).unwrap();
        SassValue {
            raw: unsafe { Unique::new(sass_sys::sass_make_error(c_str.as_ptr())) },
            is_const: false
        }
    }

    /// return a mutable raw, if available.
    pub fn as_raw(&mut self) -> Option<* mut SassValueRaw> {
        if self.is_const {
            None
        } else {
            Some(unsafe { self.raw.get_mut() })
        }
    }


    /// Attempt to extract a String from the raw value.
    pub fn to_string(&self) -> Option<String> {
        if unsafe{sass_sys::sass_value_is_string(self.raw.get())} != 0 {
            Some(util::build_string(unsafe{sass_sys::sass_string_get_value(self.raw.get())}))
        } else {
            None
        }
    }


    /// Attempt to extract a vector of strings from the raw value.
    pub fn to_vec_string(&self) -> Option<Vec<String>> {
        if unsafe{sass_sys::sass_value_is_list(self.raw.get())} != 0 {
            let mut out = Vec::new();
            for i in 0..unsafe{sass_sys::sass_list_get_length(self.raw.get())} {
                let one = unsafe{sass_sys::sass_list_get_value(self.raw.get(),i)};
                if unsafe{sass_sys::sass_value_is_string(one)} != 0 {
                    out.push(util::build_string(unsafe{sass_sys::sass_string_get_value(one)}));
                }
            }
            Some(out)
        } else {
            None
        }
    }

    /// Expect the SassValue to be a list and to contain a string,
    /// at the desired index.
    pub fn list_nth_to_string(&self,index:usize) -> Option<String> {
        if unsafe{sass_sys::sass_value_is_list(self.raw.get())} != 0 {
            if index >= unsafe{sass_sys::sass_list_get_length(self.raw.get()) as usize} {
                None
            } else {
                let one = unsafe{sass_sys::sass_list_get_value(self.raw.get(),index)};
                if unsafe{sass_sys::sass_value_is_string(one)} != 0 {
                    Some(util::build_string(unsafe{sass_sys::sass_string_get_value(one)}))
                } else {
                    None
                }
            }
        } else {
            None
        }
    }


}

impl fmt::Display for SassValue {

    /// Format arbitrary Sass Values
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {

        fn fmt_list(value: * const SassValueRaw)  -> String {
            let len = unsafe { sass_sys::sass_list_get_length(value) };
            let mut out = String::new();
            out.push_str("[");
            for i in 0..len {
                let entry = unsafe {sass_sys::sass_list_get_value(value,i)};
                if i>0 {
                    out.push_str(", ");
                }
                out.push_str(&fmt_raw(entry));
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
                        "true".to_string()
                    } else {
                        "false".to_string()
                    }
                },
                sass_sys::SASS_NUMBER => {
                    let v = unsafe { sass_sys::sass_number_get_value(value)};
                    format!("{}",v)
                },
                sass_sys::SASS_COLOR => {"color(?,?,?,?)".to_string()},
                sass_sys::SASS_MAP => {"{?,?}".to_string()},
                sass_sys::SASS_NULL => "(null)".to_string(),
                sass_sys::SASS_ERROR => util::build_string(
                    unsafe {sass_sys::sass_error_get_message(value)}
                ),
                sass_sys::SASS_WARNING => util::build_string(
                    unsafe {sass_sys::sass_error_get_message(value)}
                ),
                _ => format!("bad sass tag {}", sass_tag)
            }
        }

        fmt.pad_integral(true, "", &fmt_raw(unsafe{ self.raw.get()}))

    }
}


/// An owned SassValueBuf.
pub struct SassValueBuf {
    buf: * mut SassValue,

}


impl SassValueBuf {
    pub fn from_buf(input:&mut SassValue) -> SassValueBuf {
        SassValueBuf {
            buf: input
        }
    }
}


impl Drop for SassValueBuf {
    fn drop(&mut self) {
        unsafe {
            sass_sys::sass_delete_value(self.buf as *mut SassValueRaw)
        }
    }
}
