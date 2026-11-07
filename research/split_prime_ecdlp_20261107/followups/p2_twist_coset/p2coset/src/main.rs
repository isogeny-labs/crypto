//! p2coset: instrument for IDEA-20261107-TWIST-PURE-COSET (P2).
//! Standalone, u128 field arithmetic, p < 2^32.
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap, HashSet};

// ---------------------------------------------------------------- rng
struct Rng(u64);
impl Rng {
    fn new(seed: u64) -> Rng { Rng(seed ^ 0x9E37_79B9_7F4A_7C15) }
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: u64) -> u64 { self.next() % n }
}

// ---------------------------------------------------------------- Fp
fn mulm(a: u64, b: u64, p: u64) -> u64 { ((a as u128 * b as u128) % p as u128) as u64 }
fn addm(a: u64, b: u64, p: u64) -> u64 { let s = a + b; if s >= p { s - p } else { s } }
fn subm(a: u64, b: u64, p: u64) -> u64 { if a >= b { a - b } else { a + p - b } }
fn negm(a: u64, p: u64) -> u64 { if a == 0 { 0 } else { p - a } }
fn powm(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1 % p;
    a %= p;
    while e > 0 { if e & 1 == 1 { r = mulm(r, a, p); } a = mulm(a, a, p); e >>= 1; }
    r
}
fn invm(a: u64, p: u64) -> u64 {
    let (mut r0, mut r1) = (p as i128, (a % p) as i128);
    let (mut t0, mut t1) = (0i128, 1i128);
    while r1 != 0 {
        let q = r0 / r1;
        (r0, r1) = (r1, r0 - q * r1);
        (t0, t1) = (t1, t0 - q * t1);
    }
    assert!(r0 == 1, "not invertible");
    (((t0 % p as i128) + p as i128) % p as i128) as u64
}
fn is_square(a: u64, p: u64) -> bool { a != 0 && powm(a, (p - 1) / 2, p) == 1 }
fn sqrtm(a: u64, p: u64) -> Option<u64> {
    if a == 0 { return Some(0); }
    if !is_square(a, p) { return None; }
    if p % 4 == 3 { return Some(powm(a, (p + 1) / 4, p)); }
    let (mut q, mut s) = (p - 1, 0u32);
    while q % 2 == 0 { q /= 2; s += 1; }
    let mut z = 2;
    while is_square(z, p) { z += 1; }
    let (mut m, mut c, mut t, mut r) = (s, powm(z, q, p), powm(a, q, p), powm(a, (q + 1) / 2, p));
    while t != 1 {
        let mut i = 0; let mut tt = t;
        while tt != 1 { tt = mulm(tt, tt, p); i += 1; }
        let b = powm(c, 1u64 << (m - i - 1), p);
        m = i; c = mulm(b, b, p); t = mulm(t, c, p); r = mulm(r, b, p);
    }
    Some(r)
}
fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    let mut d = 2;
    while d * d <= n { if n % d == 0 { return false; } d += 1; }
    true
}

// F_{p^2} = F_p(i), i^2 = -1 (p = 3 mod 4)
type C2 = (u64, u64);
fn c2mul(a: C2, b: C2, p: u64) -> C2 {
    (subm(mulm(a.0, b.0, p), mulm(a.1, b.1, p), p), addm(mulm(a.0, b.1, p), mulm(a.1, b.0, p), p))
}
fn c2pow(mut a: C2, mut e: u64, p: u64) -> C2 {
    let mut r = (1, 0);
    while e > 0 { if e & 1 == 1 { r = c2mul(r, a, p); } a = c2mul(a, a, p); e >>= 1; }
    r
}

