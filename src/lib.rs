extern crate sass_sys;
extern crate libc;

mod bindings;
mod options;
// mod dispatcher;

pub use options::Options;
pub use bindings::Context;


/// Takes a file path and compiles it
pub fn compile_file(path: &str, options: Options) -> Result<String, String> {
    let mut context = Context::new_file(path);
    context.set_options(options);
    context.compile()
}

/// Takes a string and compiles it
pub fn compile_string(content: &str, options: Options) -> Result<String, String> {
    let mut context = Context::new_data(content);
    context.set_options(options);
    context.compile()
}

#[cfg(test)]
mod tests {
    use super::{Options, compile_string};

    #[test]
    fn can_compile_some_valid_scss_input() {
        let input = "body { .hey { color: red; } }";
        compile_string(input, Options::default()).is_ok();
    }

    #[test]
    fn errors_with_invalid_scss_input() {
        let input = "body { .hey { color: ; } }";
        let res = compile_string(input, Options::default());
        assert!(res.is_err());
        let err = res.unwrap_err();
        println!("{}", err);
        assert!(false);
        assert_eq!(err, "1: style declaration must contain a value");
    }
}
