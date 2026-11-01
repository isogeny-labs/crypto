//! C2 volcano experiment: crater vs floor, same r-subgroup, constrained S_4 at m = 3.
//! Self-contained (std only). Counted metrics: field multiplications (mul and sqr),
//! msolve step degree and matrix sizes. No wall time is recorded anywhere.
use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::io::Write;
use std::process::{Command, Stdio};

// ---------------------------------------------------------------- field
struct F { p: u64, cnt: Cell<u64> }
impl F {
    fn new(p: u64) -> F { F { p, cnt: Cell::new(0) } }
    #[inline] fn mul(&self, a: u64, b: u64) -> u64 { self.cnt.set(self.cnt.get() + 1); a * b % self.p }
    #[inline] fn add(&self, a: u64, b: u64) -> u64 { let s = a + b; if s >= self.p { s - self.p } else { s } }
    #[inline] fn sub(&self, a: u64, b: u64) -> u64 { if a >= b { a - b } else { a + self.p - b } }
    #[inline] fn neg(&self, a: u64) -> u64 { if a == 0 { 0 } else { self.p - a } }
    fn pow(&self, mut a: u64, mut e: u64) -> u64 {
        let mut r = 1; while e > 0 { if e & 1 == 1 { r = self.mul(r, a); } a = self.mul(a, a); e >>= 1; } r
    }
    fn inv(&self, a: u64) -> u64 { self.pow(a, self.p - 2) }
    fn cnt(&self) -> u64 { self.cnt.get() }
    fn sqrt(&self, a: u64) -> Option<u64> {
        if a == 0 { return Some(0); }
        if self.pow(a, (self.p - 1) / 2) != 1 { return None; }
        let (mut q, mut s) = (self.p - 1, 0u32);
        while q % 2 == 0 { q /= 2; s += 1; }
        let mut z = 2; while self.pow(z, (self.p - 1) / 2) == 1 { z += 1; }
        let (mut m, mut c, mut t, mut r) = (s, self.pow(z, q), self.pow(a, q), self.pow(a, (q + 1) / 2));
        while t != 1 {
            let mut i = 0; let mut tt = t; while tt != 1 { tt = self.mul(tt, tt); i += 1; }
            let mut b = c; for _ in 0..(m - i - 1) { b = self.mul(b, b); }
            m = i; c = self.mul(b, b); t = self.mul(t, c); r = self.mul(r, b);
        }
        Some(r)
    }
}
fn is_prime(n: u64) -> bool { if n < 2 { return false; } let mut d = 2; while d * d <= n { if n % d == 0 { return false; } d += 1; } true }
fn factor(mut n: u64) -> Vec<(u64, u32)> {
    let mut v = vec![]; let mut d = 2;
    while d * d <= n { if n % d == 0 { let mut e = 0; while n % d == 0 { n /= d; e += 1; } v.push((d, e)); } d += 1; }
    if n > 1 { v.push((n, 1)); } v
}
struct Rng(u64);
impl Rng { fn next(&mut self) -> u64 { let mut x = self.0; x ^= x >> 12; x ^= x << 25; x ^= x >> 27; self.0 = x; x.wrapping_mul(0x2545F4914F6CDD1D) } }

