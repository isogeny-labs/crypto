//! No arithmetic is copied/reimplemented. Compile the current owner source.
//! The unused bounded-setup API references the unrelated walk subsystem; omit
//! that one API in this facade. Used constructors/arithmetic stay byte-identical.
use std::{env, fs, path::PathBuf};
fn main() {
    let owner = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("../../../../src/cryptanalysis/fp_backend.rs");
    println!("cargo:rerun-if-changed={}", owner.display());
    let source = fs::read_to_string(&owner).unwrap();
    let needle = "    pub fn try_new(\n";
    assert_eq!(source.matches(needle).count(), 1, "bounded API signature drift");
    let source = source.replacen(needle, "    #[cfg(any())]\n    pub fn try_new(\n", 1);
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let module = out.join("owner_fp_backend.rs");
    fs::write(&module, source).unwrap();
    fs::write(out.join("wiring.rs"), format!("#[path = {:?}] pub mod fp_backend;\n", module)).unwrap();
}
