use std::{env, path::PathBuf, process::Command};
fn main() {
    println!("cargo:rerun-if-changed=phase_dump.c");
    println!("cargo:rerun-if-env-changed=VALGRIND_INCLUDE");
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let include = env::var("VALGRIND_INCLUDE").unwrap_or_else(|_| "/usr/include".into());
    assert!(Command::new("cc")
        .args(["-O2", "-I", &include, "-c", "phase_dump.c", "-o"])
        .arg(out.join("phase_dump.o"))
        .status()
        .unwrap()
        .success());
    assert!(Command::new("ar")
        .arg("crs")
        .arg(out.join("libtrace_phase.a"))
        .arg(out.join("phase_dump.o"))
        .status()
        .unwrap()
        .success());
    println!("cargo:rustc-link-search=native={}", out.display());
    println!("cargo:rustc-link-lib=static=trace_phase");
}