// ---------------------------------------------------------------- curve
#[derive(Clone, Copy, Debug)]
struct Curve { p: u64, a: u64, b: u64 }
type Pt = Option<(u64, u64)>;
impl Curve {
    fn f(&self, x: u64) -> u64 {
        let p = self.p;
        addm(addm(mulm(mulm(x, x, p), x, p), mulm(self.a, x, p), p), self.b, p)
    }
    fn neg(&self, q: Pt) -> Pt { q.map(|(x, y)| (x, negm(y, self.p))) }
    fn add(&self, q1: Pt, q2: Pt) -> Pt {
        let p = self.p;
        let (a, b) = match (q1, q2) { (None, r) | (r, None) => return r, (Some(a), Some(b)) => (a, b) };
        let lam;
        if a.0 == b.0 {
            if addm(a.1, b.1, p) == 0 { return None; }
            let num = addm(mulm(3, mulm(a.0, a.0, p), p), self.a, p);
            lam = mulm(num, invm(mulm(2, a.1, p), p), p);
        } else {
            lam = mulm(subm(b.1, a.1, p), invm(subm(b.0, a.0, p), p), p);
        }
        let x3 = subm(subm(mulm(lam, lam, p), a.0, p), b.0, p);
        let y3 = subm(mulm(lam, subm(a.0, x3, p), p), a.1, p);
        Some((x3, y3))
    }
    fn mul(&self, mut k: u64, q: Pt) -> Pt {
        let mut r = None; let mut a = q;
        while k > 0 { if k & 1 == 1 { r = self.add(r, a); } a = self.add(a, a); k >>= 1; }
        r
    }
    fn rand_point(&self, rng: &mut Rng) -> (u64, u64) {
        loop {
            let x = rng.below(self.p);
            if let Some(y) = sqrtm(self.f(x), self.p) { if y != 0 { return (x, y); } }
        }
    }
    fn legendre_count(&self) -> u64 {
        let mut n = 1u64;
        for x in 0..self.p {
            let v = self.f(x);
            if v == 0 { n += 1; } else if is_square(v, self.p) { n += 2; }
        }
        n
    }
    /// all k in [lo,hi] with kP = O by BSGS
    fn bsgs_candidates(&self, pt: (u64, u64), lo: u64, hi: u64) -> Vec<u64> {
        let w = hi - lo + 1;
        let s = (w as f64).sqrt().ceil() as u64 + 1;
        let mut table: HashMap<Pt, Vec<u64>> = HashMap::new();
        let mut cur: Pt = None;
        for j in 0..s { table.entry(cur).or_default().push(j); cur = self.add(cur, Some(pt)); }
        let step = self.mul(s, Some(pt));
        let mut g = self.mul(lo, Some(pt));
        let mut out = vec![];
        let mut i = 0u64;
        while i * s <= w + s {
            if let Some(js) = table.get(&self.neg(g)) {
                for &j in js { let k = lo + i * s + j; if k >= lo && k <= hi { out.push(k); } }
            }
            g = self.add(g, step); i += 1;
        }
        out.sort(); out.dedup(); out
    }
    /// exact order by BSGS intersection over random points, then verified
    fn order(&self, rng: &mut Rng) -> Option<u64> {
        let r = (2.0 * (self.p as f64).sqrt()).ceil() as u64 + 1;
        let (lo, hi) = (self.p + 1 - r, self.p + 1 + r);
        let mut cands: Option<Vec<u64>> = None;
        for _ in 0..12 {
            let pt = self.rand_point(rng);
            let c = self.bsgs_candidates(pt, lo, hi);
            cands = Some(match cands { None => c, Some(old) => old.into_iter().filter(|k| c.contains(k)).collect() });
            if cands.as_ref().unwrap().len() == 1 { break; }
        }
        let c = cands?;
        if c.len() != 1 { return None; }
        Some(c[0])
    }
}
struct CurveInfo { curve: Curve, n: u64 }
fn find_curve(p: u64, cseed: u64) -> CurveInfo {
    let mut rng = Rng::new(cseed ^ p.rotate_left(17));
    loop {
        let a = 1 + rng.below(p - 1);
        let b = 1 + rng.below(p - 1);
        let c = Curve { p, a, b };
        let disc = addm(mulm(4, powm(a, 3, p), p), mulm(27, mulm(b, b, p), p), p);
        if disc == 0 { continue; } // a,b nonzero => j not in {0,1728}
        if let Some(n) = c.order(&mut rng) {
            if !is_prime(n) { continue; }
            // verify n*P = O on several random points, and Hasse
            let r = (2.0 * (p as f64).sqrt()).ceil() as u64 + 1;
            assert!(n + r >= p + 1 && n <= p + 1 + r);
            let ok = (0..8).all(|_| { let q = c.rand_point(&mut rng); c.mul(n, Some(q)).is_none() });
            assert!(ok, "n*P != O");
            return CurveInfo { curve: c, n };
        }
    }
}
fn curve_json(ci: &CurveInfo, cseed: u64) -> Value {
    let c = ci.curve;
    let tr = c.p as i64 + 1 - ci.n as i64;
    let jnum = mulm(1728 * 4 % c.p, powm(c.a, 3, c.p), c.p);
    let disc = addm(mulm(4, powm(c.a, 3, c.p), c.p), mulm(27, mulm(c.b, c.b, c.p), c.p), c.p);
    let j = mulm(jnum, invm(disc, c.p), c.p);
    json!({"p": c.p, "a": c.a, "b": c.b, "n": ci.n, "trace": tr, "n_is_prime": is_prime(ci.n),
           "j": j, "cseed": cseed, "hasse_ok": tr.unsigned_abs() as f64 <= 2.0 * (c.p as f64).sqrt(),
           "q_curve": (ci.n - 1) as f64 / 2.0 / c.p as f64})
}

