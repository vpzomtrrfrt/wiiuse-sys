#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

mod inquiry;
pub use inquiry::*;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
