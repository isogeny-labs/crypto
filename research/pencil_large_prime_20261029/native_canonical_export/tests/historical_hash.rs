//! Distinguish preserved raw-file integrity from canonical freeze hashing.
use pencil_native_canonical_export::{canonical, digest, load, protocol_hash};
use std::{fs, path::Path};
#[test]
fn preserved_historical_integrity_and_independent_canonical_bytes_match() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures");
    let path = dir.join("pre-first-gate-authorization-protocol.json");
    let receipt = load(&dir.join("pre-first-gate-authorization-validation.json")).unwrap();
    assert_eq!(
        digest(&path).unwrap(),
        receipt["protocol_sha256_integrity_only"]
    );
    assert_eq!(
        digest(&path).unwrap(),
        "0f5e18e3ac9b6510d81b7dad67e2498a4c0d71dd41a66d6d8fd3e55a50b3f594"
    );
    let p = load(&path).unwrap();
    let expected = fs::read(dir.join("pre-first-gate-authorization-canonical.json")).unwrap();
    let mut body = p.clone();
    body.as_object_mut().unwrap().remove("freeze");
    assert_eq!(canonical(&body).unwrap(), expected);
    assert_eq!(
        protocol_hash(&p).unwrap(),
        "e00941ff8f17bb41091fcc11fcef11ff5137325da3127e7c85b1a461b56bd036"
    );
}
