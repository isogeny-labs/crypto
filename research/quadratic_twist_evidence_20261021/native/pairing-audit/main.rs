//! Independent coefficient-pair arithmetic and LINEAR Miller chain.
//! C++ uses the separate packed-field backend and BINARY Miller chain.
#![allow(dead_code)]
#[path = "../../../../src/hash/sha256.rs"]
pub mod sha_backend;
pub mod hash {
    pub use crate::sha_backend as sha256;
}
#[path = "../../../../src/cryptanalysis/identity_certificate.rs"]
mod identity_certificate;
use identity_certificate::{Expr as E, IdentityCertificate, IdentityStatement, ProofRef, Verdict};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};
type F = (i64, i64);
type P = Option<(F, F)>;
fn n(v: &Value) -> i64 {
    v.as_i64().unwrap()
}
fn sha(b: &[u8]) -> String {
    hex::encode(sha_backend::sha256(b))
}
fn load(p: &str) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: &str, v: &Value) {
    fs::write(p, serde_json::to_string_pretty(v).unwrap() + "\n").unwrap();
}
fn primes(mut n: i64) -> Vec<i64> {
    let mut out = vec![];
    let mut p = 2;
    while p * p <= n {
        if n % p == 0 {
            out.push(p);
            while n % p == 0 {
                n /= p;
            }
        }
        p += 1;
    }
    if n > 1 {
        out.push(n);
    }
    out
}
struct Curve {
    p: i64,
    d: i64,
    a: i64,
    b: i64,
}
impl Curve {
    fn f(&self, n: i64) -> F {
        (n.rem_euclid(self.p), 0)
    }
    fn decode(&self, n: i64) -> F {
        assert!((0..self.p * self.p).contains(&n));
        (n % self.p, n / self.p)
    }
    fn encode(&self, x: F) -> i64 {
        x.0 + self.p * x.1
    }
    fn addf(&self, x: F, y: F) -> F {
        (
            (x.0 + y.0).rem_euclid(self.p),
            (x.1 + y.1).rem_euclid(self.p),
        )
    }
    fn negf(&self, x: F) -> F {
        ((-x.0).rem_euclid(self.p), (-x.1).rem_euclid(self.p))
    }
    fn subf(&self, x: F, y: F) -> F {
        self.addf(x, self.negf(y))
    }
    fn mulf(&self, x: F, y: F) -> F {
        (
            (x.0 * y.0 + self.d * x.1 * y.1).rem_euclid(self.p),
            (x.0 * y.1 + x.1 * y.0).rem_euclid(self.p),
        )
    }
    fn pow(&self, mut x: F, mut e: i64) -> F {
        let mut z = self.f(1);
        while e > 0 {
            if e % 2 == 1 {
                z = self.mulf(z, x);
            }
            x = self.mulf(x, x);
            e /= 2;
        }
        z
    }
    fn div(&self, x: F, y: F) -> F {
        assert_ne!(y, self.f(0));
        self.mulf(x, self.pow(y, self.p * self.p - 2))
    }
    fn rhs(&self, x: F) -> F {
        self.addf(
            self.addf(self.pow(x, 3), self.mulf(self.f(self.a), x)),
            self.f(self.b),
        )
    }
    fn valid(&self, p: P) -> bool {
        p.is_none_or(|(x, y)| self.mulf(y, y) == self.rhs(x))
    }
    fn pt(&self, v: &Value) -> P {
        assert_eq!(v.as_array().unwrap().len(), 2);
        if n(&v[0]) == -1 {
            assert_eq!(n(&v[1]), -1);
            None
        } else {
            Some((self.decode(n(&v[0])), self.decode(n(&v[1]))))
        }
    }
    fn neg(&self, p: P) -> P {
        p.map(|(x, y)| (x, self.negf(y)))
    }
    fn frob(&self, p: P) -> P {
        p.map(|(x, y)| (self.pow(x, self.p), self.pow(y, self.p)))
    }
    fn add(&self, p: P, q: P) -> P {
        let Some((x, y)) = p else { return q };
        let Some((xx, yy)) = q else { return p };
        if x == xx && self.addf(y, yy) == self.f(0) {
            return None;
        }
        let m = if x == xx {
            self.div(
                self.addf(self.mulf(self.f(3), self.mulf(x, x)), self.f(self.a)),
                self.mulf(self.f(2), y),
            )
        } else {
            self.div(self.subf(yy, y), self.subf(xx, x))
        };
        let z = self.subf(self.subf(self.mulf(m, m), x), xx);
        Some((z, self.subf(self.mulf(m, self.subf(x, z)), y)))
    }
    fn scalar(&self, mut n: i64, mut p: P) -> P {
        let mut z = None;
        while n > 0 {
            if n % 2 == 1 {
                z = self.add(z, p);
            }
            p = self.add(p, p);
            n /= 2;
        }
        z
    }
    fn points(&self, base: bool) -> Vec<P> {
        let bound = if base { self.p } else { self.p * self.p };
        let mut roots = vec![vec![]; (self.p * self.p) as usize];
        for j in 0..bound {
            let y = self.decode(j);
            roots[self.encode(self.mulf(y, y)) as usize].push(y);
        }
        let mut out = vec![None];
        for j in 0..bound {
            let x = self.decode(j);
            for &y in &roots[self.encode(self.rhs(x)) as usize] {
                out.push(Some((x, y)));
            }
        }
        out
    }
    fn image(&self, p: P) -> P {
        p.map(|(x, y)| (self.div(x, self.f(self.d)), self.div(y, (0, self.d))))
    }
    fn line(&self, p: P, q: P, x: P) -> Option<F> {
        let ((px, py), (qx, qy), (xx, xy)) = (p?, q?, x?);
        if px == qx && self.addf(py, qy) == self.f(0) {
            let z = self.subf(xx, px);
            return (z != self.f(0)).then_some(z);
        }
        let den = if px == qx {
            self.mulf(self.f(2), py)
        } else {
            self.subf(qx, px)
        };
        if den == self.f(0) {
            return None;
        }
        let slope = if px == qx {
            self.div(
                self.addf(self.mulf(self.f(3), self.pow(px, 2)), self.f(self.a)),
                den,
            )
        } else {
            self.div(self.subf(qy, py), den)
        };
        let num = self.subf(self.subf(xy, py), self.mulf(slope, self.subf(xx, px)));
        let den = self
            .add(p, q)
            .map_or(self.f(1), |(sx, _)| self.subf(xx, sx));
        (num != self.f(0) && den != self.f(0)).then(|| self.div(num, den))
    }
    fn miller(&self, p: P, x: P, r: i64) -> Option<F> {
        let mut v = p;
        let mut z = self.f(1);
        for _ in 1..r {
            z = self.mulf(z, self.line(v, p, x)?);
            v = self.add(v, p);
        }
        assert_eq!(v, None);
        Some(z)
    }
    fn direct(&self, p: P, q: P, r: i64) -> Option<F> {
        if p.is_none() || q.is_none() {
            return Some(self.f(1));
        }
        let a = self.miller(p, q, r)?;
        let b = self.miller(q, p, r)?;
        let z = self.div(a, b);
        Some(if r % 2 == 1 { self.negf(z) } else { z })
    }
    fn shifted(&self, p: P, q: P, r: i64, o: P) -> Option<F> {
        if p.is_none() || q.is_none() {
            return Some(self.f(1));
        }
        let a = self.miller(p, self.add(q, o), r)?;
        let b = self.miller(p, o, r)?;
        let d = self.miller(q, self.neg(o), r)?;
        let e = self.miller(q, self.add(p, self.neg(o)), r)?;
        Some(self.mulf(self.div(a, b), self.div(d, e)))
    }
}
fn audit_ic(c: &Curve, g: F, target: F, v: &Value, r: i64) -> i64 {
    let logs = v["projected_factor_base_logs"]
        .as_array()
        .unwrap()
        .iter()
        .map(n)
        .collect::<Vec<_>>();
    assert_eq!(logs.len(), 6);
    let projection = (c.p * c.p - 1) / r;
    assert_eq!(n(&v["projection_exponent"]), projection);
    for i in 0..6 {
        assert_eq!(v["factor_base"][i], format!("u+{}", i + 1));
        assert_eq!(c.pow(g, logs[i]), c.pow((i as i64 + 1, 1), projection));
    }
    let mut rows = vec![];
    for row in v["independent_relations"].as_array().unwrap() {
        let a = row["coefficients"]
            .as_array()
            .unwrap()
            .iter()
            .map(n)
            .collect::<Vec<_>>();
        assert_eq!(a.len(), 6);
        assert_eq!(
            a.iter()
                .zip(&logs)
                .map(|(x, y)| x * y)
                .sum::<i64>()
                .rem_euclid(r),
            n(&row["rhs"]),
            "false IC coefficient or right-hand side"
        );
        rows.push(a.iter().map(|x| x.rem_euclid(r)).collect::<Vec<_>>());
    }
    let mut rank = 0;
    for col in 0..6 {
        if let Some(pivot) = (rank..rows.len()).find(|&i| rows[i][col] != 0) {
            rows.swap(rank, pivot);
            let inv = (1..r).find(|&x| rows[rank][col] * x % r == 1).unwrap();
            for j in col..6 {
                rows[rank][j] = rows[rank][j] * inv % r;
            }
            for i in rank + 1..rows.len() {
                let s = rows[i][col];
                for j in col..6 {
                    rows[i][j] = (rows[i][j] - s * rows[rank][j]).rem_euclid(r);
                }
            }
            rank += 1;
        }
    }
    assert_eq!(rank, 6);
    assert_eq!(n(&v["relation_rank"]), 6);
    let answer = n(&v["recovered_log"]);
    assert!((0..r).contains(&answer));
    assert_eq!(c.pow(g, answer), target);
    answer
}

