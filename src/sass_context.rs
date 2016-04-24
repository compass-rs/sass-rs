//! Allow access to the various flavours of sass contexts:
//! https://github.com/sass/libsass/wiki/API-Sass-Context

use std::{ffi,ptr};
use sass_sys;
use util;
use ptr::Unique;
use std::sync::{Arc,RwLock};


#[derive(Debug, Clone)]
pub enum OutputStyle {
    Nested,
    Expanded,
    Compact,
    Compressed
}

#[derive(Debug)]
pub struct SassOptions {
    pub raw: Unique<sass_sys::Struct_Sass_Options>
}

impl SassOptions {

    /// Set the sass functions in the context, expects an array
    /// of tuples, each tuple contains the signature and function pointer.
    pub fn set_sass_functions(&mut self, sf:Vec<sass_sys::Sass_C_Function_Callback>) {
        // create list of all custom functions
        let len = sf.len();
        unsafe {
            let fn_list = sass_sys::sass_make_function_list(len);
            for (i,sass_fn) in sf.into_iter().enumerate() {
                sass_sys::sass_function_set_list_entry(fn_list, i, sass_fn);
            }
            sass_sys::sass_option_set_c_functions(self.raw.get_mut(), fn_list);
        }
    }

    pub fn set_output_style(&mut self, style: OutputStyle) {
        let style = match style {
            OutputStyle::Nested => sass_sys::SASS_STYLE_NESTED,
            OutputStyle::Expanded => sass_sys::SASS_STYLE_EXPANDED,
            OutputStyle::Compact => sass_sys::SASS_STYLE_COMPACT,
            OutputStyle::Compressed => sass_sys::SASS_STYLE_COMPRESSED,
        };
        unsafe {
            sass_sys::sass_option_set_output_style(self.raw.get_mut(), style);
        }
    }
}

pub struct SassContext {
    // Need Unique to send between threads, libsass is thread safe
    pub raw: Unique<sass_sys::Struct_Sass_Context>,
    pub sass_options: Arc<RwLock<SassOptions>>
}

pub struct SassFileContext {
    // Need Unique to send between threads, libsass is thread safe
    context: Unique<sass_sys::Struct_Sass_File_Context>,
    pub sass_context: SassContext
}


impl SassFileContext {
    pub fn new(input_file_path:&str) -> SassFileContext {
        let c_str = ffi::CString::new(input_file_path).unwrap();
        let file_context = unsafe { sass_sys::sass_make_file_context(c_str.as_ptr()) };
        let file_sass_context = unsafe {sass_sys::sass_file_context_get_context(file_context)};
        let options = unsafe {sass_sys::sass_context_get_options(file_sass_context)};
        let sass_options = Arc::new(RwLock::new(SassOptions {
            raw: unsafe {Unique::new(options)}
        }));
        SassFileContext {
            context: unsafe {Unique::new(file_context)},
            sass_context: SassContext {
                raw: unsafe {Unique::new(file_sass_context)},
                sass_options: sass_options
            }
        }
    }

    pub fn compile(&mut self) -> Result<String,String> {
        unsafe { sass_sys::sass_compile_file_context(self.context.get_mut())};
        let ctx_out = unsafe {self.sass_context.raw.get_mut()};
        let error_status = unsafe {sass_sys::sass_context_get_error_status(ctx_out)};
        let error_message = unsafe {sass_sys::sass_context_get_error_message(ctx_out)};
        let output_string = unsafe {sass_sys::sass_context_get_output_string(ctx_out)};
        if error_status != 0  {
            if error_message != ptr::null() {
                Result::Err(util::build_string(error_message))
            } else {
                Result::Err("Unknown error".to_string())
            }
        } else {
            Result::Ok(util::build_string(output_string))
        }
    }

}

impl Drop for SassFileContext {
    fn drop(&mut self) {
        unsafe {
            sass_sys::sass_delete_file_context(self.context.get_mut())
        }
    }
}