// ---------------------------------------------------------------- curve
type Pt = Option<(u64, u64)>;
#[derive(Clone, Copy, Debug)]
struct Ec { a: u64, b: u64 }
impl Ec {
    fn rhs(&self, f: &F, x: u64) -> u64 { let x2 = f.mul(x, x); let t = f.add(f.mul(x2, x), f.mul(self.a, x)); f.add(t, self.b) }
    fn on(&self, f: &F, p: Pt) -> bool { match p { None => true, Some((x, y)) => f.mul(y, y) == self.rhs(f, x) } }
    fn add(&self, f: &F, p: Pt, q: Pt) -> Pt {
        match (p, q) {
            (None, _) => q, (_, None) => p,
            (Some((x1, y1)), Some((x2, y2))) => {
                let lam;
                if x1 == x2 {
                    if f.add(y1, y2) == 0 { return None; }
                    let n = f.add(f.mul(3, f.mul(x1, x1)), self.a);
                    lam = f.mul(n, f.inv(f.mul(2, y1)));
                } else { lam = f.mul(f.sub(y2, y1), f.inv(f.sub(x2, x1))); }
                let x3 = f.sub(f.sub(f.mul(lam, lam), x1), x2);
                let y3 = f.sub(f.mul(lam, f.sub(x1, x3)), y1);
                Some((x3, y3))
            }
        }
    }
    fn neg(&self, f: &F, p: Pt) -> Pt { p.map(|(x, y)| (x, f.neg(y))) }
    fn mul(&self, f: &F, mut k: u64, p: Pt) -> Pt {
        let mut r = None; let mut q = p;
        while k > 0 { if k & 1 == 1 { r = self.add(f, r, q); } q = self.add(f, q, q); k >>= 1; } r
    }
    fn random_point(&self, f: &F, rng: &mut Rng) -> Pt {
        loop { let x = rng.next() % f.p; if let Some(y) = f.sqrt(self.rhs(f, x)) { return Some((x, y)); } }
    }
    fn j(&self, f: &F) -> u64 {
        let a3 = f.mul(f.mul(self.a, self.a), self.a); let b2 = f.mul(self.b, self.b);
        let num = f.mul(1728 % f.p, f.mul(4, a3)); let den = f.add(f.mul(4, a3), f.mul(27, b2));
        f.mul(num, f.inv(den))
    }
    fn iso(&self, f: &F, u: u64, p: Pt) -> (Ec, Pt) {
        let u2 = f.mul(u, u); let u3 = f.mul(u2, u); let u4 = f.mul(u2, u2); let u6 = f.mul(u3, u3);
        (Ec { a: f.mul(self.a, u4), b: f.mul(self.b, u6) }, p.map(|(x, y)| (f.mul(u2, x), f.mul(u3, y))))
    }
}
/// ell-subgroup generators found among random points: 1 element = cyclic ell-Sylow, ell+1 = full ell-torsion.
fn ell_kernels(ec: &Ec, f: &F, n: u64, ell: u64, rng: &mut Rng) -> Vec<Pt> {
    let mut v = n; let mut ve = 0; while v % ell == 0 { v /= ell; ve += 1; }
    let mut ker: Vec<Pt> = vec![];
    for _ in 0..80 {
        let pt = ec.random_point(f, rng);
        let mut q = ec.mul(f, v, pt);
        if q.is_none() { continue; }
        // reduce to order ell
        for _ in 0..ve { let q2 = ec.mul(f, ell, q); if q2.is_none() { break; } q = q2; }
        if ec.mul(f, ell, q).is_some() { continue; }
        // same subgroup as an existing generator?
        let mut found = false;
        for k in &ker { for c in 1..ell { if ec.mul(f, c, *k) == q { found = true; } } }
        if !found { ker.push(q); }
    }
    ker
}
// ---------------------------------------------------------------- Velu (odd ell)
#[derive(Clone)]
struct Velu { ell: u64, src: Ec, dst: Ec, pts: Vec<(u64, u64, u64, u64)> } // (xq, yq, v, u)
fn velu(f: &F, src: Ec, q: Pt, ell: u64) -> Velu {
    let (mut v, mut w) = (0, 0); let mut pts = vec![];
    for i in 1..=(ell - 1) / 2 {
        let (xq, yq) = src.mul(f, i, q).unwrap();
        let gx = f.add(f.mul(3, f.mul(xq, xq)), src.a);
        let vq = f.mul(2, gx); let uq = f.mul(4, f.mul(yq, yq));
        v = f.add(v, vq); w = f.add(w, f.add(uq, f.mul(xq, vq)));
        pts.push((xq, yq, vq, uq));
    }
    let dst = Ec { a: f.sub(src.a, f.mul(5, v)), b: f.sub(src.b, f.mul(7, w)) };
    Velu { ell, src, dst, pts }
}
impl Velu {
    fn apply(&self, f: &F, p: Pt) -> Pt {
        let (x, y) = p?;
        let (mut sx, mut sy) = (0, 0);
        for &(xq, _yq, vq, uq) in &self.pts {
            let d = f.sub(x, xq); if d == 0 { return None; }
            let di = f.inv(d); let di2 = f.mul(di, di); let di3 = f.mul(di2, di);
            sx = f.add(sx, f.add(f.mul(vq, di), f.mul(uq, di2)));
            sy = f.add(sy, f.add(f.mul(vq, di2), f.mul(2, f.mul(uq, di3))));
        }
        Some((f.add(x, sx), f.mul(y, f.sub(1, sy))))
    }
}

// ---------------------------------------------------------------- univariate polys (counted)
type Poly = Vec<u64>; // low to high, trimmed
fn trim(mut a: Poly) -> Poly { while a.last() == Some(&0) { a.pop(); } a }
fn pmul(f: &F, a: &[u64], b: &[u64]) -> Poly {
    if a.is_empty() || b.is_empty() { return vec![]; }
    let mut r = vec![0; a.len() + b.len() - 1];
    for i in 0..a.len() { for j in 0..b.len() { r[i + j] = f.add(r[i + j], f.mul(a[i], b[j])); } }
    trim(r)
}
fn psub(f: &F, a: &[u64], b: &[u64]) -> Poly {
    let n = a.len().max(b.len()); let mut r = vec![0; n];
    for i in 0..n { r[i] = f.sub(*a.get(i).unwrap_or(&0), *b.get(i).unwrap_or(&0)); } trim(r)
}
/// a mod b, b nonzero; counted. Returns remainder and number of inversions used.
fn prem(f: &F, a: &[u64], b: &[u64]) -> Poly {
    let mut r = a.to_vec(); let db = b.len() - 1; let ib = f.inv(*b.last().unwrap());
    while r.len() > db {
        let lead = f.mul(*r.last().unwrap(), ib); let sh = r.len() - 1 - db;
        for i in 0..b.len() { let t = f.mul(lead, b[i]); r[sh + i] = f.sub(r[sh + i], t); }
        r = trim(r);
        if r.is_empty() { break; }
    }
    r
}
fn pgcd_deg(f: &F, a: Poly, b: Poly, stats: &mut (u64, u64)) -> usize {
    let (mut a, mut b) = (a, b);
    while !b.is_empty() { let r = prem(f, &a, &b); stats.0 += 1; a = b; b = r; }
    stats.1 += 0; a.len().saturating_sub(1)
}