// ---------------------------------------------------------------- cosets
#[derive(Clone, Copy, PartialEq, Debug)]
enum Kind { Kummer, Dickson }
fn parse_kind(s: &str) -> Kind {
    match s { "kummer" => Kind::Kummer, "dickson" => Kind::Dickson, _ => panic!("kind must be kummer|dickson") }
}
struct Coset { c: u64, elems: Vec<u64>, is_e: Vec<bool>, c_e: usize }
fn tower_step(kind: Kind, y: u64, p: u64) -> u64 {
    let s = mulm(y, y, p);
    if kind == Kind::Dickson { subm(s, 2, p) } else { s }
}
fn gen_cosets(kind: Kind, cv: &Curve, t: u32, seed: u64, cnt: usize) -> Result<Vec<Coset>, String> {
    let p = cv.p;
    let nn = 1u64 << t;
    let mut rng = Rng::new(seed ^ 0xC05E7 ^ ((t as u64) << 40));
    let mut seen = HashSet::new();
    let mut out = vec![];
    let mut tries = 0usize;
    let class = |x: u64| is_square(cv.f(x), p);
    match kind {
        Kind::Kummer => {
            if (p - 1) % nn != 0 { return Err(format!("2^{t} does not divide p-1")); }
            let zeta = loop {
                let h = 2 + rng.below(p - 2);
                let z = powm(h, (p - 1) / nn, p);
                if powm(z, nn / 2, p) == p - 1 { break z; }
            };
            while out.len() < cnt && tries < cnt * 60 + 2000 {
                tries += 1;
                let x0 = 1 + rng.below(p - 1);
                let c = powm(x0, nn, p);
                if !seen.insert(c) { continue; }
                let mut e = x0; let mut elems = vec![];
                for _ in 0..nn { elems.push(e); e = mulm(e, zeta, p); }
                out.push(finish(c, elems, &class));
            }
        }
        Kind::Dickson => {
            if (p + 1) % nn != 0 || t < 2 { return Err(format!("need t>=2 and 2^{t} | p+1")); }
            let zeta = loop {
                let z0 = (rng.below(p), rng.below(p));
                if z0 == (0, 0) { continue; }
                let h = c2pow(z0, p - 1, p); // norm-1 torus element
                let z = c2pow(h, (p + 1) / nn, p);
                if c2pow(z, nn / 2, p) == (p - 1, 0) { break z; }
            };
            while out.len() < cnt && tries < cnt * 60 + 2000 {
                tries += 1;
                let z0 = (rng.below(p), rng.below(p));
                if z0 == (0, 0) { continue; }
                let u = c2pow(z0, p - 1, p);
                if c2pow(u, 2 * nn, p) == (1, 0) { continue; } // cosets u mu and u^-1 mu must be disjoint
                let un = c2pow(u, nn, p);
                let c = mulm(2, un.0, p);
                if c == 2 || c == p - 2 || !seen.insert(c) { continue; }
                let mut w = u; let mut elems = vec![];
                for _ in 0..nn { elems.push(mulm(2, w.0, p)); w = c2mul(w, zeta, p); }
                let set: HashSet<_> = elems.iter().collect();
                if set.len() != nn as usize { seen.remove(&c); continue; }
                out.push(finish(c, elems, &class));
            }
        }
    }
    if out.len() < cnt { return Err(format!("only {} distinct valid cosets found (asked {})", out.len(), cnt)); }
    Ok(out)
}
fn finish(c: u64, elems: Vec<u64>, class: &dyn Fn(u64) -> bool) -> Coset {
    let is_e: Vec<bool> = elems.iter().map(|&x| class(x)).collect();
    let c_e = is_e.iter().filter(|&&b| b).count();
    Coset { c, elems, is_e, c_e }
}
fn select<'a>(cos: &'a [Coset], how: &str) -> (usize, &'a Coset) {
    let i = if how == "max" {
        (0..cos.len()).max_by_key(|&i| (cos[i].c_e, std::cmp::Reverse(i))).unwrap()
    } else if how == "min" {
        (0..cos.len()).min_by_key(|&i| (cos[i].c_e, i)).unwrap()
    } else if how == "med" {
        let mut ix: Vec<usize> = (0..cos.len()).collect();
        ix.sort_by_key(|&i| (cos[i].c_e, i));
        ix[ix.len() / 2]
    } else if how == "random" { 0 } else if let Some(k) = how.strip_prefix("idx:") { k.parse().unwrap() } else { panic!("select = max|med|min|random|idx:K") };
    (i, &cos[i])
}

/// exact E[max of C iid Binomial(nn,q)]
fn exp_max_binom(nn: u64, q: f64, cc: u64) -> f64 {
    let mut lf = vec![0.0f64; nn as usize + 1];
    for i in 1..=nn as usize { lf[i] = lf[i - 1] + (i as f64).ln(); }
    let pmf: Vec<f64> = (0..=nn as usize).map(|k| {
        (lf[nn as usize] - lf[k] - lf[nn as usize - k] + k as f64 * q.ln() + (nn as usize - k) as f64 * (1.0 - q).ln()).exp()
    }).collect();
    let mut e = 0.0;
    for k in 0..nn as usize {
        let u: f64 = pmf[k + 1..].iter().sum(); // P(X > k)
        e += if u >= 1.0 { 1.0 } else { -((cc as f64) * (-u).ln_1p()).exp_m1() };
    }
    e
}

