//! Independent check of the solved DLP scalars (dd·G = Q) from the
//! `JV_DUMP_INSTANCE` dumps, with the validator's own arithmetic:
//! F_p, F_{p^2} = F_p[i]/(i^2 - w), F_{p^6} = F_{p^2}[t]/(t^3 - s), and the
//! curve y^2 = x^3 + a2 x^2 + a4 x over F_{p^6} (affine group law, inversion
//! by Fermat). Nothing here comes from the repository.
//!
//! Per instance: the tower is a field (w a non-residue, s a non-cube, by
//! Euler's criterion and by exhaustive root search); the curve is
//! non-singular; G, Q on it; l prime (Miller-Rabin and trial division);
//! G != O, l G = O; dd G = Q, dd = d, 0 < d < l.  For the rho reference: the
//! j-invariant is neither 0 nor 1728 (so Aut(E) = {+-1}) and lies in neither
//! F_{p^2} nor F_{p^3} (so no curve over a proper subfield is isomorphic to E
//! and no subfield Frobenius acts).  Extras (reported, not gating): the 2-torsion
//! is rational, 4l is in the Hasse interval, and 4l kills random points.
//! Self-tests and negative controls must behave, or the instance fails.

use crate::util::*;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::{json, Value};
use std::path::PathBuf;

type E2 = [u64; 2];
type E6 = [E2; 3];
type Pt = Option<(E6, E6)>;

#[derive(Clone, Copy)]
struct F {
    p: u64,
    w: u64,
    s: E2,
}

impl F {
    fn a2(&self, x: E2, y: E2) -> E2 {
        [(x[0] + y[0]) % self.p, (x[1] + y[1]) % self.p]
    }
    fn s2(&self, x: E2, y: E2) -> E2 {
        [(x[0] + self.p - y[0]) % self.p, (x[1] + self.p - y[1]) % self.p]
    }
    fn m2(&self, x: E2, y: E2) -> E2 {
        let p = self.p;
        [
            (x[0] * y[0] % p + self.w * (x[1] * y[1] % p)) % p,
            (x[0] * y[1] % p + x[1] * y[0] % p) % p,
        ]
    }
    fn pow2(&self, mut x: E2, mut e: u64) -> E2 {
        let mut r = [1, 0];
        while e > 0 {
            if e & 1 == 1 {
                r = self.m2(r, x);
            }
            x = self.m2(x, x);
            e >>= 1;
        }
        r
    }
    fn zero() -> E6 {
        [[0, 0]; 3]
    }
    fn one() -> E6 {
        [[1, 0], [0, 0], [0, 0]]
    }
    fn add(&self, x: E6, y: E6) -> E6 {
        [self.a2(x[0], y[0]), self.a2(x[1], y[1]), self.a2(x[2], y[2])]
    }
    fn sub(&self, x: E6, y: E6) -> E6 {
        [self.s2(x[0], y[0]), self.s2(x[1], y[1]), self.s2(x[2], y[2])]
    }
    fn neg(&self, x: E6) -> E6 {
        self.sub(F::zero(), x)
    }
    fn scal(&self, k: u64, x: E6) -> E6 {
        let p = self.p;
        let k = k % p;
        [
            [k * x[0][0] % p, k * x[0][1] % p],
            [k * x[1][0] % p, k * x[1][1] % p],
            [k * x[2][0] % p, k * x[2][1] % p],
        ]
    }
    fn mul(&self, x: E6, y: E6) -> E6 {
        let mut c = [[0u64; 2]; 5];
        for i in 0..3 {
            for j in 0..3 {
                c[i + j] = self.a2(c[i + j], self.m2(x[i], y[j]));
            }
        }
        // t^3 = s, t^4 = s t
        [self.a2(c[0], self.m2(self.s, c[3])), self.a2(c[1], self.m2(self.s, c[4])), c[2]]
    }
    fn pow(&self, mut x: E6, mut e: u64) -> E6 {
        let mut r = F::one();
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(r, x);
            }
            x = self.mul(x, x);
            e >>= 1;
        }
        r
    }
    fn q(&self) -> u64 {
        self.p.pow(6)
    }
    fn inv(&self, x: E6) -> E6 {
        assert!(x != F::zero(), "inverse of 0");
        self.pow(x, self.q() - 2)
    }
    fn rand(&self, rng: &mut StdRng) -> E6 {
        let p = self.p;
        let mut e = || [rng.gen_range(0..p), rng.gen_range(0..p)];
        [e(), e(), e()]
    }
    fn is_square(&self, a: E6) -> bool {
        a == F::zero() || self.pow(a, (self.q() - 1) / 2) == F::one()
    }
    /// Tonelli-Shanks in F_{p^6}.
    fn sqrt(&self, a: E6, rng: &mut StdRng) -> Option<E6> {
        if a == F::zero() {
            return Some(a);
        }
        if !self.is_square(a) {
            return None;
        }
        let q = self.q();
        let (mut e, mut m) = (0u32, q - 1);
        while m % 2 == 0 {
            e += 1;
            m /= 2;
        }
        let z = loop {
            let z = self.rand(rng);
            if z != F::zero() && self.pow(z, (q - 1) / 2) == self.neg(F::one()) {
                break z;
            }
        };
        let (mut c, mut x, mut t, mut r) = (self.pow(z, m), self.pow(a, (m + 1) / 2), self.pow(a, m), e);
        while t != F::one() {
            let (mut i, mut tt) = (0u32, t);
            while tt != F::one() {
                tt = self.mul(tt, tt);
                i += 1;
            }
            let b = self.pow(c, 1u64 << (r - i - 1));
            x = self.mul(x, b);
            c = self.mul(b, b);
            t = self.mul(t, c);
            r = i;
        }
        Some(x)
    }
}

