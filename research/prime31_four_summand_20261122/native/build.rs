//! Expose a verbatim public-API test module from the actual production F4 file.
//! No solver source or test assertion is rewritten by this selector.

use std::{env, fs, path::PathBuf};

fn main() {
    let manifest = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let source_path = manifest.join("../../../src/cryptanalysis/f4_fp.rs");
    println!("cargo:rerun-if-changed={}", source_path.display());
    let source = fs::read_to_string(&source_path).expect("read actual production F4 source");
    let begin = "// BEGIN PUBLIC F4 RESOURCE LIMIT CONTROLS\n";
    let end = "// END PUBLIC F4 RESOURCE LIMIT CONTROLS\n";
    assert_eq!(
        source.matches(begin).count(),
        1,
        "unique public-control start"
    );
    assert_eq!(source.matches(end).count(), 1, "unique public-control end");
    let first = source.find(begin).unwrap();
    let last = source.find(end).unwrap() + end.len();
    assert!(first < last);
    let bytes = &source.as_bytes()[first..last];
    let destination =
        PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("f4_resource_controls.rs");
    fs::write(&destination, bytes).expect("write exact public F4 control source");
    assert_eq!(fs::read(destination).unwrap(), bytes);
}
