use std::ffi;

use sass_sys::{self, Sass_Output_Style};

use bindings::ptr::Unique;

use options::OutputStyle;

/// The internal options we will pass to libsass
#[derive(Debug)]
pub struct SassOptions {
    pub raw: Unique<sass_sys::Sass_Options>
}

impl SassOptions {
    pub fn set_output_style(&mut self, style: OutputStyle) {
        let style = match style {
            OutputStyle::Nested => Sass_Output_Style::SASS_STYLE_NESTED,
            OutputStyle::Expanded => Sass_Output_Style::SASS_STYLE_EXPANDED,
            OutputStyle::Compact => Sass_Output_Style::SASS_STYLE_COMPACT,
            OutputStyle::Compressed => Sass_Output_Style::SASS_STYLE_COMPRESSED,
        };

        unsafe {
            sass_sys::sass_option_set_output_style(self.raw.get_mut(), style);
        }
    }

    pub fn set_precision(&mut self, precision: usize) {
        unsafe {
            sass_sys::sass_option_set_precision(self.raw.get_mut(), precision as i32);
        }
    }

    pub fn set_is_indented_syntax(&mut self) {
        unsafe {
            sass_sys::sass_option_set_is_indented_syntax_src(self.raw.get_mut(), true);
        }
    }

    pub fn set_include_path(&mut self, paths: Vec<String>) {
        let include_path = if cfg!(windows) {
            paths.join(";")
        } else {
            paths.join(",")
        };
        let c_str = ffi::CString::new(include_path).unwrap();
        let ptr = c_str.into_raw();
        unsafe {
            sass_sys::sass_option_set_include_path(self.raw.get_mut(), ptr);
            let _ = ffi::CString::from_raw(ptr);
        }
    }
}
