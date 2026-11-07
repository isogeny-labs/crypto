//! Deductive secant-graph identity checker and complete-system exporter.
//! No candidate search, relation benchmark, F4 trace, or experimental degree fit.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs::{self, OpenOptions};
use std::io::Write;

// Reuse the repository certificate implementation without the large root crate.
mod hash {
    pub mod sha256 {
        use sha2::{Digest, Sha256};
        pub fn sha256(bytes: &[u8]) -> [u8; 32] {
            Sha256::digest(bytes).into()
        }
    }
}
#[allow(dead_code)]
#[path = "../../../src/cryptanalysis/identity_certificate.rs"]
mod identity;
#[path = "secant_top.rs"]
mod top;
use identity::{Expr as E, IdentityCertificate, IdentityStatement, Verdict};

type ZPoly = BTreeMap<Vec<u32>, i128>;
fn sum(v: Vec<E>) -> E {
    E::sum(v)
}
fn product(v: Vec<E>) -> E {
    E::product(v)
}
fn negative(x: E) -> E {
    E::minus(x)
}
fn square(x: E) -> E {
    product(vec![x.clone(), x])
}
fn difference(x: E, y: E) -> E {
    sum(vec![x, negative(y)])
}

fn statement() -> IdentityStatement {
    let [t, x, z, m, r, s, a, b] = ["T", "x", "z", "m", "r", "s", "A", "B"].map(E::var);
    let line = difference(
        product(vec![m.clone(), difference(t.clone(), r.clone())]),
        s.clone(),
    );
    let f = sum(vec![
        product(vec![t.clone(), t.clone(), t.clone()]),
        product(vec![a.clone(), t.clone()]),
        b.clone(),
    ]);
    let g1 = sum(vec![
        square(m.clone()),
        negative(x.clone()),
        negative(z.clone()),
        negative(r.clone()),
    ]);
    let g2 = sum(vec![
        product(vec![x.clone(), z.clone()]),
        negative(square(r.clone())),
        negative(product(vec![r.clone(), square(m.clone())])),
        negative(a.clone()),
        negative(product(vec![E::Const(2), m, s.clone()])),
    ]);
    let curve = sum(vec![
        square(s),
        negative(product(vec![r.clone(), r.clone(), r.clone()])),
        negative(product(vec![a, r.clone()])),
        negative(b),
    ]);
    IdentityStatement {
        name: "Secant cubic factorization modulo two relation constraints".into(),
        source: "research/prime_factor_base_degree_20261031/SECANT_GRAPH.md, equation (3); direct group-law identity, novelty not claimed".into(),
        variables: ["T", "x", "z", "m", "r", "s", "A", "B"].map(String::from).to_vec(),
        degree_bound: 4,
        lhs: difference(f, square(line)),
        rhs: difference(product(vec![difference(t.clone(), r), sum(vec![product(vec![difference(t.clone(), x), difference(t.clone(), z)]), negative(product(vec![t, g1])), negative(g2)])]), curve),
    }
}

