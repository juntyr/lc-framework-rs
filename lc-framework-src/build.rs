#![allow(missing_docs)]

use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

#[expect(clippy::expect_used)]
fn main() {
    let src = Path::new("LC-framework");

    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed={}", src.display());

    let out_dir = env::var("OUT_DIR")
        .map(PathBuf::from)
        .expect("missing OUT_DIR");

    fs_extra::copy_items(
        &[
            src.join("components"),
            src.join("include"),
            src.join("preprocessors"),
            src.join("verifiers"),
        ],
        &out_dir,
        &fs_extra::dir::CopyOptions::new().overwrite(true),
    )
    .expect("failed to copy src");

    let mut cmd = Command::new("python3");
    cmd.arg("generate_Host_LC-Framework.py")
        .arg("--output_dir")
        .arg(&out_dir)
        .arg("--verbose")
        .current_dir(src);
    eprintln!("executing {cmd:?}");
    cmd.status()
        .expect("generate_Host_LC-Framework must not fail");

    println!("cargo::metadata=root={}", out_dir.display());
}
