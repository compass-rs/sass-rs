extern crate "sass-rs" as sass_rs;

fn main() {
  println!("libsass: {}", sass_rs::libsass_version());
  println!("sass2scss: {}", sass_rs::sass2scss_version());
}
