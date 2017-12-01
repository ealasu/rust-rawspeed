// build.rs

extern crate cc;

fn main() {
    cc::Build::new()
        .file("foo.c")
        .compile("foo");
}