const PROOF_PATH: &str = "research/quadratic_twist_evidence_20261021/lean/PairingTransfer.lean";

fn norm_statement() -> IdentityStatement {
    let v = E::var;
    let square = |x: E| E::product(vec![x.clone(), x]);
    let mul = |x: E, y: E| E::product(vec![x, y]);
    let sub = |x: E, y: E| E::plus(x, E::minus(y));
    IdentityStatement {
        name: "twist-pairing-norm-product".into(),
        source: format!("{PROOF_PATH}:PairingTransfer.norm_product_identity; polynomial part only; no general pairing theorem"),
        variables: ["d", "x", "y", "s", "t"].map(str::to_string).into(),
        degree_bound: 6,
        lhs: sub(
            square(E::plus(mul(v("x"), v("s")), mul(v("d"), mul(v("y"), v("t"))))),
            mul(v("d"), square(E::plus(mul(v("x"), v("t")), mul(v("y"), v("s"))))),
        ),
        rhs: mul(sub(square(v("x")), mul(v("d"), square(v("y")))),
                 sub(square(v("s")), mul(v("d"), square(v("t"))))),
    }
}

fn verify_norm(root: &Path, cert: &IdentityCertificate) -> Result<(), String> {
    if serde_json::to_value(norm_statement()).unwrap()
        != serde_json::to_value(&cert.statement).unwrap()
    {
        return Err("statement differs from the specified norm polynomial".into());
    }
    let proof = cert.proof.as_ref().ok_or("missing Lean source reference")?;
    if proof.kind != "lean4" || proof.path != PROOF_PATH {
        return Err("wrong Lean source path/kind".into());
    }
    if proof.sha256 != sha(&fs::read(root.join(PROOF_PATH)).map_err(|e| e.to_string())?) {
        return Err("Lean source SHA-256 mismatch".into());
    }
    match identity_certificate::check_with_extra(cert, Some((2381, 16)))
        .map_err(|e| e.to_string())?
    {
        Verdict::Accept {
            points: 32,
            extra_points: 16,
            ..
        } => Ok(()),
        other => Err(format!("certificate not accepted: {other:?}")),
    }
}