fn plus_z(out: &mut ZPoly, key: Vec<u32>, c: i128) -> Result<(), String> {
    let v = out
        .get(&key)
        .copied()
        .unwrap_or(0)
        .checked_add(c)
        .ok_or("coefficient overflow")?;
    if v == 0 {
        out.remove(&key);
    } else {
        out.insert(key, v);
    }
    Ok(())
}
fn expand(e: &E, vars: &[String]) -> Result<ZPoly, String> {
    match e {
        E::Const(c) => Ok(if *c == 0 {
            ZPoly::new()
        } else {
            BTreeMap::from([(vec![0; vars.len()], i128::from(*c))])
        }),
        E::Var(name) => {
            let i = vars
                .iter()
                .position(|v| v == name)
                .ok_or("undeclared variable")?;
            let mut key = vec![0; vars.len()];
            key[i] = 1;
            Ok(BTreeMap::from([(key, 1)]))
        }
        E::Neg(e) => expand(e, vars)?
            .into_iter()
            .map(|(k, c)| Ok((k, c.checked_neg().ok_or("coefficient overflow")?)))
            .collect(),
        E::Add(terms) => {
            let mut out = ZPoly::new();
            for e in terms {
                for (k, c) in expand(e, vars)? {
                    plus_z(&mut out, k, c)?;
                }
            }
            Ok(out)
        }
        E::Mul(terms) => {
            let mut out: ZPoly = BTreeMap::from([(vec![0; vars.len()], 1i128)]);
            for e in terms {
                let right = expand(e, vars)?;
                let mut next = ZPoly::new();
                for (k, c) in &out {
                    for (l, d) in &right {
                        let key = k
                            .iter()
                            .zip(l)
                            .map(|(i, j)| i.checked_add(*j).ok_or("exponent overflow"))
                            .collect::<Result<Vec<_>, _>>()?;
                        plus_z(
                            &mut next,
                            key,
                            c.checked_mul(*d).ok_or("coefficient overflow")?,
                        )?;
                    }
                }
                out = next;
            }
            Ok(out)
        }
    }
}
fn exact_identity(s: &IdentityStatement) -> Result<(), String> {
    if expand(&s.lhs, &s.variables)? != expand(&s.rhs, &s.variables)? {
        return Err("integer polynomial identity fails".into());
    }
    Ok(())
}
fn verify(cert: &IdentityCertificate) -> Result<Value, Box<dyn std::error::Error>> {
    if cert.statement != statement() {
        return Err("certificate does not contain the registered secant statement".into());
    }
    exact_identity(&cert.statement)?;
    let verdict = identity::check_with_extra(cert, Some((0x5345_4341_4e54, 32)))?;
    if !matches!(verdict, Verdict::Accept { .. }) {
        return Err(format!("certificate rejected: {verdict:?}").into());
    }
    Ok(
        json!({"schema":"secant-graph-identity-check/v1","id":cert.id,"exact_integer_polynomial_equality":true,"certificate_replay":true,"independent_reviewer":false,"formal_polynomial_proof":false}),
    )
}
fn write_once(path: &str, value: &Value) -> Result<(), Box<dyn std::error::Error>> {
    let mut f = OpenOptions::new().write(true).create_new(true).open(path)?;
    f.write_all(&serde_json::to_vec_pretty(value)?)?;
    f.write_all(b"\n")?;
    Ok(())
}
fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn add(a: u64, b: u64) -> u64 {
    (a + b) % 17
}
fn mul(a: u64, b: u64) -> u64 {
    a * b % 17
}
fn eval(h: &[u64], x: u64) -> u64 {
    h.iter().rev().fold(0, |s, c| add(mul(s, x), *c))
}
fn univariate_product(a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut out = vec![0; a.len() + b.len() - 1];
    for (i, c) in a.iter().enumerate() {
        for (j, d) in b.iter().enumerate() {
            out[i + j] = add(out[i + j], mul(*c, *d));
        }
    }
    out
}
fn graph_factorization() -> bool {
    let h = [9, 9, 4];
    let mut lhs = univariate_product(&h, &h);
    for (i, c) in [2, 2, 0, 1].into_iter().enumerate() {
        lhs[i] = add(lhs[i], 17 - c);
    }
    let rhs = [5, 6, 10, 16]
        .into_iter()
        .fold(vec![16], |p, x| univariate_product(&p, &[17 - x, 1]));
    lhs == rhs
}
// All metadata here has ASCII keys/values and integer numbers. serde_json's
// default sorted map encoding therefore matches the ecbench canonical codec.
fn dump(xs: &[u64], h: &[u64], slug: &str, icv1: &str) -> Value {
    let mut points = Vec::new();
    let mut keys = Vec::new();
    for (col, x) in xs.iter().copied().enumerate() {
        let y = eval(h, x).min(17 - eval(h, x));
        for (v, c) in [(y, 1), (17 - y, 18)] {
            points.push(json!({"x":format!("0x{x:x}"),"y":format!("0x{v:x}"),"col":col,"coef":c}));
            keys.push(((x + 1) << 1) | u64::from(v > 17 - v));
        }
    }
    keys.sort_unstable();
    let bytes: Vec<u8> = keys.into_iter().flat_map(u64::to_be_bytes).collect();
    let points_sha = digest(&bytes);
    let params = json!({"xs":xs.iter().map(u64::to_string).collect::<Vec<_>>().join(";")});
    let preimage = json!({"schema":"ecbench.factor_base/v1","curve":slug,"family":"explicit-x","params":params,"columns":xs.len(),"signed_points":points.len(),"points_sha256":points_sha});
    let sha = digest(&serde_json::to_vec(&preimage).expect("ASCII integer metadata"));
    json!({"schema":"ecbench.factor_base_dump/v1","curve":{"slug":slug,"icv1":icv1,"family":"prime","construction":"provided_exact_proof_fixture","field_degree":null,"field_bits":5,"group_order":19,"r":19,"cofactor":1,"generator":["0x5","0x1"],"automorphisms_available":2,"registered":false,"ec1":null,"curve_uid":null},"factor_base":{"fb_id":format!("FB1h{}",&sha[..12]),"fb_sha256":sha,"family":"explicit-x","params":params,"description":"explicit x coordinates; both signs, one column per supplied x","signed_points":points.len(),"abscissae":xs.len(),"columns":xs.len(),"dimension":null,"points_sha256":points_sha},"build_adds":0,"build_doubles":0,"points":points})
}
fn fixture() -> Value {
    let model = json!({"a":"2","b":"2","field":"fp-17","form":"y^2=x^3+a*x+b","p":"17","v":"1"});
    let hash = digest(&serde_json::to_vec(&model).expect("model"));
    let slug = format!("icv1-fp5-tm1-{}", &hash[..8]);
    let icv1 = format!("ICV1:fp-17:-1:19:3:unk:unk:r:{}", &hash[..12]);
    assert!(graph_factorization());
    json!({"schema":"secant-graph-proof-fixture/v1","status":"exact_algebraic_fixture_not_experimental_panel","model":model,"candidate_h_low_to_high":[9,9,4],"control_h_low_to_high":[6,7,6,8],"candidate":dump(&[5,6,10,16],&[9,9,4],&slug,&icv1),"control":dump(&[0,5,6,10],&[6,7,6,8],&slug,&icv1),"derived_homogeneous_degrees":{"candidate_dense":5,"candidate_secant_each_branch":3,"control_dense":5,"control_secant_each_branch":4},"sign_branches":[[1,1],[1,-1],[-1,1],[-1,-1]],"measured_f4_degree":null,"measured_relation_yield":null,"registry_join":"pending_not_claimed"})
}

