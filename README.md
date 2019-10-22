# sass-rs

[![Build Status](https://travis-ci.org/compass-rs/sass-rs.svg?branch=master)](https://travis-ci.org/compass-rs/sass-rs)
[![Windows build status](https://ci.appveyor.com/api/projects/status/j8enle2iod2nxtor/branch/master?svg=true)](https://ci.appveyor.com/project/Keats/sass-rs-rmnm5/branch/master)

[Api documentation on docs.rs](https://docs.rs/sass-rs)


This crate is a wrapper around [libsass](https://github.com/sass/libsass), currently tracking
[v3.6.2](https://github.com/sass/libsass/releases/tag/3.6.2).

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


## Building (Windows)

Windows compilation using VS 2019 requires that all the environment variables for MSBuild to be availble.

An indicator that the environment is not properly setup is the following error message:

```
error MSB4019: The imported project "C:\Microsoft.Cpp.Default.props" was not found. Confirm that the path in the <Import> declaration is correct, and that the file exists on disk.
```

If you find this error, you have a couple of of options to select:

- [Easiest] Open the `Developer Command Prompt for VS 2019` application to compile the project. This terminal will setup all the needed environment variables to let it compile.
- Setup the environment variables (eg: `PATH`, `LIB`) as [documented on Microsoft's website](https://docs.microsoft.com/en-us/cpp/build/setting-the-path-and-environment-variables-for-command-line-builds?view=vs-2019)
- Install the complete setup of Visual Studio 2015 - not only the Visual C++ Build tools