fn certificate(mode: &str, root: &Path, out: &Path) {
    let path = out.join("norm-product.json");
    let cert: IdentityCertificate = if mode == "issue" {
        assert!(!out.exists(), "refusing to replace certificates");
        fs::create_dir_all(out).unwrap();
        let proof = ProofRef {
            kind: "lean4".into(),
            path: PROOF_PATH.into(),
            sha256: sha(&fs::read(root.join(PROOF_PATH)).unwrap()),
        };
        let cert = identity_certificate::issue(
            norm_statement(),
            20261024,
            32,
            "Norm polynomial only; 16 independent-seed extra points; no performance claim",
            Some(proof),
        )
        .unwrap();
        save(
            path.to_str().unwrap(),
            &serde_json::to_value(&cert).unwrap(),
        );
        cert
    } else {
        assert_eq!(mode, "check");
        serde_json::from_slice(&fs::read(&path).unwrap()).unwrap()
    };
    verify_norm(root, &cert).unwrap();
    let mut wrong_source = cert.clone();
    wrong_source.proof.as_mut().unwrap().sha256 = "0".repeat(64);
    let rejection = verify_norm(root, &wrong_source).unwrap_err();
    assert_eq!(rejection, "Lean source SHA-256 mismatch");
    let mut false_statement = norm_statement();
    false_statement.rhs = E::plus(false_statement.rhs, E::Const(1));
    let counterexample = match identity_certificate::issue(
        false_statement.clone(),
        20261024,
        32,
        "false-plus-one control",
        None,
    )
    .unwrap_err()
    {
        identity_certificate::IssueError::False(p) => p,
        other => panic!("expected arithmetic rejection: {other:?}"),
    };
    let controls = json!({"false_identity":{"statement":false_statement,"counterexample":{
        "index":counterexample.index,"assignment":counterexample.assignment,"lhs":counterexample.lhs,"rhs":counterexample.rhs}},
        "false_source":{"certificate":wrong_source,"rejection":rejection}});
    let receipt = json!({"status":"passed","id":cert.id,"proof":cert.proof,"issue_points":32,"extra_points":16,
        "false_plus_one":"rejected","false_source":"rejected",
        "scope":"norm polynomial only; no general Weil theorem, IC formalization or speed claim"});
    for (file, value) in [("controls.json", controls), ("receipt.json", receipt)] {
        let path = out.join(file);
        let text = serde_json::to_string_pretty(&value).unwrap() + "\n";
        if mode == "issue" {
            fs::write(path, text).unwrap();
        } else {
            assert_eq!(fs::read_to_string(path).unwrap(), text);
        }
    }
    println!("PASS source-bound norm polynomial, 32+16 points, false-identity/source controls");
}

