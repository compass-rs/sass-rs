/// Example file on how to compile a scss file.

extern crate sass_rs;
use sass_rs::sass_context::SassFileContext;
use sass_rs::sass_function::*;
use sass_rs::sass_value::*;
use sass_rs::dispatcher::Dispatcher;
use std::thread;

/// Function to be exported to libsass.
struct Foo;

impl SassFunction for Foo {
    fn custom(&self, value:& SassValue) -> SassValue {
        let out = format!("Called with {}", value);
        SassValue::sass_string(&out)
    }

}

/// Setup the environment and compile a file.
fn compile(filename:&str) {
    let mut file_context = SassFileContext::new(filename);
    let foo = Foo;
    let fns:Vec<(&'static str,Box<SassFunction>)> = vec![("foo($x)", Box::new(foo))];
    let options = file_context.sass_context.sass_options.clone();
    let dispatcher = Dispatcher::build(fns,options);
    thread::spawn(move|| {
        while dispatcher.dispatch().is_ok() {}
    });
    let out = file_context.compile();
    match out {
        Ok(css) => println!("------- css  ------\n{}\n--------", css),
        Err(err) => println!("{}", err)
    };
}

pub fn main() {
    let mut args = std::env::args();
    let _ = args.next();
    let file = args.next().expect("Please pass in a file name");
    println!("Compiling sass file: `{}`.", file);
    compile(&file);
}