// ---------------------------------------------------------------- the S_4 pair loop (counted routine)
struct RunOut { work: u64, fixed: u64, var: u64, first: u64, hits: BTreeSet<[usize; 3]>, verified: usize, bad: usize, degenerate: u64, gcd_steps: u64, gcd_nontrivial: u64 }
fn s4_decompose(f: &F, ec: &Ec, base: &[(u64, u64)], r_pt: (u64, u64)) -> RunOut {
    let c0 = f.cnt(); let (a, b) = (ec.a, ec.b); let xr = r_pt.0;
    // per-target constants: a2, b2, c2 as polynomials in z
    let xr2 = f.mul(xr, xr);
    let a2 = vec![xr2, f.neg(f.mul(2, xr)), 1];
    let b2 = vec![f.neg(f.mul(2, f.add(f.mul(a, xr), f.mul(2, b)))), f.neg(f.mul(2, f.add(a, xr2))), f.neg(f.mul(2, xr))];
    let c2 = vec![f.sub(f.mul(a, a), f.mul(4, f.mul(b, xr))), f.sub(f.neg(f.mul(2, f.mul(a, xr))), f.mul(4, b)), xr2];
    let mut fixed = f.cnt() - c0; let mut var = 0u64; let mut first = 0u64; let mut got_first = false;
    let mut hits = BTreeSet::new(); let (mut verified, mut bad, mut degenerate) = (0, 0, 0u64);
    let mut st = (0u64, 0u64); let mut gcd_nontrivial = 0u64;
    let nb = base.len();
    for i in 0..nb { for j in i..nb {
        let s0 = f.cnt();
        let (x1, x2) = (base[i].0, base[j].0);
        let d = f.sub(x1, x2); let a1 = f.mul(d, d);
        let s = f.add(x1, x2); let m = f.mul(x1, x2);
        let b1 = f.neg(f.mul(2, f.add(f.mul(s, f.add(m, a)), f.mul(2, b))));
        let mm = f.sub(m, a); let c1 = f.sub(f.mul(mm, mm), f.mul(f.mul(4, b), s));
        let scale = |k: u64, v: &Poly| -> Poly { trim(v.iter().map(|&c| f.mul(k, c)).collect()) };
        let p1 = psub(f, &scale(a1, &c2), &scale(c1, &a2));
        let p2 = psub(f, &scale(a1, &b2), &scale(b1, &a2));
        let p3 = psub(f, &scale(b1, &c2), &scale(c1, &b2));
        let q = psub(f, &pmul(f, &p1, &p1), &pmul(f, &p2, &p3));
        fixed += f.cnt() - s0;
        if q.is_empty() { degenerate += 1; continue; }
        if q.len() == 1 { continue; }
        // h = prod (z - b) mod q, by Horner-style accumulation (monic reduction)
        let s1 = f.cnt();
        let iq = f.inv(*q.last().unwrap()); let qm: Poly = q.iter().map(|&c| f.mul(c, iq)).collect();
        let dq = qm.len() - 1; let mut h: Poly = vec![1];
        for bp in base.iter() {
            let mut nh = vec![0u64; h.len() + 1];
            for k in 0..h.len() { nh[k + 1] = h[k]; nh[k] = f.sub(nh[k], f.mul(bp.0, h[k])); }
            while nh.len() > dq {
                let lead = *nh.last().unwrap(); let sh = nh.len() - 1 - dq;
                if lead != 0 { for k in 0..=dq { let t = f.mul(lead, qm[k]); nh[sh + k] = f.sub(nh[sh + k], t); } }
                nh.pop();
            }
            h = trim(nh); if h.is_empty() { break; }
        }
        fixed += f.cnt() - s1;
        let s2 = f.cnt();
        let g = if h.is_empty() { dq } else { pgcd_deg(f, qm.clone(), h, &mut st) };
        var += f.cnt() - s2;
        if g > 0 {
            gcd_nontrivial += 1;
            let s3 = f.cnt();
            // extract roots: evaluate q at each base abscissa
            let mut roots = vec![];
            for (k, bp) in base.iter().enumerate() {
                let mut acc = 0u64; for &c in q.iter().rev() { acc = f.add(f.mul(acc, bp.0), c); }
                if acc == 0 { roots.push(k); }
            }
            var += f.cnt() - s3;
            for k in roots {
                let mut t = [i, j, k]; t.sort();
                if hits.insert(t) {
                    if verify(f, ec, base, &t, r_pt) { verified += 1; } else { bad += 1; }
                    if !got_first && verified > 0 { got_first = true; first = f.cnt() - c0; }
                }
            }
        }
    } }
    let work = f.cnt() - c0;
    // verification arithmetic is setup-side and must not be charged: it used a separate field.
    RunOut { work, fixed, var, first: if got_first { first } else { work }, hits, verified, bad, degenerate, gcd_steps: st.0, gcd_nontrivial }
}
fn verify(_f: &F, ec: &Ec, base: &[(u64, u64)], t: &[usize; 3], r: (u64, u64)) -> bool {
    let vf = F::new(_f.p); // separate counter
    let pts: Vec<Pt> = t.iter().map(|&i| Some(base[i])).collect();
    for s2 in 0..2 { for s3 in 0..2 {
        let p2 = if s2 == 0 { pts[1] } else { ec.neg(&vf, pts[1]) };
        let p3 = if s3 == 0 { pts[2] } else { ec.neg(&vf, pts[2]) };
        let sum = ec.add(&vf, ec.add(&vf, pts[0], p2), p3);
        if sum == Some(r) || sum == ec.neg(&vf, Some(r)) { return true; }
    } }
    false
}