fn field_polynomial(expr: &E) -> Value {
    let vars = ["x", "z", "m"].map(String::from);
    let terms: Vec<_> = expand(expr, &vars)
        .expect("small fixed polynomial")
        .into_iter()
        .filter_map(|(exponents, c)| {
            let c = c.rem_euclid(17) as u64;
            (c != 0).then(|| json!({"exponents":exponents,"coefficient":c}))
        })
        .collect();
    json!(terms)
}
fn h_expr(h: &[u64], x: E) -> E {
    h.iter().rev().fold(E::Const(0), |out, c| {
        sum(vec![product(vec![out, x.clone()]), E::Const(*c as i64)])
    })
}
fn systems(r: u64, s: u64) -> Result<Value, String> {
    if r >= 17 || s >= 17 || mul(s, s) != add(add(mul(mul(r, r), r), mul(2, r)), 2) {
        return Err("expected an affine point on the fixed proof-fixture curve".into());
    }
    let [x, z, m] = ["x", "z", "m"].map(E::var);
    let g1 = sum(vec![
        square(m.clone()),
        negative(x.clone()),
        negative(z.clone()),
        E::Const(-(r as i64)),
    ]);
    let g2 = sum(vec![
        product(vec![x.clone(), z.clone()]),
        E::Const(-((r * r + 2) as i64)),
        negative(product(vec![E::Const(r as i64), square(m.clone())])),
        negative(product(vec![E::Const(2 * s as i64), m.clone()])),
    ]);
    let mut arms = Vec::new();
    for (name, h, xs, include_membership) in [
        ("split-graph", vec![9, 9, 4], vec![5, 6, 10, 16], false),
        (
            "matched-nongraph",
            vec![6, 7, 6, 8],
            vec![0, 5, 6, 10],
            true,
        ),
    ] {
        let mut branches = Vec::new();
        for (e1, e2) in [(1, 1), (1, -1), (-1, 1), (-1, -1)] {
            let mut generators = vec![field_polynomial(&g1), field_polynomial(&g2)];
            for (v, e) in [(x.clone(), e1), (z.clone(), e2)] {
                let line = sum(vec![
                    product(vec![E::Const(e), h_expr(&h, v.clone())]),
                    negative(product(vec![
                        m.clone(),
                        difference(v.clone(), E::Const(r as i64)),
                    ])),
                    E::Const(s as i64),
                ]);
                generators.push(field_polynomial(&line));
                if include_membership {
                    generators.push(field_polynomial(&product(
                        xs.iter()
                            .map(|u| difference(v.clone(), E::Const(*u)))
                            .collect(),
                    )));
                }
            }
            branches.push(json!({"signs":[e1,e2],"polynomials":generators}));
        }
        arms.push(json!({"name":name,"branches":branches,"membership_omission":if include_membership {"none"}else{"proved redundant by complete split graph factorization"}}));
    }
    Ok(
        json!({"schema":"secant-graph-complete-systems/v1","p":17,"variable_order":["x","z","m"],"target":[r,s],"target_kind":"provided_affine_point","arms":arms,"field_equations":"redundant: all membership roots rational; slope rational on every finite-target relation","infinity_target":"excluded; handle separately by exact inverse pairs","roots_enumerated":false,"f4_measured":false}),
    )
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("seal") if args.len() == 3 => {
            let s = statement();
            exact_identity(&s)?;
            let cert=identity::issue(s,315,32,"same-session issuer and replay; exact integer expansion also checked; independent review pending",None)?;
            verify(&cert)?;
            write_once(&args[2], &serde_json::to_value(&cert)?)?;
            println!("{}", cert.id);
        }
        Some("verify") if args.len() == 3 => {
            let bytes = fs::read(&args[2])?;
            if bytes.len() > 1_048_576 {
                return Err("certificate exceeds 1 MiB".into());
            }
            let cert: IdentityCertificate = serde_json::from_slice(&bytes)?;
            println!("{}", serde_json::to_string_pretty(&verify(&cert)?)?);
        }
        Some("fixture") if args.len() == 3 => {
            write_once(&args[2], &fixture())?;
        }
        Some("systems") if args.len() == 5 => {
            write_once(&args[4], &systems(args[2].parse()?, args[3].parse()?)?)?;
        }
        Some("degrees") if args.len() == 3 => {
            let cert = top::make();
            top::check(&cert)?;
            write_once(&args[2], &serde_json::to_value(cert)?)?;
        }
        Some("check-degrees") if args.len() == 3 => {
            let data = fs::read(&args[2])?;
            if data.len() > 65536 {
                return Err("degree certificate exceeds 64 KiB".into());
            }
            let cert: top::Certificate = serde_json::from_slice(&data)?;
            top::check(&cert)?;
            println!(
                "{}",
                json!({"degree":3,"sign_branches_verified":4,"all_degree_three_monomials_verified":true,"degree_two_dual_verified":true,"metric":"homogeneous_top_ideal","f4_degree":null})
            );
        }
        _ => {
            return Err(
                "usage: secant-graph-certificate seal|verify|fixture PATH | systems R S OUTPUT"
                    .into(),
            )
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn secant_identity_over_integers() {
        exact_identity(&statement()).unwrap();
    }
    #[test]
    fn omitted_curve_constraint_is_detected() {
        let mut s = statement();
        s.rhs = E::plus(s.rhs, E::var("B"));
        assert!(exact_identity(&s).is_err());
    }
    #[test]
    fn fully_split_graph_factorization() {
        assert!(graph_factorization());
        for x in [5, 6, 10, 16] {
            let y = eval(&[9, 9, 4], x);
            assert_eq!(mul(y, y), add(add(mul(mul(x, x), x), mul(2, x)), 2));
        }
    }
    #[test]
    fn certificate_is_bound_and_replayed() {
        let committed: IdentityCertificate =
            serde_json::from_str(include_str!("../secant/identity.json")).unwrap();
        verify(&committed).unwrap();
        let c = identity::issue(statement(), 315, 32, "unit fixture", None).unwrap();
        assert!(verify(&c).is_ok());
        let mut bad = c;
        bad.value_digest = "00".repeat(32);
        assert!(verify(&bad).is_err());
    }
    #[test]
    fn fixture_preserves_signs_columns_and_canonical_hashes() {
        let f = fixture();
        let committed: Value =
            serde_json::from_str(include_str!("../secant/fixture.json")).unwrap();
        assert_eq!(f, committed);
        for arm in ["candidate", "control"] {
            let d = &f[arm];
            assert_eq!(d["points"].as_array().unwrap().len(), 8);
            assert_eq!(d["factor_base"]["columns"], 4);
            assert!(d["factor_base"]["fb_id"]
                .as_str()
                .unwrap()
                .starts_with("FB1h"));
        }
    }
    #[test]
    fn exporter_preserves_every_branch_and_control_membership() {
        let b = systems(10, 6).unwrap();
        for arm in b["arms"].as_array().unwrap() {
            assert_eq!(arm["branches"].as_array().unwrap().len(), 4);
        }
        assert_eq!(
            b["arms"][0]["branches"][0]["polynomials"]
                .as_array()
                .unwrap()
                .len(),
            4
        );
        assert_eq!(
            b["arms"][1]["branches"][0]["polynomials"]
                .as_array()
                .unwrap()
                .len(),
            6
        );
        assert!(systems(0, 0).is_err());
    }
    #[test]
    fn exported_inputs_include_known_secant_and_tangent_roots() {
        for (r, s, x, z, m) in [(10, 6, 5, 6, 2), (6, 3, 5, 5, 13)] {
            let bundle = systems(r, s).unwrap();
            for f in bundle["arms"][0]["branches"][0]["polynomials"]
                .as_array()
                .unwrap()
            {
                let value = f.as_array().unwrap().iter().fold(0, |out, t| {
                    let powers = t["exponents"].as_array().unwrap();
                    let term = [x, z, m]
                        .iter()
                        .zip(powers)
                        .fold(t["coefficient"].as_u64().unwrap(), |a, (v, e)| {
                            (0..e.as_u64().unwrap()).fold(a, |a, _| mul(a, *v))
                        });
                    add(out, term)
                });
                assert_eq!(value, 0);
            }
        }
    }
}