// ---------------------------------------------------------------- polynomials
#[derive(Clone)]
struct Poly { nv: usize, p: u64, t: BTreeMap<Vec<u8>, u64> }
impl Poly {
    fn zero(nv: usize, p: u64) -> Poly { Poly { nv, p, t: BTreeMap::new() } }
    fn cst(nv: usize, p: u64, c: u64) -> Poly { let mut z = Poly::zero(nv, p); if c % p != 0 { z.t.insert(vec![0; nv], c % p); } z }
    fn var(nv: usize, p: u64, i: usize) -> Poly { let mut m = vec![0; nv]; m[i] = 1; let mut z = Poly::zero(nv, p); z.t.insert(m, 1); z }
    fn addto(&mut self, m: Vec<u8>, c: u64) {
        let p = self.p;
        let e = self.t.entry(m.clone()).or_insert(0);
        *e = addm(*e, c, p);
        if *e == 0 { self.t.remove(&m); }
    }
    fn add(&self, o: &Poly) -> Poly { let mut r = self.clone(); for (m, &c) in &o.t { r.addto(m.clone(), c); } r }
    fn neg(&self) -> Poly { let mut r = self.clone(); for c in r.t.values_mut() { *c = negm(*c, self.p); } r }
    fn sub(&self, o: &Poly) -> Poly { self.add(&o.neg()) }
    fn scale(&self, k: u64) -> Poly {
        let mut r = Poly::zero(self.nv, self.p);
        for (m, &c) in &self.t { r.addto(m.clone(), mulm(c, k % self.p, self.p)); }
        r
    }
    fn mul(&self, o: &Poly) -> Poly {
        let mut r = Poly::zero(self.nv, self.p);
        for (m1, &c1) in &self.t { for (m2, &c2) in &o.t {
            let m: Vec<u8> = m1.iter().zip(m2).map(|(a, b)| a + b).collect();
            r.addto(m, mulm(c1, c2, self.p));
        } }
        r
    }
    fn eval(&self, v: &[u64]) -> u64 {
        let p = self.p; let mut s = 0;
        for (m, &c) in &self.t {
            let mut t = c;
            for (i, &e) in m.iter().enumerate() { if e > 0 { t = mulm(t, powm(v[i], e as u64, p), p); } }
            s = addm(s, t, p);
        }
        s
    }
    fn to_ms(&self, names: &[String]) -> String {
        if self.t.is_empty() { return "0".into(); }
        let mut parts = vec![];
        for (m, &c) in &self.t {
            let mut f = vec![];
            for (i, &e) in m.iter().enumerate() {
                if e == 1 { f.push(names[i].clone()); } else if e > 1 { f.push(format!("{}^{}", names[i], e)); }
            }
            parts.push(if f.is_empty() { format!("{c}") } else if c == 1 { f.join("*") } else { format!("{c}*{}", f.join("*")) });
        }
        parts.join("+")
    }
}
/// coefficients (in z) of S_3(u,v,z) = a2 z^2 + a1 z + a0, y^2 = x^3 + a x + b
fn s3_coeffs(u: &Poly, v: &Poly, a: u64, b: u64) -> (Poly, Poly, Poly) {
    let (nv, p) = (u.nv, u.p);
    let uv = u.mul(v);
    let a2 = u.sub(v).mul(&u.sub(v));
    let a1 = u.add(v).mul(&uv.add(&Poly::cst(nv, p, a))).add(&Poly::cst(nv, p, mulm(2, b, p))).scale(p - 2);
    let t = uv.sub(&Poly::cst(nv, p, a));
    let a0 = t.mul(&t).sub(&u.add(v).scale(mulm(4, b, p)));
    (a2, a1, a0)
}
fn resultant2(a: &(Poly, Poly, Poly), b: &(Poly, Poly, Poly)) -> Poly {
    let (a2, a1, a0) = a; let (b2, b1, b0) = b;
    let d1 = a2.mul(b0).sub(&a0.mul(b2));
    let d2 = a2.mul(b1).sub(&a1.mul(b2));
    let d3 = a1.mul(b0).sub(&a0.mul(b1));
    d1.mul(&d1).sub(&d2.mul(&d3))
}
/// S_{m+1}(x_1..x_m, xr) as polynomial in nv variables (x_i = var i), m in {2,3}
fn summation_poly(m: usize, nv: usize, cv: &Curve, xr: u64) -> Poly {
    let p = cv.p;
    let x = |i| Poly::var(nv, p, i);
    let r = Poly::cst(nv, p, xr);
    match m {
        2 => {
            let (a2, a1, a0) = s3_coeffs(&x(0), &x(1), cv.a, cv.b);
            a2.mul(&r).mul(&r).add(&a1.mul(&r)).add(&a0)
        }
        3 => resultant2(&s3_coeffs(&x(0), &x(1), cv.a, cv.b), &s3_coeffs(&x(2), &r, cv.a, cv.b)),
        _ => panic!("m must be 2 or 3"),
    }
}
fn var_names(m: usize, t: u32) -> Vec<String> {
    let mut v: Vec<String> = (1..=m).map(|i| format!("x{i}")).collect();
    for i in 1..=m { for j in 1..t { v.push(format!("y{i}_{j}")); } }
    v
}
fn vidx_y(m: usize, t: u32, i: usize, j: u32) -> usize { // i in 0..m, level j in 0..t-1 (j=0 is x_i)
    if j == 0 { i } else { m + i * (t as usize - 1) + (j as usize - 1) }
}
/// tower equations as polys in mt variables (c is the coset constant)
fn tower_eqs(kind: Kind, m: usize, t: u32, p: u64, c: u64) -> Vec<Poly> {
    let nv = m * t as usize;
    let mut eqs = vec![];
    let step = |y: &Poly| { let s = y.mul(y); if kind == Kind::Dickson { s.sub(&Poly::cst(nv, p, 2)) } else { s } };
    for i in 0..m {
        for j in 1..t {
            let prev = Poly::var(nv, p, vidx_y(m, t, i, j - 1));
            eqs.push(Poly::var(nv, p, vidx_y(m, t, i, j)).sub(&step(&prev)));
        }
        let last = Poly::var(nv, p, vidx_y(m, t, i, t - 1));
        eqs.push(step(&last).sub(&Poly::cst(nv, p, c)));
    }
    eqs
}

// ---------------------------------------------------------------- args
struct Args(HashMap<String, String>);
impl Args {
    fn new(v: &[String]) -> Args {
        let mut m = HashMap::new();
        let mut i = 0;
        while i < v.len() {
            let k = v[i].trim_start_matches("--").to_string();
            let val = if i + 1 < v.len() && !v[i + 1].starts_with("--") { i += 1; v[i].clone() } else { "true".into() };
            m.insert(k, val); i += 1;
        }
        Args(m)
    }
    fn s(&self, k: &str) -> String { self.0.get(k).unwrap_or_else(|| panic!("missing --{k}")).clone() }
    fn u(&self, k: &str) -> u64 { self.s(k).parse().unwrap_or_else(|_| panic!("bad --{k}")) }
    fn uo(&self, k: &str, d: u64) -> u64 { self.0.get(k).map(|s| s.parse().unwrap()).unwrap_or(d) }
    fn so(&self, k: &str, d: &str) -> String { self.0.get(k).cloned().unwrap_or(d.into()) }
}
fn curve_from(a: &Args) -> CurveInfo {
    let p = a.u("p");
    assert!(p < (1 << 32) && is_prime(p) && p > 5, "p must be a prime < 2^32");
    find_curve(p, a.uo("cseed", 1))
}