// ---------------------------------------------------------------- multivariate polys for msolve
type MP = BTreeMap<[u8; 3], u64>;
fn mp_add(p: u64, a: &MP, b: &MP, sgn: bool) -> MP {
    let mut r = a.clone();
    for (k, v) in b { let e = r.entry(*k).or_insert(0); *e = if sgn { (*e + v) % p } else { (*e + p - v) % p }; }
    r.retain(|_, v| *v != 0); r
}
fn mp_mul(p: u64, a: &MP, b: &MP) -> MP {
    let mut r: MP = BTreeMap::new();
    for (ka, va) in a { for (kb, vb) in b {
        let k = [ka[0] + kb[0], ka[1] + kb[1], ka[2] + kb[2]];
        let e = r.entry(k).or_insert(0); *e = (*e + va * vb) % p;
    } }
    r.retain(|_, v| *v != 0); r
}
fn mp_c(p: u64, c: u64) -> MP { let mut m = BTreeMap::new(); if c % p != 0 { m.insert([0, 0, 0], c % p); } m }
fn mp_var(i: usize) -> MP { let mut k = [0u8; 3]; k[i] = 1; let mut m = BTreeMap::new(); m.insert(k, 1); m }
fn mp_scale(p: u64, a: &MP, c: u64) -> MP { let mut r = a.clone(); for v in r.values_mut() { *v = *v * (c % p) % p; } r.retain(|_, v| *v != 0); r }
fn s4_poly(p: u64, a: u64, b: u64, xr: u64) -> MP {
    let (x1, x2, x3) = (mp_var(0), mp_var(1), mp_var(2));
    let ad = |u: &MP, v: &MP| mp_add(p, u, v, true); let sb = |u: &MP, v: &MP| mp_add(p, u, v, false); let ml = |u: &MP, v: &MP| mp_mul(p, u, v);
    let d = sb(&x1, &x2); let a1 = ml(&d, &d);
    let s = ad(&x1, &x2); let m = ml(&x1, &x2);
    let b1 = mp_scale(p, &ad(&ml(&s, &ad(&m, &mp_c(p, a))), &mp_c(p, 2 * b)), p - 2);
    let mm = sb(&m, &mp_c(p, a)); let c1 = sb(&ml(&mm, &mm), &mp_scale(p, &mp_c(p, 4 * b), 1).iter().fold(MP::new(), |_, _| MP::new()));
    let c1 = sb(&c1, &mp_scale(p, &s, 4 * b % p)); // c1 = mm^2 - 4 b s
    let xr_c = mp_c(p, xr);
    let d2 = sb(&x3, &xr_c); let a2 = ml(&d2, &d2);
    let sz = ad(&x3, &xr_c); let mz = ml(&x3, &xr_c);
    let b2 = mp_scale(p, &ad(&ml(&sz, &ad(&mz, &mp_c(p, a))), &mp_c(p, 2 * b)), p - 2);
    let mz2 = sb(&mz, &mp_c(p, a)); let c2 = sb(&ml(&mz2, &mz2), &mp_scale(p, &sz, 4 * b % p));
    let t1 = sb(&ml(&a1, &c2), &ml(&a2, &c1));
    let t2 = sb(&ml(&a1, &b2), &ml(&a2, &b1));
    let t3 = sb(&ml(&b1, &c2), &ml(&b2, &c1));
    sb(&ml(&t1, &t1), &ml(&t2, &t3))
}
fn mp_fmt(a: &MP) -> String {
    if a.is_empty() { return "0".into(); }
    a.iter().map(|(k, v)| format!("{}*x1^{}*x2^{}*x3^{}", v, k[0], k[1], k[2])).collect::<Vec<_>>().join("+")
}
fn base_poly(p: u64, base: &[(u64, u64)], var: &str) -> String {
    let f = F::new(p); let mut c: Vec<u64> = vec![1];
    for b in base { let mut n = vec![0; c.len() + 1]; for k in 0..c.len() { n[k + 1] = f.add(n[k + 1], c[k]); n[k] = f.sub(n[k], f.mul(b.0, c[k])); } c = n; }
    c.iter().enumerate().filter(|(_, &v)| v != 0).map(|(k, v)| format!("{}*{}^{}", v, var, k)).collect::<Vec<_>>().join("+")
}
#[derive(Default, Clone, Debug)]
struct MsolveOut { status: String, max_deg: u32, rounds: u32, max_rows: u64, max_cols: u64, sum_cells: u64, traj: Vec<(u32, u64, u64)> }
fn run_msolve(path: &str, timeout_s: u64) -> MsolveOut {
    let child = Command::new("timeout").args([&timeout_s.to_string(), "msolve", "-v", "2", "-g", "2", "-t", "1", "-f", path, "-o", &format!("{}.out", path)])
        .stdout(Stdio::piped()).stderr(Stdio::null()).output();
    let mut o = MsolveOut::default();
    match child {
        Err(e) => { o.status = format!("spawn-failed:{}", e); o }
        Ok(out) => {
            let s = String::from_utf8_lossy(&out.stdout).to_string();
            let mut in_tab = false;
            for line in s.lines() {
                let t: Vec<&str> = line.split_whitespace().collect();
                if line.starts_with("deg     sel") { in_tab = true; continue; }
                if line.starts_with("reduce final basis") { in_tab = false; continue; }
                if in_tab && t.len() >= 7 && t[0].parse::<u32>().is_ok() && t[4] == "x" {
                    let d: u32 = t[0].parse().unwrap(); let r: u64 = t[3].parse().unwrap(); let c: u64 = t[5].parse().unwrap();
                    o.traj.push((d, r, c)); o.max_deg = o.max_deg.max(d); o.rounds += 1; o.max_rows = o.max_rows.max(r); o.max_cols = o.max_cols.max(c); o.sum_cells += r * c;
                }
            }
            o.status = if out.status.success() { "ok".into() } else if out.status.code() == Some(124) { format!("timeout>{}s", timeout_s) } else { format!("exit{:?}", out.status.code()) };
            o
        }
    }
}