struct Curve {
    f: F,
    a2: E6,
    a4: E6,
}

impl Curve {
    fn rhs(&self, x: E6) -> E6 {
        let f = &self.f;
        let x2 = f.mul(x, x);
        f.add(f.add(f.mul(x2, x), f.mul(self.a2, x2)), f.mul(self.a4, x))
    }
    fn on(&self, p: &Pt) -> bool {
        match p {
            None => true,
            Some((x, y)) => self.f.mul(*y, *y) == self.rhs(*x),
        }
    }
    fn neg(&self, p: &Pt) -> Pt {
        p.map(|(x, y)| (x, self.f.neg(y)))
    }
    fn add(&self, p: &Pt, q: &Pt) -> Pt {
        let f = &self.f;
        let (Some((x1, y1)), Some((x2, y2))) = (p, q) else {
            return if p.is_none() { *q } else { *p };
        };
        let lam = if x1 == x2 {
            if f.add(*y1, *y2) == F::zero() {
                return None;
            }
            let num = f.add(f.add(f.scal(3, f.mul(*x1, *x1)), f.scal(2, f.mul(self.a2, *x1))), self.a4);
            f.mul(num, f.inv(f.scal(2, *y1)))
        } else {
            f.mul(f.sub(*y2, *y1), f.inv(f.sub(*x2, *x1)))
        };
        let x3 = f.sub(f.sub(f.sub(f.mul(lam, lam), self.a2), *x1), *x2);
        let y3 = f.sub(f.mul(lam, f.sub(*x1, x3)), *y1);
        Some((x3, y3))
    }
    fn mul(&self, mut k: u64, p: &Pt) -> Pt {
        let (mut r, mut b) = (None, *p);
        while k > 0 {
            if k & 1 == 1 {
                r = self.add(&r, &b);
            }
            b = self.add(&b, &b);
            k >>= 1;
        }
        r
    }
}

fn mulmod(a: u64, b: u64, n: u64) -> u64 {
    ((a as u128 * b as u128) % n as u128) as u64
}

fn powmod(mut a: u64, mut e: u64, n: u64) -> u64 {
    let mut r = 1 % n;
    a %= n;
    while e > 0 {
        if e & 1 == 1 {
            r = mulmod(r, a, n);
        }
        a = mulmod(a, a, n);
        e >>= 1;
    }
    r
}

