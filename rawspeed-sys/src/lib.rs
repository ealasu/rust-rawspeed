#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub mod data {
    pub static CAMERAS_XML: &str = include_str!("../rawspeed/data/cameras.xml");
}
