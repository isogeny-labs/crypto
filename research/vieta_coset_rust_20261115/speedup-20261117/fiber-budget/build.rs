//! Separate immutable instrument: only experiment operation budget increases.
mod scale_facade {
    include!("../fiber-scale/build.rs");
    pub fn generate() {
        main();
    }
}
fn main() {
    println!("cargo:rerun-if-changed=../fiber-scale/build.rs");
    scale_facade::generate();
    let path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("worker.rs");
    let s = std::fs::read_to_string(&path).unwrap();
    let anchor = "max_muls: 4_000_000_000";
    assert_eq!(s.matches(anchor).count(), 1);
    std::fs::write(path, s.replace(anchor, "max_muls: 8_000_000_000")).unwrap();
}
