//! Same experimental evaluator; raise only its local coset resource cap.
use std::{env, fs, path::PathBuf};
mod original {
    include!("../fiber-core/build.rs");
    pub fn generate() {
        main();
    }
}
fn main() {
    original::generate();
    println!("cargo:rerun-if-changed=../fiber-core/build.rs");
    let path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("vieta_product_seam.rs");
    let s = fs::read_to_string(&path).unwrap();
    let anchor = "if !m.is_power_of_two() || m > 16384";
    assert_eq!(s.matches(anchor).count(), 1);
    let mut s = s
        .replace(anchor, "if !m.is_power_of_two() || m > 32768")
        .replace(
            "m must be a power of two <=16384",
            "m must be a power of two <=32768",
        );
    s.push_str(
        r#"
#[cfg(test)] mod scale_cap_tests {
use super::*;
#[test] fn local_coset_cap_accepts_doubled_scale_and_rejects_next() {
let a=Arithmetic::new(WordField(4179340454199820289), Limits::default()).unwrap();
assert!(a.coset(32768,1).is_ok());assert!(a.coset(65536,1).is_err());
assert!(a.coset(32767,1).is_err());assert!(a.coset(32768,0).is_err());
}}
"#,
    );
    fs::write(path, s).unwrap();
}
