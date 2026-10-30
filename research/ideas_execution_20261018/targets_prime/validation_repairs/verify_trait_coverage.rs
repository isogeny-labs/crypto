//! Validate a real v2 generator result before replacing the stale artifact.
use crypto_lib::cryptanalysis::curve_traits::{read_registry, sources, Source, TraitFile, SCHEMA};
use std::collections::{BTreeMap, BTreeSet};

fn digest(s: &[u8]) -> String {
    crypto_lib::hash::sha256::sha256(s)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    assert_eq!(args.len(), 3, "usage: verify_trait_coverage OLD NEW");
    let old_text = std::fs::read_to_string(&args[1]).unwrap();
    let new_text = std::fs::read_to_string(&args[2]).unwrap();
    let old: serde_json::Value = serde_json::from_str(&old_text).unwrap();
    let new: TraitFile = serde_json::from_str(&new_text).unwrap();
    assert_eq!(new.schema, SCHEMA);
    assert_eq!(old["schema"], "curve-traits/1");
    let registry_text = std::fs::read_to_string("docs/curves/registry.json").unwrap();
    assert_eq!(new.registry_sha256, digest(registry_text.as_bytes()));
    for i in &new.inputs {
        let actual = std::fs::read(&i.path).unwrap();
        assert_eq!(digest(&actual), i.sha256, "input changed: {}", i.path);
    }
    let registry = read_registry(&registry_text).unwrap();
    let registry_count = registry.len();
    let mut expected: BTreeMap<String, Source> = registry
        .into_iter()
        .map(|c| (c.slug, c.source))
        .collect();
    let manifest = std::fs::read_to_string("docs/curves/trait-sources.json").unwrap();
    for spec in sources::read_manifest(&manifest).unwrap() {
        let text = std::fs::read_to_string(&spec.path).unwrap();
        let curves = if spec.kind == "isogeny-walk" {
            sources::read_walk(&spec, &text).unwrap()
        } else {
            sources::read_crosswalk(&spec, &text).unwrap()
        };
        for c in curves {
            expected.entry(c.slug).or_insert(c.source);
        }
    }
    let actual: BTreeSet<_> = new.curves.iter().map(|c| c.slug.clone()).collect();
    assert_eq!(actual.len(), new.curves.len(), "duplicate slug");
    assert_eq!(actual, expected.keys().cloned().collect());
    let old_rows = old["curves"].as_array().unwrap();
    let old_slugs: BTreeSet<_> = old_rows
        .iter()
        .map(|c| c["slug"].as_str().unwrap().to_string())
        .collect();
    assert!(old_slugs.is_subset(&actual), "historical row dropped");
    let mut kinds = BTreeMap::<String, usize>::new();
    for c in &new.curves {
        assert_eq!(&c.source, &expected[&c.slug], "wrong provenance");
        *kinds.entry(c.source.kind.clone()).or_default() += 1;
    }
    let keys_changed: Vec<_> = old_rows
        .iter()
        .filter_map(|old_row| {
            let slug = old_row["slug"].as_str().unwrap();
            let current = new.curves.iter().find(|c| c.slug == slug).unwrap();
            (old_row["keys"] != serde_json::to_value(&current.keys).unwrap())
                .then_some(slug.to_string())
        })
        .collect();
    println!("{}", serde_json::to_string_pretty(&serde_json::json!({
        "validation": "pass", "old_sha256": digest(old_text.as_bytes()),
        "new_sha256": digest(new_text.as_bytes()), "old_schema": old["schema"],
        "new_schema": new.schema, "old_rows": old_rows.len(),
        "registry_rows": registry_count, "new_rows": new.curves.len(),
        "expected_coverage": expected.len(), "rows_by_source": kinds,
        "all_old_rows_preserved": true, "all_recorded_input_digests_verified": true,
        "all_record_sources_match_actual_manifest": true,
        "source_input_count": new.inputs.len(), "old_key_maps_changed": keys_changed.len(),
        "old_key_maps_changed_slugs": keys_changed
    })).unwrap());
}
