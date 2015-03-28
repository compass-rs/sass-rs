/// Allow user to define custom functions to be called from libsass.
/// https://github.com/sass/libsass/wiki/Custom-Functions-internal


use sass_value::SassValue;
use sass_sys;
use std::ffi;
use std::mem;

/// Type of the function to be defined by the user.
pub type SassFunction = fn(& SassValue)->SassValue;


/// Dispatcher function called from libsass (C interface).
/// The cookie argument is setup in SassFunctionCallback::from_sig_fn.
/// Note that the SassFunctionCallback is not used directly in the dispatch.
extern "C" fn dispatch(arg1: *const sass_sys::Union_Sass_Value,
                       cookie: *mut ::libc::c_void) -> *mut sass_sys::Union_Sass_Value {
    let _fn :SassFunction = unsafe {mem::transmute(cookie)};
    let result = _fn(&SassValue::from_raw(arg1)).as_raw();
    match result {
        Some(raw) => raw,
        None => SassValue::sass_error("bad call").as_raw().unwrap()
    }

}

/// Associate the signature with the C callback.
#[derive(Debug)]
pub struct SassFunctionCallback {
    pub signature: String,
    pub c_callback:sass_sys::Sass_C_Function_Callback
}


impl SassFunctionCallback {
    /// Create the C callback structure used by libsass.
    pub fn make_sass_c_callback(signature:&str,_fn:SassFunction) -> sass_sys::Sass_C_Function_Callback {
        let c_sig = ffi::CString::new(signature).unwrap();
        unsafe {sass_sys::sass_make_function(c_sig.as_ptr(), Some(dispatch), mem::transmute(_fn))}
    }

    /// Build a SassFunctionCallback.
    pub fn from_sig_fn(signature:String,_fn:SassFunction) -> SassFunctionCallback {
        let cb = SassFunctionCallback::make_sass_c_callback(&signature,_fn);
        SassFunctionCallback {
            signature: signature,
            c_callback: cb
        }
    }

}
