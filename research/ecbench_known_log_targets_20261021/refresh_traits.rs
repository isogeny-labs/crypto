//! Guarded native trait refresh, following the existing catalogue workflow in
//! research/full_volcano_classes_20261017/refresh_traits.rs. This is explicitly
//! incremental; a timed-out full catalogue build is not reported as passing.
use crypto_lib::cryptanalysis::curve_traits::{self, Factoring, TraitFile};
use serde_json::{json, Value};
use std::{collections::BTreeMap, fs};

fn digest(bytes: &[u8]) -> String {
    crypto_lib::cryptanalysis::ecbench::canonical::sha256_hex(bytes)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert_eq!(args.len(), 2, "pass the unchanged base registry JSON");
    let original_registry = fs::read_to_string(&args[1]).unwrap();
    let text = fs::read_to_string("docs/curves/traits.json").unwrap();
    let mut file: TraitFile = serde_json::from_str(&text).unwrap();
    assert_eq!(file.registry_sha256, digest(original_registry.as_bytes()));
    for input in &file.inputs {
        if input.path != "docs/curves/registry.json" {
            assert_eq!(
                input.sha256,
                digest(&fs::read(&input.path).unwrap()),
                "changed non-registry trait input {}",
                input.path
            );
        }
    }
    let baseline: Value = serde_json::from_str(&original_registry).unwrap();
    let old_models: BTreeMap<_, _> = baseline["curves"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| (c["slug"].as_str().unwrap().to_string(), c.clone()))
        .collect();
    let registry = fs::read_to_string("docs/curves/registry.json").unwrap();
    let current: Value = serde_json::from_str(&registry).unwrap();
    let mut changed_representations = Vec::new();
    for row in current["curves"].as_array().unwrap() {
        if let Some(old) = old_models.get(row["slug"].as_str().unwrap()) {
            let (mut old_model, mut new_model) = (old.clone(), row.clone());
            old_model.as_object_mut().unwrap().remove("representations");
            new_model.as_object_mut().unwrap().remove("representations");
            assert_eq!(old_model, new_model, "an existing curve model changed");
            if old["representations"] != row["representations"] {
                changed_representations.push(row["slug"].as_str().unwrap().to_string());
            }
        }
    }
    let before: BTreeMap<_, _> = file
        .curves
        .iter()
        .map(|c| (c.slug.clone(), c.clone()))
        .collect();
    let hints =
        curve_traits::read_hints(&fs::read_to_string("docs/curves/factor-hints.txt").unwrap())
            .unwrap();
    let factoring = Factoring::new(file.factor_budget, hints);
    let mut remaining = before.clone();
    let mut rows = Vec::new();
    let mut added = Vec::new();
    for curve in curve_traits::read_registry(&registry).unwrap() {
        if let Some(old) = remaining.remove(&curve.slug) {
            if changed_representations.contains(&curve.slug) {
                assert_eq!(
                    curve_traits::compute(&curve, &factoring).unwrap(),
                    old,
                    "the changed generator representation changed its derived traits"
                );
            }
            rows.push(old);
        } else {
            assert!([
                "icv1-fp16-tm166-90917c6c",
                "icv1-fp16-tm124-317fecaa",
                "icv1-fp16-tm328-cbf43d6b"
            ]
            .contains(&curve.slug.as_str()));
            added.push(curve.slug.clone());
            rows.push(curve_traits::compute(&curve, &factoring).unwrap());
        }
    }
    assert_eq!(added.len(), 3);
    for old in &file.curves {
        if remaining.contains_key(&old.slug) {
            rows.push(old.clone());
        }
    }
    for row in &rows {
        if let Some(old) = before.get(&row.slug) {
            assert_eq!(row, old);
        }
    }
    file.registry_sha256 = digest(registry.as_bytes());
    file.inputs
        .iter_mut()
        .find(|i| i.path == "docs/curves/registry.json")
        .unwrap()
        .sha256 = file.registry_sha256.clone();
    file.curves = rows;
    let output = file.to_json();
    fs::write("docs/curves/traits.json", &output).unwrap();
    println!(
        "{}",
        json!({"status":"incremental_native_refresh","baseline_sha256":digest(text.as_bytes()),
        "output_sha256":digest(output.as_bytes()),"added":added,"preserved_rows":before.len(),
        "changed_representation_traits_recomputed_equal":changed_representations,
        "other_input_hashes_unchanged":true,"full_catalogue_replay":"not established: bounded full build timed out; see verification/curve-traits-build.log"})
    );
}
