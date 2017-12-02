extern crate cc;
extern crate cmake;

use std::env;

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
    //println!("cargo:rustc-link-search=native={}", rawspeed_dst.display());
    //println!("cargo:rustc-link-lib=static=rawspeed");

    cc::Build::new()
        .cpp(true)
        .flag("-std=c++14")
        .object(rawspeed_dst.join("librawspeed.a"))
        .include("rawspeed/src/librawspeed")
        .include("rawspeed/src/external")
        .include(rawspeed_dst)
        .file("interop.cpp")
        .compile("rawspeed_interop");
}
