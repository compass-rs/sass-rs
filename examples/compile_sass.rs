#![feature(env)]
#![feature(os)]
#![feature(core)]
extern crate "sass-rs" as sass_rs;

fn compile(filename:&str) {
  let mut file_context = sass_rs::SassFileContext::new(filename);
  let out = file_context.compile();
  match out {
    Ok(css) => println!("------- css  ------\n{}\n--------", css),
    Err(err) => println!("{}", err)
  };
}

pub fn main() {
  let mut args = std::env::args();
  let _ = args.next();
  let file = args.next().expect("Please pass in a file name").into_string().unwrap();
  println!("Compiling sass file: `{}`.", file);
  compile(file.as_slice());
}
