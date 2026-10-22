//! Reuses the repository's native ICV1 and IDC1 implementations without
//! running the historical Python registry builder or experiment harness.
#![allow(dead_code)]
#[path = "../../../../src/binary_ecc/f2m.rs"]
mod field_backend;
pub mod binary_ecc {
    pub use super::field_backend::IrreduciblePoly;
}
#[path = "../../../../src/hash/sha256.rs"]
pub mod sha_backend;
pub mod hash {
    pub use crate::sha_backend as sha256;
}
#[path = "../../../../src/cryptanalysis/curve_id.rs"]
mod curve_id;
#[path = "../../../../src/cryptanalysis/identity_certificate.rs"]
mod identity_certificate;

use identity_certificate::{Expr as E, IdentityStatement, ProofRef, Verdict};
use num_bigint::BigUint;
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};
const STUDY: &str = "research/quadratic_twist_evidence_20261021";
const SOURCE: &str = "docs/curves/sources/quadratic-twist-native-20261021.json";
fn sha(bytes: &[u8]) -> String {
    hex::encode(sha_backend::sha256(bytes))
}
fn read_json(p: &Path) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn write_json(p: &Path, v: &Value) {
    fs::write(p, serde_json::to_string_pretty(v).unwrap() + "\n").unwrap();
}
fn u(v: &Value, k: &str) -> u64 {
    v[k].as_u64().unwrap()
}
fn var(s: &str) -> E {
    E::var(s)
}
fn c(n: i64) -> E {
    E::Const(n)
}
fn add(xs: Vec<E>) -> E {
    E::sum(xs)
}
fn mul(xs: Vec<E>) -> E {
    E::product(xs)
}
fn sub(a: E, b: E) -> E {
    add(vec![a, E::minus(b)])
}
fn pow(a: E, n: usize) -> E {
    mul(vec![a; n])
}
fn certificates(root: &Path, out: &Path) {
    let proof_path = format!("{STUDY}/lean/TwistProof.lean");
    let proof = ProofRef {
        kind: "lean4".into(),
        path: proof_path.clone(),
        sha256: sha(&fs::read(root.join(&proof_path)).unwrap()),
    };
    let (x, y, z, a, b, d, m, n, uu) = (
        var("x"),
        var("y"),
        var("z"),
        var("a"),
        var("b"),
        var("d"),
        var("m"),
        var("n"),
        var("u"),
    );
    let xy = mul(vec![x.clone(), y.clone()]);
    let s1 = add(vec![x.clone(), y.clone(), z.clone()]);
    let s2 = add(vec![
        xy.clone(),
        mul(vec![x.clone(), z.clone()]),
        mul(vec![y.clone(), z.clone()]),
    ]);
    let s3 = mul(vec![x.clone(), y.clone(), z.clone()]);
    let canonical = add(vec![
        mul(vec![pow(sub(x.clone(), y.clone()), 2), pow(z.clone(), 2)]),
        E::minus(mul(vec![
            c(2),
            add(vec![
                mul(vec![
                    add(vec![x.clone(), y.clone()]),
                    add(vec![xy.clone(), a.clone()]),
                ]),
                mul(vec![c(2), b.clone()]),
            ]),
            z.clone(),
        ])),
        pow(sub(xy, a.clone()), 2),
        E::minus(mul(vec![c(4), b.clone(), add(vec![x.clone(), y.clone()])])),
    ]);
    let symmetric = sub(
        pow(sub(s2, a.clone()), 2),
        mul(vec![c(4), s1, add(vec![s3, b.clone()])]),
    );
    let vieta = sub(
        pow(
            sub(
                sub(a.clone(), mul(vec![c(2), d.clone(), m.clone(), n.clone()])),
                a.clone(),
            ),
            2,
        ),
        mul(vec![
            c(4),
            d.clone(),
            pow(m, 2),
            add(vec![
                mul(vec![d.clone(), pow(n, 2)]),
                E::minus(b.clone()),
                b.clone(),
            ]),
        ]),
    );
    let f = add(vec![pow(x, 3), mul(vec![a, var("x")]), b]);
    let twist_left = sub(
        mul(vec![pow(uu.clone(), 2), f.clone()]),
        mul(vec![d.clone(), f.clone()]),
    );
    let twist_right = mul(vec![sub(pow(uu, 2), d), f]);
    let statements = vec![
        (
            "symmetric-s3",
            "symmetric_s3",
            vec!["x", "y", "z", "a", "b"],
            4,
            canonical,
            symmetric,
        ),
        (
            "vieta-cancellation",
            "vieta_twist_cancellation",
            vec!["a", "b", "d", "m", "n"],
            6,
            vieta,
            c(0),
        ),
        (
            "twist-equation",
            "twist_equation_identity",
            vec!["u", "d", "x", "a", "b"],
            5,
            twist_left,
            twist_right,
        ),
    ];
    let mut ids = vec![];
    for (name, decl, vars, degree, lhs, rhs) in statements {
        let statement=IdentityStatement{name:name.into(),source:format!("{proof_path}:TwistProof.{decl}; exact Z coefficient cancellation also checked by native/full.cpp"),variables:vars.into_iter().map(String::from).collect(),degree_bound:degree,lhs,rhs};
        let cert=identity_certificate::issue(statement.clone(),20261021,32,"Issued and checked with the repository native backend; extra checker seed 2381, 16 points. Same-host checks, not an independent-host claim.",Some(proof.clone())).unwrap();
        let path = out.join(format!("{name}.json"));
        fs::write(&path, serde_json::to_string_pretty(&cert).unwrap() + "\n").unwrap();
        let replay = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        assert!(matches!(
            identity_certificate::check_with_extra(&replay, Some((2381, 16))).unwrap(),
            Verdict::Accept {
                points: 32,
                extra_points: 16,
                ..
            }
        ));
        let mut false_statement = statement;
        false_statement.rhs = E::plus(false_statement.rhs, c(1));
        assert!(identity_certificate::issue(
            false_statement,
            20261021,
            32,
            "negative control",
            None
        )
        .is_err());
        ids.push(json!({"id":cert.id,"file":path.file_name().unwrap().to_str().unwrap(),"lean_declaration":format!("TwistProof.{decl}"),"native_check":"passed","false_plus_one_control":"rejected"}));
    }
    write_json(
        &out.join("identity-index.json"),
        &json!({"schema":"quadratic-twist-identities/v1","proof_sha256":proof.sha256,"certificates":ids}),
    );
}

