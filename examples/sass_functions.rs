extern crate "sass-rs" as sass_rs;
use sass_rs::sass_value::SassValue;

fn one(a:&SassValue) {
  println!("{:?}", a );
}

pub fn main() {
  let foo:fn(&SassValue) = one;
  let value = SassValue::from_string("hello");
  foo(&value);
}
