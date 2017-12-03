extern crate cc;
extern crate cmake;
extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let rawspeed_dst = cmake::Config::new("rawspeed")
        .define("BUILD_TESTING", "OFF")
        .define("BUILD_FUZZERS", "OFF")
        .define("WITH_OPENMP", "OFF")
        .define("USE_BUNDLED_PUGIXML", "ON")
        .define("PUGIXML_PATH", env::current_dir().unwrap().join("pugixml"))
        .define("CMAKE_BUILD_TYPE", "")
        .build()
        .join("build/src");

    cc::Build::new()
        .cpp(true)
        .flag("-std=c++14")
        .object(rawspeed_dst.join("librawspeed.a"))
        .include("src")
        .include("rawspeed/src/librawspeed")
        .include("rawspeed/src/external")
        .include(rawspeed_dst)
        .file("src/bindings.cpp")
        .compile("rawspeed_interop");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindgen::Builder::default()
        .header("src/bindings.h")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
