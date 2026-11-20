use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("manifest directory"));
    let repository_modules = manifest.join("../../../src/cryptanalysis");
    let output = PathBuf::from(env::var("OUT_DIR").expect("build output directory"));
    let f4_directory = output.join("f4_fp");
    fs::create_dir_all(&f4_directory).expect("create generated F4 module directory");

    for relative in ["f4_fp.rs", "f4_fp/reuse.rs", "f4_fp/pair_certificate.rs"] {
        let source = repository_modules.join(relative);
        println!("cargo:rerun-if-changed={}", source.display());
        fs::copy(&source, output.join(relative)).expect("copy exact repository F4 source");
    }
    fs::write(
        f4_directory.join("certified.rs"),
        "//! Certified quotient APIs are not used by the RR9 runner.\n",
    )
    .expect("write unused certified-module stub");

    let bridge = format!(
        "#[allow(dead_code, clippy::all)]\n#[path = {:?}]\npub mod f4_fp_host {{\n    pub use crate::{{fx_hash, glv_invariant_base, groebner_f4, jv_bitops, residual_walk}};\n    pub mod f4_fp;\n}}\n",
        output
    );
    fs::write(output.join("f4_fp_bridge.rs"), bridge).expect("write F4 source bridge");
}
