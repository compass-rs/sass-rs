//! Allow access to the various flavours of sass contexts:
//! https://github.com/sass/libsass/wiki/API-Sass-Context

use std::{ffi,ptr};
use sass_sys;
use sass_function::*;
use util;
use core::num::ToPrimitive;

pub struct SassOptions {
    pub raw: *mut sass_sys::Struct_Sass_Options
}

impl SassOptions {
    /// Set the sass functions in the context so that they are available to libsass.
    pub fn set_sass_functions_from_callbacs(&mut self, sf:Vec<SassFunctionCallback>) {
        // create list of all custom functions
        let len:u64 = sf.len().to_u64().unwrap();
        unsafe {
            let fn_list = sass_sys::sass_make_function_list(len);
            for (i,item) in sf.iter().enumerate() {
                sass_sys::sass_function_set_list_entry(fn_list, i.to_u64().unwrap(), item.c_callback);
            }
            sass_sys::sass_option_set_c_functions(self.raw, fn_list);
        }
    }

    /// Set the sass functions in the context, expects an array
    /// of tuples, each tuple contains the signature and function pointer.
    pub fn set_sass_functions(&mut self, sf:Vec<(&'static str,SassFunction)>) {
        // create list of all custom functions
        let len:u64 = sf.len().to_u64().unwrap();
        unsafe {
            let fn_list = sass_sys::sass_make_function_list(len);
            for (i,item) in sf.iter().enumerate() {
                let c_cb = SassFunctionCallback::make_sass_c_callback(item.0,item.1);
                sass_sys::sass_function_set_list_entry(fn_list, i.to_u64().unwrap(), c_cb);
            }
            sass_sys::sass_option_set_c_functions(self.raw, fn_list);
        }
    }
}

pub struct SassContext {
    pub raw: *mut sass_sys::Struct_Sass_Context,
    pub sass_options: SassOptions
}

pub struct SassFileContext {
    context: *mut sass_sys::Struct_Sass_File_Context,
    pub sass_context: SassContext
}


impl SassFileContext {
    pub fn new(input_file_path:&str) -> SassFileContext {
        let c_str = ffi::CString::new(input_file_path).unwrap();
        let file_context = unsafe { sass_sys::sass_make_file_context(c_str.as_ptr()) };
        let file_sass_context = unsafe {sass_sys::sass_file_context_get_context(file_context)};
        let options = unsafe {sass_sys::sass_context_get_options(file_sass_context)};
        SassFileContext {
            context: file_context,
            sass_context: SassContext {
                raw: file_sass_context,
                sass_options: SassOptions {
                    raw: options
                }
            }
        }
    }

    pub fn compile(&mut self) -> Result<String,String> {
        unsafe { sass_sys::sass_compile_file_context(self.context)};
        let ctx_out = self.sass_context.raw;
        let error_status = unsafe {sass_sys::sass_context_get_error_status(ctx_out)};
        let error_message = unsafe {sass_sys::sass_context_get_error_message(ctx_out)};
        let output_string = unsafe {sass_sys::sass_context_get_output_string(ctx_out)};
        if error_status != 0  {
            if error_message != ptr::null() {
                Result::Err(util::build_string(error_message))
            } else {
                Result::Err(String::from_str("Unknown error"))
            }
        } else {
            Result::Ok(util::build_string(output_string))
        }
    }

}

impl Drop for SassFileContext {
    fn drop(&mut self) {
        unsafe {
            sass_sys::sass_delete_file_context(self.context)
        }
    }
}