// ---------------------------------------------------------------- instance + volcano
fn class_one_j(d: i64) -> i128 { match d { -3 => 0, -4 => 1728, -7 => -3375, -8 => 8000, -11 => -32768, -19 => -884736, -43 => -884736000, -67 => -147197952000, -163 => -262537412640768000, _ => panic!("D") } }
struct Inst { p: u64, t: i64, d: i64, n: u64, r: u64, h: u64 }
fn find_instances(ell: u64, dlist: &[i64], lo: u64, hi: u64, rmin: u64) -> Vec<Inst> {
    let fc = (ell * ell) as i64; let mut out = vec![];
    for &d in dlist { for t in 2..4000i64 {
        if t % ell as i64 != 2 % ell as i64 { continue; }
        let num = t * t - d * fc * fc; if num % 4 != 0 { continue; }
        let p = (num / 4) as u64; if p < lo || p >= hi || !is_prime(p) { continue; }
        for tt in [t, -t] {
            let n = (p as i64 + 1 - tt) as u64; if n % (ell * ell) != 0 { continue; }
            if tt % ell as i64 != 2 % ell as i64 && (tt.rem_euclid(ell as i64)) != 2 % ell as i64 { continue; }
            let fa = factor(n); let (r, e) = *fa.last().unwrap(); if e != 1 || r < rmin { continue; }
            if r == ell { continue; }
            out.push(Inst { p, t: tt, d, n, r, h: n / r });
        }
    } }
    out
}
fn crater_curve(f: &F, inst: &Inst, rng: &mut Rng) -> Option<Ec> {
    let j = class_one_j(inst.d).rem_euclid(f.p as i128) as u64;
    let k = f.mul(j, f.inv(f.sub(1728 % f.p, j)));
    for d in 1..f.p.min(200) {
        let d2 = f.mul(d, d); let d3 = f.mul(d2, d);
        let ec = Ec { a: f.mul(f.mul(3, k), d2), b: f.mul(f.mul(2, k), d3) };
        let ok = (0..6).all(|_| { let p = ec.random_point(f, rng); ec.mul(f, inst.n, p).is_none() });
        if ok { return Some(ec); }
    }
    None
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).map(|s| s.as_str()).unwrap_or("find");
    match cmd {
        "find" => {
            let ell: u64 = args[2].parse().unwrap();
            let v = find_instances(ell, &[-8, -11, -19, -43, -67, -163], 1 << 16, 1 << 20, 1000);
            println!("ell,D,p,t,N,r,h,v_ell(N)");
            for i in v { let mut ve = 0; let mut n = i.n; while n % ell == 0 { n /= ell; ve += 1; } println!("{},{},{},{},{},{},{},{}", ell, i.d, i.p, i.t, i.n, i.r, i.h, ve); }
        }
        "run" => run(&args),
        "msolve" => msolve_stage(&args),
        "summarize" => summarize(&args),
        _ => eprintln!("usage: find <ell> | run ..."),
    }
}

