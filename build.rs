//! Embeds the compiler's identity in the binary.
//!
//! The ecbench host capsule records which compiler built the code it
//! measures.  Asking `rustc -Vv` at run time answers for whatever
//! toolchain `PATH` (and a toolchain file in the current directory) names
//! today, not for the one that built this binary, so the class of a host
//! changed between a shell and `sudo`.  The value is fixed here, at build
//! time, and `host.rs` reads it with `option_env!`.

use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=RUSTC");
    let rustc = std::env::var("RUSTC").unwrap_or_else(|_| "rustc".into());
    if let Ok(out) = Command::new(rustc).arg("-Vv").output() {
        if out.status.success() {
            // `cargo:rustc-env` values are one line: newlines are written
            // as the two characters `\n` and decoded by `host.rs`.
            let text = String::from_utf8_lossy(&out.stdout)
                .trim()
                .replace('\n', "\\n");
            println!("cargo:rustc-env=ECBENCH_BUILD_RUSTC_VV={text}");
        }
    }
}
