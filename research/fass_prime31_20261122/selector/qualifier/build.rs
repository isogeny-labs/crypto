use serde_json::json;
use sha2::{Digest, Sha256};
use std::{env, fs, path::PathBuf};

fn main() {
    let here = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let root = here.join("../../../..").canonicalize().unwrap();
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let mut bindings = String::new();
    let mut sources = Vec::new();
    let modules = [
        ("pkm_smooth_tower", "src/cryptanalysis/pkm_smooth_tower.rs"),
        (
            "semaev_prime_poly",
            "src/cryptanalysis/semaev_prime_poly.rs",
        ),
        (
            "semaev_projection",
            "src/cryptanalysis/semaev_projection.rs",
        ),
        ("semaev_selector", "src/cryptanalysis/semaev_selector.rs"),
        (
            "identity_certificate",
            "src/cryptanalysis/identity_certificate.rs",
        ),
        ("source_sha256", "src/hash/sha256.rs"),
    ];
    for (name, relative) in modules {
        let path = root.join(relative);
        println!("cargo:rerun-if-changed={}", path.display());
        let bytes = fs::read(&path).unwrap();
        let end = if name == "pkm_smooth_tower" {
            let source = std::str::from_utf8(&bytes).unwrap();
            let anchor =
                "// ------------------------------- Method 2 with class number > 1 (Hilbert)";
            assert_eq!(
                source.matches(anchor).count(),
                1,
                "ambiguous pkm prefix anchor"
            );
            source.find(anchor).unwrap()
        } else {
            bytes.len()
        };
        let selected = &bytes[..end];
        let dest = out.join(format!("{name}.rs"));
        fs::write(&dest, selected).unwrap();
        assert_eq!(fs::read(&dest).unwrap(), selected, "source copy changed");
        bindings.push_str(&format!("#[path = {:?}]\npub mod {name};\n", dest));
        sources.push(
            json!({"path":relative,"sha256":format!("{:x}",Sha256::digest(&bytes)),
            "bytes":bytes.len(),"selected_start":0,"selected_end_exclusive":end,
            "selected_sha256":format!("{:x}",Sha256::digest(selected)),
            "selection":if end==bytes.len(){"whole_file"}else{"unchanged_complete_item_prefix"},
            "rewritten":false}),
        );
    }
    for relative in [
        "tests/semaev_selector.rs",
        "src/cryptanalysis/mod.rs",
        "research/fass_prime31_20261122/selector/PLAN.md",
        "research/fass_prime31_20261122/selector/qualifier/Cargo.toml",
        "research/fass_prime31_20261122/selector/qualifier/Cargo.lock",
        "research/fass_prime31_20261122/selector/qualifier/build.rs",
        "research/fass_prime31_20261122/selector/qualifier/src/lib.rs",
        "research/fass_prime31_20261122/selector/qualifier/src/record.rs",
    ] {
        let path = root.join(relative);
        println!("cargo:rerun-if-changed={}", path.display());
        let bytes = fs::read(path).unwrap();
        sources.push(json!({"path":relative,"sha256":format!("{:x}",Sha256::digest(&bytes)),"bytes":bytes.len(),"selection":"inventory"}));
    }
    let manifest = json!({"schema":"fass-selector.source-binding/v1","scope":"development-only", "whole_library":false,"sources":sources});
    fs::write(out.join("bindings.rs"), bindings).unwrap();
    fs::write(
        out.join("source-build.json"),
        serde_json::to_vec_pretty(&manifest).unwrap(),
    )
    .unwrap();
    println!("cargo:rustc-env=FASS_SOURCE_ROOT={}", root.display());
}