fn run(args: &[String]) {
    let ell: u64 = args[2].parse().unwrap(); let d: i64 = args[3].parse().unwrap(); let p: u64 = args[4].parse().unwrap(); let t: i64 = args[5].parse().unwrap();
    let outdir = args[6].clone(); let n_models: usize = args[7].parse().unwrap(); let n_targets: usize = args[8].parse().unwrap();
    let msolve_cells: usize = args[9].parse().unwrap();
    let f = F::new(p); let mut rng = Rng(0x9E3779B97F4A7C15 ^ p);
    let n = (p as i64 + 1 - t) as u64; let fa = factor(n); let (r, _) = *fa.last().unwrap();
    let inst = Inst { p, t, d, n, r, h: n / r };
    let bsz = (r as f64).cbrt().round() as usize;
    let mut log = std::fs::File::create(format!("{}/setup.log", outdir)).unwrap();
    macro_rules! L { ($($a:tt)*) => {{ let s = format!($($a)*); println!("{}", s); writeln!(log, "{}", s).unwrap(); }} }
    L!("instance ell={} D={} p={} t={} N={} factors={:?} r={} h={} B=round(r^(1/3))={}", ell, d, p, t, n, fa, r, inst.h, bsz);
    let crater = crater_curve(&f, &inst, &mut rng).expect("crater curve");
    L!("crater a={} b={} j={}", crater.a, crater.b, crater.j(&f));
    let kc = ell_kernels(&crater, &f, n, ell, &mut rng);
    L!("crater ell-kernels found: {} (full E[ell] rational iff {})", kc.len(), ell + 1);
    assert!(kc.len() as u64 == ell + 1, "crater ell-Sylow is cyclic: level test would certify nothing");
    // descent: crater -> mid -> floor via Velu; keep floors with cyclic ell-Sylow
    let jc = crater.j(&f);
    let mut floors: Vec<(Velu, Velu)> = vec![]; let mut mids = 0; let mut mid_noncyc = 0;
    for k1 in &kc {
        let v1 = velu(&f, crater, *k1, ell); let e1 = v1.dst;
        let k1s = ell_kernels(&e1, &f, n, ell, &mut rng);
        mids += 1; if k1s.len() > 1 { mid_noncyc += 1; }
        if e1.j(&f) == jc { continue; }
        for k2 in &k1s {
            let v2 = velu(&f, e1, *k2, ell); let e2 = v2.dst;
            if e2.j(&f) == jc { continue; }
            // reject a backtrack to the same mid vertex's dual: image j equal to a crater-adjacent j is fine only if floor
            let k2s = ell_kernels(&e2, &f, n, ell, &mut rng);
            if k2s.len() == 1 { floors.push((v1.clone(), v2)); }
        }
    }
    let mut fj: Vec<u64> = floors.iter().map(|(_, v2)| v2.dst.j(&f)).collect(); fj.sort(); fj.dedup();
    L!("kernels from crater: {}; mid curves non-cyclic ell-Sylow: {}; floor paths with cyclic ell-Sylow: {}; distinct floor j: {}", mids, mid_noncyc, floors.len(), fj.len());
    assert!(!floors.is_empty(), "no floor found");
    // r-subgroup generator and targets on the crater
    let g = loop { let pt = crater.random_point(&f, &mut rng); let q = crater.mul(&f, n / r, pt); if q.is_some() { break q; } };
    assert!(crater.mul(&f, r, g).is_none());
    let ks: Vec<u64> = (0..n_targets).map(|_| 1 + rng.next() % (r - 1)).collect();
    // models: (kind, id, ec, G, targets)
    struct Model { kind: &'static str, id: usize, ec: Ec, j: u64, g: Pt, tg: Vec<Pt>, tag: String }
    let mut models: Vec<Model> = vec![];
    for m in 0..n_models {
        let u = if m == 0 { 1 } else { 2 + rng.next() % (p - 3) };
        let (ec, gg) = crater.iso(&f, u, g);
        let tg: Vec<Pt> = ks.iter().map(|&k| ec.mul(&f, k, gg)).collect();
        models.push(Model { kind: "crater", id: m, ec, j: ec.j(&f), g: gg, tg, tag: format!("u={}", u) });
    }
    for m in 0..n_models {
        let (v1, v2) = &floors[m % floors.len()];
        let g1 = v1.apply(&f, g); let g2 = v2.apply(&f, g1);
        let u = if m < floors.len() { 1 } else { 2 + rng.next() % (p - 3) };
        let (ec, gg) = v2.dst.iso(&f, u, g2);
        assert!(ec.on(&f, gg) && ec.mul(&f, r, gg).is_none() && gg.is_some(), "floor generator transport failed");
        // homomorphism check on the transported scalar multiples
        let tg: Vec<Pt> = ks.iter().map(|&k| ec.mul(&f, k, gg)).collect();
        // exact transport check: phi([k]G) == [k]phi(G) for the first target (before iso)
        let r0 = crater.mul(&f, ks[0], g); let im = v2.apply(&f, v1.apply(&f, r0)); let chk = v2.dst.mul(&f, ks[0], g2);
        assert_eq!(im, chk, "isogeny is not a homomorphism on the target");
        models.push(Model { kind: "floor", id: m, ec, j: ec.j(&f), g: gg, tg, tag: format!("path={} u={}", m % floors.len(), u) });
    }
    let mut csv = std::fs::File::create(format!("{}/work_per_target.csv", outdir)).unwrap();
    writeln!(csv, "kind,model,j,target,scalar,work_total,work_fixed_by_B,work_variable_gcd_roots,work_to_first_relation,distinct_triples,verified,false_hits,degenerate_pairs,gcd_steps,gcd_nontrivial_pairs").unwrap();
    let mut mcsv = std::fs::File::create(format!("{}/models.csv", outdir)).unwrap();
    writeln!(mcsv, "kind,model,j,a,b,note,B,max_x_used,r,N,group_ell_cyclic").unwrap();
    for md in &models {
        // base: first B abscissae in the r-subgroup, x = 0,1,2,...
        let mut base: Vec<(u64, u64)> = vec![]; let mut xm = 0; let mut x = 0u64;
        while base.len() < bsz && x < p {
            if let Some(y) = f.sqrt(md.ec.rhs(&f, x)) { if md.ec.mul(&f, r, Some((x, y))).is_none() { base.push((x, y)); xm = x; } }
            x += 1;
        }
        assert_eq!(base.len(), bsz);
        let cyc = ell_kernels(&md.ec, &f, n, ell, &mut rng).len() == 1;
        writeln!(mcsv, "{},{},{},{},{},{},{},{},{},{},{}", md.kind, md.id, md.j, md.ec.a, md.ec.b, md.tag, bsz, xm, r, n, cyc).unwrap();
        println!("model {} {} j={} max_x={} cyclic_ell={}", md.kind, md.id, md.j, xm, cyc);
        for (ti, tp) in md.tg.iter().enumerate() {
            let rp = tp.unwrap();
            let out = s4_decompose(&f, &md.ec, &base, rp);
            if ti < msolve_cells {
                let ctl = std::env::var("C2_CONTROL").unwrap_or_default();
                let (ca, cb) = match ctl.as_str() { "b0" => (md.ec.a, 0), "a0" => (0, md.ec.b), _ => (md.ec.a, md.ec.b) };
                let p4 = s4_poly(p, ca, cb, rp.0);
                let path = format!("{}/msolve/{}_{}_t{}.ms", outdir, md.kind, md.id, ti);
                let mut fh = std::fs::File::create(&path).unwrap();
                writeln!(fh, "x1,x2,x3\n{}\n{},\n{},\n{},\n{}", p, mp_fmt(&p4), base_poly(p, &base, "x1"), base_poly(p, &base, "x2"), base_poly(p, &base, "x3")).unwrap();
            }
            writeln!(csv, "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}", md.kind, md.id, md.j, ti, ks[ti], out.work, out.fixed, out.var, out.first, out.hits.len(), out.verified, out.bad, out.degenerate, out.gcd_steps, out.gcd_nontrivial).unwrap();
        }
        csv.flush().unwrap();
    }
    let _ = (HashMap::<u8, u8>::new(), BTreeSet::<u8>::new());
}

fn msolve_stage(args: &[String]) {
    let dir = args[2].clone(); let timeout: u64 = args[3].parse().unwrap(); let threads: usize = args[4].parse().unwrap();
    let mut files: Vec<String> = std::fs::read_dir(format!("{}/msolve", dir)).unwrap().filter_map(|e| e.ok()).map(|e| e.path().to_string_lossy().to_string()).filter(|p| p.ends_with(".ms")).collect();
    files.sort();
    if let Some(mx) = args.get(5) { let mx: usize = mx.parse().unwrap(); files.retain(|f| { let st = std::path::Path::new(f).file_stem().unwrap().to_string_lossy().to_string(); st.split('_').nth(1).unwrap().parse::<usize>().unwrap() < mx }); }
    let queue = std::sync::Arc::new(std::sync::Mutex::new(files.clone()));
    let res = std::sync::Arc::new(std::sync::Mutex::new(Vec::<(String, MsolveOut)>::new()));
    let mut hs = vec![];
    for _ in 0..threads {
        let (q, r) = (queue.clone(), res.clone());
        hs.push(std::thread::spawn(move || loop {
            let job = { q.lock().unwrap().pop() };
            let Some(path) = job else { break };
            let o = run_msolve(&path, timeout);
            println!("{} {} rounds={} max_deg={}", path, o.status, o.rounds, o.max_deg);
            r.lock().unwrap().push((path, o));
        }));
    }
    for h in hs { h.join().unwrap(); }
    let mut v = res.lock().unwrap().clone(); v.sort_by(|a, b| a.0.cmp(&b.0));
    let mut s = std::fs::File::create(format!("{}/msolve_summary.csv", dir)).unwrap();
    let mut t = std::fs::File::create(format!("{}/msolve_trajectories.csv", dir)).unwrap();
    writeln!(s, "cell,status,timeout_s,rounds,max_deg,max_rows,max_cols,sum_cells").unwrap();
    writeln!(t, "cell,round,deg,rows,cols").unwrap();
    for (path, o) in v {
        let cell = std::path::Path::new(&path).file_stem().unwrap().to_string_lossy().to_string();
        writeln!(s, "{},{},{},{},{},{},{},{}", cell, o.status, timeout, o.rounds, o.max_deg, o.max_rows, o.max_cols, o.sum_cells).unwrap();
        for (i, (d, r, c)) in o.traj.iter().enumerate() { writeln!(t, "{},{},{},{},{}", cell, i + 1, d, r, c).unwrap(); }
    }
}

fn median(v: &mut Vec<f64>) -> f64 { v.sort_by(|a, b| a.partial_cmp(b).unwrap()); let n = v.len(); if n == 0 { return f64::NAN; } if n % 2 == 1 { v[n / 2] } else { (v[n / 2 - 1] + v[n / 2]) / 2.0 } }
fn summarize(args: &[String]) {
    let dir = &args[2];
    let txt = std::fs::read_to_string(format!("{}/work_per_target.csv", dir)).unwrap();
    let mut rows: Vec<Vec<String>> = txt.lines().skip(1).map(|l| l.split(',').map(|x| x.to_string()).collect()).collect();
    rows.retain(|r| r.len() >= 15);
    let mut out = std::fs::File::create(format!("{}/summary.csv", dir)).unwrap();
    writeln!(out, "metric,crater_median_of_model_medians,crater_min_model_median,crater_max_model_median,floor_median_of_model_medians,floor_min_model_median,floor_max_model_median,ratio_crater_over_floor,n_crater_models,n_floor_models").unwrap();
    for (name, col) in [("work_total", 5usize), ("work_fixed_by_B", 6), ("work_variable_gcd_roots", 7), ("gcd_steps", 13)] {
        let mut side: HashMap<&str, Vec<f64>> = HashMap::new();
        let mut per: BTreeMap<(String, String), Vec<f64>> = BTreeMap::new();
        for r in &rows { per.entry((r[0].clone(), r[1].clone())).or_default().push(r[col].parse().unwrap()); }
        for ((k, _), mut v) in per { let m = median(&mut v); side.entry(if k == "crater" { "crater" } else { "floor" }).or_default().push(m); }
        let (mut c, mut f) = (side.remove("crater").unwrap_or_default(), side.remove("floor").unwrap_or_default());
        let (nc, nf) = (c.len(), f.len());
        let (mc, mf) = (median(&mut c.clone()), median(&mut f.clone()));
        let mm = |v: &Vec<f64>| (v.iter().cloned().fold(f64::INFINITY, f64::min), v.iter().cloned().fold(f64::NEG_INFINITY, f64::max));
        let ((cl, ch), (fl, fh)) = (mm(&c), mm(&f));
        writeln!(out, "{},{},{},{},{},{},{},{:.6},{},{}", name, mc, cl, ch, mf, fl, fh, mc / mf, nc, nf).unwrap();
        c.clear(); f.clear();
    }
    // decomposability and work to first relation (decomposable targets only)
    for kind in ["crater", "floor"] {
        let dec = rows.iter().filter(|r| r[0] == kind && r[10] != "0").count(); let tot = rows.iter().filter(|r| r[0] == kind).count();
        let mut w: Vec<f64> = rows.iter().filter(|r| r[0] == kind && r[10] != "0").map(|r| r[8].parse().unwrap()).collect();
        let fh: usize = rows.iter().filter(|r| r[0] == kind).map(|r| r[11].parse::<usize>().unwrap()).sum();
        writeln!(out, "# {} decomposable_targets={}/{} median_work_to_first_relation_decomposable={} false_hits_total={}", kind, dec, tot, median(&mut w), fh).unwrap();
    }
    // msolve common-prefix comparison
    if let Ok(tt) = std::fs::read_to_string(format!("{}/msolve_trajectories.csv", dir)) {
        let mut traj: BTreeMap<String, Vec<(u32, u64, u64)>> = BTreeMap::new();
        for l in tt.lines().skip(1) { let x: Vec<&str> = l.split(',').collect(); traj.entry(x[0].to_string()).or_default().push((x[2].parse().unwrap(), x[3].parse().unwrap(), x[4].parse().unwrap())); }
        let sm = std::fs::read_to_string(format!("{}/msolve_summary.csv", dir)).unwrap();
        let status: BTreeMap<String, String> = sm.lines().skip(1).map(|l| { let x: Vec<&str> = l.split(',').collect(); (x[0].to_string(), x[1].to_string()) }).collect();
        let rmin = traj.values().map(|v| v.len()).min().unwrap_or(0);
        writeln!(out, "# msolve cells={} common_prefix_rounds={}", traj.len(), rmin).unwrap();
        writeln!(out, "msolve_cell,status,rounds_total,max_deg_total,max_deg_in_common_prefix,sum_rowsXcols_in_common_prefix,max_rows_in_prefix,max_cols_in_prefix").unwrap();
        for (cell, v) in &traj {
            let pre = &v[..rmin];
            writeln!(out, "{},{},{},{},{},{},{},{}", cell, status.get(cell).cloned().unwrap_or_default(), v.len(), v.iter().map(|x| x.0).max().unwrap(), pre.iter().map(|x| x.0).max().unwrap(), pre.iter().map(|x| x.1 * x.2).sum::<u64>(), pre.iter().map(|x| x.1).max().unwrap(), pre.iter().map(|x| x.2).max().unwrap()).unwrap();
        }
    }
}
