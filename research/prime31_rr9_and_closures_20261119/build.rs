use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let repo_crypt = manifest_dir.join("../../src/cryptanalysis");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let sub_dir = out_dir.join("f4_fp");
    fs::create_dir_all(&sub_dir).unwrap();

    for rel in [
        "f4_fp.rs",
        "f4_fp/reuse.rs",
        "f4_fp/pair_certificate.rs",
    ] {
        let src = repo_crypt.join(rel);
        println!("cargo:rerun-if-changed={}", src.display());
        fs::copy(&src, out_dir.join(rel)).unwrap();
    }
    fs::write(sub_dir.join("certified.rs"), "//! Unused in standalone crate.\n").unwrap();

    let wrapper = format!(
        "#[allow(dead_code, clippy::all)]\n#[path = {:?}]\npub mod f4_fp_host {{\n    pub use crate::{{fx_hash, glv_invariant_base, groebner_f4, jv_bitops, residual_walk}};\n    pub mod f4_fp;\n}}\n",
        out_dir
    );
    fs::write(out_dir.join("f4_fp_bridge.rs"), wrapper).unwrap();
}
