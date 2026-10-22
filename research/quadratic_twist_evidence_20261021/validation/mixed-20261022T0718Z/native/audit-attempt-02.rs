//! Independent tuple arithmetic and multiplicity-DP replay of mixed.cpp.
//! The only shared implementation is JSON/SHA-256 and the repository IDC1 backend.
#![allow(dead_code)]
#[path = "../../../../src/hash/sha256.rs"]
pub mod sha_backend;
pub mod hash {
    pub use crate::sha_backend as sha256;
}
#[path = "../../../../src/cryptanalysis/identity_certificate.rs"]
mod identity_certificate;
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};
type F = (i64, i64);
type P = Option<(F, F)>;
type Row = Vec<i64>;
fn sha(bytes: &[u8]) -> String {
    hex::encode(sha_backend::sha256(bytes))
}
fn n(v: &Value) -> i64 {
    v.as_i64().unwrap()
}
fn load(p: &str) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn residue(x: i64, p: i64) -> i64 {
    x.rem_euclid(p)
}
struct Curve {
    p: i64,
    d: i64,
    a: i64,
    b: i64,
}
impl Curve {
    fn addf(&self, x: F, y: F) -> F {
        (residue(x.0 + y.0, self.p), residue(x.1 + y.1, self.p))
    }
    fn negf(&self, x: F) -> F {
        (residue(-x.0, self.p), residue(-x.1, self.p))
    }
    fn subf(&self, x: F, y: F) -> F {
        self.addf(x, self.negf(y))
    }
    fn mulf(&self, x: F, y: F) -> F {
        (
            residue(x.0 * y.0 + self.d * x.1 * y.1, self.p),
            residue(x.0 * y.1 + x.1 * y.0, self.p),
        )
    }
    fn pow(&self, mut x: F, mut e: i64) -> F {
        let mut z = (1, 0);
        while e > 0 {
            if e % 2 == 1 {
                z = self.mulf(z, x)
            }
            x = self.mulf(x, x);
            e /= 2;
        }
        z
    }
    fn div(&self, x: F, y: F) -> F {
        assert_ne!(y, (0, 0), "division by zero");
        self.mulf(x, self.pow(y, self.p * self.p - 2))
    }
    fn rhs(&self, x: F) -> F {
        self.addf(
            self.addf(self.pow(x, 3), self.mulf((self.a, 0), x)),
            (self.b, 0),
        )
    }
    fn valid(&self, p: P) -> bool {
        p.is_none_or(|(x, y)| self.mulf(y, y) == self.rhs(x))
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
        if x == xx && self.addf(y, yy) == (0, 0) {
            return None;
        }
        let m = if x == xx {
            self.div(
                self.addf(self.mulf((3, 0), self.mulf(x, x)), (self.a, 0)),
                self.mulf((2, 0), y),
            )
        } else {
            self.div(self.subf(yy, y), self.subf(xx, x))
        };
        let z = self.subf(self.subf(self.mulf(m, m), x), xx);
        Some((z, self.subf(self.mulf(m, self.subf(x, z)), y)))
    }
    fn scalar(&self, mut k: i64, mut p: P) -> P {
        if k < 0 {
            p = self.neg(p);
            k = -k;
        }
        let mut q = None;
        while k > 0 {
            if k % 2 == 1 {
                q = self.add(q, p)
            }
            p = self.add(p, p);
            k /= 2;
        }
        q
    }
    fn decode(&self, v: &Value) -> P {
        if v.is_null() {
            return None;
        }
        let x = n(&v[0]);
        let y = n(&v[1]);
        if x == -1 {
            assert_eq!(y, -1);
            return None;
        }
        assert!((0..self.p * self.p).contains(&x) && (0..self.p * self.p).contains(&y));
        Some(((x % self.p, x / self.p), (y % self.p, y / self.p)))
    }
    fn points(&self, twist: bool) -> Vec<P> {
        let mut out = vec![None];
        for x in 0..self.p {
            for y in 0..self.p {
                if residue((if twist { self.d } else { 1 }) * y * y, self.p) == self.rhs((x, 0)).0 {
                    out.push(Some(((x, 0), if twist { (0, y) } else { (y, 0) })));
                }
            }
        }
        out
    }
}
fn primes(mut x: i64) -> Vec<i64> {
    let mut out = vec![];
    let mut p = 2;
    while p * p <= x {
        if x % p == 0 {
            out.push(p);
            while x % p == 0 {
                x /= p;
            }
        }
        p += 1;
    }
    if x > 1 {
        out.push(x)
    }
    out
}
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        (a, b) = (b, a % b)
    }
    a
}
fn rank(rows: &BTreeSet<Row>, r: i64, width: usize) -> usize {
    // Forward Gaussian elimination, independently of the producer's RREF.
    let mut pivots: BTreeMap<usize, Row> = BTreeMap::new();
    for input in rows {
        let mut row: Row = input.iter().map(|x| residue(*x, r)).collect();
        for (&i, old) in &pivots {
            let k = row[i];
            for j in i..width {
                row[j] = residue(row[j] - k * old[j], r);
            }
        }
        if let Some(i) = row.iter().position(|x| *x != 0) {
            let inv = (1..r).find(|z| z * row[i] % r == 1).unwrap();
            for x in &mut row {
                *x = *x * inv % r;
            }
            pivots.insert(i, row);
        }
    }
    pivots.len()
}
fn rows(v: &Value) -> BTreeSet<Row> {
    let a = v.as_array().unwrap();
    let out: BTreeSet<_> = a
        .iter()
        .map(|x| x.as_array().unwrap().iter().map(n).collect())
        .collect();
    assert_eq!(a.len(), out.len(), "duplicate recorded rows");
    out
}
fn array_counts(v: &Value) -> Vec<u64> {
    v.as_array()
        .unwrap()
        .iter()
        .map(|x| x.as_u64().unwrap())
        .collect()
}
fn sum_row(c: &Curve, row: &[i64], basis: &[P]) -> P {
    assert_eq!(row.len(), basis.len());
    row.iter()
        .zip(basis)
        .fold(None, |sum, (&k, &p)| c.add(sum, c.scalar(k, p)))
}
fn replay_subgroup(c: &Curve, s: &Value, ep: &[P], tp: &[P]) -> Value {
    let r = n(&s["r"]);
    assert!(primes(r) == [r] && r != 2);
    let p = c.decode(&s["P"]);
    let t = c.decode(&s["T"]);
    assert!(p.is_some() && t.is_some() && c.valid(p) && c.valid(t));
    assert_eq!(c.scalar(r, p), None);
    assert_eq!(c.scalar(r, t), None);
    assert_eq!(
        p,
        *ep.iter()
            .find(|&&x| x.is_some() && c.scalar(r, x).is_none())
            .unwrap()
    );
    assert_eq!(
        t,
        *tp.iter()
            .find(|&&x| x.is_some() && c.scalar(r, x).is_none())
            .unwrap()
    );
    assert_eq!(c.frob(p), p);
    assert_eq!(c.frob(t), c.neg(t));
    let mut product = BTreeSet::new();
    for a in 0..r {
        for b in 0..r {
            let q = c.add(c.scalar(a, p), c.scalar(b, t));
            assert!(c.valid(q));
            assert!(product.insert(q), "dependent actual point table");
        }
    }
    assert_eq!(product.len() as i64, r * r);
    assert_eq!(n(&s["product_table_size"]), r * r);
    assert!(
        c.add(p, c.neg(t)).is_some(),
        "identified-log false row accepted"
    );
    assert_eq!(s["identified_log_control_rejected"], true);
    assert_eq!(s["identified_log_false_relation"], json!([1, -1]));
    let b = (r - 1) as usize / 2;
    let width = 2 * b;
    assert_eq!(n(&s["basis_size_per_component"]), b as i64);
    let basis: Vec<P> = [p, t]
        .into_iter()
        .flat_map(|g| (1..=b).map(move |j| c.scalar(j as i64, g)))
        .collect();
    let recorded = rows(&s["all_rows"]);
    for row in &recorded {
        assert_eq!(
            sum_row(c, row, &basis),
            None,
            "recorded relation is not zero on the actual curve"
        );
    }
    let mut dp: BTreeMap<(Row, usize), u64> = BTreeMap::from([((vec![0; width], 0), 1)]);
    let (mut all, mut plus, mut minus) = (BTreeSet::new(), BTreeSet::new(), BTreeSet::new());
    let (mut candidates, mut accepted, mut mixed, mut zero_mixed) = (0_u64, 0_u64, 0_u64, 0_u64);
    for length in 1..=6 {
        let mut next = BTreeMap::new();
        for ((row, k), multiplicity) in dp {
            for col in 0..width {
                for sign in [-1, 1] {
                    let mut v = row.clone();
                    v[col] += sign;
                    *next.entry((v, k + usize::from(col < b))).or_insert(0_u64) += multiplicity;
                }
            }
        }
        dp = next;
        if length < 3 {
            continue;
        }
        let (mut count, mut hits, mut zero) = (
            vec![0_u64; length + 1],
            vec![0_u64; length + 1],
            vec![0_u64; length + 1],
        );
        let mut exact = BTreeSet::new();
        for ((row, k), &multiplicity) in &dp {
            count[*k] += multiplicity;
            // Decide every DP state with actual tuple point arithmetic; no
            // one-dimensional log label is used as a relation oracle.
            if sum_row(c, row, &basis).is_some() {
                continue;
            }
            hits[*k] += multiplicity;
            exact.insert(row.clone());
            all.insert(row.clone());
            if row.iter().all(|x| *x == 0) {
                zero[*k] += multiplicity;
            }
            if *k == 0 {
                minus.insert(row.clone());
            }
            if *k == length {
                plus.insert(row.clone());
            }
            if *k > 0 && *k < length {
                let mut a = row.clone();
                a[b..].fill(0);
                let mut z = row.clone();
                z[..b].fill(0);
                assert!(
                    sum_row(c, &a, &basis).is_none() && sum_row(c, &z, &basis).is_none(),
                    "mixed component counterexample"
                );
            }
        }
        let level = &s["levels"][length - 3];
        assert_eq!(n(&level["length"]), length as i64);
        assert_eq!(array_counts(&level["candidate_words_by_plus_arity"]), count);
        assert_eq!(array_counts(&level["zero_words_by_plus_arity"]), hits);
        assert_eq!(
            array_counts(&level["zero_coefficient_words_by_plus_arity"]),
            zero
        );
        assert_eq!(n(&level["distinct_rows_exact_length"]), exact.len() as i64);
        let rp = rank(&plus, r, width);
        let rt = rank(&minus, r, width);
        let ra = rank(&all, r, width);
        assert_eq!(ra, rp + rt, "mixed rank gain");
        assert_eq!(n(&level["all_rank_up_to_length"]), ra as i64);
        assert_eq!(n(&level["pure_plus_rank_up_to_length"]), rp as i64);
        assert_eq!(n(&level["pure_minus_rank_up_to_length"]), rt as i64);
        candidates += count.iter().sum::<u64>();
        accepted += hits.iter().sum::<u64>();
        mixed += hits[1..length].iter().sum::<u64>();
        zero_mixed += zero[1..length].iter().sum::<u64>();
    }
    assert_eq!(s["levels"].as_array().unwrap().len(), 4);
    assert_eq!(recorded, all);
    assert_eq!(rows(&s["pure_plus_rows"]), plus);
    assert_eq!(rows(&s["pure_minus_rows"]), minus);
    json!({"r":r,"candidates":candidates,"accepted":accepted,"mixed_accepted":mixed,
      "mixed_zero_coefficient_words":zero_mixed,"rank_at_6":rank(&all,r,width),"rank_gain":0})
}
fn audit(input: &str, registry: &str) -> Value {
    let raw = fs::read(input).unwrap();
    let data: Value = serde_json::from_slice(&raw).unwrap();
    assert_eq!(data["schema"], "quadratic-twist-mixed-relations/v1");
    assert_eq!(data["status"], "passed");
    let registry_bytes = fs::read(registry).unwrap();
    assert_eq!(data["registry_sha256"], sha(&registry_bytes));
    let reg: Value = serde_json::from_slice(&registry_bytes).unwrap();
    let mut names = BTreeMap::new();
    for m in reg["curves"].as_array().unwrap() {
        let par = &m["params"];
        if m["family"] != "prime" {
            continue;
        }
        let val = |k: &str| -> Option<i64> {
            par[k]
                .as_i64()
                .or_else(|| par[k].as_str().and_then(|s| s.parse().ok()))
        };
        if let (Some(p), Some(a), Some(b)) = (val("p"), val("a"), val("b")) {
            names.insert((p, a, b), m["slug"].clone());
        }
    }
    let (mut seen, mut summary) = (BTreeSet::new(), vec![]);
    let (mut pairs, mut exceptions, mut subgroups, mut square_controls, mut two_controls) =
        (0, 0, 0, 0, 0);
    for m in data["models"].as_array().unwrap() {
        let (p, a, b, d) = (n(&m["p"]), n(&m["a"]), n(&m["b"]), n(&m["d"]));
        assert!([5, 7, 11, 13, 17, 19].contains(&p) && (0..p).contains(&a) && (0..p).contains(&b));
        assert_ne!(residue(4 * a * a * a + 27 * b * b, p), 0);
        assert!(seen.insert((p, a, b)), "duplicate model");
        let c = Curve { p, d, a, b };
        assert_eq!(
            d,
            (2..p)
                .find(|&x| c.pow((x, 0), (p - 1) / 2) == (p - 1, 0))
                .unwrap()
        );
        assert_eq!(m["curve"], names[&(p, a, b)]);
        assert_eq!(
            m["twist_normalized"],
            names[&(p, a * d * d % p, b * d * d % p * d % p)]
        );
        let ep = c.points(false);
        let tp = c.points(true);
        assert_eq!(n(&m["curve_order"]), ep.len() as i64);
        assert_eq!(n(&m["twist_order"]), tp.len() as i64);
        let (mut rational, mut nonzero) = (0, 0);
        for &a in &ep {
            for &b in &tp {
                assert!(c.valid(a) && c.valid(b));
                let z = c.add(a, b);
                assert!(c.valid(z));
                let fixed = c.frob(z) == z;
                assert_eq!(fixed, c.scalar(2, b).is_none());
                if fixed {
                    rational += 1;
                    if b.is_some() {
                        nonzero += 1;
                    }
                }
            }
        }
        assert_eq!(n(&m["mixed_pairs"]), (ep.len() * tp.len()) as i64);
        assert_eq!(n(&m["rational_mixed_sums"]), rational);
        assert_eq!(n(&m["nonzero_two_torsion_mixed_sums"]), nonzero);
        pairs += ep.len() * tp.len();
        exceptions += nonzero;
        let two = ep
            .iter()
            .copied()
            .find(|&x| x.is_some() && c.scalar(2, x).is_none());
        assert_eq!(c.decode(&m["two_torsion_control"]), two.flatten());
        if let Some(t) = two {
            assert!(tp.contains(&t));
            assert_eq!(c.add(c.neg(t), t), None);
            two_controls += 1;
        }
        let square = ep
            .iter()
            .copied()
            .find(|&x| x.is_some() && c.scalar(2, x).is_some());
        assert_eq!(c.decode(&m["square_parameter_control"]), square.flatten());
        if let Some(t) = square {
            assert_eq!(c.frob(t), t);
            assert_ne!(c.frob(t), c.neg(t));
            square_controls += 1;
        }
        let rs: Vec<_> = primes(gcd(ep.len() as i64, tp.len() as i64))
            .into_iter()
            .filter(|x| *x != 2)
            .collect();
        assert_eq!(
            m["subgroups"]
                .as_array()
                .unwrap()
                .iter()
                .map(|s| n(&s["r"]))
                .collect::<Vec<_>>(),
            rs
        );
        for s in m["subgroups"].as_array().unwrap() {
            let mut record = replay_subgroup(&c, s, &ep, &tp);
            record["p"] = json!(p);
            record["curve"] = m["curve"].clone();
            summary.push(record);
            subgroups += 1;
        }
    }
    let mut expected = BTreeSet::new();
    for p in [5, 7, 11, 13, 17, 19] {
        for a in 0..p {
            for b in 0..p {
                if residue(4 * a * a * a + 27 * b * b, p) != 0 {
                    expected.insert((p, a, b));
                }
            }
        }
    }
    assert_eq!(seen, expected, "missing model coverage");
    assert_eq!(seen.len(), 942);
    assert_eq!(subgroups, 274);
    assert_eq!(n(&data["totals"]["models"]), 942);
    assert_eq!(n(&data["totals"]["shared_odd_prime_subgroups"]), subgroups);
    assert_eq!(n(&data["totals"]["mixed_pairs"]), pairs as i64);
    assert_eq!(
        n(&data["totals"]["nonzero_two_torsion_mixed_sums"]),
        exceptions
    );
    assert_eq!(n(&data["totals"]["mixed_component_counterexamples"]), 0);
    assert_eq!(n(&data["totals"]["rank_gain_cases"]), 0);
    let total = |key: &str| {
        summary
            .iter()
            .map(|s| s[key].as_u64().unwrap())
            .sum::<u64>()
    };
    json!({"schema":"quadratic-twist-mixed-audit/v1","status":"passed","input_sha256":sha(&raw),
      "arithmetic":"independent Rust coefficient pairs; inversion by exponent p^2-2; Frobenius by q-power",
      "enumeration":"multiplicity dynamic program over coefficient rows, independent of ordered-word DFS",
      "models":seen.len(),"subgroups":subgroups,"mixed_pairs":pairs,"two_torsion_exceptions":exceptions,
      "two_torsion_control_models":two_controls,"square_parameter_control_models":square_controls,
      "candidate_words":total("candidates"),"accepted_words":total("accepted"),"mixed_accepted_words":total("mixed_accepted"),
      "mixed_zero_coefficient_words":total("mixed_zero_coefficient_words"),"rank_gain_cases":0,"subgroup_replays":summary})
}
fn certify(proof_path: &str, out: &str) {
    use identity_certificate::{Expr as E, IdentityStatement, ProofRef, Verdict};
    let v = E::var;
    let power = |s: &str, n: usize| E::product(vec![v(s); n]);
    let minus = |a: E, b: E| E::plus(a, E::minus(b));
    let f = E::sum(vec![
        power("x", 3),
        E::product(vec![power("d", 2), v("a"), v("x")]),
        E::product(vec![power("d", 3), v("b")]),
    ]);
    let statement=IdentityStatement{
      name:"cleared-monic-twist".into(),
      source:format!("{proof_path}:TwistEmbedding.cleared_monic_twist_identity; polynomial part only, field inverses and Frobenius checked by Lean"),
      variables:["d","u","x","y","a","b"].into_iter().map(str::to_string).collect(),degree_bound:6,
      lhs:minus(E::product(vec![v("d"),power("y",2)]),E::product(vec![power("u",2),f.clone()])),
      rhs:minus(E::product(vec![v("d"),minus(power("y",2),f.clone())]),E::product(vec![minus(power("u",2),v("d")),f]))};
    let proof = ProofRef {
        kind: "lean4".into(),
        path: proof_path.into(),
        sha256: sha(&fs::read(proof_path).unwrap()),
    };
    let cert = identity_certificate::issue(
        statement.clone(),
        20261022,
        32,
        "Native IDC1 issue/replay; independent seed 2381 at 16 extra points; same host.",
        Some(proof.clone()),
    )
    .unwrap();
    assert!(matches!(
        identity_certificate::check_with_extra(&cert, Some((2381, 16))).unwrap(),
        Verdict::Accept {
            points: 32,
            extra_points: 16,
            ..
        }
    ));
    let mut false_statement = statement;
    false_statement.rhs = E::plus(false_statement.rhs, E::Const(1));
    assert!(identity_certificate::issue(
        false_statement,
        20261022,
        32,
        "false-plus-one control",
        None
    )
    .is_err());
    fs::create_dir_all(out).unwrap();
    fs::write(
        Path::new(out).join("cleared-monic-twist.json"),
        serde_json::to_string_pretty(&cert).unwrap() + "\n",
    )
    .unwrap();
    fs::write(Path::new(out).join("receipt.json"),serde_json::to_string_pretty(&json!({"status":"passed","id":cert.id,
      "proof_sha256":proof.sha256,"declaration":"TwistEmbedding.cleared_monic_twist_identity","issue_points":32,"extra_points":16,"false_plus_one":"rejected"})).unwrap()+"\n").unwrap();
    println!("PASS: source-bound cleared-map IDC1 certificate and false identity control");
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    assert_eq!(
        args.len(),
        4,
        "mixed-audit INPUT REGISTRY OUTPUT | mixed-audit certify PROOF OUTDIR"
    );
    if args[1] == "certify" {
        certify(&args[2], &args[3]);
        return;
    }
    let receipt = audit(&args[1], &args[2]);
    fs::write(
        Path::new(&args[3]),
        serde_json::to_string_pretty(&receipt).unwrap() + "\n",
    )
    .unwrap();
    println!("PASS: 942 models, 274 subgroups, independent mixed-relation/rank replay");
}
