//! Native metadata/coverage audit of regenerated merged traits; not a new solver.
use crypto_lib::cryptanalysis::curve_traits::{read_registry, sources, Source, TraitFile, SCHEMA};
use std::collections::{BTreeMap, BTreeSet};
fn digest(b: &[u8]) -> String {
    crypto_lib::hash::sha256::sha256(b)
        .iter()
        .map(|x| format!("{x:02x}"))
        .collect()
}
fn old_slugs(path: &str) -> BTreeSet<String> {
    let v: serde_json::Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
    v["curves"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| c["slug"].as_str().unwrap().to_string())
        .collect()
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    assert_eq!(args.len(), 4);
    let text = std::fs::read_to_string(&args[3]).unwrap();
    let new: TraitFile = serde_json::from_str(&text).unwrap();
    assert_eq!(new.schema, SCHEMA);
    let registry_text = std::fs::read_to_string("docs/curves/registry.json").unwrap();
    assert_eq!(new.registry_sha256, digest(registry_text.as_bytes()));
    let registry = read_registry(&registry_text).unwrap();
    let registry_count = registry.len();
    let mut expected: BTreeMap<String, Source> =
        registry.into_iter().map(|c| (c.slug, c.source)).collect();
    for input in &new.inputs {
        assert_eq!(
            input.sha256,
            digest(&std::fs::read(&input.path).unwrap()),
            "input changed:{}",
            input.path
        );
    }
    let manifest = std::fs::read_to_string("docs/curves/trait-sources.json").unwrap();
    for s in sources::read_manifest(&manifest).unwrap() {
        let t = std::fs::read_to_string(&s.path).unwrap();
        let rows = if s.kind == "isogeny-walk" {
            sources::read_walk(&s, &t).unwrap()
        } else {
            sources::read_crosswalk(&s, &t).unwrap()
        };
        for c in rows {
            expected.entry(c.slug).or_insert(c.source);
        }
    }
    let actual: BTreeSet<_> = new.curves.iter().map(|c| c.slug.clone()).collect();
    assert_eq!(actual.len(), new.curves.len());
    assert_eq!(actual, expected.keys().cloned().collect());
    for c in &new.curves {
        assert_eq!(&c.source, &expected[&c.slug]);
    }
    let before_feature = old_slugs(&args[1]);
    let before_main = old_slugs(&args[2]);
    let missing_feature: Vec<_> = before_feature.difference(&actual).cloned().collect();
    let missing_main: Vec<_> = before_main.difference(&actual).cloned().collect();
    assert!(
        missing_feature.is_empty(),
        "feature parent curve coverage lost: {missing_feature:?}"
    );
    assert!(
        missing_main.is_empty(),
        "main parent curve coverage lost: {missing_main:?}"
    );
    println!("{}",serde_json::to_string_pretty(&serde_json::json!({"status":"pass","schema":new.schema,"traits_sha256":digest(text.as_bytes()),"registry_rows":registry_count,"generated_rows":actual.len(),"input_count":new.inputs.len(),"all_input_hashes_valid":true,"all_current_registry_manifest_sources_covered":true,"feature_parent_rows":before_feature.len(),"main_parent_rows":before_main.len(),"missing_feature_parent_slugs":missing_feature,"missing_main_parent_slugs":missing_main})).unwrap());
}
