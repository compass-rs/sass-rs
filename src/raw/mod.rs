/// Alias the sass_sys type to a prettier name.
/// Should not use this outside this library.
use sass_sys;

pub type SassValueRaw = sass_sys::Union_Sass_Value;
