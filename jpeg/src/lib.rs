/*!
    This crate is just a demo of how to add a C -sys library to a wasm32-unknown-unknown target.

    The C library is libjpeg by Independent JPEG Group: https://ijg.org/
*/
pub mod bindings {
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]
    #![allow(non_upper_case_globals)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
