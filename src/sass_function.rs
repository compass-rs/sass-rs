/// Allow user to define custom functions to be called from libsass.
/// https://github.com/sass/libsass/wiki/Custom-Functions-internal



use sass_value::SassValue;

/// Trait to be implemented by providers of custom sass functions.
pub trait SassFunction:Send {
    fn custom(&self, input: &SassValue)->SassValue;
}