struct Catalog {
    rows: BTreeMap<String, Value>,
    links: Vec<Value>,
}
impl Catalog {
    fn new() -> Self {
        Self {
            rows: BTreeMap::new(),
            links: vec![],
        }
    }
    fn record(&mut self, id: curve_id::CurveId, params: Value, family: &str) -> String {
        let slug = id.slug.clone();
        let parts: Vec<_> = id.icv1.split(':').collect();
        let row = json!({"slug":slug,"icv1":id.icv1,"family":family,"params":params,"model_json":id.model_json,"trace":parts[2].parse::<i64>().unwrap(),"order":parts[3],"j":parts[4],"end":parts[5],"aliases":[],"standard_names":[],"sources":[SOURCE],"representations":[],"ec1_unresolved":"Full-group census: no cyclic subgroup and generator selected for this model."});
        if let Some(previous) = self.rows.get(&slug) {
            assert_eq!(previous["icv1"], row["icv1"], "model identity collision");
            assert_eq!(
                previous["model_json"], row["model_json"],
                "full model collision"
            );
        } else {
            self.rows.insert(slug.clone(), row);
        }
        slug
    }
    fn prime(&mut self, p: u64, a: u64, b: u64, n: u64) -> String {
        self.record(
            curve_id::prime(
                &BigUint::from(p),
                &BigUint::from(a),
                &BigUint::from(b),
                &BigUint::from(n),
            )
            .unwrap(),
            json!({"p":p.to_string(),"a":a.to_string(),"b":b.to_string()}),
            "prime",
        )
    }
    fn extension(&mut self, p: u64, modulus: Vec<u64>, a: Vec<u64>, b: Vec<u64>, n: u64) -> String {
        let id = curve_id::extension(p, &modulus, &a, &b, &BigUint::from(n)).unwrap();
        self.record(
            id,
            json!({"p":p.to_string(),"k":modulus.len(),"modulus":modulus,"a":a,"b":b}),
            "extension",
        )
    }
    fn pair(&mut self, p: u64, a: u64, b: u64, d: u64, n: u64, nt: u64, label: Value) {
        let original = self.prime(p, a, b, n);
        let twist = self.prime(p, a * d * d % p, b * d * d % p * d % p, nt);
        let extension = self.extension(p, vec![p - d, 0], vec![a, 0], vec![b, 0], n * nt);
        let twist_extension = self.extension(
            p,
            vec![p - d, 0],
            vec![a * d * d % p, 0],
            vec![b * d * d % p * d % p, 0],
            n * nt,
        );
        self.links.push(json!({"parameters":label,"original":original,"twist_normalized":twist,"extension":extension,"twist_extension_normalized":twist_extension,"normalization":"(x,y) -> (d*x,d^2*y) from d*y^2=x^3+a*x+b","x_fixing_extension_map":"(x,y) -> (x,u*y), u^2=d; applies before monic normalization","transport_status":"native finite-domain checks; not a base-field DLP transfer"}));
    }
    fn representation(&mut self, slug: &str, field: Value, curve: Value, p: u64, k: usize) {
        let preimage = json!({"field":field,"curve":curve});
        let full = sha(&serde_json::to_vec(&preimage).unwrap());
        let field_sha = sha(&serde_json::to_vec(&field).unwrap());
        let prefix = if k == 1 {
            format!("P{}", 64 - p.leading_zeros())
        } else {
            format!("Q{p}D{k}")
        };
        let tag = if k == 1 { "fp" } else { "fpk" };
        let rep = json!({"ec1":format!("EC1{prefix}C{tag}h{}",&full[..12]),"curve_uid":format!("urn:ec-record:1:sha256:{full}"),"field_sha256":field_sha,"field":field,"curve":curve,"sources":[SOURCE]});
        let row = self.rows.get_mut(slug).unwrap();
        row.as_object_mut().unwrap().remove("ec1_unresolved");
        let rs = row["representations"].as_array_mut().unwrap();
        if !rs.contains(&rep) {
            rs.push(rep);
        }
    }
}
fn nonsquare(p: u64) -> u64 {
    (2..p)
        .find(|d| modpow(*d, (p - 1) / 2, p) == p - 1)
        .unwrap()
}
fn modpow(mut a: u64, mut n: u64, p: u64) -> u64 {
    let mut r = 1;
    while n > 0 {
        if n % 2 == 1 {
            r = r * a % p;
        }
        a = a * a % p;
        n /= 2;
    }
    r
}
fn prime_rep(p: u64, a: u64, b: u64, r: u64, h: u64, g: [u64; 2]) -> (Value, Value) {
    (
        json!({"characteristic":p,"degree":1,"representation":"prime","element_encoding":"hex integer modulo the characteristic"}),
        json!({"model":"short Weierstrass","coefficients":[0,0,0,a,b],"subgroup_order":r.to_string(),"cofactor":h,"generator":[format!("0x{:x}",g[0]),format!("0x{:x}",g[1])],"target_group":"prime-order subgroup"}),
    )
}
fn ext_rep(
    p: u64,
    modulus: &[u64],
    a: Value,
    b: Value,
    r: u64,
    h: u64,
    g: Value,
) -> (Value, Value) {
    let k = modulus.len();
    let mut monic = modulus.to_vec();
    monic.push(1);
    let zero = vec![0u64; k];
    (
        json!({"characteristic":p,"degree":k,"representation":"polynomial","modulus":monic,"element_encoding":"decimal coefficient list [e_0, ..., e_{k-1}] of e_0 + e_1 t + ... + e_{k-1} t^{k-1} modulo the monic modulus"}),
        json!({"model":"short Weierstrass","coefficients":[zero,zero,zero,a,b],"subgroup_order":r.to_string(),"cofactor":h,"generator":g,"target_group":"prime-order subgroup"}),
    )
}
fn element(s: &str) -> Vec<u64> {
    if let Some(s) = s.trim().strip_suffix('u') {
        let (a, b) = s.split_once('+').unwrap();
        vec![a.parse().unwrap(), b.parse().unwrap()]
    } else {
        vec![s.trim().parse().unwrap(), 0]
    }
}
fn point(s: &str) -> Vec<Vec<u64>> {
    let s = s
        .trim()
        .strip_prefix('(')
        .unwrap()
        .strip_suffix(')')
        .unwrap();
    let (x, y) = s.split_once(',').unwrap();
    vec![element(x), element(y)]
}
fn catalog(v: &Value, basis: &Value) -> Catalog {
    assert_eq!(basis["status"], "passed");
    assert_eq!(basis["field_elements_bijected"], 2401);
    assert_eq!(basis["point_equations_checked"], 2475);
    let mut c = Catalog::new();
    for m in v["exhaustive"]["census_models"].as_array().unwrap() {
        c.pair(
            u(m, "p"),
            u(m, "a"),
            u(m, "b"),
            u(m, "d"),
            u(m, "curve_order"),
            u(m, "twist_order"),
            json!({"census":[m["p"],m["a"],m["b"]]}),
        );
    }
    for p in v["pairings"].as_array().unwrap() {
        let (q, r, d) = (u(p, "p"), u(p, "r"), u(p, "d"));
        c.pair(q, 1, 0, d, q + 1, q + 1, json!({"pairing":[q,r]}));
        let g = point(p["P"].as_str().unwrap());
        let slug = c.prime(q, 1, 0, q + 1);
        let (f, cv) = prime_rep(q, 1, 0, r, (q + 1) / r, [g[0][0], g[1][0]]);
        c.representation(&slug, f, cv, q, 1);
        let slug = c.extension(q, vec![q - d, 0], vec![1, 0], vec![0, 0], (q + 1) * (q + 1));
        let (f, cv) = ext_rep(
            q,
            &[q - d, 0],
            json!([1, 0]),
            json!([0, 0]),
            r,
            (q + 1) * (q + 1) / r,
            json!(g),
        );
        c.representation(&slug, f, cv, q, 2);
        let s = point(p["S"].as_str().unwrap());
        let s0 = point(p["S0"].as_str().unwrap());
        let twist_slug = c.prime(q, d * d % q, 0, q + 1);
        let (tf, tc) = prime_rep(
            q,
            d * d % q,
            0,
            r,
            (q + 1) / r,
            [d * s0[0][0] % q, d * d % q * s0[1][0] % q],
        );
        c.representation(&twist_slug, tf, tc, q, 1);
        let (f, cv) = ext_rep(
            q,
            &[q - d, 0],
            json!([1, 0]),
            json!([0, 0]),
            r,
            (q + 1) * (q + 1) / r,
            json!(s),
        );
        c.representation(&slug, f, cv, q, 2);
    }
    for key in ["smooth_twist", "anomalous_twist"] {
        let m = &v["no_transfer"][key];
        let p = u(m, "p");
        c.pair(
            p,
            u(m, "a"),
            u(m, "b"),
            nonsquare(p),
            u(m, "curve_order"),
            u(m, "twist_order"),
            json!({"no_transfer":key}),
        );
    }
    let m = &v["no_transfer"]["smooth_twist"];
    let p = u(m, "p");
    let slug = c.prime(p, u(m, "a"), u(m, "b"), 1019);
    let g = point(m["P"].as_str().unwrap());
    let (f, cv) = prime_rep(p, u(m, "a"), u(m, "b"), 1019, 1, [g[0][0], g[1][0]]);
    c.representation(&slug, f, cv, p, 1);
    for m in v["transported_relation_collection"].as_array().unwrap() {
        let p = u(m, "p");
        let n = u(m, "r");
        let d = nonsquare(p);
        let a = u(m, "a");
        let b = u(m, "b");
        let nt = 2 * (p + 1) - n;
        c.pair(p, a, b, d, n, nt, json!({"relation_collection":p}));
        let g = point(m["generator"].as_str().unwrap());
        let slug = c.prime(p, a, b, n);
        let (f, cv) = prime_rep(p, a, b, n, 1, [g[0][0], g[1][0]]);
        c.representation(&slug, f, cv, p, 1);
        let slug = c.extension(p, vec![p - d, 0], vec![a, 0], vec![b, 0], n * nt);
        let (f, cv) = ext_rep(
            p,
            &[p - d, 0],
            json!([a, 0]),
            json!([b, 0]),
            n,
            nt,
            json!(g),
        );
        c.representation(&slug, f, cv, p, 2);
        let aa = a * d * d % p;
        let bb = b * d * d % p * d % p;
        let tg = point(m["transported_generator_normalized"].as_str().unwrap());
        let slug = c.extension(p, vec![p - d, 0], vec![aa, 0], vec![bb, 0], n * nt);
        let (f, cv) = ext_rep(
            p,
            &[p - d, 0],
            json!([aa, 0]),
            json!([bb, 0]),
            n,
            nt,
            json!(tg),
        );
        c.representation(&slug, f, cv, p, 2);
    }
    for m in v["trace_zero"].as_array().unwrap() {
        let p = u(m, "p");
        let modulus = m["modulus"].as_array().unwrap();
        let poly: Vec<u64> = modulus[..modulus.len() - 1]
            .iter()
            .map(|x| x.as_u64().unwrap())
            .collect();
        let k = poly.len();
        let mut a = vec![0; k];
        a[0] = 1;
        c.extension(p, poly, a.clone(), a, u(m, "extension_order"));
        c.prime(p, 1, 1, u(m, "base_order"));
    }
    let source = c.extension(7, vec![4, 0], vec![4, 2], vec![3, 6], 45);
    let (f, cv) = ext_rep(
        7,
        &[4, 0],
        json!([4, 2]),
        json!([3, 6]),
        3,
        15,
        json!(point(
            v["subfield_descent"]["source_point"].as_str().unwrap()
        )),
    );
    c.representation(&source, f, cv, 7, 2);
    let target = c.extension(
        7,
        vec![5, 0, 5, 0],
        vec![1, 0, 0, 0],
        vec![1, 0, 0, 0],
        2475,
    );
    let (f, cv) = ext_rep(
        7,
        &[5, 0, 5, 0],
        json!([1, 0, 0, 0]),
        json!([1, 0, 0, 0]),
        3,
        825,
        basis["mapped_generator"].clone(),
    );
    c.representation(&target, f, cv, 7, 4);
    c
}
fn registry_row_text(row: &Value) -> String {
    // The metadata join uses one-space indentation at each nesting level.
    // Keep the existing registry prefix intact while matching that layout for
    // newly appended rows. Arbitrary-precision JSON numbers stay exact.
    let mut bytes = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b" ");
    let mut serializer = serde_json::Serializer::with_formatter(&mut bytes, formatter);
    serde::Serialize::serialize(row, &mut serializer).unwrap();
    String::from_utf8(bytes)
        .unwrap()
        .lines()
        .map(|line| format!("  {line}"))
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn appended_registry_row_matches_metadata_layout_without_rounding() {
    let row: Value = serde_json::from_str(
        r#"{"aliases":[],"params":{"p":"7"},"trace":24589614856193402413}"#,
    )
    .unwrap();
    let rendered = registry_row_text(&row);
    assert_eq!(
        rendered,
        "  {\n   \"aliases\": [],\n   \"params\": {\n    \"p\": \"7\"\n   },\n   \"trace\": 24589614856193402413\n  }"
    );
    assert_eq!(serde_json::from_str::<Value>(&rendered).unwrap(), row);
}