// ---------------------------------------------------------------- subcommands
fn cmd_curve(a: &Args) {
    let ci = curve_from(a);
    println!("{}", curve_json(&ci, a.uo("cseed", 1)));
}
fn cmd_cosets(a: &Args) {
    let ci = curve_from(a);
    let kind = parse_kind(&a.s("kind"));
    let t = a.u("t") as u32;
    let cnt = a.u("C") as usize;
    let cos = gen_cosets(kind, &ci.curve, t, a.u("seed"), cnt).unwrap_or_else(|e| { eprintln!("{e}"); std::process::exit(2) });
    let nn = 1u64 << t;
    let ces: Vec<usize> = cos.iter().map(|c| c.c_e).collect();
    let mean = ces.iter().sum::<usize>() as f64 / cos.len() as f64;
    let (am, best) = select(&cos, "max");
    let q_emp = mean / nn as f64;
    let q_curve = (ci.n - 1) as f64 / 2.0 / ci.curve.p as f64;
    let mut hist = vec![0usize; nn as usize + 1];
    for &c in &ces { hist[c] += 1; }
    let em_emp = exp_max_binom(nn, q_emp, cos.len() as u64);
    let em_cur = exp_max_binom(nn, q_curve, cos.len() as u64);
    let out = json!({
        "kind": a.s("kind"), "p": ci.curve.p, "t": t, "C": cos.len(), "seed": a.u("seed"),
        "curve": curve_json(&ci, a.uo("cseed", 1)),
        "c_mean": mean, "c_max": best.c_e, "argmax_index": am, "argmax_c": best.c, "c_min": ces.iter().min(),
        "q_emp": q_emp, "q_curve": q_curve,
        "ratio_sel_over_mean": best.c_e as f64 / mean,
        "exact_expected_max_binom_q_emp": em_emp, "exact_expected_max_binom_q_curve": em_cur,
        "exact_ratio_q_emp": em_emp / (nn as f64 * q_emp),
        "gaussian_ratio_law": (1.0 + (2.0 * (cos.len() as f64).ln() / nn as f64).sqrt()).min(2.0),
        "c_E_histogram": hist,
        "coset_c_values_first8": cos.iter().take(8).map(|c| c.c).collect::<Vec<_>>(),
    });
    if let Some(path) = a.0.get("out") {
        let full: Vec<Value> = cos.iter().map(|c| json!({"c": c.c, "c_E": c.c_e, "elems": c.elems, "is_E": c.is_e})).collect();
        std::fs::write(path, serde_json::to_string(&full).unwrap()).unwrap();
    }
    println!("{out}");
}
fn e_points(cv: &Curve, co: &Coset) -> Vec<(u64, u64)> {
    co.elems.iter().zip(&co.is_e).filter(|(_, &e)| e).map(|(&x, _)| (x, sqrtm(cv.f(x), cv.p).unwrap())).collect()
}
fn cmd_coverage(a: &Args) {
    let ci = curve_from(a);
    let cv = ci.curve;
    let kind = parse_kind(&a.s("kind"));
    let t = a.u("t") as u32;
    let cos = gen_cosets(kind, &cv, t, a.u("seed"), a.u("C") as usize).unwrap_or_else(|e| { eprintln!("{e}"); std::process::exit(2) });
    let (idx, co) = select(&cos, &a.so("select", "max"));
    let pts = e_points(&cv, co);
    let mut aset: Vec<Pt> = vec![];
    for &(x, y) in &pts { aset.push(Some((x, y))); aset.push(Some((x, negm(y, cv.p)))); }
    let mut s2: HashSet<Pt> = HashSet::new();
    for &u in &aset { for &v in &aset { s2.insert(cv.add(u, v)); } }
    let n = ci.n as f64;
    let cnt2 = s2.iter().filter(|q| q.is_some()).count();
    let mut out = json!({"kind": a.s("kind"), "p": cv.p, "t": t, "n": ci.n, "selected_index": idx, "c": co.c, "c_E": co.c_e,
        "m2_distinct_nonzero_R": cnt2, "m2_coverage": cnt2 as f64 / n, "m2_nominal_2cE2": 2 * co.c_e * co.c_e});
    if (1u64 << t) <= 64 {
        let mut s3: HashSet<Pt> = HashSet::new();
        for &w in &aset { for &u in &s2 { s3.insert(cv.add(u, w)); } }
        let cnt3 = s3.iter().filter(|q| q.is_some()).count();
        out["m3_distinct_nonzero_R"] = json!(cnt3);
        out["m3_coverage"] = json!(cnt3 as f64 / n);
    }
    println!("{out}");
}
fn cmd_emit(a: &Args) {
    let ci = curve_from(a);
    let cv = ci.curve; let p = cv.p;
    let kind = parse_kind(&a.s("kind"));
    let t = a.u("t") as u32;
    let m = a.u("m") as usize;
    assert!(m == 2 || m == 3);
    let cos = gen_cosets(kind, &cv, t, a.u("seed"), a.u("C") as usize).unwrap_or_else(|e| { eprintln!("{e}"); std::process::exit(2) });
    let (idx, co) = select(&cos, &a.so("select", "max"));
    let pts = e_points(&cv, co);
    assert!(pts.len() >= m, "coset has fewer than m E-columns");
    let mut rng = Rng::new(a.uo("tseed", 1) ^ 0x7A6E7 ^ co.c);
    // planted target: sum of m distinct base E points with random signs
    let (chosen, r) = loop {
        let mut idxs: Vec<usize> = vec![];
        while idxs.len() < m { let k = rng.below(pts.len() as u64) as usize; if !idxs.contains(&k) { idxs.push(k); } }
        let mut acc: Pt = None;
        for &k in &idxs {
            let (x, y) = pts[k];
            let y = if rng.below(2) == 0 { y } else { negm(y, p) };
            acc = cv.add(acc, Some((x, y)));
        }
        if let Some(r) = acc { break (idxs.iter().map(|&k| pts[k].0).collect::<Vec<_>>(), r); }
    };
    let xr = r.0;
    let nv = m * t as usize;
    let names = var_names(m, t);
    let s = summation_poly(m, nv, &cv, xr);
    let mut eqs = vec![s.clone()];
    eqs.extend(tower_eqs(kind, m, t, p, co.c));
    // planted assignment
    let mut asg = vec![0u64; nv];
    for i in 0..m {
        let mut y = chosen[i];
        asg[vidx_y(m, t, i, 0)] = y;
        for j in 1..t { y = tower_step(kind, y, p); asg[vidx_y(m, t, i, j)] = y; }
        assert_eq!(tower_step(kind, y, p), co.c, "tower does not end at c");
    }
    for (k, e) in eqs.iter().enumerate() { assert_eq!(e.eval(&asg), 0, "planted solution fails equation {k}"); }
    let body = eqs.iter().map(|e| e.to_ms(&names)).collect::<Vec<_>>().join(",\n");
    let text = format!("{}\n{}\n{}\n", names.join(","), p, body);
    let out = a.s("out");
    std::fs::write(&out, text).unwrap();
    let meta = json!({"kind": a.s("kind"), "p": p, "t": t, "m": m, "n": ci.n, "curve": curve_json(&ci, a.uo("cseed", 1)),
        "seed": a.u("seed"), "C": a.u("C"), "select": a.so("select", "max"), "selected_index": idx, "c": co.c, "c_E": co.c_e,
        "tseed": a.uo("tseed", 1), "planted_x": chosen, "xR": xr, "yR": r.1, "vars": nv, "equations": eqs.len(),
        "S_terms": s.t.len(), "planted_verified": true});
    println!("{meta}");
}

