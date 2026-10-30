//! An audited incremental refresh for a source edit invisible to read_crosswalk.
//! Never changes computed traits. A subsequent real generator check is required.
use crypto_lib::cryptanalysis::curve_traits::{sources, TraitFile};

fn digest(bytes: &[u8]) -> String {
    crypto_lib::hash::sha256::sha256(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    assert_eq!(args.len(), 4, "usage: refresh_crosswalk_digest TRAITS OLD_SOURCE RECEIPT");
    let trait_text = std::fs::read_to_string(&args[1]).unwrap();
    let mut traits: TraitFile = serde_json::from_str(&trait_text).unwrap();
    assert_eq!(traits.to_json(), trait_text, "serializer round-trip changed data");
    let old_text = std::fs::read_to_string(&args[2]).unwrap();
    let manifest = std::fs::read_to_string("docs/curves/trait-sources.json").unwrap();
    let specs = sources::read_manifest(&manifest).unwrap();
    let spec = specs.iter().find(|s| s.path == "docs/curves/ic/curves.yaml").unwrap();
    let new_text = std::fs::read_to_string(&spec.path).unwrap();
    let before = sources::read_crosswalk(spec, &old_text).unwrap();
    let after = sources::read_crosswalk(spec, &new_text).unwrap();
    // RegistryCurve derives Debug over every field consumed by compute();
    // comparing these within one binary preserves the complete parsed input,
    // including model, order, generator, source and certified levels.
    let before_debug = format!("{before:?}");
    let after_debug = format!("{after:?}");
    assert_eq!(before_debug, after_debug, "parsed source changed: full rebuild required");
    let before_hash = digest(old_text.as_bytes());
    let after_hash = digest(new_text.as_bytes());
    let mut refreshed = 0;
    for input in &mut traits.inputs {
        if input.path == spec.path {
            assert_eq!(input.sha256, before_hash, "old source hash mismatch");
            input.sha256 = after_hash.clone();
            refreshed += 1;
        } else {
            let actual = std::fs::read(&input.path).unwrap();
            assert_eq!(input.sha256, digest(&actual), "other input changed");
        }
    }
    assert_eq!(refreshed, 1);
    let updated = traits.to_json();
    let mut before_value: serde_json::Value = serde_json::from_str(&trait_text).unwrap();
    let mut after_value: serde_json::Value = serde_json::from_str(&updated).unwrap();
    before_value.as_object_mut().unwrap().remove("inputs");
    after_value.as_object_mut().unwrap().remove("inputs");
    assert_eq!(before_value, after_value, "computed trait data changed");
    let receipt = serde_json::json!({
        "method": "native observational-equivalence check plus digest-only incremental refresh",
        "old_source_sha256": before_hash, "new_source_sha256": after_hash,
        "parsed_curve_count_before": before.len(), "parsed_curve_count_after": after.len(),
        "complete_parsed_input_sha256": digest(before_debug.as_bytes()),
        "complete_parsed_source_equal": true, "all_other_input_hashes_unchanged": true,
        "computed_trait_data_unchanged": true, "old_traits_sha256": digest(trait_text.as_bytes()),
        "new_traits_sha256": digest(updated.as_bytes()),
        "required_followup": "real curve_traits check on final inputs; this receipt alone is not a full rebuild receipt"
    });
    std::fs::write(&args[1], updated).unwrap();
    std::fs::write(&args[3], serde_json::to_string_pretty(&receipt).unwrap() + "\n").unwrap();
    println!("PASS: parsed crosswalk is unchanged; refreshed one input digest only. Full native check still required.");
}