/// Deterministic Miller-Rabin (bases up to 37: exact below 3.3e24).
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let small = [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    for &q in &small {
        if n % q == 0 {
            return n == q;
        }
    }
    let (mut d, mut r) = (n - 1, 0);
    while d % 2 == 0 {
        d /= 2;
        r += 1;
    }
    'base: for &a in &small {
        let mut x = powmod(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 0..r - 1 {
            x = mulmod(x, x, n);
            if x == n - 1 {
                continue 'base;
            }
        }
        return false;
    }
    true
}

fn trial_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut i = 2u64;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += if i == 2 { 1 } else { 2 };
    }
    true
}

fn e2(v: &Value) -> E2 {
    [v[0].as_u64().unwrap(), v[1].as_u64().unwrap()]
}

fn e6(v: &Value) -> E6 {
    [e2(&v[0]), e2(&v[1]), e2(&v[2])]
}

fn pt(v: &Value) -> Pt {
    if v["inf"].as_bool().unwrap() {
        None
    } else {
        Some((e6(&v["x"]), e6(&v["y"])))
    }
}

/// Verify one dumped instance; returns the per-check record.
pub fn verify_instance(inst: &Value, rng: &mut StdRng) -> Value {
    let p = inst["p"].as_u64().unwrap();
    let w = inst["w"].as_u64().unwrap() % p;
    let s = e2(&inst["s"]);
    let f = F { p, w, s };
    let l = inst["l"].as_u64().unwrap();
    let d = inst["d"].as_u64().unwrap();
    let dd = inst["dd"].as_u64().unwrap();
    let e = Curve { f, a2: e6(&inst["a2"]), a4: e6(&inst["a4"]) };
    let g = pt(&inst["g"]);
    let qp = pt(&inst["q"]);
    let mut c = serde_json::Map::new();
    let mut put = |k: &str, v: bool| {
        c.insert(k.to_string(), json!(v));
    };
    // tower
    put("w_nonresidue_euler", powmod(w, (p - 1) / 2, p) == p - 1);
    put("w_no_root_exhaustive", (0..p).all(|a| (a * a + p - w) % p != 0));
    let s_euler = p % 3 != 0 && s != [0, 0] && f.pow2(s, (p * p - 1) / 3) != [1, 0];
    put("s_noncube_euler", s_euler);
    let mut s_is_cube = false;
    for a in 0..p {
        for b in 0..p {
            let t = [a, b];
            if f.m2(f.m2(t, t), t) == s {
                s_is_cube = true;
            }
        }
    }
    put("s_no_root_exhaustive", !s_is_cube);
    // curve and points
    let fe = &e.f;
    let disc = fe.scal(16, fe.mul(fe.mul(e.a4, e.a4), fe.sub(fe.mul(e.a2, e.a2), fe.scal(4, e.a4))));
    put("curve_nonsingular", disc != F::zero());
    put("G_on_curve", g.is_some() && e.on(&g));
    put("Q_on_curve", qp.is_some() && e.on(&qp));
    put("l_prime_miller_rabin", is_prime(l));
    put("l_prime_trial_division", trial_prime(l));
    put("G_not_O", g.is_some());
    put("lG_is_O", e.mul(l, &g).is_none());
    put("ddG_eq_Q", e.mul(dd, &g) == qp);
    put("dd_eq_d", dd == d);
    put("d_in_range", 0 < d && d < l);
    // j-invariant: c4 = 16 a2^2 - 48 a4, c6 = -64 a2^3 + 288 a2 a4, 1728 Delta = c4^3 - c6^2
    let a2sq = fe.mul(e.a2, e.a2);
    let c4 = fe.sub(fe.scal(16, a2sq), fe.scal(48, e.a4));
    let c6 = fe.sub(fe.scal(288, fe.mul(e.a2, e.a4)), fe.scal(64, fe.mul(a2sq, e.a2)));
    let c4cube = fe.mul(fe.mul(c4, c4), c4);
    let disc_identity = fe.scal(1728 % p, disc) == fe.sub(c4cube, fe.mul(c6, c6));
    let j = fe.mul(c4cube, fe.inv(disc));
    put("j_disc_identity_1728D_eq_c4c4c4_minus_c6c6", disc_identity);
    put("j_not_0", c4 != F::zero());
    put("j_not_1728", c6 != F::zero());
    put("j_not_in_F_p2", fe.pow(j, p * p) != j);
    put("j_not_in_F_p3", fe.pow(j, p * p * p) != j);
    // negative controls: each must be false
    let gpt = g.unwrap();
    let neg = json!({
        "dd_plus_1_times_G_eq_Q": e.mul(dd + 1, &g) == qp,
        "perturbed_G_on_curve": e.on(&Some((gpt.0, fe.add(gpt.1, F::one())))),
        "composite_3l_prime": is_prime(3 * l),
        "residue_4_passes_nonresidue_test": powmod(4 % p, (p - 1) / 2, p) == p - 1,
        "cube_s_passes_noncube_test": f.pow2(f.m2(f.m2([2, 1], [2, 1]), [2, 1]), (p * p - 1) / 3) != [1, 0],
        "l_minus_1_times_G_is_O": e.mul(l - 1, &g).is_none(),
        "j_of_subfield_value_passes_subfield_test": fe.pow([[5, 0], [0, 0], [0, 0]], p * p) != [[5, 0], [0, 0], [0, 0]],
    });
    let neg_ok = neg.as_object().unwrap().values().all(|v| v == &json!(false));
    // self-tests: each must be true
    let (x, y, z) = (fe.rand(rng), fe.rand(rng), fe.rand(rng));
    let (a, b) = (rng.gen_range(1..l), rng.gen_range(1..l));
    let st = json!({
        "mul_assoc": fe.mul(fe.mul(x, y), z) == fe.mul(x, fe.mul(y, z)),
        "distrib": fe.mul(x, fe.add(y, z)) == fe.add(fe.mul(x, y), fe.mul(x, z)),
        "inverse": x == F::zero() || fe.mul(x, fe.inv(x)) == F::one(),
        "frobenius_order_x_pow_q_eq_x": fe.pow(x, fe.q()) == x,
        "scalar_linearity": e.add(&e.mul(a, &g), &e.mul(b, &g)) == e.mul(a + b, &g),
        "l_minus_1_is_neg": e.mul(l - 1, &g) == e.neg(&g),
        "results_on_curve": e.on(&e.mul(a, &g)) && e.on(&e.add(&g, &qp)),
    });
    let st_ok = st.as_object().unwrap().values().all(|v| v == &json!(true));
    // extras, not gating
    let q = fe.q();
    let (lo, hi) = (q + 1 - 2 * p.pow(3), q + 1 + 2 * p.pow(3));
    let mut kills = true;
    let mut found = 0;
    while found < 3 {
        let xr = fe.rand(rng);
        if let Some(yr) = fe.sqrt(e.rhs(xr), rng) {
            let r = Some((xr, yr));
            kills &= e.on(&r) && e.mul(4 * l, &r).is_none();
            found += 1;
        }
    }
    let two_torsion_rational = fe.is_square(fe.sub(a2sq, fe.scal(4, e.a4)));
    let gating = [
        "w_nonresidue_euler", "w_no_root_exhaustive", "s_noncube_euler", "s_no_root_exhaustive",
        "curve_nonsingular", "G_on_curve", "Q_on_curve", "l_prime_miller_rabin", "l_prime_trial_division",
        "G_not_O", "lG_is_O", "ddG_eq_Q", "dd_eq_d", "d_in_range",
    ];
    let rho_ref = ["j_disc_identity_1728D_eq_c4c4c4_minus_c6c6", "j_not_0", "j_not_1728", "j_not_in_F_p2", "j_not_in_F_p3"];
    let verified = gating.iter().all(|k| c[*k] == json!(true)) && neg_ok && st_ok;
    let aut_pm1_no_frobenius = rho_ref.iter().all(|k| c[*k] == json!(true));
    let mut out = json!({
        "p": p, "seed": inst["seed"], "lin": inst["lin"], "l": l, "d": d, "dd": dd,
        "checks": Value::Object(c),
        "negative_controls": neg, "negative_controls_all_false": neg_ok,
        "self_tests": st, "self_tests_all_true": st_ok,
        "extra_4l_in_hasse_interval": lo <= 4 * l && 4 * l <= hi,
        "extra_4l_kills_3_random_points": kills,
        "extra_2_torsion_rational": two_torsion_rational,
        "gating_checks": gating,
        "verified": verified,
        "aut_is_pm1_and_no_subfield_frobenius": aut_pm1_no_frobenius,
    });
    out["j"] = json!(j.iter().map(|c| json!([c[0], c[1]])).collect::<Vec<_>>());
    out
}

