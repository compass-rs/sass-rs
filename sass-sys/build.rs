extern crate bindgen;
#[cfg(target_env="msvc")]
extern crate gcc;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;
use std::process::Command;


// Automatically write bindings to libsass
#[allow(dead_code)]
fn write_bindings() {
    let bindings = bindgen::Builder::default()
        .header("libsass/include/sass.h")
        // https://github.com/servo/rust-bindgen/issues/550
        .hide_type("max_align_t")
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn get_libsass_folder() -> PathBuf {
    env::current_dir().unwrap().join("libsass")
}

// linux/unix
#[cfg(not(target_env="msvc"))]
fn compile() {
    let target = env::var("TARGET").expect("TARGET not found");
    let src = get_libsass_folder();
    let is_bsd = target.contains("dragonfly") || target.contains("freebsd") ||
        target.contains("netbsd") || target.contains("openbsd");
    let r = Command::new(if is_bsd { "gmake" } else { "make" }).current_dir(&src).
        output().expect("error running make");

    if !r.status.success() {
        let err = String::from_utf8_lossy(&r.stderr);
        let out = String::from_utf8_lossy(&r.stdout);
        panic!("Build error:\nSTDERR:{}\nSTDOUT:{}", err, out);
    }

    println!("cargo:rustc-link-search=native={}", src.join("lib").display());
    println!("cargo:rustc-link-lib=static=sass");
    println!("cargo:rustc-link-lib=dylib={}", if target.contains("darwin") { "c++" } else { "stdc++" });
}

// windows
#[cfg(target_env="msvc")]
fn compile() {
    let src = get_libsass_folder();
    let target = env::var("TARGET").expect("TARGET not found in environment");
    let msvc_platform = if target.contains("x86_64") { "Win64" } else { "Win32" };
    let r = gcc::windows_registry::find(target.as_str(),"msbuild.exe")
        .expect("could not find msbuild")
        .args(&["win\\libsass.sln", "/p:LIBSASS_STATIC_LIB=1", "/p:Configuration=Release",
            "/p:WholeProgramOptimization=false",
            format!("/p:Platform={}", msvc_platform).as_str()])
        .current_dir(&src)
        .output()
        .expect("error running msbuild");

    if !r.status.success() {
        let err = String::from_utf8_lossy(&r.stderr);
        let out = String::from_utf8_lossy(&r.stdout);
        panic!("Build error:\nSTDERR:{}\nSTDOUT:{}", err, out);
    }

    println!("cargo:rustc-link-search=native={}", src.join("win").join("bin").display());
    println!("cargo:rustc-link-lib=static=libsass");
}


fn main() {
    // Uncomment the line below to generate bindings. Doesn't work on CI as it
    // requires additional tooling
    // write_bindings();

    // Is it already built?
    if let Ok(_) = pkg_config::find_library("sass") {
        println!("Sass lib already exists");
        println!("or libsass? {:?}", pkg_config::find_library("libsass"));
        return;
    }

    let _ = Command::new("git").args(&["submodule", "update", "--init"]).status();

    compile();
}
