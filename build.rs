extern crate cc;
extern crate cmake;

use std::env;

fn main() {
    let mut pugixml_path = env::current_dir().unwrap();
    pugixml_path.push("pugixml");
    let rawspeed_dst = cmake::Config::new("rawspeed")
        .define("BUILD_TESTING", "OFF")
        .define("BUILD_FUZZERS", "OFF")
        .define("WITH_OPENMP", "OFF")
        .define("USE_BUNDLED_PUGIXML", "ON")
        .define("PUGIXML_PATH", pugixml_path)
        .build();
    println!("cargo:rustc-link-search=native={}", rawspeed_dst.display());
    println!("cargo:rustc-link-lib=static=rawspeed");

    cc::Build::new()
        .cpp(true)
        .include("rawspeed/src/librawspeed")
        .file("rawspeed_interop.cpp")
        .compile("rawspeed_interop");
}
