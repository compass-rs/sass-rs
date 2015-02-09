#![crate_type = "lib"]
#![feature(std_misc)]
#![feature(libc)]
#![feature(collections)]

extern crate "sass-sys" as sass_sys;
extern crate libc;

use libc::c_char;
use std::{ffi,str,ptr};

pub struct SassImporter {
  pub callback: sass_sys::Sass_C_Import_Callback
}

impl SassImporter {
  pub fn new(arg1: sass_sys::Sass_C_Import_Fn,
    cookie: *mut ::libc::c_void) -> SassImporter {
      SassImporter { callback:
        unsafe{sass_sys::sass_make_importer(arg1,cookie)}
      }
  }
}

impl Drop for SassImporter {
  fn drop(&mut self) {
    unsafe { sass_sys::sass_delete_importer(self.callback) }
  }
}

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
        Result::Err(build_string(error_message))
      } else {
        Result::Err(String::from_str("Unknown error"))
      }
    } else {
      Result::Ok(build_string(output_string))
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

fn build_string(c_buf:*const c_char) -> String {
  let buf: &[u8] = unsafe { ffi::c_str_to_bytes(&c_buf) };
  let str_slice: &str = str::from_utf8(buf).unwrap();
  let mut s = String::new();
  s.push_str(str_slice);
  s
}

pub fn libsass_version() -> String {
  let c_buf = unsafe { sass_sys::libsass_version() };
  build_string(c_buf)
}

pub fn sass2scss_version() -> String {
  let c_buf = unsafe { sass_sys::sass2scss_version() };
  build_string(c_buf)
}