// ---- parse
fn cmd_parse(a: &Args) {
    let path = a.s("log");
    let txt = std::fs::read_to_string(&path).unwrap();
    println!("{}", parse_log(&txt));
}
fn parse_log(txt: &str) -> Value { parse_log_impl(txt) }

// ---- purity
struct Tabs { p: u64, cls: Vec<u8>, sqrt: Vec<u32>, inv: Vec<u32> }
fn tabs(cv: &Curve) -> Tabs {
    let p = cv.p as usize;
    let mut sqrt = vec![u32::MAX; p];
    for x in 0..p as u64 { let s = mulm(x, x, cv.p); if sqrt[s as usize] == u32::MAX { sqrt[s as usize] = x as u32; } }
    let cls = (0..cv.p).map(|x| { let v = cv.f(x); if v == 0 { 0 } else if sqrt[v as usize] != u32::MAX { 1 } else { 2 } }).collect();
    let mut inv = vec![0u32; p];
    for x in 1..cv.p { inv[x as usize] = invm(x, cv.p) as u32; }
    Tabs { p: cv.p, cls, sqrt, inv }
}
fn s3c(t: &Tabs, a: u64, b: u64, x1: u64, x2: u64) -> (u64, u64, u64) {
    let p = t.p;
    let uv = mulm(x1, x2, p); let s = addm(x1, x2, p); let d = subm(x1, x2, p);
    let a2 = mulm(d, d, p);
    let a1 = negm(mulm(2, addm(mulm(s, addm(uv, a, p), p), mulm(2, b, p), p), p), p);
    let w = subm(uv, a, p);
    let a0 = subm(mulm(w, w, p), mulm(mulm(4, b, p), s, p), p);
    (a2, a1, a0)
}
fn cmd_purity(a: &Args) {
    let ci = curve_from(a);
    let cv = ci.curve; let p = cv.p;
    assert!(p <= 10_000);
    let m = a.uo("m", 2);
    let t = tabs(&cv);
    let names = ["E", "T"]; // class index 1->E, 2->T
    let cn = |c: u8| if c == 1 { 0 } else { 1 };
    let mut count_e_target: HashMap<String, u64> = HashMap::new();
    let mut count_t_target: HashMap<String, u64> = HashMap::new();
    let mut zero_class = 0u64;
    let mut degenerate = 0u64;
    let mut e_target_nontrivial_twist = 0u64;
    let mut e_target_trivial_twist = 0u64;
    let mut e_target_total = 0u64;
    let key = |cs: &[u8]| cs.iter().map(|&c| names[cn(c)]).collect::<String>();
    if m == 2 {
        for x1 in 0..p { for x2 in 0..p {
            let (c1, c2) = (t.cls[x1 as usize], t.cls[x2 as usize]);
            if c1 == 0 || c2 == 0 { zero_class += 1; continue; }
            let (a2, a1, a0) = s3c(&t, cv.a, cv.b, x1, x2);
            let mut roots = [u64::MAX; 2]; let nr;
            if a2 == 0 {
                if a1 == 0 { if a0 == 0 { degenerate += 1; } continue; }
                roots[0] = mulm(negm(a0, p), t.inv[a1 as usize] as u64, p); nr = 1;
            } else {
                let disc = subm(mulm(a1, a1, p), mulm(4, mulm(a2, a0, p), p), p);
                let s = t.sqrt[disc as usize];
                if s == u32::MAX { continue; }
                let i2a = t.inv[mulm(2, a2, p) as usize] as u64;
                roots[0] = mulm(addm(negm(a1, p), s as u64, p), i2a, p);
                roots[1] = mulm(subm(negm(a1, p), s as u64, p), i2a, p);
                nr = if s == 0 { 1 } else { 2 };
            }
            for &z in &roots[..nr] {
                let cz = t.cls[z as usize];
                if cz == 1 {
                    e_target_total += 1;
                    *count_e_target.entry(key(&[c1, c2])).or_default() += 1;
                    if c1 == 2 || c2 == 2 { e_target_nontrivial_twist += 1; }
                } else if cz == 2 { *count_t_target.entry(key(&[c1, c2])).or_default() += 1; }
            }
        } }
    } else if m == 3 {
        assert!(p <= 200, "m=3 purity is limited to p <= 200");
        let n = p as usize;
        let xe: Vec<u64> = (0..p).filter(|&x| t.cls[x as usize] == 1).collect();
        let mut ta = vec![(0u64, 0u64, 0u64); n * n];
        for x1 in 0..p { for x2 in 0..p { ta[(x1 as usize) * n + x2 as usize] = s3c(&t, cv.a, cv.b, x1, x2); } }
        for x1 in 0..p { for x2 in 0..p {
            let (c1, c2) = (t.cls[x1 as usize], t.cls[x2 as usize]);
            if c1 == 0 || c2 == 0 { zero_class += 1; continue; }
            let (a2, a1, a0) = ta[(x1 as usize) * n + x2 as usize];
            for x3 in 0..p {
                let c3 = t.cls[x3 as usize];
                if c3 == 0 { continue; }
                for &xr in &xe {
                    let (b2, b1, b0) = ta[(x3 as usize) * n + xr as usize];
                    let d1 = subm(mulm(a2, b0, p), mulm(a0, b2, p), p);
                    let d2 = subm(mulm(a2, b1, p), mulm(a1, b2, p), p);
                    let d3 = subm(mulm(a1, b0, p), mulm(a0, b1, p), p);
                    if subm(mulm(d1, d1, p), mulm(d2, d3, p), p) != 0 { continue; }
                    e_target_total += 1;
                    *count_e_target.entry(key(&[c1, c2, c3])).or_default() += 1;
                    if c1 == 2 || c2 == 2 || c3 == 2 {
                        // trivial: some pair x_a = x_b with the remaining summand x_c = x_R
                        let xs = [x1, x2, x3];
                        let triv = (0..3).any(|c| { let (i, j) = ((c + 1) % 3, (c + 2) % 3); xs[i] == xs[j] && xs[c] == xr });
                        if triv { e_target_trivial_twist += 1; } else { e_target_nontrivial_twist += 1; }
                    }
                }
            }
        } }
    } else { panic!("m must be 2 or 3"); }
    let mut ce: Vec<_> = count_e_target.into_iter().collect(); ce.sort();
    let mut ct: Vec<_> = count_t_target.into_iter().collect(); ct.sort();
    println!("{}", json!({"m": m, "p": p, "n": ci.n, "curve": curve_json(&ci, a.uo("cseed", 1)),
        "E_target_solutions_total": e_target_total, "E_target_by_class": ce,
        "E_target_with_twist_summand_nontrivial": e_target_nontrivial_twist,
        "E_target_with_twist_summand_trivial_cancelling_pair": e_target_trivial_twist,
        "twist_target_by_class_info_m2": ct, "skipped_f_zero_pairs": zero_class, "degenerate_all_z": degenerate,
        "lemma_holds_nontrivial": e_target_nontrivial_twist == 0}));
}

