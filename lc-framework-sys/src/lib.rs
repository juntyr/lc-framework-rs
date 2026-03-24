//! [![CI Status]][workflow] [![MSRV]][repo] [![Latest Version]][crates.io]
//! [![Rust Doc Crate]][docs.rs] [![Rust Doc Main]][docs]
//!
//! [CI Status]: https://img.shields.io/github/actions/workflow/status/juntyr/lc-framework-rs/ci.yml?branch=main
//! [workflow]: https://github.com/juntyr/lc-framework-rs/actions/workflows/ci.yml?query=branch%3Amain
//!
//! [MSRV]: https://img.shields.io/badge/MSRV-1.85.0-blue
//! [repo]: https://github.com/juntyr/lc-framework-rs
//!
//! [Latest Version]: https://img.shields.io/crates/v/lc-framework-sys
//! [crates.io]: https://crates.io/crates/lc-framework-sys
//!
//! [Rust Doc Crate]: https://img.shields.io/docsrs/lc-framework-sys
//! [docs.rs]: https://docs.rs/lc-framework-sys/
//!
//! [Rust Doc Main]: https://img.shields.io/badge/docs-main-blue
//! [docs]: https://juntyr.github.io/lc-framework-rs/lc_framework_sys
//!
//! # lc-framework-sys
//!
//! Low-level Rust bindigs to the [LC] compression framework.
//!
//! [LC]: https://github.com/burtscher/LC-framework

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(missing_docs)]
#![allow(unsafe_code)]

use std::{
    ffi::{c_char, c_int, c_longlong, c_short},
    mem::size_of,
};
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use lc_framework_src as _;

#[expect(clippy::manual_assert)]
pub const MAX_STAGES: usize = const {
    if max_stages < 0 {
        panic!("max_stages must not be negative");
    }

    let _: c_int = max_stages;

    if size_of::<c_int>() > size_of::<usize>() {
        panic!("max_stages might not fit into usize");
    }

    max_stages as usize
};

#[expect(clippy::manual_assert)]
const _: () = const {
    if CS % 8 != 0 {
        panic!("CS must be a multiple of 8")
    }
};

#[cfg(not(target_endian = "little"))]
compile_error!("LC framework only supports little-endian systems");

#[expect(clippy::manual_assert)]
const _: () = const {
    if size_of::<c_longlong>() != 8 {
        panic!("long long must be 8 bytes")
    }
};
#[expect(clippy::manual_assert)]
const _: () = const {
    if size_of::<c_int>() != 4 {
        panic!("int must be 4 bytes")
    }
};
#[expect(clippy::manual_assert)]
const _: () = const {
    if size_of::<c_short>() != 2 {
        panic!("short must be 2 bytes")
    }
};
#[expect(clippy::manual_assert)]
const _: () = const {
    if size_of::<c_char>() != 1 {
        panic!("char must be 1 byte")
    }
};
