// extern crate bindgen;
#[cfg(target_env = "msvc")]
extern crate cc;
extern crate pkg_config;
extern crate num_cpus;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

// Automatically write bindings to libsass
//#[allow(dead_code)]
//fn write_bindings() {
//    let bindings = bindgen::Builder::default()
//        .header("libsass/include/sass.h")
//        .clang_arg("-Ilibsass/include")
//        // To avoid a test failing
//        .blacklist_type("max_align_t")
//        // we do static linking so it should be fine
//        // https://github.com/rust-lang/rust/issues/36927
//        .rustified_enum(".*")
//        .generate()
//        .expect("Unable to generate bindings");
//
//    // Write the bindings to the $OUT_DIR/bindings.rs file.
//    let out_path = PathBuf::from("src");
//    bindings
//        .write_to_file(out_path.join("bindings.rs"))
//        .expect("Couldn't write bindings!");
//}

macro_rules! t {
    ($e:expr) => (match $e {
        Ok(n) => n,
        Err(e) => panic!("\n{} failed with {}\n", stringify!($e), e),
    })
}

fn cp_r(dir: &Path, dest: &Path) {
    for entry in t!(fs::read_dir(dir)) {
        let entry = t!(entry);
        let path = entry.path();
        let dst = dest.join(path.file_name().unwrap());
        if t!(fs::metadata(&path)).is_file() {
            t!(fs::copy(path, dst));
        } else {
            t!(fs::create_dir_all(&dst));
            cp_r(&path, &dst);
        }
    }
}

fn get_libsass_folder() -> PathBuf {
    env::current_dir().unwrap().join("libsass")
}

// linux/unix
#[cfg(not(target_env = "msvc"))]
fn compile() {
    let target = env::var("TARGET").expect("TARGET not found");
    let src = get_libsass_folder();
    let dest = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let build = dest.join("build");
    t!(fs::create_dir_all(&build));
    cp_r(&src, &build);
    let is_bsd = target.contains("dragonfly")
        || target.contains("freebsd")
        || target.contains("netbsd")
        || target.contains("openbsd");
    let libprobe = | lib: &str | -> bool {
      Command::new("cc")
        .arg("-xc++")
        .arg("-o/dev/null")
        .arg(format!("-l{}",lib))
        .arg("-shared")
        .stderr(Stdio::null())
        .status()
        .expect("")
        .success()
    };

    let jobs = env::var("MAKE_LIBSASS_JOBS").unwrap_or(num_cpus::get().to_string());
    let r = Command::new(if is_bsd { "gmake" } else { "make" })
        .current_dir(&build)
        .args(&["--jobs", &jobs])
        .output()
        .expect("error running make");

    if !r.status.success() {
        let err = String::from_utf8_lossy(&r.stderr);
        let out = String::from_utf8_lossy(&r.stdout);
        panic!("Build error:\nSTDERR:{}\nSTDOUT:{}", err, out);
    }

    println!(
        "cargo:rustc-link-search=native={}",
        build.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=sass");

    if libprobe("c++_shared") {
        println!("cargo:rustc-link-lib=dylib=c++_shared");
    }
    else if libprobe("c++") {
        println!("cargo:rustc-link-lib=dylib=c++");
    }
    else if libprobe("stdc++") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }
    else {
        panic!("no c++ library found");
    }

}

// windows
#[cfg(target_env = "msvc")]
fn compile() {
    let src = get_libsass_folder();
    let target = env::var("TARGET").expect("TARGET not found in environment");
    let msvc_platform = if target.contains("x86_64") {
        "Win64"
    } else {
        "Win32"
    };
    let dest = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let build = dest.join("build");
    t!(fs::create_dir_all(&build));
    cp_r(&src, &build);

    // Find an instance of devenv.exe from Visual Studio IDE in order to upgrade
    // libsass.sln to the current available IDE. Do nothing if no devenv.exe found
    let d = cc::windows_registry::find(target.as_str(), "devenv.exe");
    if let Some(mut d) = d {
        let d = d
            .args(&["/upgrade", "win\\libsass.sln"])
            .current_dir(&build)
            .output()
            .expect("error running devenv");
        if !d.status.success() {
            let err = String::from_utf8_lossy(&d.stderr);
            let out = String::from_utf8_lossy(&d.stdout);
            println!("Upgrade error:\nSTDERR:{}\nSTDOUT:{}", err, out);
        }
    }

    let r = cc::windows_registry::find(target.as_str(), "msbuild.exe")
        .expect("could not find msbuild")
        .args(&[
            "win\\libsass.sln",
            "/p:LIBSASS_STATIC_LIB=1",
            "/p:Configuration=Release",
            "/p:WholeProgramOptimization=false",
            format!("/p:Platform={}", msvc_platform).as_str(),
        ])
        .current_dir(&build)
        .output()
        .expect("error running msbuild");

    if !r.status.success() {
        let err = String::from_utf8_lossy(&r.stderr);
        let out = String::from_utf8_lossy(&r.stdout);
        panic!("Build error:\nSTDERR:{}\nSTDOUT:{}", err, out);
    }

    println!(
        "cargo:rustc-link-search=native={}",
        build.join("win").join("bin").display()
    );
    println!("cargo:rustc-link-lib=static=libsass");
}

fn main() {
    // Uncomment the line below to generate bindings. Doesn't work on CI as it
    // requires additional tooling
    // write_bindings();

    // Is it already built?
    if let Ok(_) = pkg_config::find_library("libsass") {
        return;
    }

    compile();
}
