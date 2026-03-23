#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::{
    ffi::{c_char, c_int, c_longlong, c_short},
    mem,
};
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

const _: () = const {
    if CS % 8 != 0 {
        panic!("CS must be a multiple of 8")
    }
};

#[cfg(not(target_endian = "little"))]
compile_error!("LC framework only supports little-endian systems");

const _: () = const {
    if mem::size_of::<c_longlong>() != 8 {
        panic!("long long must be 8 bytes")
    }
};
const _: () = const {
    if mem::size_of::<c_int>() != 4 {
        panic!("int must be 4 bytes")
    }
};
const _: () = const {
    if mem::size_of::<c_short>() != 2 {
        panic!("short must be 2 bytes")
    }
};
const _: () = const {
    if mem::size_of::<c_char>() != 1 {
        panic!("char must be 1 byte")
    }
};
