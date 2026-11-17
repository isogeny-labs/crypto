//! Source-bound arithmetic only. No filtered closure or LFD certificate.
#[allow(dead_code)]
#[path = "../prime_first_fall_donor.rs"]
mod donor;
#[path = "../prime_terminal_math.rs"]
mod math;
#[path = "../prime_linear_source_donor.rs"]
mod source_donor;
use math::*;
use serde_json::{json, Value};
use std::{collections::BTreeSet, fs, path::Path};

fn keys() -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for a in ["original", "baseline", "spike40"] {
        for m in 3..=9 {
            for t in ["planted", "natural"] {
                out.insert(format!("{a}-prime-m{m}-{t}"));
            }
        }
    }
    out
}
fn check_keys(cases: &[Value]) {
    assert_eq!(cases.len(), 42);
    assert_eq!(
        cases
            .iter()
            .map(|v| v["key"].as_str().unwrap().to_owned())
            .collect::<BTreeSet<_>>(),
        keys()
    );
}
fn circuit(t: u128, p: u128) -> Value {
    let tt = mul(t, t, p);
    let minus2t = neg(mul(2, t, p), p);
    let low = neg(add(mul(4, t, p), 12, p), p);
    let b = vec![
        vec![add(4, neg(mul(12, t, p), p), p), low, tt],
        vec![low, neg(add(mul(2, tt, p), 4, p), p), minus2t],
        vec![tt, minus2t, 1],
    ];
    json!(b
        .iter()
        .map(|r| r.iter().map(|x| x.to_string()).collect::<Vec<_>>())
        .collect::<Vec<_>>())
}
fn verify(
    case: &Value,
    input_bytes: &[u8],
    source: &str,
    certificate_bytes: &[u8],
    top_bytes: &[u8],
    factors: &[Vec<u8>],
) -> (Value, Vec<u8>) {
    let p = donor::field();
    assert_eq!(sha(input_bytes), case["input_sha256"]);
    assert_eq!(sha(source.as_bytes()), case["source_sha256"]);
    assert_eq!(sha(certificate_bytes), case["terminal_certificate_sha256"]);
    assert_eq!(sha(top_bytes), case["top_certificate_sha256"]);
    let input: Value = serde_json::from_slice(input_bytes).unwrap();
    let cert: Value = serde_json::from_slice(certificate_bytes).unwrap();
    let top: Value = serde_json::from_slice(top_bytes).unwrap();
    let arm = case["arm"].as_str().unwrap();
    let key = case["key"].as_str().unwrap();
    assert_eq!(
        key,
        format!(
            "{arm}-prime-m{}-{}",
            input["m"],
            input["target_kind"].as_str().unwrap()
        )
    );
    assert_eq!(
        input["factor_base_manifest_sha256"],
        case["factor_base_manifest_sha256"]
    );
    let source_check = source_donor::source(&input, source, arm);
    assert_eq!(cert["schema"], "n83.prime-terminal-certificate/v1");
    assert_eq!(cert["key"], key);
    assert_eq!(cert["input"], input);
    assert_eq!(cert["source"], source_check);
    for field in [
        "key",
        "input_sha256",
        "source_sha256",
        "factor_base_manifest_sha256",
    ] {
        assert!(!case[field].is_null());
        assert_eq!(top[field], case[field]);
    }
    assert_eq!(top["m"], input["m"]);
    assert_eq!(top["arm"], arm);
    assert_eq!(top["target_kind"], input["target_kind"]);
    assert_eq!(top["first_fall_lower_bound"], 17);
    assert_eq!(top["first_fall_upper_bound"], 17);
    assert_eq!(
        top["claim_status"],
        "Conditional on the written Lefschetz/Koszul argument; independent review pending"
    );
    let weights: Vec<u128> = source_check["weights"]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| x.as_str().unwrap().parse().unwrap())
        .collect();
    let t = input["target_x"].as_str().unwrap().parse::<u128>().unwrap();
    let h = h_poly(27, p);
    let a1 = multiply_x(&h, &weights, p);
    let a2 = multiply_x(&a1, &weights, p);
    let aa = [h, a1, a2];
    assert_eq!(factors.len(), 3);
    let file_records = cert["factor_files"].as_array().unwrap();
    assert_eq!(file_records.len(), 3);
    let mut factor_hashes = vec![];
    for i in 0..3 {
        let f = &file_records[i];
        let raw = &factors[i];
        assert_eq!(f["path"], format!("A{i}.poly"));
        assert_eq!(f["bytes"], raw.len());
        assert_eq!(f["sha256"], sha(raw));
        let (n, field, a) = parse(raw);
        assert_eq!(n, 27);
        assert_eq!(field, p);
        assert_eq!(a, aa[i]);
        assert_eq!(*raw, bytes(&a, n, p));
        assert_eq!(f["degree"], degree(&a));
        assert_eq!(f["terms"], a.len());
        factor_hashes.push(sha(raw));
    }
    let wc = &cert["witness_circuit"];
    assert_eq!(wc["operation"], "sum_i_Ai_times_Bi_U");
    assert_eq!(wc["B_coefficients_low_to_high"], circuit(t, p));
    assert_eq!(
        wc["U_variable_zero_based"],
        source_check["terminal_intermediate_zero_based"]
    );
    assert_eq!(wc["U_is_unbounded_indeterminate"], true);
    assert_eq!(wc["A_variables"], "normalized last selector block z0..z26");
    assert_eq!(wc["relation"], "g=NF(H*S3(U,X,t))");
    assert_eq!(cert["algebra"]["affine_derivation_degree_upper_bound"], 17);
    let mut c = aa[2].clone();
    scaled_add(&mut c, &aa[1], neg(mul(2, t, p), p), p);
    scaled_add(&mut c, &aa[0], mul(t, t, p), p);
    let actual = pairing(&aa[0], &c, 27, p);
    let sum = weights.iter().fold(0, |s, &w| add(s, w, p));
    let delta = add(sum, neg(mul(2, t, p), p), p);
    let power = (0..13).fold(1, |v, _| mul(v, p - 2, p));
    assert_eq!(actual, mul(power, delta, p));
    let raw = bytes(&c, 27, p);
    (
        json!({"schema":"n83.source-terminal-functional/v1","key":key,"arm":arm,"m":input["m"],
        "target_kind":input["target_kind"],"input_sha256":case["input_sha256"],"source_sha256":case["source_sha256"],
        "factor_base_manifest_sha256":case["factor_base_manifest_sha256"],"terminal_certificate_sha256":case["terminal_certificate_sha256"],
        "top_certificate_sha256":case["top_certificate_sha256"],"factor_sha256":factor_hashes,
        "target_x":t.to_string(),"delta":delta.to_string(),"pairing":actual.to_string(),"pairing_nonzero":actual!=0,
        "coefficient_sha256":sha(&raw),"coefficient_terms":c.len(),"coefficient_degree":degree(&c),
        "full_source_and_factors_verified":true,"filtered_closure_computed":false,"strict_filtered_fall_verified":false,
        "last_fall_lower_bound":null,"last_fall_degree":null,"regularity":null,
        "claim_status":"Exact source-bound arithmetic; filtered implication requires the separate reviewed syzygy argument"}),
        raw,
    )
}
fn hash(path: &Path) -> String {
    sha(&fs::read(path).unwrap())
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    assert_eq!(args.len(), 5, "run|replay ROOT PROTOCOL OUT");
    assert_eq!(
        std::env::var("N83_SOURCE_FUNCTIONAL_FROZEN").as_deref(),
        Ok("1")
    );
    let replay = args[1] == "replay";
    assert!(replay || args[1] == "run");
    let root = Path::new(&args[2]);
    let protocol = read(Path::new(&args[3]));
    let out = Path::new(&args[4]);
    assert_eq!(protocol["freeze"]["status"], "frozen");
    let cases = protocol["extensions"]["cases"].as_array().unwrap();
    check_keys(cases);
    assert_eq!(
        hash(&std::env::current_exe().unwrap()),
        protocol["arms"][0]["instrument"]["binary_sha256"]
    );
    for row in protocol["extensions"]["source_files"].as_array().unwrap() {
        assert_eq!(
            hash(&root.join("source").join(row["path"].as_str().unwrap())),
            row["sha256"]
        );
    }
    if replay {
        assert!(out.is_dir());
    } else {
        fs::create_dir(out).unwrap();
    }
    let put = |name: &str, data: &[u8]| {
        use std::io::Write;
        let path = out.join(name);
        if replay {
            assert_eq!(fs::read(path).unwrap(), data);
        } else {
            let mut f = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(path)
                .unwrap();
            f.write_all(data).unwrap();
        }
    };
    put(
        "primality.json",
        (serde_json::to_string_pretty(&donor::primality()).unwrap() + "\n").as_bytes(),
    );
    let mut rows = vec![];
    for c in cases {
        let key = c["key"].as_str().unwrap();
        let input = root.join("inputs").join(key);
        let terminal = root.join("terminal").join(key);
        let factors: Vec<_> = (0..3)
            .map(|i| fs::read(terminal.join(format!("A{i}.poly"))).unwrap())
            .collect();
        let (record, coefficient) = verify(
            c,
            &fs::read(input.join("input.json")).unwrap(),
            &fs::read_to_string(input.join("system.sing")).unwrap(),
            &fs::read(terminal.join("certificate.json")).unwrap(),
            &fs::read(root.join("top").join(format!("{key}.json"))).unwrap(),
            &factors,
        );
        put(&format!("{key}.poly"), &coefficient);
        put(
            &format!("{key}.json"),
            (serde_json::to_string_pretty(&record).unwrap() + "\n").as_bytes(),
        );
        rows.push(record);
    }
    let summary = json!({"schema":"n83.source-terminal-functional-summary/v1","rows":rows,"new_lfd_claim":false,"independent_review":"pending"});
    put(
        "summary.json",
        (serde_json::to_string_pretty(&summary).unwrap() + "\n").as_bytes(),
    );
    println!(
        "{}",
        json!({"cases":42,"replay":replay,"new_lfd_claim":false})
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    type Fixture = (Value, Vec<u8>, String, Vec<u8>, Vec<u8>, Vec<Vec<u8>>);
    fn jbytes(v: &Value) -> Vec<u8> {
        (serde_json::to_string_pretty(v).unwrap() + "\n").into_bytes()
    }
    fn fixture() -> Fixture {
        let p = donor::field();
        let key = "original-prime-m3-natural";
        let s3 = |x: &str, y: &str, z: &str| {
            format!("(({x})-({y}))^2*({z})^2-2*((({x})+({y}))*(({x})*({y})+2)+2*3)*({z})+(({x})*({y})-2)^2-4*3*(({x})+({y}))")
        };
        let mut source = format!("ring r=(integer,{p}),(v(1..82)),dp;\nideal I;\n");
        for i in 0..3 {
            source += &format!(
                "poly x{i}={};\n",
                (0..27)
                    .map(|j| format!("{}*v({})", 1u128 << j, i * 27 + j + 1))
                    .collect::<Vec<_>>()
                    .join("+")
            );
        }
        source += &format!(
            "I[1]={};\nI[2]={};\n",
            s3("x0", "x1", "v(82)"),
            s3("v(82)", "x2", "7")
        );
        for i in 0..81 {
            source += &format!("I[{}]=v({})^2-v({});\n", 3 + i, i + 1, i + 1);
        }
        let input = json!({"family":"prime","n":83,"ell":27,"m":3,"n_vars":82,"input_equations":83,
            "field_modulus":p.to_string(),"curve_a":"2","curve_b":"3","target_x":"7","target_kind":"natural","witness":null,
            "source_sha256":sha(source.as_bytes()),"factor_base_manifest_sha256":"199d1b214c78650b2e2e2b539f458eae2688296b9bc51985830951b1cac5aaa5"});
        let source_check = source_donor::source(&input, &source, "original");
        let w: Vec<_> = (0..27).map(|j| 1u128 << j).collect();
        let h = h_poly(27, p);
        let a1 = multiply_x(&h, &w, p);
        let a2 = multiply_x(&a1, &w, p);
        let aa = [h, a1, a2];
        let factors: Vec<_> = aa.iter().map(|a| bytes(a, 27, p)).collect();
        let input_bytes = jbytes(&input);
        let mut case = json!({"key":key,"arm":"original","input_sha256":sha(&input_bytes),"source_sha256":input["source_sha256"],"factor_base_manifest_sha256":input["factor_base_manifest_sha256"]});
        let top = json!({"key":key,"arm":"original","m":3,"target_kind":"natural","input_sha256":case["input_sha256"],"source_sha256":case["source_sha256"],"factor_base_manifest_sha256":case["factor_base_manifest_sha256"],"first_fall_lower_bound":17,"first_fall_upper_bound":17,"claim_status":"Conditional on the written Lefschetz/Koszul argument; independent review pending"});
        let cert = json!({"schema":"n83.prime-terminal-certificate/v1","key":key,"input":input,"source":source_check,
            "algebra":{"affine_derivation_degree_upper_bound":17},
            "factor_files":(0..3).map(|i|json!({"path":format!("A{i}.poly"),"bytes":factors[i].len(),"sha256":sha(&factors[i]),"degree":degree(&aa[i]),"terms":aa[i].len()})).collect::<Vec<_>>(),
            "witness_circuit":{"operation":"sum_i_Ai_times_Bi_U","B_coefficients_low_to_high":circuit(7,p),"U_variable_zero_based":81,"U_is_unbounded_indeterminate":true,"A_variables":"normalized last selector block z0..z26","relation":"g=NF(H*S3(U,X,t))"}});
        let cb = jbytes(&cert);
        let tb = jbytes(&top);
        case["terminal_certificate_sha256"] = json!(sha(&cb));
        case["top_certificate_sha256"] = json!(sha(&tb));
        (case, input_bytes, source, cb, tb, factors)
    }
    fn call(f: &Fixture) -> (Value, Vec<u8>) {
        verify(&f.0, &f.1, &f.2, &f.3, &f.4, &f.5)
    }
    #[test]
    fn synthetic_source_roundtrip_and_retained_fixture() {
        let f = fixture();
        let (row, c) = call(&f);
        assert_eq!(row["full_source_and_factors_verified"], true);
        assert!(row["last_fall_lower_bound"].is_null());
        assert_eq!(row["pairing_nonzero"], true);
        if let Ok(dir) = std::env::var("N83_SOURCE_FUNCTIONAL_DEV_OUT") {
            let root = Path::new(&dir);
            fs::create_dir(root.join("fixture")).unwrap();
            let root = root.join("fixture");
            fs::write(root.join("input.json"), f.1).unwrap();
            fs::write(root.join("system.sing"), f.2).unwrap();
            fs::write(root.join("certificate.json"), f.3).unwrap();
            fs::write(root.join("top.json"), f.4).unwrap();
            for (i, a) in f.5.iter().enumerate() {
                fs::write(root.join(format!("A{i}.poly")), a).unwrap();
            }
            fs::write(root.join("coefficient.poly"), c).unwrap();
            save(&root.join("case.json"), &f.0);
            save(
                &root.join("record.json"),
                &json!({"fixture_kind":"synthetic source template; not a saved production case","record":row}),
            );
        }
    }
    #[test]
    fn changed_source_base_and_certificate_hash_rejected() {
        let f = fixture();
        let mut bad = f.clone();
        bad.2.push_str("I[1]=0;\n");
        assert!(std::panic::catch_unwind(|| call(&bad)).is_err());
        let mut bad = f.clone();
        bad.0["factor_base_manifest_sha256"] = json!("wrong");
        assert!(std::panic::catch_unwind(|| call(&bad)).is_err());
        let mut bad = f;
        bad.3.push(b' ');
        assert!(std::panic::catch_unwind(|| call(&bad)).is_err());
    }
    #[test]
    fn top_binding_and_circuit_semantic_mutations_rejected() {
        let f = fixture();
        let mut bad = f.clone();
        let mut top: Value = serde_json::from_slice(&bad.4).unwrap();
        top["input_sha256"] = json!("wrong");
        bad.4 = jbytes(&top);
        bad.0["top_certificate_sha256"] = json!(sha(&bad.4));
        assert!(std::panic::catch_unwind(|| call(&bad)).is_err());
        let mut bad = f;
        let mut cert: Value = serde_json::from_slice(&bad.3).unwrap();
        cert["witness_circuit"]["B_coefficients_low_to_high"][2][2] = json!("2");
        bad.3 = jbytes(&cert);
        bad.0["terminal_certificate_sha256"] = json!(sha(&bad.3));
        assert!(std::panic::catch_unwind(|| call(&bad)).is_err());
    }
    #[test]
    fn forged_factor_metadata_still_fails_recomputation() {
        let mut f = fixture();
        let (n, p, mut a) = parse(&f.5[1]);
        let mask = *a.keys().next().unwrap();
        accum(&mut a, mask, 1, p);
        f.5[1] = bytes(&a, n, p);
        let mut cert: Value = serde_json::from_slice(&f.3).unwrap();
        cert["factor_files"][1]["sha256"] = json!(sha(&f.5[1]));
        cert["factor_files"][1]["bytes"] = json!(f.5[1].len());
        cert["factor_files"][1]["terms"] = json!(a.len());
        cert["factor_files"][1]["degree"] = json!(degree(&a));
        f.3 = jbytes(&cert);
        f.0["terminal_certificate_sha256"] = json!(sha(&f.3));
        assert!(std::panic::catch_unwind(|| call(&f)).is_err());
    }
    #[test]
    fn complete_scope_and_duplicate_rejection() {
        let mut cases: Vec<_> = keys().into_iter().map(|k| json!({"key":k})).collect();
        check_keys(&cases);
        cases[0] = cases[1].clone();
        assert!(std::panic::catch_unwind(|| check_keys(&cases)).is_err());
        cases.pop();
        assert!(std::panic::catch_unwind(|| check_keys(&cases)).is_err());
    }
    #[test]
    fn input_hash_mismatch_rejected_before_parsing() {
        assert!(std::panic::catch_unwind(|| verify(
            &json!({"input_sha256":"wrong"}),
            b"{}",
            "",
            b"{}",
            b"{}",
            &[]
        ))
        .is_err());
    }
}
