//! Lossless metadata registration. No curve arithmetic or research statistics.
use crypto_lib::cryptanalysis::ecbench::canonical::sha256_hex;
use serde::{Deserialize, Serialize};
use serde_json::{json, value::RawValue, Value};
use std::{collections::BTreeMap, fs, io::Write};
const ROOT: &str = "research/conductor_gap_replication_20261025";
#[derive(Deserialize)]
struct RawRegistry<'a> {
    #[serde(borrow)]
    curves: Vec<&'a RawValue>,
}
#[derive(Serialize)]
struct NewRow<'a> {
    slug: &'a Value,
    icv1: &'a Value,
    family: &'a Value,
    params: &'a Value,
    model_json: &'a Value,
    trace: &'a Value,
    order: &'a Value,
    j: &'a Value,
    end: &'a Value,
    aliases: Vec<String>,
    standard_names: Vec<String>,
    sources: &'a Value,
    representations: &'a Value,
}
#[derive(Serialize)]
struct AliasDocument<'a> {
    schema_version: &'a Value,
    generated_from: &'a Value,
    generated_by: &'a Value,
    aliases: &'a Value,
}
fn pretty(v: &impl Serialize, indent: &[u8]) -> String {
    let mut out = Vec::new();
    let mut ser = serde_json::Serializer::with_formatter(
        &mut out,
        serde_json::ser::PrettyFormatter::with_indent(indent),
    );
    v.serialize(&mut ser).unwrap();
    String::from_utf8(out).unwrap()
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    assert!(
        args.len() == 2 || (args.len() == 3 && args[2] == "--check"),
        "PRIOR_REGISTRY_JSON [--check] (the original pre-registration snapshot)"
    );
    let check_only = args.len() == 3;
    let prior_text = fs::read_to_string(&args[1]).unwrap();
    let prior: RawRegistry = serde_json::from_str(&prior_text).unwrap();
    let current_text = fs::read_to_string("docs/curves/registry.json").unwrap();
    let current: RawRegistry = serde_json::from_str(&current_text).unwrap();
    let current_map: BTreeMap<String, Value> = current
        .curves
        .iter()
        .map(|raw| {
            let v: Value = serde_json::from_str(raw.get()).unwrap();
            (v["slug"].as_str().unwrap().into(), v)
        })
        .collect();
    let mut changed_old = Vec::new();
    let mut rows: BTreeMap<String, (Value, String)> = BTreeMap::new();
    for raw in &prior.curves {
        let value: Value = serde_json::from_str(raw.get()).unwrap();
        let slug = value["slug"].as_str().unwrap().to_string();
        if current_map.get(&slug) != Some(&value) {
            changed_old.push(slug.clone());
        }
        assert!(rows.insert(slug, (value, raw.get().to_string())).is_none());
    }
    let mut aliases: Value =
        serde_json::from_slice(&fs::read("src/cryptanalysis/curve_aliases.json").unwrap()).unwrap();
    let mut added = 0;
    for panel in ["prepared-a", "prepared-b-v2"] {
        let identities: Value =
            serde_json::from_slice(&fs::read(format!("{ROOT}/{panel}/identities.json")).unwrap())
                .unwrap();
        for v in identities.as_array().unwrap() {
            let slug = v["slug"].as_str().unwrap().to_string();
            let row = NewRow {
                slug: &v["slug"],
                icv1: &v["icv1"],
                family: &v["family"],
                params: &v["params"],
                model_json: &v["model_json"],
                trace: &v["trace"],
                order: &v["order"],
                j: &v["j"],
                end: &v["end"],
                aliases: vec![],
                standard_names: vec![],
                sources: &v["sources"],
                representations: &v["representations"],
            };
            let new_value = serde_json::to_value(&row).unwrap();
            if let Some((old, _)) = rows.get(&slug) {
                assert_eq!(
                    old, &new_value,
                    "unexpected overlap; explicitly reconcile existing representations"
                );
            } else {
                if let Some(recorded) = current_map.get(&slug) {
                    assert_eq!(recorded, &new_value, "measured new model must not change");
                }
                let raw = pretty(&row, b" ")
                    .lines()
                    .enumerate()
                    .map(|(i, s)| {
                        if i == 0 {
                            s.to_string()
                        } else {
                            format!("  {s}")
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                rows.insert(slug.clone(), (new_value, raw));
                added += 1;
            }
            for alias in v["normalized_aliases"].as_array().unwrap() {
                let key = alias.as_str().unwrap();
                if let Some(existing) = aliases["aliases"].get(key) {
                    assert_eq!(existing, &v["slug"]);
                }
                aliases["aliases"][key] = v["slug"].clone();
            }
        }
    }
    // Keep the already frozen registry order and all measured new models.
    // For a fresh registration, merge new models by the same family/size/p/slug key.
    let rank = |v: &Value| match v["family"].as_str().unwrap() {
        "koblitz" => 0,
        "subfield" => 1,
        "binary" => 2,
        "prime" => 3,
        "extension" => 4,
        _ => panic!("family"),
    };
    let integer = |v: &Value| -> num_bigint::BigUint {
        v.as_str()
            .map(str::to_string)
            .unwrap_or_else(|| v.to_string())
            .parse()
            .unwrap()
    };
    let dim = |v: &Value| {
        integer(
            v["params"]
                .get("n")
                .or_else(|| v["params"].get("m"))
                .unwrap_or(&Value::from(0)),
        )
    };
    let field = |v: &Value| integer(v["params"].get("p").unwrap_or(&Value::from(0)));
    for slug in current_map.keys() {
        assert!(
            rows.contains_key(slug),
            "refusing to discard current model {slug}"
        );
    }
    let mut ordered: Vec<_> = rows.into_values().collect();
    ordered.sort_by(|(a, _), (b, _)| {
        rank(a)
            .cmp(&rank(b))
            .then_with(|| dim(a).cmp(&dim(b)))
            .then_with(|| field(a).cmp(&field(b)))
            .then_with(|| {
                if a["family"] == "koblitz" {
                    a["params"]["a"].as_u64().cmp(&b["params"]["a"].as_u64())
                } else {
                    std::cmp::Ordering::Equal
                }
            })
            .then_with(|| a["slug"].as_str().cmp(&b["slug"].as_str()))
    });
    #[derive(Deserialize)]
    struct ArraySpan<'a> {
        #[serde(borrow)]
        curves: &'a RawValue,
    }
    let span: ArraySpan = serde_json::from_str(&prior_text).unwrap();
    let start = span.curves.get().as_ptr() as usize - prior_text.as_ptr() as usize;
    assert_eq!(
        &prior_text[start..start + span.curves.get().len()],
        span.curves.get()
    );
    let array = format!(
        "[\n{}\n ]",
        ordered
            .iter()
            .map(|(_, raw)| format!("  {raw}"))
            .collect::<Vec<_>>()
            .join(",\n")
    );
    let output = format!(
        "{}{}{}",
        &prior_text[..start],
        array,
        &prior_text[start + span.curves.get().len()..]
    );
    // The maintained alias file also has a canonical property order.
    let alias_output = pretty(
        &AliasDocument {
            schema_version: &aliases["schema_version"],
            generated_from: &aliases["generated_from"],
            generated_by: &aliases["generated_by"],
            aliases: &aliases["aliases"],
        },
        b"",
    ) + "\n";
    if check_only {
        assert!(
            changed_old.is_empty(),
            "legacy records differ from the frozen snapshot"
        );
        assert_eq!(
            current_text, output,
            "registry is not the exact lossless merge"
        );
        assert_eq!(
            fs::read_to_string("src/cryptanalysis/curve_aliases.json").unwrap(),
            alias_output
        );
        println!(
            "verified {} prior raw objects and {added} unchanged experiment models",
            prior.curves.len()
        );
        return;
    }
    let receipt = json!({"schema":"conductor-gap.lossless-registration/v2","prior_models":prior.curves.len(),
        "added_models":added,"total_models":ordered.len(),"legacy_models_restored":changed_old,
        "prior_registry_sha256":sha256_hex(prior_text.as_bytes()),"replaced_registry_sha256":sha256_hex(current_text.as_bytes()),
        "registry_sha256":sha256_hex(output.as_bytes()),"all_prior_objects_and_raw_numbers_preserved":true,
        "measured_new_models_unchanged":true,"parser":"Rust serde_json arbitrary_precision + borrowed RawValue"});
    let mut receipt_file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(format!("{ROOT}/registration-native.json"))
        .unwrap();
    writeln!(
        receipt_file,
        "{}",
        serde_json::to_string_pretty(&receipt).unwrap()
    )
    .unwrap();
    fs::write("docs/curves/registry.json", output).unwrap();
    fs::write("src/cryptanalysis/curve_aliases.json", alias_output).unwrap();
    println!(
        "registered {added} experiment models; restored {} legacy models",
        receipt["legacy_models_restored"].as_array().unwrap().len()
    );
}

#[test]
fn metadata_integers_and_raw_objects_round_trip_exactly() {
    let text = r#"{"curves":[{"slug":"exact","trace":-340282366920938463463374607431768211457,"cofactor":340282366920938463463374607431768211457}]}"#;
    let raw: RawRegistry = serde_json::from_str(text).unwrap();
    assert_eq!(raw.curves[0].get(), &text[11..text.len() - 2]);
    let value: Value = serde_json::from_str(raw.curves[0].get()).unwrap();
    let encoded = serde_json::to_string(&value).unwrap();
    assert!(encoded.contains("\"trace\":-340282366920938463463374607431768211457"));
    assert!(encoded.contains("\"cofactor\":340282366920938463463374607431768211457"));
}
