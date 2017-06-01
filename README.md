Wrapper library for low level sass-sys
https://github.com/compass-rs/sass-sys

[![Travis build status: (https://travis-ci.org/compass-rs/sass-rs)]

Documentation: http://compass-rs.github.io/sass-rs/

This is work in progress. To test that it works run the examples

```
cargo run --example versions

Running `target/examples/versions`
libsass: 3.1.0-beta.2-2-g420d
sass2scss: 1.0.3
```


The example below expands sass variables and calls custom functions defined in Rust.

```
cargo run --example compile_sass examples/simple.scss

Running `target/examples/compile_sass examples/simple.scss`
Compiling sass file: `examples/simple.scss`.
------- css  ------
body {
  font: 100% Helvetica, sans-serif;
  color: #333;
  content: foo-ed; }
--------
```

# C function interface


There are two phases of the integration with libsass:

1. register functions with libsass, these functions are implemented by providers

2. dispatch to those functions


## Providers

The functions can be implemented by various Rust modules. These modules provide
the function signature and the function implementation. Users should be able
to add these modules without changing code parts of the library. The modules
may need state and as such the interface with the dispatcher involves trait
objects.

In order to simplify the ownership structure the main function of your executable
should provide a wrapping struct to contain all your data structure and the
dispatcher. During the build process of this struct construct all your
providers then pass a list of SassFunctions to the dispatcher.

## Dispatching

The dispatcher will create a single consumer FIFO queue to communicate with
the C code. 
