extern crate sass_rs;

use sass_rs::{compile_string, Options, OutputStyle};
use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut opts = Options::default();

    // SCSS vs. SASS.
    if std::env::args().any(|i| i == "--sass") {
        opts.indented_syntax = true;
    }

    // Output style.
    if std::env::args().any(|i| i == "--expanded") {
        opts.output_style = OutputStyle::Expanded;
    }
    if std::env::args().any(|i| i == "--compact") {
        opts.output_style = OutputStyle::Compact;
    }
    if std::env::args().any(|i| i == "--compressed") {
        opts.output_style = OutputStyle::Compressed;
    }

    let sass = compile_string(buffer.as_str(), opts).unwrap();
    println!("{}", sass);
}
