//! Native source-bundle hash gate for the minimal AWS CLI task image.
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;
const MANIFEST_SHA: &str = "b3337bca2282e2cd039c46960f39617d6b75bebef10d751f2b0e3a14ee60fde8";
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|x| format!("{x:02x}")).collect()
}
fn main() {
    let arg = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "/work/repo".to_owned());
    let root = Path::new(&arg);
    let manifest = fs::read(root.join("meta/local-sha256sums.txt")).unwrap();
    assert_eq!(
        hex(&Sha256::digest(&manifest)),
        MANIFEST_SHA,
        "frozen manifest mismatch"
    );
    let text = std::str::from_utf8(&manifest).unwrap();
    let mut files = 0;
    for line in text.lines() {
        let (expected, path) = line.split_once("  ").expect("sha256sum manifest entry");
        let path = path.strip_prefix("./").unwrap_or(path);
        let actual = hex(&Sha256::digest(fs::read(root.join(path)).unwrap()));
        assert_eq!(actual, expected, "member mismatch {path}");
        println!("{expected}  {path}");
        files += 1;
    }
    assert_eq!(files, 11, "manifest file count");
}
