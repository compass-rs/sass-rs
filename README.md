# sass-rs

[![Build Status](https://travis-ci.org/compass-rs/sass-rs.svg?branch=master)](https://travis-ci.org/compass-rs/sass-rs)
[![Windows build status](https://ci.appveyor.com/api/projects/status/j8enle2iod2nxtor/branch/master?svg=true)](https://ci.appveyor.com/project/Keats/sass-rs-rmnm5/branch/master)

[Api documentation on docs.rs](https://docs.rs/sass-rs)


This crate is a wrapper around [libsass](https://github.com/sass/libsass), currently tracking
[v3.5.5](https://github.com/sass/libsass/releases/tag/3.5.5).

To build this crate on Windows, you will need to have Visual Studio installed.

You can control the number of CPU used to build `libsass` by setting the `MAKE_LIBSASS_JOBS` variable to the desired value. It defaults to the number of CPUs in the machine.

## Binary
This package also provides a small binary that can be `cargo install`ed to convert Sass files and print the output CSS.
Example usage:

```bash
$ sass-rs < source/style.scss  # for SCSS
$ sass-rs --sass < source/style.sass  # for SASS
$ sass-rs --sass --expanded < source/style.sass
$ sass-rs --sass --compact < source/style.sass
$ sass-rs --sass --compressed < source/style.sass
$ sass-rs --sass --compressed < source/style.sass > build/style.css
```

This is a small added feature that isn't meant to fulfill every usecases. If you want to have something added to the binary, do a PR as I will not implement it myself.

## Not supported yet
[Importers](https://github.com/sass/libsass/blob/master/docs/api-importer.md) and
[functions](https://github.com/sass/libsass/blob/master/docs/api-function.md) are not supported yet.