fn lines(b: &[u8]) -> Vec<String> {
    String::from_utf8_lossy(b).lines().filter(|l| !l.trim().is_empty()).map(|s| s.to_string()).collect()
}

/// `vtool verify-scalars --snapshot C --out FILE`
pub fn main(args: &[String]) {
    let snap = full_commit(&need(args, "--snapshot"));
    let out = PathBuf::from(need(args, "--out"));
    let round2 = PathBuf::from(REPO).join(STUDY).join("validation/round2");
    let nonce = std::fs::read_to_string(round2.join("nonce.txt")).unwrap();
    let seed = u64::from_str_radix(&sha_hex(nonce.trim().as_bytes())[..16], 16).unwrap();
    let mut rng = StdRng::seed_from_u64(seed);
    let mut led = Ledger::default();
    let mut insts = Vec::new();
    let mut inputs = serde_json::Map::new();
    let sets: [(&str, &str, &str); 4] = [
        ("R3-D1", "runs/R3-D1.instances.jsonl", "sparse"),
        ("R3-D2", "runs/R3-D2.instances.jsonl", "fast"),
        ("round1-dump-sparse", "validation/scalar-dump/instances-sparse.jsonl", "sparse"),
        ("round1-dump-fast", "validation/scalar-dump/instances-fast.jsonl", "fast"),
    ];
    let mut by_set: std::collections::BTreeMap<String, Vec<(u64, u64, String)>> = Default::default();
    for (tag, rel_path, arm) in sets {
        let path = format!("{STUDY}/{rel_path}");
        let raw = git_show(&snap, &path);
        inputs.insert(path.clone(), json!(sha_hex(&raw)));
        for line in lines(&raw) {
            let v: Value = serde_json::from_str(&line).unwrap();
            let key = (v["p"].as_u64().unwrap(), v["seed"].as_u64().unwrap());
            led.check(&format!("{tag} {key:?}: lin label = {arm}"), v["lin"] == json!(arm), v["lin"].clone());
            let mut r = verify_instance(&v, &mut rng);
            r["set"] = json!(tag);
            led.check(&format!("{tag} {key:?}: verified (tower, curve, l, dd G = Q, dd = d)"), r["verified"] == json!(true), Value::Null);
            led.check(&format!("{tag} {key:?}: Aut(E) = +-1, j in no proper subfield"), r["aut_is_pm1_and_no_subfield_frobenius"] == json!(true), r["j"].clone());
            insts.push(r);
            by_set.entry(tag.to_string()).or_default().push((key.0, key.1, line));
        }
    }
    // ties: the R3 dumps belong to the R3-D rows, and equal the round-1 dumps
    for (tag, card) in [("R3-D1", "R3-D1"), ("R3-D2", "R3-D2")] {
        let rows = git_json(&snap, &format!("{STUDY}/runs/{card}.json"));
        let rows = rows.as_array().unwrap();
        let mine = &by_set[tag];
        let mut keys: Vec<(u64, u64)> = mine.iter().map(|(p, s, _)| (*p, *s)).collect();
        keys.sort();
        led.check(&format!("{tag}: one dump per solved row, (p, seed) = {{53,101}} x {{1,2}}"),
                  keys == vec![(53, 1), (53, 2), (101, 1), (101, 2)] && rows.len() == 4, json!(keys));
        for (p, s, line) in mine {
            let v: Value = serde_json::from_str(line).unwrap();
            let row = rows.iter().find(|r| r["p"] == json!(p) && r["seed"] == json!(s)).unwrap();
            led.check(&format!("{tag} ({p}, {s}): dumped l = row l, row solved and correct"),
                      v["l"] == row["l"] && row["solved"] == json!(true) && row["correct"] == json!(true), Value::Null);
        }
    }
    for (a, b) in [("R3-D1", "R3-D2"), ("R3-D1", "round1-dump-sparse"), ("R3-D2", "round1-dump-fast")] {
        for (p, s, la) in &by_set[a] {
            let lb = by_set[b].iter().find(|(q, t, _)| q == p && t == s).map(|x| x.2.clone()).unwrap_or_default();
            let (va, vb): (Value, Value) = (serde_json::from_str(la).unwrap(), serde_json::from_str(&lb).unwrap_or(Value::Null));
            let same = if a == "R3-D1" && b == "R3-D2" {
                ["w", "s", "a2", "a4", "l", "g", "q", "d", "dd"].iter().all(|k| va[*k] == vb[*k])
            } else {
                la == &lb
            };
            led.check(&format!("({p}, {s}): {a} instance {} {b}", if a == "R3-D1" && b == "R3-D2" { "equals (all but lin)" } else { "line byte-identical to" }), same, Value::Null);
        }
    }
    // the validator's own round-2 dump-binary runs
    let mut own = Vec::new();
    if let Ok(rd) = std::fs::read_dir(round2.join("out")) {
        let mut paths: Vec<PathBuf> = rd.filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.to_string_lossy().ends_with(".instances.jsonl")).collect();
        paths.sort();
        for path in paths {
            let raw = std::fs::read(&path).unwrap();
            for line in lines(&raw) {
                let v: Value = serde_json::from_str(&line).unwrap();
                let tag = if v["lin"] == json!("sparse") { "R3-D1" } else { "R3-D2" };
                let key = (v["p"].as_u64().unwrap(), v["seed"].as_u64().unwrap());
                let mut r = verify_instance(&v, &mut rng);
                r["set"] = json!(format!("validator-replay:{}", path.file_name().unwrap().to_string_lossy()));
                let same = by_set[tag].iter().any(|(p, s, l)| (*p, *s) == key && l == &line);
                led.check(&format!("own replay {key:?} {}: verified, and line byte-identical to {tag}", v["lin"]),
                          r["verified"] == json!(true) && same, Value::Null);
                own.push(json!({"file": path.strip_prefix(REPO).unwrap().to_string_lossy(), "sha256": sha_hex(&raw),
                                "key": [key.0, key.1], "lin": v["lin"], "byte_identical_to": tag, "identical": same}));
                insts.push(r);
            }
        }
    }
    let all = led.problems.is_empty();
    let n_ver = insts.iter().filter(|r| r["verified"] == json!(true)).count();
    let report = json!({
        "schema": "postgb-la-scalar-verification/v2",
        "verifier": "validation/vtool (Rust; scalars.rs), the validator's own field and curve arithmetic",
        "independent_arithmetic": true,
        "snapshot_commit": snap,
        "inputs": inputs,
        "instances_verified": n_ver,
        "instances_total": insts.len(),
        "own_replay_dumps": own,
        "all_verified": all,
        "checks": led.checks,
        "problems": led.problems,
        "instances": insts,
    });
    write_json(&out, &report);
    println!("verify-scalars: {n_ver}/{} instances verified; {} checks, {} problems", insts.len(), report["checks"].as_array().unwrap().len(), report["problems"].as_array().unwrap().len());
    for p in report["problems"].as_array().unwrap() {
        println!("PROBLEM {p}");
    }
}
