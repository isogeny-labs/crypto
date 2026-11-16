//! Exact parent source and target-control binding for centered coefficients.
#[path = "prime_linear_source_donor.rs"]
mod source_donor;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, fs, path::Path};
fn sha(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn scalar(v: &Value) -> u128 {
    let s = v.as_str().unwrap();
    let a: u128 = s.parse().unwrap();
    assert_eq!(s, a.to_string());
    a
}
fn s3(x: &str, y: &str, z: &str) -> String {
    format!("(({x})-({y}))^2*({z})^2-2*((({x})+({y}))*(({x})*({y})+2)+2*3)*({z})+(({x})*({y})-2)^2-4*3*(({x})+({y}))")
}
pub fn make(parent_text: &str, parent_source: &str, case: &Value, kind: &str) -> Value {
    assert!(["retained", "centered", "adjacent"].contains(&kind));
    assert_eq!(sha(parent_text.as_bytes()), case["input_sha256"]);
    assert_eq!(sha(parent_source.as_bytes()), case["source_sha256"]);
    let parent: Value = serde_json::from_str(parent_text).unwrap();
    let arm = case["arm"].as_str().unwrap();
    let m = parent["m"].as_u64().unwrap();
    assert_eq!(case["m"], parent["m"]);
    assert_eq!(case["target_kind"], parent["target_kind"]);
    assert_eq!(
        case["factor_base_manifest_sha256"],
        parent["factor_base_manifest_sha256"]
    );
    assert_eq!(
        case["key"],
        format!(
            "{arm}-prime-m{m}-{}",
            parent["target_kind"].as_str().unwrap()
        )
    );
    let checked = source_donor::source(&parent, parent_source, arm);
    let p = scalar(&parent["field_modulus"]);
    let w: Vec<_> = checked["weights"]
        .as_array()
        .unwrap()
        .iter()
        .map(scalar)
        .collect();
    let sum: u128 = w.iter().sum();
    assert!(sum < p);
    let c = if sum % 2 == 0 { sum / 2 } else { (sum + p) / 2 };
    let mut effective = parent.clone();
    let mut source = parent_source.to_string();
    let mut changed_line = Value::Null;
    let origin = if kind == "retained" {
        format!("retained_{}", parent["target_kind"].as_str().unwrap())
    } else {
        assert_eq!(
            parent["target_kind"], "natural",
            "synthetic parent must be natural"
        );
        assert!(parent["witness"].is_null());
        let t = if kind == "centered" { c } else { (c + 1) % p };
        effective["target_x"] = json!(t.to_string());
        effective["id"] = json!(format!("{}-{kind}", case["key"].as_str().unwrap()));
        for name in ["target_y", "points", "factor_base_membership"] {
            effective.as_object_mut().unwrap().remove(name);
        }
        effective["target_origin"] = json!(format!("synthetic_{kind}_control"));
        effective["target_lift_checked"] = json!(false);
        let prefix = format!("I[{}]=", m - 1);
        let old: Vec<_> = parent_source
            .lines()
            .filter(|l| l.starts_with(&prefix))
            .collect();
        assert_eq!(old.len(), 1);
        let line = format!(
            "{prefix}{};",
            s3(
                &format!("v({})", 27 * m + m - 2),
                &format!("x{}", m - 1),
                &t.to_string()
            )
        );
        source = parent_source
            .lines()
            .map(|l| {
                if l.starts_with(&prefix) {
                    line.as_str()
                } else {
                    l
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
            + "\n";
        assert_eq!(
            source
                .lines()
                .zip(parent_source.lines())
                .filter(|(a, b)| a != b)
                .count(),
            usize::from(line != old[0])
        );
        changed_line = json!({"equation_one_based":m-1,"parent_line":old[0],"effective_line":line});
        effective["source_sha256"] = json!(sha(source.as_bytes()));
        format!("synthetic_{kind}_control")
    };
    let effective_check = source_donor::source(&effective, &source, arm);
    json!({"schema":"n83.centered-source-bound-input/v1","modulus":p.to_string(),"ell":27,"weights":checked["weights"],"target":effective["target_x"],"target_origin":origin,"m":m,"arm":arm,"factor_base_manifest_sha256":case["factor_base_manifest_sha256"],"binding":{"kind":kind,"parent_case":case,"parent_input_text":parent_text,"parent_source_text":parent_source,"effective_input":effective,"effective_source_text":source,"effective_source_sha256":sha(source.as_bytes()),"full_source_template_verified":effective_check["full_source_template_verified"],"planted_witness_checked":effective_check["planted_witness_checked"],"terminal_equation_one_based":effective_check["terminal_equation_one_based"],"terminal_intermediate_zero_based":effective_check["terminal_intermediate_zero_based"],"changed_terminal_line":changed_line,"coordinate_change":"y_j=w_j*(b_j-1/2); U retained as unrestricted indeterminate","rational_target_lift_claim":false}})
}
pub fn validate(input: &Value) {
    let b = &input["binding"];
    let want = make(
        b["parent_input_text"].as_str().unwrap(),
        b["parent_source_text"].as_str().unwrap(),
        &b["parent_case"],
        b["kind"].as_str().unwrap(),
    );
    assert_eq!(*input, want, "source binding or normalized input mismatch");
}
pub fn prepare(root: &Path, cases_path: &Path, out: &Path) {
    let cases: Value = serde_json::from_slice(&fs::read(cases_path).unwrap()).unwrap();
    let cases = cases.as_array().unwrap();
    assert_eq!(cases.len(), 42);
    let mut expected = BTreeSet::new();
    for arm in ["original", "baseline", "spike40"] {
        for m in 3..=9 {
            for t in ["planted", "natural"] {
                expected.insert(format!("{arm}-prime-m{m}-{t}"));
            }
        }
    }
    assert_eq!(
        cases
            .iter()
            .map(|c| c["key"].as_str().unwrap().to_string())
            .collect::<BTreeSet<_>>(),
        expected
    );
    assert!(!out.exists());
    fs::create_dir_all(out).unwrap();
    let mut index = Vec::new();
    for case in cases {
        let key = case["key"].as_str().unwrap();
        let input = fs::read_to_string(root.join("inputs").join(key).join("input.json")).unwrap();
        let source = fs::read_to_string(root.join("inputs").join(key).join("system.sing")).unwrap();
        let kinds = if case["target_kind"] == "natural" {
            vec!["retained", "centered", "adjacent"]
        } else {
            vec!["retained"]
        };
        for kind in kinds {
            let record = make(&input, &source, case, kind);
            validate(&record);
            let name = format!("{key}-{kind}.json");
            let bytes = serde_json::to_string_pretty(&record).unwrap() + "\n";
            fs::write(out.join(&name), bytes.as_bytes()).unwrap();
            index.push(json!({"key":format!("{key}-{kind}"),"file":name,"sha256":sha(bytes.as_bytes()),"parent_key":key,"parent_input_sha256":case["input_sha256"],"parent_source_sha256":case["source_sha256"],"effective_source_sha256":record["binding"]["effective_source_sha256"],"m":record["m"],"arm":record["arm"],"kind":kind,"target_origin":record["target_origin"],"factor_base_manifest_sha256":record["factor_base_manifest_sha256"]}));
        }
    }
    assert_eq!(index.len(), 84);
    fs::write(out.join("index.json"),serde_json::to_string_pretty(&json!({"schema":"n83.centered-prepared-panel/v1","cases":index,"coefficient_generation_performed":false})).unwrap()+"\n").unwrap();
    println!("prepared 84 source-bound inputs");
}
#[cfg(test)]
mod tests {
    use super::*;
    fn fixtures() -> Vec<(String, String, Value)> {
        let root =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("validation-centered-binding/fixtures");
        let c: Value = serde_json::from_slice(&fs::read(root.join("cases.json")).unwrap()).unwrap();
        c.as_array()
            .unwrap()
            .iter()
            .map(|v| {
                let d = root.join(v["key"].as_str().unwrap());
                (
                    fs::read_to_string(d.join("input.json")).unwrap(),
                    fs::read_to_string(d.join("system.sing")).unwrap(),
                    v.clone(),
                )
            })
            .collect()
    }
    #[test]
    fn retained_and_synthetic_binding() {
        for (i, s, c) in fixtures() {
            let v = make(&i, &s, &c, "retained");
            validate(&v);
            assert_eq!(v["binding"]["effective_source_text"], s);
            if c["target_kind"] == "natural" {
                for kind in ["centered", "adjacent"] {
                    let v = make(&i, &s, &c, kind);
                    validate(&v);
                    assert!(v["binding"]["effective_input"].get("target_y").is_none());
                    assert!(v["binding"]["effective_input"].get("points").is_none());
                    assert!(v["target_origin"]
                        .as_str()
                        .unwrap()
                        .starts_with("synthetic_"));
                    assert_eq!(
                        v["binding"]["effective_source_text"]
                            .as_str()
                            .unwrap()
                            .lines()
                            .zip(s.lines())
                            .filter(|(a, b)| a != b)
                            .count(),
                        1
                    );
                }
            } else {
                assert!(std::panic::catch_unwind(|| make(&i, &s, &c, "centered")).is_err());
            }
        }
    }
    #[test]
    fn changed_parent_or_case_rejected() {
        let (i, s, c) = fixtures().remove(0);
        assert!(std::panic::catch_unwind(|| make(&(i.clone() + " "), &s, &c, "retained")).is_err());
        assert!(std::panic::catch_unwind(|| make(&i, &(s.clone() + " "), &c, "retained")).is_err());
        for field in ["key", "m", "target_kind", "factor_base_manifest_sha256"] {
            let mut bad = c.clone();
            bad[field] = json!("mutated");
            assert!(std::panic::catch_unwind(|| make(&i, &s, &bad, "retained")).is_err());
        }
    }
    #[test]
    fn wrapper_mutations_rejected() {
        let (i, s, c) = fixtures().remove(0);
        let v = make(&i, &s, &c, "retained");
        for name in [
            "target",
            "weights",
            "target_origin",
            "m",
            "factor_base_manifest_sha256",
        ] {
            let mut bad = v.clone();
            bad[name] = json!("mutated");
            assert!(std::panic::catch_unwind(|| validate(&bad)).is_err());
        }
        let mut bad = v;
        bad["binding"]["coordinate_change"] = json!("U=0");
        assert!(std::panic::catch_unwind(|| validate(&bad)).is_err());
    }
}
