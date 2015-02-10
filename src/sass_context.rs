use std::{ffi,ptr};
use sass_sys;
use util;


pub struct SassContext {
  pub raw: *mut sass_sys::Struct_Sass_Context
}

pub struct SassFileContext {
  context: *mut sass_sys::Struct_Sass_File_Context,
  pub sass_context: SassContext
}


impl SassFileContext {
  pub fn new(input_file_path:&str) -> SassFileContext {
    let c_str = ffi::CString::from_slice(input_file_path.as_bytes());
    let file_context = unsafe { sass_sys::sass_make_file_context(c_str.as_ptr()) };
    let file_sass_context = unsafe {sass_sys::sass_file_context_get_context(file_context)};
    SassFileContext {
      context: file_context,
      sass_context: SassContext { raw: file_sass_context }
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
