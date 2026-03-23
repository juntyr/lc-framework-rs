use std::{env, path::PathBuf};

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=wrapper.h");

    let _out_dir = env::var("OUT_DIR")
        .map(PathBuf::from)
        .expect("missing OUT_DIR");

    let lc_src = env::var("DEP_LC_SRC_ROOT")
        .map(PathBuf::from)
        .expect("missing lc-framework-src dependency");

    let mut build = cc::Build::new();
    build
        .cpp(true)
        .std("c++17")
        .include(&lc_src)
        .file(lc_src.join("lc.cpp"))
        .flag_if_supported("-mno-fma")
        .flag_if_supported("-ffp-contract=off")
        .define("USE_CPU", None)
        .warnings(false);
    build.compile("lc");
}
