//! Fixed-domain presentation census. No production invocation before protocol freeze.
#[allow(dead_code)]
#[path = "../radix_control_donor.rs"]
mod controls;
#[allow(dead_code)]
#[path = "../radix_hilbert_donor.rs"]
mod donor;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
const P: u128 = 4835703278761981908090881;

fn sha(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn bytes(v: &Value) -> Vec<u8> {
    (serde_json::to_string_pretty(v).unwrap() + "\n").into_bytes()
}

fn candidate(parts: &[usize]) -> Value {
    assert!(!parts.is_empty() && parts.iter().all(|&k| (1..=27).contains(&k)));
    assert!(parts.windows(2).all(|w| w[0] <= w[1]));
    let bits: usize = parts.iter().sum();
    assert!((1..=27).contains(&bits));
    let radices: Vec<_> = parts.iter().map(|&k| 1usize << k).collect();
    let d: usize = radices.iter().map(|r| r - 1).sum();
    let mut weights = Vec::new();
    let mut count = 1u128;
    for &r in &radices {
        weights.push(count);
        count *= r as u128;
    }
    assert_eq!(count, 1u128 << bits);
    assert!(count < P && (d as u128) < P);
    let mut row = json!({"parts":parts,"radices":radices,"bits":bits,"socle_degree":d,"candidate_x_count":count.to_string(),"weights":weights.iter().map(|w|w.to_string()).collect::<Vec<_>>(),"candidate_interval":{"start_inclusive":"0","end_exclusive":count.to_string()},"domain_bijection":"mixed-radix expansion; product radices equals interval length, less than modulus","presentation":{"selector_equation":"product_(a=0..radix_j-1)(digit_j-a)=0 in each of two disjoint blocks","x_coordinate":"sum_j weight_j*digit_j","curve_equation":"original m2 S3(X0,X1,target_x), a=2,b=3","top_quotient":"(A tensor A)/(L0^2 L1^2), A=k[z_j]/(z_j^radix_j)"},"regularity_convention":"first zero Hilbert coefficient of the original highest-form quotient","first_vanishing_degree":null,"lower_bound":d+2,"lower_witness_bidegree":[d,1],"lower_witness_dimension":radices.len(),"exact_lfd":null,"ffd":null,"characteristic_gt_socle":true});
    if d <= 40 {
        let h = controls::block_hilbert(&radices);
        assert_eq!(h.iter().sum::<u128>(), count);
        let ranks: Vec<_> = (0..h.len())
            .map(|i| if i < 2 { 0 } else { h[i].min(h[i - 2]) })
            .collect();
        let mut q = vec![0u128; 2 * d + 2];
        for i in 0..=d {
            for j in 0..=d {
                q[i + j] += h[i] * h[j] - ranks[i] * ranks[j];
            }
        }
        let first = q.iter().position(|&v| v == 0).unwrap();
        assert!(q[first..].iter().all(|&v| v == 0));
        assert!(first >= d + 2);
        // All counts fit u64: total unquotiented dimension <= 2^54.
        let encode = |v: &[u128]| {
            v.iter()
                .map(|&x| u64::try_from(x).unwrap())
                .collect::<Vec<_>>()
        };
        row["status"] = json!("exact_hilbert_under_large_characteristic_theorem");
        row["block_hilbert"] = json!(encode(&h));
        row["multiplication_L_squared_ranks"] = json!(encode(&ranks));
        row["quotient_hilbert"] = json!(encode(&q));
        row["first_vanishing_degree"] = json!(first);
        row["last_nonzero_coefficient"] = json!(u64::try_from(q[first - 1]).unwrap());
        row["quotient_dimension"] = json!(u64::try_from(q.iter().sum::<u128>()).unwrap());
    } else {
        row["status"] = json!("rigorous_lower_bound_only");
    }
    row
}

fn payloads(input_bytes: &[u8], source_bytes: &[u8]) -> BTreeMap<String, Vec<u8>> {
    let input: Value = serde_json::from_slice(input_bytes).unwrap();
    assert_eq!(input["target_kind"], "natural");
    assert_eq!(input["candidate_x_count"], "134217728");
    let source = std::str::from_utf8(source_bytes).unwrap();
    let binding = donor::bind(&input, source);
    let prime = donor::primality();
    let parts = controls::all_partitions(27);
    let mut out = BTreeMap::new();
    out.insert("input.json".into(), input_bytes.to_vec());
    out.insert("system.sing".into(), source_bytes.to_vec());
    out.insert("binding.json".into(),bytes(&json!({"schema":"n83.radix-binding/v1","input_sha256":sha(input_bytes),"source_sha256":sha(source_bytes),"factor_base_manifest_sha256":input["factor_base_manifest_sha256"],"original_source":binding,"primality":prime,"same_point_base_by_domain_bijection":true,"curve_lift_convention":input["membership"],"scope":"original prime m2 natural; alternative radix presentations of identical candidate interval"})));
    let mut summary_rows = Vec::new();
    for (i, part) in parts.iter().enumerate() {
        let row = candidate(part);
        let path = format!("cases/{i:04}.json");
        let data = bytes(&row);
        summary_rows.push(json!({"file":path,"sha256":sha(&data),"parts":part,"socle_degree":row["socle_degree"],"status":row["status"],"first_vanishing_degree":row["first_vanishing_degree"],"lower_bound":row["lower_bound"]}));
        out.insert(path, data);
    }
    let exact = summary_rows
        .iter()
        .filter(|r| r["first_vanishing_degree"].is_number())
        .count();
    out.insert("summary.json".into(),bytes(&json!({"schema":"n83.radix-census/v1","partitions":parts.len(),"exact_cases":exact,"lower_bound_cases":parts.len()-exact,"baseline_reference_first_vanishing_degree":42,"rows":summary_rows,"mathematical_dependency":"Phuong-Tran Theorem3.1 and retained HILBERT_DESIGN tensor-image argument; no full Lean formalization","exact_lfd":null,"ffd":null})));
    let manifest: Vec<_> = out
        .iter()
        .map(|(p, b)| json!({"path":p,"sha256":sha(b),"bytes":b.len()}))
        .collect();
    out.insert("manifest.json".into(), bytes(&json!(manifest)));
    out
}

fn main() {
    let a: Vec<_> = std::env::args().collect();
    assert!(
        a.len() >= 3,
        "generate INPUT_DIR FRESH_OUTPUT or verify OUTPUT"
    );
    match a[1].as_str() {
        "prepare" => {
            assert_eq!(a.len(), 4);
            let source = Path::new(&a[2]);
            let out = Path::new(&a[3]);
            assert!(!out.exists());
            let input_bytes = fs::read(source.join("input.json")).unwrap();
            let source_bytes = fs::read(source.join("system.sing")).unwrap();
            let input: Value = serde_json::from_slice(&input_bytes).unwrap();
            assert_eq!(input["target_kind"], "natural");
            assert_eq!(input["candidate_x_count"], "134217728");
            let binding = donor::bind(&input, std::str::from_utf8(&source_bytes).unwrap());
            let parts = controls::all_partitions(27);
            fs::create_dir_all(out).unwrap();
            fs::write(out.join("input.json"), &input_bytes).unwrap();
            fs::write(out.join("system.sing"), &source_bytes).unwrap();
            fs::write(out.join("index.json"), bytes(&json!({"schema":"n83.radix-prepared/v1","input_sha256":sha(&input_bytes),"source_sha256":sha(&source_bytes),"factor_base_manifest_sha256":input["factor_base_manifest_sha256"],"source_validation":binding,"parts":parts,"candidate_count":parts.len(),"hilbert_census_performed":false}))).unwrap();
            println!(
                "{}",
                json!({"prepared_candidates":parts.len(),"hilbert_census_performed":false})
            );
        }
        "generate" => {
            assert_eq!(a.len(), 4);
            let source = Path::new(&a[2]);
            let out = Path::new(&a[3]);
            assert!(!out.exists());
            let data = payloads(
                &fs::read(source.join("input.json")).unwrap(),
                &fs::read(source.join("system.sing")).unwrap(),
            );
            fs::create_dir_all(out.join("cases")).unwrap();
            for (p, b) in &data {
                fs::write(out.join(p), b).unwrap();
            }
            println!(
                "{}",
                String::from_utf8(data["summary.json"].clone()).unwrap()
            );
        }
        "verify" => {
            assert_eq!(a.len(), 3);
            let out = Path::new(&a[2]);
            let expected = payloads(
                &fs::read(out.join("input.json")).unwrap(),
                &fs::read(out.join("system.sing")).unwrap(),
            );
            for (p, b) in &expected {
                assert_eq!(&fs::read(out.join(p)).unwrap(), b, "replay {p}");
            }
            println!(
                "{}",
                json!({"native_replay_verified":true,"payloads":expected.len()})
            );
        }
        _ => panic!("unknown command"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn small_one_digit_quotient() {
        // In k[x,y]/(x^4,y^4,x^2*y^2), the top survivors are x^3*y and x*y^3.
        let r = candidate(&[2]);
        assert_eq!(r["first_vanishing_degree"], 5);
        assert_eq!(r["last_nonzero_coefficient"], 2);
    }
    #[test]
    fn small_two_boolean_digits() {
        let r = candidate(&[1, 1]);
        assert_eq!(r["quotient_hilbert"], json!([1, 4, 6, 4, 0, 0]));
    }
    #[test]
    fn bounded_path_has_no_exact_claim() {
        let r = candidate(&[6]);
        assert_eq!(r["lower_bound"], 65);
        assert!(r["first_vanishing_degree"].is_null());
    }
}
