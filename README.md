# sass-rs

[![Travis build status: (https://travis-ci.org/compass-rs/sass-rs)]

TODO: update appveyor url
[![AppVeyor build status](https://ci.appveyor.com/api/projects/status/mup239rroe6wsndt?svg=true)](https://ci.appveyor.com/project/winding-lines/sass-srs)


[Api documentation on docs.rs](https://docs.rs/sass-rs)


This crate is a wrapper around [libsass](https://github.com/sass/libsass), currently tracking
[v3.4.5](https://github.com/sass/libsass/releases/tag/3.4.5).

## How to use

`sass-rs` exposes 2 functions that are self-explanatory:

- `compile_file(path: &str, options: Options)`
- `compile_string(content: &str, options: Options)`

Most of the time, you should be able to use the `Options::default()` but you can change the
output style that way for example:

```rs
use sass_rs::{Options, OutputStyle};

let mut options = Options::default();
options.output_style = OutputStyle::Compressed;
```

You can see an example in the `examples` directory, which can be ran with the following command: 
`cargo run --example compile_sass examples/simple.scss`

## Not supported yet
[Importers](https://github.com/sass/libsass/blob/master/docs/api-importer.md) and 
[functions](https://github.com/sass/libsass/blob/master/docs/api-function.md) are not supported yet.
