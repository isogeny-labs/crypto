//! Native mathematical/model binding of the original immutable public fixtures.
#![allow(dead_code)]
include!("fixture.rs");

fn trim(mut a: Vec<u64>) -> Vec<u64> {
    while a.last() == Some(&0) {
        a.pop();
    }
    a
}
fn poly_rem(mut a: Vec<u64>, b: &[u64], p: u64) -> Vec<u64> {
    assert!(!b.is_empty() && b.last() != Some(&0));
    a = trim(a);
    while a.len() >= b.len() {
        let shift = a.len() - b.len();
        let q = mm(*a.last().unwrap(), pow(*b.last().unwrap(), p - 2, p), p);
        for (j, &v) in b.iter().enumerate() {
            a[shift + j] = (a[shift + j] + p - mm(q, v, p)) % p;
        }
        a = trim(a);
    }
    a
}
fn irreducible(f: Field) -> bool {
    // Rabin for degree 4: x^(p^4)=x and gcd(f,x^(p^2)-x)=1.
    let x = [0, 1, 0, 0];
    let x2 = f.pow(x, f.p * f.p);
    if f.pow(x2, f.p * f.p) != x {
        return false;
    }
    let mut a = vec![neg(f.d, f.p), 0, 0, 0, 1];
    let mut b = trim(f.sub(x2, x).to_vec());
    while !b.is_empty() {
        let r = poly_rem(a, &b, f.p);
        a = b;
        b = r;
    }
    a.len() == 1
}
fn whole_order(p: u64, n: u64) -> u64 {
    let q = p.checked_pow(4).unwrap();
    let radius = 2 * p * p;
    let lo = q + 1 - radius;
    let hi = q + 1 + radius;
    let order = lo.div_ceil(n).checked_mul(n).unwrap();
    assert!(order >= lo && order <= hi, "no Hasse multiple");
    assert!(
        order.checked_add(n).unwrap() > hi,
        "ambiguous Hasse multiple"
    );
    order
}
fn canonical_bytes(value: &Value) -> Vec<u8> {
    fn emit(v: &Value, out: &mut String) {
        match v {
            Value::Object(m) => {
                out.push('{');
                let mut keys: Vec<_> = m.keys().collect();
                keys.sort();
                for (i, k) in keys.into_iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    out.push_str(&serde_json::to_string(k).unwrap());
                    out.push(':');
                    emit(&m[k], out);
                }
                out.push('}');
            }
            Value::Array(a) => {
                out.push('[');
                for (i, x) in a.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    emit(x, out);
                }
                out.push(']');
            }
            Value::Number(n) => {
                assert!(n.is_u64() || n.is_i64(), "no floating identities");
                out.push_str(&n.to_string());
            }
            _ => out.push_str(&serde_json::to_string(v).unwrap()),
        }
    }
    let mut out = String::new();
    emit(value, &mut out);
    out.into_bytes()
}
fn ec1(field: &Value, curve: &Value, p: u64) -> Value {
    let sha = digest(&canonical_bytes(&json!({"field":field,"curve":curve})));
    json!({"curve_id":format!("EC1Q{p}D4Cfpkh{}",&sha[..12]),"curve_sha256":sha,"curve_uid":format!("urn:ec-record:1:sha256:{sha}"),"field_sha256":digest(&canonical_bytes(field))})
}
fn bind(row: &Value) -> Value {
    let raw = load(Path::new(s(&row["public_path"])), s(&row["public_sha256"]));
    let case = Case::new(&raw);
    let f = case.c.f;
    assert!((2..=f.p.isqrt()).all(|d| f.p % d != 0));
    assert!(irreducible(f));
    assert!(case.c.b != [0; 4] && case.g != O && case.c.on(case.g));
    assert_eq!(case.c.mul(case.g, case.n), O);
    // Case::new proves n prime. Thus the nonzero G has exact order n.
    // Lagrange and the now-verified field/nonsingular model put #E at the
    // unique multiple of n in the complete Hasse interval.
    let order = whole_order(f.p, case.n);
    let modulus = [neg(f.d, f.p), 0, 0, 0];
    let id = crypto_lib::cryptanalysis::curve_id::extension(
        f.p,
        &modulus,
        &[0; 4],
        &case.c.b,
        &order.into(),
    )
    .unwrap();
    let model: Value = serde_json::from_str(&id.model_json).unwrap();
    assert_eq!(canonical_bytes(&model), id.model_json.as_bytes());
    let model_sha = digest(id.model_json.as_bytes());
    assert!(id.icv1.ends_with(&model_sha[..12]));
    assert!(id.slug.ends_with(&model_sha[..8]));
    let field = json!({"characteristic":f.p,"degree":4,"representation":"polynomial","modulus":[modulus[0],0,0,0,1],"element_encoding":"decimal coefficient list [e_0, ..., e_{k-1}] of e_0 + e_1 t + ... + e_{k-1} t^{k-1} modulo the monic modulus"});
    let zero = [0u64; 4];
    let curve = json!({"model":"short Weierstrass","coefficients":[zero,zero,zero,zero,case.c.b],"subgroup_order":case.n.to_string(),"cofactor":order/case.n,"generator":[case.g.x,case.g.y],"target_group":"prime-order subgroup"});
    let representation = ec1(&field, &curve, f.p);
    let parts: Vec<_> = id.icv1.split(':').collect();
    assert_eq!(parts.len(), 9);
    let model_row = json!({"slug":id.slug,"icv1":id.icv1,"family":"extension","model_json":id.model_json,"model_sha256":model_sha,"trace":parts[2].parse::<i64>().unwrap(),"order":order.to_string(),"j":parts[4],"end":"unk","params":{"p":f.p.to_string(),"k":4,"a":["0","0","0","0"],"b":case.c.b.map(|x|x.to_string()),"modulus":modulus.map(|x|x.to_string())}});
    json!({"p":raw["p"],"seed":raw["seed"],"public_path":row["public_path"],"public_sha256":row["public_sha256"],"model":model_row,"field":field,"curve":curve,"identity":representation,"checks":{"prime_characteristic":true,"rabin_irreducible_degree4":true,"nonsingular":true,"prime_subgroup_order":true,"generator_nonzero_on_curve":true,"n_times_generator_zero":true,"unique_hasse_multiple":order,"hasse_interval":[f.p.pow(4)+1-2*f.p*f.p,f.p.pow(4)+1+2*f.p*f.p],"whole_curve_order_is_not_assumed_from_subgroup":true,"native_frobenius_cubic_and_image_actions":true}})
}
fn registry(workload: &Path, out: &Path) {
    assert!(!out.exists());
    let catalog = load(workload, WORKLOAD_SHA);
    assert_eq!(catalog["case_arms"], 42);
    let mut seen = BTreeSet::new();
    let mut records = Vec::new();
    let mut models = BTreeMap::<String, Value>::new();
    let mut representations = BTreeMap::<String, BTreeMap<String, Value>>::new();
    for row in catalog["workloads"].as_array().unwrap() {
        if !seen.insert((u(&row["p"]), u(&row["seed"]))) {
            continue;
        }
        let r = bind(row);
        let key = s(&r["model"]["model_sha256"]).to_string();
        if let Some(old) = models.insert(key.clone(), r["model"].clone()) {
            assert_eq!(old, r["model"]);
        }
        let rep = json!({"ec1":r["identity"]["curve_id"],"curve_uid":r["identity"]["curve_uid"],"field_sha256":r["identity"]["field_sha256"],"field":r["field"],"curve":r["curve"],"sources":[r["public_path"]]});
        representations
            .entry(key)
            .or_default()
            .insert(s(&r["identity"]["curve_uid"]).to_string(), rep);
        records.push(r);
    }
    let expected: BTreeSet<_> = PRIMES
        .iter()
        .flat_map(|&p| (0..3).map(move |seed| (p, seed)))
        .collect();
    assert_eq!(seen, expected);
    let curves: Vec<_> = models
        .into_iter()
        .map(|(key, mut row)| {
            row["representations"] = json!(representations
                .remove(&key)
                .unwrap()
                .into_values()
                .collect::<Vec<_>>());
            row
        })
        .collect();
    std::fs::create_dir_all(out).unwrap();
    write(
        &out.join("catalog.json"),
        &json!({"schema":"ecbench.native_quartic_registry/v1","workload_sha256":WORKLOAD_SHA,"public_fixture_count":records.len(),"models":curves.len(),"curves":curves,"fixtures":records,"status":"native-checked-pending-registry-import","scope":"Representation binding of existing public fixtures; no new relation collection, target recovery, complete price or endomorphism-ring certification."}),
    );
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    assert_eq!(args.len(), 4);
    assert_eq!(args[1], "bind");
    registry(Path::new(&args[2]), Path::new(&args[3]));
}