// ---- selftest
fn check(name: &str, ok: bool, fails: &mut u32) {
    println!("{} {name}", if ok { "PASS" } else { "FAIL" });
    if !ok { *fails += 1; }
}
fn cmd_selftest() {
    let mut f = 0u32;
    // curve order vs Legendre count
    for (p, cs) in [(10007u64, 1u64), (65521, 2), (1009, 3)] {
        let ci = find_curve(p, cs);
        check(&format!("order p={p} BSGS == Legendre count ({})", ci.n), ci.curve.legendre_count() == ci.n && is_prime(ci.n), &mut f);
    }
    // S3 / S4 vanish on genuine sums
    let ci = find_curve(10007, 5); let cv = ci.curve; let p = cv.p;
    let mut rng = Rng::new(99);
    let mut ok3 = true; let mut ok4 = true;
    for _ in 0..200 {
        let p1 = cv.rand_point(&mut rng); let p2 = cv.rand_point(&mut rng); let p3 = cv.rand_point(&mut rng);
        let r = cv.add(cv.add(Some(p1), Some(p2)), Some(p3));
        let s2 = cv.add(Some(p1), Some(p2));
        if let Some(s2) = s2 {
            let v = summation_poly(2, 2, &cv, s2.0).eval(&[p1.0, p2.0]);
            ok3 &= v == 0;
        }
        if let Some(r) = r {
            let v = summation_poly(3, 3, &cv, r.0).eval(&[p1.0, p2.0, p3.0]);
            ok4 &= v == 0;
        }
    }
    check("S_3 vanishes on P1+P2=R", ok3, &mut f);
    check("S_4 (resultant) vanishes on P1+P2+P3=R", ok4, &mut f);
    // S_3 nonvanishing on random non-relations (rate ~ 1/p)
    let mut zero = 0; for _ in 0..2000 { let (a, b, c) = (rng.below(p), rng.below(p), rng.below(p)); if summation_poly(2, 2, &cv, c).eval(&[a, b]) == 0 { zero += 1; } }
    check(&format!("S_3 random zero rate small ({zero}/2000)"), zero < 20, &mut f);
    // cosets
    for (kind, p, t) in [(Kind::Kummer, 12289u64, 4u32), (Kind::Dickson, 65519 - 0, 4)] {
        // pick valid primes
        let (p, t) = match kind { Kind::Kummer => (p, t), Kind::Dickson => { let q = (2..65536u64).rev().find(|&q| is_prime(q) && (q + 1) % 16 == 0).unwrap(); (q, t) } };
        let ci = find_curve(p, 7);
        let cos = gen_cosets(kind, &ci.curve, t, 3, 20).unwrap();
        let mut good = true;
        for c in &cos {
            good &= c.elems.len() == 1 << t && c.elems.iter().collect::<HashSet<_>>().len() == 1 << t;
            for &x in &c.elems { let mut y = x; for _ in 0..t { y = tower_step(kind, y, p); } good &= y == c.c; }
        }
        check(&format!("{:?} p={p} t={t}: 20 cosets, 2^t distinct elems, tower ends at c", kind), good, &mut f);
        // completeness: number of x in F_p with tower(x)=c equals 2^t
        let c0 = &cos[0]; let mut cnt = 0;
        for x in 0..p { let mut y = x; for _ in 0..t { y = tower_step(kind, y, p); } if y == c0.c { cnt += 1; } }
        check(&format!("{:?}: V_c is the full root set in F_p ({cnt} == {})", kind, 1 << t), cnt == 1 << t, &mut f);
    }
    // binomial order statistic
    check("E[max] C=1 equals N q", (exp_max_binom(16, 0.5, 1) - 8.0).abs() < 1e-9, &mut f);
    check("E[max] N=1 equals 1-(1-q)^C", (exp_max_binom(1, 0.3, 5) - (1.0 - 0.7f64.powi(5))).abs() < 1e-9, &mut f);
    let e = exp_max_binom(16, 0.5, 10000);
    check(&format!("E[max] N=16,q=.5,C=1e4 in (14,16) ({e:.4})"), e > 14.0 && e < 16.0, &mut f);
    // small-prime purity m=2 quick (p=1009) and S4 consistency at m=3 tiny
    println!("{}", if f == 0 { "SELFTEST OK" } else { "SELFTEST FAILED" });
    if f != 0 { std::process::exit(1); }
}