fn corrupt(mode: &str, input: &str, out: &str) {
    assert!(!Path::new(out).exists(), "refusing to replace a control");
    let mut v = load(input);
    match mode {
        "false-value" => {
            let s = v["models"]
                .as_array_mut()
                .unwrap()
                .iter_mut()
                .flat_map(|m| m["primes"].as_array_mut().unwrap())
                .find(|s| s["shared"] == true)
                .unwrap();
            let p = s["auxiliaries"]
                .as_array_mut()
                .unwrap()
                .iter_mut()
                .find(|p| p["S"][0] != -1)
                .unwrap();
            p["value"] = json!(1);
        }
        "missing-model" => {
            v["models"].as_array_mut().unwrap().pop();
        }
        "false-count" => {
            v["summary"]["nontrivial_images"] = json!(3313);
        }
        "false-ic-coefficient" => {
            let row = &mut v["fixture"]["logs"][7]["index_calculus"]["independent_relations"][0];
            row["coefficients"][0] = json!(n(&row["coefficients"][0]) + 1);
        }
        _ => panic!("unknown corruption"),
    }
    save(out, &v);
}
fn audit(input: &str, registry: &str) -> Value {
    let bytes = fs::read(input).unwrap();
    let data: Value = serde_json::from_slice(&bytes).unwrap();
    let reg_bytes = fs::read(registry).unwrap();
    assert_eq!(data["schema"], "quadratic-twist-pairing-transfer/v1");
    assert_eq!(data["status"], "passed");
    assert_eq!(data["registry_sha256"], sha(&reg_bytes));
    let reg: Value = serde_json::from_slice(&reg_bytes).unwrap();
    let mut ids = BTreeMap::new();
    for m in reg["curves"].as_array().unwrap() {
        if m["family"] != "prime" {
            continue;
        }
        let val = |key: &str| {
            m["params"][key]
                .as_i64()
                .or_else(|| m["params"][key].as_str().and_then(|s| s.parse().ok()))
        };
        if let (Some(p), Some(a), Some(b)) = (val("p"), val("a"), val("b")) {
            ids.insert((p, a, b), m["slug"].clone());
        }
    }
    let (mut seen, mut shared, mut nonshared, mut aux, mut zero, mut nonzero) =
        (BTreeSet::new(), 0, 0, 0, 0, 0);
    let (mut same_source_undefined, mut wrong_subgroup_available) = (0, 0);
    let mut by_prime = BTreeMap::<i64, [i64; 4]>::new();
    for m in data["models"].as_array().unwrap() {
        let (p, a, b, d) = (n(&m["p"]), n(&m["a"]), n(&m["b"]), n(&m["d"]));
        assert!([5, 7, 11, 13, 17, 19].contains(&p) && (0..p).contains(&a) && (0..p).contains(&b));
        assert_ne!((4 * a * a * a + 27 * b * b) % p, 0);
        assert!(seen.insert((p, a, b)));
        let c = Curve { p, d, a, b };
        assert_eq!(
            d,
            (2..p)
                .find(|&v| c.pow(c.f(v), (p - 1) / 2) == c.f(-1))
                .unwrap()
        );
        let (ta, tb) = ((d * d * a) % p, (d * d * d * b) % p);
        let t = Curve { p, d, a: ta, b: tb };
        let ep = c.points(true);
        let tp = t.points(true);
        let offsets = c.points(false);
        assert_eq!(m["icv1"], ids[&(p, a, b)]);
        assert_eq!(m["twist_icv1"], ids[&(p, ta, tb)]);
        assert_eq!(n(&m["base_order"]), ep.len() as i64);
        assert_eq!(n(&m["twist_order"]), tp.len() as i64);
        let expected = primes(ep.len() as i64)
            .into_iter()
            .filter(|&r| r != 2)
            .collect::<Vec<_>>();
        let cases = m["primes"].as_array().unwrap();
        assert_eq!(
            cases.iter().map(|s| n(&s["r"])).collect::<Vec<_>>(),
            expected
        );
        for s in cases {
            let r = n(&s["r"]);
            let joint = tp.len() as i64 % r == 0;
            assert_eq!(s["shared"], joint);
            if !joint {
                nonshared += 1;
                assert_eq!(tp.iter().filter(|&&x| t.scalar(r, x).is_none()).count(), 1);
                assert_eq!(s["twist_r_torsion_points"], 1);
                continue;
            }
            shared += 1;
            assert_eq!((p + 1) % r, 0);
            assert_ne!(p % r, 1);
            let source = c.pt(&s["P"]);
            let projection = ep
                .iter()
                .map(|&x| c.scalar(ep.len() as i64 / r, x))
                .find(|p| p.is_some())
                .unwrap();
            assert_eq!(source, projection);
            assert!(c.valid(source));
            assert!(c.scalar(r, source).is_none());
            assert_eq!(c.frob(source), source);
            let values = s["auxiliaries"].as_array().unwrap();
            assert_eq!(values.len(), tp.len());
            let mut image_values = BTreeSet::new();
            for (&point, v) in tp.iter().zip(values) {
                aux += 1;
                assert_eq!(t.pt(&v["T"]), point);
                let u = t.scalar(tp.len() as i64 / r, point);
                assert_eq!(t.pt(&v["cofactor_T"]), u);
                let dest = c.image(u);
                assert_eq!(c.pt(&v["S"]), dest);
                assert!(c.valid(dest));
                assert!(c.scalar(r, dest).is_none());
                assert_eq!(c.frob(dest), c.neg(dest));
                let value = c.decode(n(&v["value"]));
                assert_eq!(
                    c.direct(source, dest, r),
                    Some(value),
                    "pairing value mismatch"
                );
                let o = c.pt(&v["offset"]);
                if dest.is_none() {
                    zero += 1;
                    assert_eq!(value, c.f(1));
                    assert_eq!(o, None);
                } else {
                    nonzero += 1;
                    assert_ne!(value, c.f(1));
                    assert_eq!(c.pow(value, r), c.f(1));
                    assert_eq!(c.pow(value, p + 1), c.f(1));
                    assert!(c.valid(o));
                    assert_eq!(c.shifted(source, dest, r, o), Some(value));
                    let first = offsets
                        .iter()
                        .find(|&&z| c.shifted(source, dest, r, z).is_some())
                        .copied()
                        .unwrap();
                    assert_eq!(o, first);
                }
                image_values.insert(value);
                let counts = by_prime.entry(r).or_default();
                counts[1] += 1;
                counts[if dest.is_none() { 2 } else { 3 }] += 1;
            }
            assert_eq!(image_values.len() as i64, r);
            by_prime.entry(r).or_default()[0] += 1;
            let control = &s["same_source_control"];
            let o = c.pt(&control["offset"]);
            assert_eq!(control["value"], 1);
            assert_eq!(c.shifted(source, source, r, o), Some(c.f(1)));
            assert_eq!(
                control["direct_defined"],
                c.direct(source, source, r).is_some()
            );
            same_source_undefined += i64::from(c.direct(source, source, r).is_none());
            let wrong = tp.iter().find(|&&x| t.scalar(r, x).is_some()).copied();
            match wrong {
                Some(x) => {
                    wrong_subgroup_available += 1;
                    assert_eq!(t.pt(&s["wrong_subgroup_control"]), x);
                    assert!(t.valid(x));
                    assert!(t.scalar(r, x).is_some());
                }
                None => assert!(s["wrong_subgroup_control"].is_null()),
            }
        }
    }
    let expected = [5, 7, 11, 13, 17, 19]
        .into_iter()
        .flat_map(|p| {
            (0..p).flat_map(move |a| {
                (0..p)
                    .filter(move |b| (4 * a * a * a + 27 * b * b) % p != 0)
                    .map(move |b| (p, a, b))
            })
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(seen, expected, "missing model coverage");
    let summary = json!({"models":seen.len(),"shared_primes":shared,"nonshared_primes":nonshared,"auxiliary_points":aux,"cofactor_zero_images":zero,"nontrivial_images":nonzero});
    assert_eq!(
        data["summary"], summary,
        "declared summary differs from recomputed counts"
    );
    assert_eq!(shared, 274);
    let f = &data["fixture"];
    assert_eq!(f["p"], 43);
    assert_eq!(f["d"], 2);
    assert_eq!(f["r"], 11);
    let c = Curve {
        p: 43,
        d: 2,
        a: 1,
        b: 0,
    };
    assert_eq!(f["icv1"], ids[&(43, 1, 0)]);
    assert_eq!(f["twist_icv1"], ids[&(43, 4, 0)]);
    let p = c.pt(&f["P"]);
    let s = c.pt(&f["S"]);
    let q = c.pt(&f["Q"]);
    assert!(c.valid(p) && c.valid(s) && c.valid(q));
    let t = Curve {
        p: 43,
        d: 2,
        a: 4,
        b: 0,
    };
    let source_t = t.pt(&f["monic_twist_point"]);
    assert!(t.valid(source_t));
    assert_eq!(c.image(source_t), s);
    assert_eq!(c.scalar(7, p), q);
    assert_eq!(f["known_scalar"], 7);
    let g = c.decode(n(&f["g"]));
    assert_eq!(g, (2, 25));
    assert_eq!(c.direct(p, s, 11), Some(g));
    assert_ne!(g, c.f(1));
    assert_eq!(c.pow(g, 11), c.f(1));
    let logs = f["logs"].as_array().unwrap();
    assert_eq!(logs.len(), 11);
    for (i, v) in logs.iter().enumerate() {
        assert_eq!(n(&v["scalar"]), i as i64);
        let q = c.scalar(i as i64, p);
        assert_eq!(c.pt(&v["Q"]), q);
        let target = c.decode(n(&v["target"]));
        assert_eq!(c.direct(q, s, 11), Some(target));
        assert_eq!(audit_ic(&c, g, target, &v["index_calculus"], 11), i as i64);
    }
    json!({"schema":"quadratic-twist-pairing-audit/v1","status":"passed","input_sha256":sha(&bytes),"summary":summary,"by_prime_order":by_prime,"fixture_logs_verified":11,
   "controls":{"same_source_trivial":shared,"same_source_direct_undefined":same_source_undefined,
     "wrong_subgroup_available":wrong_subgroup_available,"wrong_subgroup_null_cases":shared-wrong_subgroup_available},
   "arithmetic":"independent coefficient pairs; exponent inversion; linear Miller chain; shifted/direct checks; independent Gaussian rank","general_weil_pairing_proof":false,"performance_claim":false})
}
fn main() {
    let a = std::env::args().collect::<Vec<_>>();
    if a.len() == 5 && a[1] == "corrupt" {
        corrupt(&a[2], &a[3], &a[4]);
        return;
    }
    if a.len() == 5 && a[1] == "certificate" {
        certificate(&a[2], Path::new(&a[3]), Path::new(&a[4]));
        return;
    }
    assert_eq!(a.len(), 4, "pairing-audit INPUT REGISTRY AUDIT");
    save(&a[3], &audit(&a[1], &a[2]));
    println!("PASS independent pairing sweep, offsets, source bindings and eleven IC logs");
}
