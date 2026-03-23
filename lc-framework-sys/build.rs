use std::{env, path::PathBuf};

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=lib.cpp");
    println!("cargo::rerun-if-changed=wrapper.hpp");

    let out_dir = env::var("OUT_DIR")
        .map(PathBuf::from)
        .expect("missing OUT_DIR");

    let lc_src = env::var("DEP_LC_SRC_ROOT")
        .map(PathBuf::from)
        .expect("missing lc-framework-src dependency");

    let cargo_callbacks = bindgen::CargoCallbacks::new();
    let bindings = bindgen::Builder::default()
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++17")
        .clang_arg(format!("-I{}", lc_src.display()))
        .clang_arg("-DUSE_CPU")
        .header("wrapper.hpp")
        .parse_callbacks(Box::new(cargo_callbacks))
        .allowlist_function("lc_available_preprocessors")
        .allowlist_function("lc_available_components")
        .allowlist_function("lc_compress")
        .allowlist_function("lc_decompress")
        .allowlist_function("lc_free_bytes")
        .allowlist_var("CS")
        // MSRV 1.85
        .rust_target(match bindgen::RustTarget::stable(85, 0) {
            Ok(target) => target,
            #[expect(clippy::panic)]
            Err(err) => panic!("{err}"),
        })
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let mut build = cc::Build::new();
    build
        .cpp(true)
        .std("c++17")
        .include(&lc_src)
        .file("lib.cpp")
        .flag_if_supported("-mno-fma")
        .flag_if_supported("-ffp-contract=off")
        .define("USE_CPU", None)
        .warnings(false);
    build.compile("lc");
}
