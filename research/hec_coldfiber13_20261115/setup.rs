use std::{path::PathBuf, sync::OnceLock};
pub fn dir() -> PathBuf {
    std::env::var_os("HEC_SETUP_DIR")
        .map(PathBuf::from)
        .expect("HEC_SETUP_DIR is mandatory; no supplied setup fallback")
}
fn array<const N: usize>(name: &str) -> [u64; N] {
    std::fs::read_to_string(dir().join(name))
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}
pub fn hc() -> [u64; 8] {
    static V: OnceLock<[u64; 8]> = OnceLock::new();
    *V.get_or_init(|| array("HC.txt"))
}
pub fn qf() -> [u64; 15] {
    static V: OnceLock<[u64; 15]> = OnceLock::new();
    *V.get_or_init(|| array("QF.txt"))
}
pub fn verify_model() {
    assert_eq!(
        qf(),
        [1, 58, 103, 47, 66, 88, 51, 80, 84, 123, 34, 83, 109, 1, 111],
        "fresh quotient outside registered section model"
    );
}
