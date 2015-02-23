//! Allow user to define custom functions to be called from libsass.
//! https://github.com/sass/libsass/wiki/Custom-Functions-internal


use sass_value::SassValueBuf;
use sass_sys;
use libc;
use std::ffi;
use std::mem;

/// Type of the function to be defined by the user.
pub type SassFunction = fn(& SassValueBuf)->SassValueBuf;

/// Dispatcher function called from libsass (C interface).
/// The cookie argument is setup in SassFunctionCallback::from_sig_fn.
/// Note that the SassFunctionCallback is not used directly in the dispatch.
extern "C" fn dispatch(arg1: *const sass_sys::Union_Sass_Value,
  cookie: *mut ::libc::c_void) -> *mut sass_sys::Union_Sass_Value {
    let _fn :SassFunction = unsafe {mem::transmute(cookie)};
    let result = _fn(&SassValueBuf::from_raw(arg1)).raw();
    match result {
      Some(raw) => raw,
      None => SassValueBuf::sass_error("bad call").raw().unwrap()
    }

}

/// Associate the signature with the C callback.$one,$two
pub struct SassFunctionCallback {
  pub signature: String,
  pub c_callback:sass_sys::Sass_C_Function_Callback
}


impl SassFunctionCallback {
  pub fn from_sig_fn(signature:String,_fn:SassFunction) -> SassFunctionCallback {
    let c_sig = ffi::CString::new(signature.as_slice()).unwrap();
    let _fn_c = unsafe {sass_sys::sass_make_function(c_sig.as_ptr(), Some(dispatch), mem::transmute(_fn))};
    SassFunctionCallback {
      signature: signature,
      c_callback: _fn_c
    }
  }

}
