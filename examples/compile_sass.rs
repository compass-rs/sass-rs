/// Example file on how to compile a scss file.

extern crate sass_rs;
use sass_rs::{Options, compile_file};


pub fn main() {
    let mut args = std::env::args();
    let _ = args.next();
    let file = args.next().expect("Please pass in a file name");
    println!("Compiling sass file: `{}`.", file);
    println!("> Default:\n{}", compile_file(&file, Options::default()).unwrap());
}