#[cfg(test)]
mod binding_tests {
    use super::*;
    #[test]
    fn rabin_rejects_square_and_accepts_frozen_field() {
        assert!(irreducible(Field { p: 37, d: 2 }));
        assert!(!irreducible(Field { p: 37, d: 4 }));
    }
    #[test]
    fn unique_hasse_order_and_ambiguous_control() {
        assert_eq!(whole_order(37, 1_873_633), 1_873_633);
        assert!(std::panic::catch_unwind(|| whole_order(37, 7)).is_err());
    }
    #[test]
    fn canonical_json_uses_sorted_keys_and_rejects_floats() {
        assert_eq!(
            canonical_bytes(&json!({"z":[2,1],"a":"v"})),
            br#"{"a":"v","z":[2,1]}"#
        );
        assert!(std::panic::catch_unwind(|| canonical_bytes(&json!(1.5))).is_err());
    }
    #[test]
    fn representation_binds_generator_and_field() {
        let field = json!({"characteristic":37,"degree":4,"representation":"polynomial","modulus":[35,0,0,0,1],"element_encoding":"test"});
        let curve = json!({"model":"short Weierstrass","subgroup_order":"1873633","cofactor":1,"generator":[[1,0,0,0],[36,11,4,6]],"target_group":"prime-order subgroup"});
        let a = ec1(&field, &curve, 37);
        let mut b = curve.clone();
        b["generator"][0][0] = json!(2);
        assert_ne!(a, ec1(&field, &b, 37));
        let mut f = field.clone();
        f["modulus"][0] = json!(33);
        assert_ne!(a, ec1(&f, &curve, 37));
    }
}
