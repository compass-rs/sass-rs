extern crate sass_sys;
extern crate libc;

mod bindings;
mod options;
// mod dispatcher;

pub use options::{Options, OutputStyle};
pub use bindings::Context;


/// Takes a file path and compiles it with the options given
pub fn compile_file(path: &str, options: Options) -> Result<String, String> {
    let mut context = Context::new_file(path);
    context.set_options(options);
    context.compile()
}

/// Takes a string and compiles it with the options given
pub fn compile_string(content: &str, options: Options) -> Result<String, String> {
    let mut context = Context::new_data(content);
    context.set_options(options);
    context.compile()
}

#[cfg(test)]
mod tests {
    use super::{Options, OutputStyle, compile_string};

    #[test]
    fn can_compile_some_valid_scss_input() {
        let input = "body { .hey { color: red; } }";
        assert_eq!(compile_string(input, Options::default()),
            Ok("body .hey {\n  color: red; }\n".to_string()));
    }

    #[test]
    fn errors_with_invalid_scss_input() {
        let input = "body { .hey { color: ; } }";
        let res = compile_string(input, Options::default());
        assert!(res.is_err());
    }

    #[test]
    fn can_use_alternative_options() {
        let input = "body { .hey { color: red; } }";
        let mut opts = Options::default();
        opts.output_style = OutputStyle::Compressed;
        assert_eq!(compile_string(input, opts),
            Ok("body .hey{color:red}\n".to_string()));
    }
}