fn main() {
    let v: Vec<String> = std::env::args().collect();
    if v.len() < 2 { eprintln!("usage: p2coset curve|cosets|coverage|emit|parse|purity|selftest [--key value ...]"); std::process::exit(2); }
    let a = Args::new(&v[2..]);
    match v[1].as_str() {
        "curve" => cmd_curve(&a), "cosets" => cmd_cosets(&a), "coverage" => cmd_coverage(&a),
        "emit" => cmd_emit(&a), "parse" => cmd_parse(&a), "purity" => cmd_purity(&a), "selftest" => cmd_selftest(),
        _ => { eprintln!("unknown subcommand"); std::process::exit(2) }
    }
}

fn parse_log_impl(txt: &str) -> Value {
    // F4 round rows: "deg sel pairs R x C density% N new Z zero t | t"
    let (mut maxdeg, mut maxcols, mut maxrc, mut sumrc, mut sumrows, mut steps) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut maxrc_dims = (0u64, 0u64);
    let mut deg_seq = vec![];
    for line in txt.lines() {
        let tk: Vec<&str> = line.split_whitespace().collect();
        // deg sel pairs R x C ...
        if tk.len() >= 7 && tk[4] == "x" {
            let nums: Option<Vec<u64>> = [tk[0], tk[1], tk[2], tk[3], tk[5]].iter().map(|s| s.parse().ok()).collect();
            if let Some(n) = nums {
                if !tk[6].ends_with('%') { continue; }
                let (d, r, c) = (n[0], n[3], n[4]);
                steps += 1; deg_seq.push(d);
                maxdeg = maxdeg.max(d); maxcols = maxcols.max(c);
                sumrc += r * c; sumrows += r;
                if r * c > maxrc { maxrc = r * c; maxrc_dims = (r, c); }
            }
        }
    }
    let after = |key: &str| txt.find(key).map(|i| txt[i + key.len()..].trim_start());
    let quotient = after("Dimension of quotient:").and_then(|s| s.split_whitespace().next()).and_then(|s| s.parse::<u64>().ok());
    let wall = after("overall(elapsed)").and_then(|s| s.split_whitespace().next()).and_then(|s| s.parse::<f64>().ok());
    let cpu = after("overall(cpu)").and_then(|s| s.split_whitespace().next()).and_then(|s| s.parse::<f64>().ok());
    json!({"f4_steps": steps, "D_max_deg": maxdeg, "widest_cols": maxcols, "max_rows_x_cols": maxrc,
           "max_matrix_rows": maxrc_dims.0, "max_matrix_cols": maxrc_dims.1,
           "sum_rows_x_cols": sumrc, "sum_rows": sumrows, "quotient_dim": quotient,
           "wall_s_descriptive": wall, "cpu_s_descriptive": cpu, "deg_sequence": deg_seq})
}
