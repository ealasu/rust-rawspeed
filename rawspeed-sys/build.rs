extern crate cc;
extern crate cmake;
extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let rawspeed_dst = cmake::Config::new("rawspeed")
        .define("BUILD_TESTING", "OFF")
        .define("BUILD_FUZZERS", "OFF")
        .define("WITH_OPENMP", "OFF")
        .define("WITH_PTHREADS", "OFF")
        .define("WITH_JPEG", "ON")
        .define("WITH_ZLIB", "ON")
        .define("USE_BUNDLED_PUGIXML", "ON")
        .define("CMAKE_CXX_FLAGS", "-w")
        .define("PUGIXML_PATH", env::current_dir().unwrap().join("pugixml"))
        .define("CMAKE_BUILD_TYPE", "")
        .build();
    println!("cargo:rustc-link-search=native={}", rawspeed_dst.join("build/src").display());
    println!("cargo:rustc-link-lib=static=rawspeed");
    println!("cargo:rustc-link-search=native={}", rawspeed_dst.join("build/pugixml/pugixml-build").display());
    println!("cargo:rustc-link-lib=static=pugixml");
    println!("cargo:rustc-link-lib=jpeg");
    println!("cargo:rustc-link-lib=z");

    println!("Compiling src");
    cc::Build::new()
        .cpp(true)
        .flag("-std=c++1z")
        .object(rawspeed_dst.join("build/src/librawspeed.a"))
        .include("src")
        .include("rawspeed/src/librawspeed")
        .include("rawspeed/src/external")
        .include(rawspeed_dst.join("build/src"))
        .file("src/bindings.cpp")
        .compile("rawspeed_interop");

    println!("Generating bindings");
    bindgen::Builder::default()
        .header("src/bindings.h")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
