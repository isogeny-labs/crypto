//! Native arithmetic checks for the written Lefschetz argument, not a formal proof.
#[allow(dead_code)]
#[path = "../prime_first_fall_donor.rs"]
mod donor;
#[path = "../prime_terminal_math.rs"]
mod math;
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};
const P: u128 = 4835703278761981908090881;
fn comb(n: u64, k: u64) -> u64 {
    if k > n {
        return 0;
    }
    (0..k.min(n - k)).fold(1, |v, i| v * (n - i) / (i + 1))
}
fn structure(ell: u64, p: u128) -> Value {
    assert_eq!(ell, 27);
    assert_eq!(p, P);
    let strings:Vec<_>=(0..=ell/2).map(|k|json!({"primitive_degree":k,"multiplicity":comb(ell,k)-if k==0{0}else{comb(ell,k-1)},"maximum_exponent":ell-2*k,"length":ell-2*k+1})).collect();
    let hilbert: Vec<_> = (0..=ell)
        .map(|h| {
            let dim: u64 = strings
                .iter()
                .filter(|s| {
                    let k = s["primitive_degree"].as_u64().unwrap();
                    k <= h && h <= ell - k
                })
                .map(|s| s["multiplicity"].as_u64().unwrap())
                .sum();
            assert_eq!(dim, comb(ell, h));
            json!({"degree":h,"dimension":dim})
        })
        .collect();
    assert_eq!(
        strings
            .iter()
            .map(|s| s["multiplicity"].as_u64().unwrap() * s["length"].as_u64().unwrap())
            .sum::<u64>(),
        1 << ell
    );
    let degrees: Vec<_> = (4..=16)
        .map(|d| {
            let slack = ell as i64 - 2 * d + 6;
            assert!(slack >= 0);
            json!({"degree":d,"minimum_slack":slack})
        })
        .collect();
    json!({"schema":"n83.native-lefschetz-structure/v1","ell":ell,"p":p.to_string(),"strings":strings,"hilbert":hilbert,"degrees":degrees,"first_uncovered_degree":17,"first_uncovered_slack":-1,"argument":"PRIME_LEFSCHETZ_ARGUMENT.md","basis_explicitly_expanded":false,"proof_status":"Written argument and arithmetic checks; independent review and formalization pending"})
}
fn up(h: &math::Poly) -> math::Poly {
    let mut out = BTreeMap::new();
    for (&mask, &c) in h {
        for i in 0..27 {
            if mask & (1 << i) == 0 {
                math::accum(&mut out, mask | (1 << i), c, P);
            }
        }
    }
    out
}
fn witness() -> math::Poly {
    let mut h = BTreeMap::from([(0, 1)]);
    for i in (0..26).step_by(2) {
        let mut next = BTreeMap::new();
        for (mask, c) in h {
            math::accum(&mut next, mask | (1 << i), c, P);
            math::accum(&mut next, mask | (1 << (i + 1)), math::neg(c, P), P);
        }
        h = next;
    }
    h
}
fn upper() -> Value {
    let h = witness();
    assert_eq!(h.len(), 8192);
    assert!(h.iter().all(|(m, c)| m.count_ones() == 13 && *c != 0));
    let first = up(&h);
    assert!(up(&first).is_empty());
    json!({"schema":"n83.native-lefschetz-upper/v1","normalized_terms":h.iter().map(|(m,c)|json!([m,c.to_string()])).collect::<Vec<_>>(),"degree":13,"terms":8192,"after_one_L_terms":first.len(),"after_two_L_terms":0,"first_fall_upper_bound":17,"source_projection_required":true})
}
fn leading(m: usize, w: &[usize]) -> Value {
    assert!((3..=9).contains(&m));
    assert_eq!(w.len(), 2 * m - 2);
    let mut records = Vec::new();
    let mut supports = BTreeSet::new();
    let mut disjoint = true;
    let mut strict = true;
    for k in 0..m - 1 {
        let x = if k == 0 { 0 } else { m + k - 1 };
        let y = k + 1;
        let mut expected = vec![x, x, y, y];
        expected.sort();
        let mut terms = vec![(expected.clone(), 1i64)];
        if k < m - 2 {
            let z = m + k;
            terms.extend([
                (vec![x, x, z, z], 1),
                (vec![y, y, z, z], 1),
                (vec![x, x, y, z], -2),
                (vec![x, y, y, z], -2),
                (vec![x, y, z, z], -2),
            ]);
        }
        for (mon, _) in &mut terms {
            mon.sort();
        }
        terms.sort();
        let score = |mon: &Vec<usize>| mon.iter().map(|i| w[*i]).sum::<usize>();
        let max = terms.iter().map(|(mon, _)| score(mon)).max().unwrap();
        let maximal: Vec<_> = terms
            .iter()
            .filter(|(mon, _)| score(mon) == max)
            .map(|(mon, _)| mon.clone())
            .collect();
        let ok = maximal == vec![expected.clone()];
        strict &= ok;
        for i in BTreeSet::from([x, y]) {
            disjoint &= supports.insert(i);
        }
        records.push(json!({"generator":k,"expected":expected,"maximal":maximal,"strict_expected":ok,"terms":terms.iter().map(|(mon,c)|json!([mon,c,score(mon)])).collect::<Vec<_>>()}));
    }
    json!({"m":m,"weights":w,"positive":w.iter().all(|x|*x>0),"pairwise_disjoint":disjoint,"records":records,"valid":strict&&disjoint&&w.iter().all(|x|*x>0)})
}
fn keys() -> BTreeSet<String> {
    ["original", "baseline", "spike40"]
        .iter()
        .flat_map(|a| {
            (3..=9).flat_map(move |m| {
                ["planted", "natural"]
                    .iter()
                    .map(move |t| format!("{a}-prime-m{m}-{t}"))
            })
        })
        .collect()
}
fn check_keys(rows: &[Value]) {
    assert_eq!(rows.len(), 42);
    assert_eq!(
        rows.iter()
            .map(|v| v["key"].as_str().unwrap().to_owned())
            .collect::<BTreeSet<_>>(),
        keys()
    );
}
fn hash(path: &Path) -> String {
    math::sha(&fs::read(path).unwrap())
}
fn bind(case: &Value, a: &Value, input: &Value, u: &Value) {
    for field in [
        "key",
        "input_sha256",
        "source_sha256",
        "factor_base_manifest_sha256",
    ] {
        assert_eq!(case[field], a[field]);
        assert!(!case[field].is_null());
    }
    for field in [
        "key",
        "m",
        "arm",
        "target_kind",
        "source_sha256",
        "factor_base_manifest_sha256",
    ] {
        assert_eq!(u[field], a[field]);
        assert!(!u[field].is_null());
    }
    assert_eq!(u["first_fall_upper_bound"], 17);
    assert_eq!(a["source"]["full_source_template_verified"], true);
    assert_eq!(input["m"], a["m"]);
    assert!((3..=9).contains(&input["m"].as_u64().unwrap()));
    assert_eq!(input["ell"], 27);
    assert_eq!(input["field_modulus"], P.to_string());
    assert_eq!(a["p"], P.to_string());
    assert_eq!(
        input["factor_base_manifest_sha256"],
        case["factor_base_manifest_sha256"]
    );
    let weights = a["source"]["weights"].as_array().unwrap();
    let inv = a["normalization_inverses"].as_array().unwrap();
    assert_eq!(weights.len(), 27);
    assert_eq!(inv.len(), 27);
    for (w, i) in weights.iter().zip(inv) {
        let w = w.as_str().unwrap().parse::<u128>().unwrap();
        let i = i.as_str().unwrap().parse::<u128>().unwrap();
        assert!(w > 0 && w < P && i < P);
        assert_eq!(math::mul(w, i, P), 1);
    }
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    assert_eq!(args.len(), 5, "run|replay ROOT PROTOCOL OUT");
    assert_eq!(
        std::env::var("N83_LEFSCHETZ_FROZEN_PRODUCTION").as_deref(),
        Ok("1")
    );
    let replay = args[1] == "replay";
    assert!(replay || args[1] == "run");
    let root = Path::new(&args[2]);
    let protocol = math::read(Path::new(&args[3]));
    let out = Path::new(&args[4]);
    assert_eq!(protocol["freeze"]["status"], "frozen");
    let cases = protocol["extensions"]["cases"].as_array().unwrap();
    check_keys(cases);
    for source in protocol["extensions"]["source_files"].as_array().unwrap() {
        assert_eq!(
            hash(&root.join("source").join(source["path"].as_str().unwrap())),
            source["sha256"]
        );
    }
    assert_eq!(
        hash(&root.join("instrument")),
        protocol["arms"][0]["instrument"]["binary_sha256"]
    );
    assert_eq!(
        hash(&root.join("certificate-instrument")),
        protocol["extensions"]["certificate_binary_sha256"]
    );
    assert_eq!(
        hash(&root.join("upper-summary.json")),
        protocol["extensions"]["upper_summary_sha256"]
    );
    let prior = math::read(&root.join("upper-summary.json"));
    let uppers = prior["rows"].as_array().unwrap();
    check_keys(uppers);
    if replay {
        assert!(out.is_dir());
    } else {
        fs::create_dir(out).unwrap();
    }
    let save = |name: &str, v: &Value| {
        let path = out.join(name);
        if replay {
            assert_eq!(math::read(&path), *v, "{name}");
        } else {
            use std::io::Write;
            let mut f = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(path)
                .unwrap();
            writeln!(f, "{}", serde_json::to_string_pretty(v).unwrap()).unwrap();
        }
    };
    save("structure.json", &structure(27, P));
    save("upper-witness.json", &upper());
    for m in 3..=9 {
        let mut w = vec![m; m];
        w.extend((1..=m - 2).rev());
        let g = leading(m, &w);
        assert_eq!(g["valid"], true);
        save(&format!("leading-m{m}.json"), &g);
    }
    let mut rows = Vec::new();
    for case in cases {
        let key = case["key"].as_str().unwrap();
        let ap = root.join("source-audit").join(format!("{key}.json"));
        let a = math::read(&ap);
        let ip = root.join("inputs").join(key).join("input.json");
        let sp = ip.with_file_name("system.sing");
        assert_eq!(hash(&ip), case["input_sha256"]);
        assert_eq!(hash(&sp), case["source_sha256"]);
        let input = math::read(&ip);
        let u = uppers.iter().find(|v| v["key"] == key).unwrap();
        bind(case, &a, &input, u);
        let row = json!({"key":key,"m":input["m"],"arm":a["arm"],"target_kind":a["target_kind"],"input_sha256":case["input_sha256"],"source_sha256":case["source_sha256"],"factor_base_manifest_sha256":case["factor_base_manifest_sha256"],"first_fall_lower_bound":17,"first_fall_upper_bound":17,"first_fall_degree":17,"last_fall_lower_bound":null,"last_fall_degree":null,"claim_status":"Conditional on the written Lefschetz/Koszul argument; independent review pending"});
        let mut record = row.clone();
        record["schema"] = json!("n83.native-lefschetz-case/v1");
        record["native_source_audit_sha256"] = json!(hash(&ap));
        for (field, file) in [
            ("structure_sha256", "structure.json".to_string()),
            ("upper_witness_sha256", "upper-witness.json".to_string()),
            ("leading_sha256", format!("leading-m{}.json", input["m"])),
        ] {
            record[field] = json!(hash(&out.join(file)));
        }
        record["normalization_inverses"] = a["normalization_inverses"].clone();
        save(&format!("{key}.json"), &record);
        rows.push(row);
    }
    save(
        "summary.json",
        &json!({"schema":"n83.native-lefschetz-summary/v1","rows":rows,"shared_structural_certificate":true,"all_prior_upper_source_hashes_matched":true,"new_lfd_claim":false,"independent_review":"pending"}),
    );
    println!(
        "{}",
        json!({"cases_verified":42,"replay":replay,"conditional_first_fall_degree":17,"new_lfd_claim":false})
    );
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn retain_arithmetic_controls() {
        let Some(path) = std::env::var_os("N83_NATIVE_LEFSCHETZ_TEST_ARTIFACTS") else {
            return;
        };
        let out = Path::new(&path);
        fs::create_dir(out).unwrap();
        math::save(&out.join("structure.json"), &structure(27, P));
        math::save(&out.join("upper-witness.json"), &upper());
        for m in 3..=9 {
            let mut weights = vec![m; m];
            weights.extend((1..=m - 2).rev());
            let good = leading(m, &weights);
            let bad = leading(m, &vec![1; 2 * m - 2]);
            assert_eq!(good["valid"], true);
            assert_eq!(bad["valid"], false);
            math::save(&out.join(format!("leading-m{m}.json")), &good);
            math::save(&out.join(format!("tied-m{m}.json")), &bad);
        }
        math::save(
            &out.join("identity.json"),
            &json!({
                "schema":"n83.native-lefschetz-development/v1",
                "development_only":true,"production_cases":0,"degree_claim":null,
                "source_sha256":math::sha(include_bytes!("prime_lefschetz_native.rs")),
                "explicit_witness_terms":8192,"positive_leading_cases":7,"negative_leading_cases":7
            }),
        );
    }
    #[test]
    fn binding_mutations() {
        let c = json!({"key":"original-prime-m3-natural","input_sha256":"input","source_sha256":"source","factor_base_manifest_sha256":"base"});
        let mut a = c.clone();
        a["m"] = json!(3);
        a["arm"] = json!("original");
        a["target_kind"] = json!("natural");
        a["p"] = json!(P.to_string());
        a["source"] = json!({"full_source_template_verified":true,"weights":vec!["1";27]});
        a["normalization_inverses"] = json!(vec!["1"; 27]);
        let input = json!({"m":3,"ell":27,"field_modulus":P.to_string(),"factor_base_manifest_sha256":"base"});
        let mut u = a.clone();
        u["first_fall_upper_bound"] = json!(17);
        bind(&c, &a, &input, &u);
        for field in [
            "key",
            "input_sha256",
            "source_sha256",
            "factor_base_manifest_sha256",
        ] {
            let mut bad = a.clone();
            bad[field] = json!("changed");
            assert!(std::panic::catch_unwind(|| bind(&c, &bad, &input, &u)).is_err());
        }
        let mut bad = a.clone();
        bad["normalization_inverses"][0] = json!("2");
        assert!(std::panic::catch_unwind(|| bind(&c, &bad, &input, &u)).is_err());
        let mut bad = u.clone();
        bad["source_sha256"] = json!("changed");
        assert!(std::panic::catch_unwind(|| bind(&c, &a, &input, &bad)).is_err());
        let mut bad = a.clone();
        bad["source"]["full_source_template_verified"] = json!(false);
        assert!(std::panic::catch_unwind(|| bind(&c, &bad, &input, &u)).is_err());
    }
    #[test]
    fn block_and_boundary() {
        let v = structure(27, P);
        assert_eq!(v["first_uncovered_slack"], -1);
        assert!(std::panic::catch_unwind(|| structure(26, P)).is_err());
        assert!(std::panic::catch_unwind(|| structure(27, 101)).is_err());
    }
    #[test]
    fn explicit_witness_and_mutation() {
        assert_eq!(upper()["terms"], 8192);
        let mut h = witness();
        let first = *h.keys().next().unwrap();
        math::accum(&mut h, first, 1, P);
        assert!(!up(&up(&h)).is_empty());
    }
    #[test]
    fn leading_positive_and_ties() {
        for m in 3..=9 {
            let mut w = vec![m; m];
            w.extend((1..=m - 2).rev());
            assert_eq!(leading(m, &w)["valid"], true);
            assert_eq!(leading(m, &vec![1; 2 * m - 2])["valid"], false);
        }
    }
    #[test]
    fn scope_duplicates_and_missing() {
        let mut c: Vec<_> = keys().into_iter().map(|k| json!({"key":k})).collect();
        check_keys(&c);
        c[0] = c[1].clone();
        assert!(std::panic::catch_unwind(|| check_keys(&c)).is_err());
        c.pop();
        assert!(std::panic::catch_unwind(|| check_keys(&c)).is_err());
    }
}