fn append_registry(root: &Path, c: &Catalog, out: &Path, write: bool) {
    let path = root.join("docs/curves/registry.json");
    let text = fs::read_to_string(&path).unwrap();
    let doc: Value = serde_json::from_str(&text).unwrap();
    let old = doc["curves"].as_array().unwrap();
    let oldmap: BTreeMap<_, _> = old
        .iter()
        .map(|r| (r["slug"].as_str().unwrap(), r))
        .collect();
    let mut added = vec![];
    for (slug, row) in &c.rows {
        if let Some(existing) = oldmap.get(slug.as_str()) {
            assert_eq!(
                existing["icv1"], row["icv1"],
                "existing registry disagrees with native counted model"
            );
            assert_eq!(
                existing["model_json"], row["model_json"],
                "registered model bytes differ"
            );
            assert_eq!(existing["order"], row["order"], "registered order differs");
            for rep in row["representations"].as_array().unwrap() {
                assert!(
                    existing["representations"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .any(|held| held["curve_uid"] == rep["curve_uid"]
                            && held["field"] == rep["field"]
                            && held["curve"] == rep["curve"]),
                    "registered subgroup representation differs"
                );
            }
        } else {
            added.push(row.clone());
        }
    }
    let idx = text.rfind("\n ]").expect("registry layout changed");
    let mut updated = text[..idx].to_string();
    for row in &added {
        updated += ",\n";
        updated += &registry_row_text(row);
    }
    updated += &text[idx..];
    let checked: Value = serde_json::from_str(&updated).unwrap();
    assert_eq!(
        checked["curves"].as_array().unwrap().len(),
        old.len() + added.len()
    );
    let ap = root.join("src/cryptanalysis/curve_aliases.json");
    let at = fs::read_to_string(&ap).unwrap();
    let av: Value = serde_json::from_str(&at).unwrap();
    let mut aliases = BTreeMap::new();
    for (slug, row) in &c.rows {
        for key in [
            slug.to_lowercase(),
            row["icv1"].as_str().unwrap().to_lowercase(),
        ] {
            if let Some(known) = av["aliases"].get(&key) {
                assert_eq!(known.as_str().unwrap(), slug);
            } else {
                aliases.insert(key, slug.clone());
            }
        }
    }
    let ai = at.rfind("\n}\n}").expect("aliases layout changed");
    let mut au = at[..ai].to_string();
    for (k, v) in &aliases {
        au += ",\n";
        au += &serde_json::to_string(k).unwrap();
        au += ": ";
        au += &serde_json::to_string(v).unwrap();
    }
    au += &at[ai..];
    let _: Value = serde_json::from_str(&au).unwrap();
    write_json(
        &out.join("registry-update.json"),
        &json!({"schema":"quadratic-twist-registry-update/v1","models_counted":c.rows.len(),"new_registry_rows":added.len(),"new_alias_keys":aliases.len(),"base_registry_sha256":sha(text.as_bytes()),"updated_registry_sha256":sha(updated.as_bytes()),"base_aliases_sha256":sha(at.as_bytes()),"updated_aliases_sha256":sha(au.as_bytes()),"existing_rows_preserved":true}),
    );
    if write {
        fs::write(path, updated).unwrap();
        fs::write(ap, au).unwrap();
    }
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    assert!(
        args.len() == 4 || args.len() == 5,
        "metadata REPOSITORY NATIVE_RESULTS FRESH_OUTPUT [--write-registry]"
    );
    let root = PathBuf::from(&args[1]);
    let out = PathBuf::from(&args[3]);
    assert!(!out.exists(), "refusing to overwrite metadata output");
    let input_hash = sha(&fs::read(&args[2]).unwrap());
    let run_dir = Path::new(&args[2]).parent().unwrap();
    let parity = read_json(&run_dir.join("parity.json"));
    assert_eq!(parity["status"], "passed");
    assert_eq!(
        parity["native_results_sha256"], input_hash,
        "native result source binding"
    );
    assert_eq!(parity["full_942_model_census_rerun"], true);
    let audit = read_json(&run_dir.join("independent-ic-audit.json"));
    assert_eq!(audit["status"], "passed");
    assert_eq!(
        audit["certificate_input_sha256"], input_hash,
        "independent audit source binding"
    );
    assert_eq!(audit["additional_native_ic_certificates_verified"], 81);
    let v = read_json(Path::new(&args[2]));
    let basis = read_json(&Path::new(&args[2]).parent().unwrap().join("basis.json"));
    let c = catalog(&v, &basis);
    fs::create_dir_all(&out).unwrap();
    certificates(&root, &out);
    let rows: Vec<_> = c.rows.values().cloned().collect();
    let document = json!({"schema":"quadratic-twist-native-models/v1","native_results_sha256":sha(&fs::read(&args[2]).unwrap()),"basis_certificate_sha256":sha(&fs::read(Path::new(&args[2]).parent().unwrap().join("basis.json")).unwrap()),"models":rows,"links":c.links,"subfield_tower_basis":"u^2=3, v^2=1+u; w=v gives w^4+5*w^2+5=0; u=w^2-1. Coordinate conversion, points, and group law checked in basis.json."});
    write_json(&out.join("curve-models.json"), &document);
    append_registry(
        &root,
        &c,
        &out,
        args.get(4).is_some_and(|s| s == "--write-registry"),
    );
    println!("PASS: 3 IDC1 certificates, extra-point checks and false-identity controls; {} exact curve models",c.rows.len());
}
