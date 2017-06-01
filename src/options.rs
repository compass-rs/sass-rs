#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputStyle {
    Nested,
    Expanded,
    Compact,
    Compressed,
}

/// The user facing Options struct, where they can select the libsass
/// options
#[derive(Debug, PartialEq, Clone)]
pub struct Options {
    /// The output format of the final CSS style.
    pub output_style: OutputStyle,
    /// How many digits after the decimal will be allowed.
    pub precision: usize,
    /// `true` values enable Sass Indented Syntax for parsing the data string or file.
    pub indented_syntax: bool,
    /// An array of paths that LibSass can look in to attempt to resolve your @import declarations.
    pub include_paths: Vec<String>,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            output_style: OutputStyle::Nested,
            precision: 5,
            indented_syntax: false,
            include_paths: vec![],
        }
    }
}
