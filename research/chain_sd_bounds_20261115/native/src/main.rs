//! chainsd: exact degree-bounded closure engine for the normalized two-summand
//! Kummer chain system over F_p.
//!
//! System (depth k, N = 2^k):  variables y_{s,j}, s in {1,2}, j in 0..k;
//! rules  y_{s,j}^2 = y_{s,j+1} (j < k-1),  y_{s,k-1}^2 = 1;
//! generator  S_3(y_{1,0}, y_{2,0}, t)  on  y^2 = x^3 + A x + B.
//!
//! Normal forms are squarefree monomials, i.e. characters x1^a x2^b with
//! a, b in Z/N (mask = a | b << k); weight = popcount(a) + popcount(b).
//! Multiplication by y_{s,j} is the shift a -> a + 2^j mod N (or b).
//! V_D = least subspace containing g and closed under every shift applied
//! to elements of weight <= D-1.  LFD = least D with 1 in V_D.
//!
//! Exactness: p < 2^20; rows are stored lazily reduced (entries < 2^63).

use std::collections::HashMap;
use std::io::Write;
use std::time::Instant;

// ---------------------------------------------------------------- F_p
#[derive(Clone, Copy)]
struct Fp {
    p: u64,
}
impl Fp {
    fn add(&self, a: u64, b: u64) -> u64 {
        let s = a + b;
        if s >= self.p { s - self.p } else { s }
    }
    fn sub(&self, a: u64, b: u64) -> u64 {
        if a >= b { a - b } else { a + self.p - b }
    }
    fn mul(&self, a: u64, b: u64) -> u64 {
        (a * b) % self.p
    }
    fn pow(&self, mut a: u64, mut e: u64) -> u64 {
        let mut r = 1u64;
        a %= self.p;
        while e > 0 {
            if e & 1 == 1 { r = self.mul(r, a); }
            a = self.mul(a, a);
            e >>= 1;
        }
        r
    }
    fn inv(&self, a: u64) -> u64 {
        assert!(a % self.p != 0, "inverse of zero");
        self.pow(a, self.p - 2)
    }
    fn legendre(&self, a: u64) -> i32 {
        let a = a % self.p;
        if a == 0 { return 0; }
        if self.pow(a, (self.p - 1) / 2) == 1 { 1 } else { -1 }
    }
    fn sqrt(&self, a: u64) -> Option<u64> {
        // Tonelli-Shanks
        let a = a % self.p;
        if a == 0 { return Some(0); }
        if self.legendre(a) != 1 { return None; }
        let p = self.p;
        let (mut q, mut s) = (p - 1, 0u32);
        while q % 2 == 0 { q /= 2; s += 1; }
        let mut z = 2;
        while self.legendre(z) != -1 { z += 1; }
        let (mut m, mut c, mut t, mut r) = (s, self.pow(z, q), self.pow(a, q), self.pow(a, (q + 1) / 2));
        while t != 1 {
            let (mut i, mut tt) = (0u32, t);
            while tt != 1 { tt = self.mul(tt, tt); i += 1; }
            let b = self.pow(c, 1u64 << (m - i - 1));
            m = i;
            c = self.mul(b, b);
            t = self.mul(t, c);
            r = self.mul(r, b);
        }
        debug_assert_eq!(self.mul(r, r), a);
        Some(r)
    }
    fn neg(&self, a: u64) -> u64 {
        if a == 0 { 0 } else { self.p - a }
    }
    fn from_i(&self, v: i64) -> u64 {
        let m = v.rem_euclid(self.p as i64);
        m as u64
    }
}

// ---------------------------------------------------------------- F_{p^2} = F_p[s]/(s^2 - d)
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct F2(u64, u64);
struct Fp2 {
    f: Fp,
    d: u64,
}
impl Fp2 {
    fn new(f: Fp) -> Fp2 {
        let mut d = 2;
        while f.legendre(d) != -1 { d += 1; }
        Fp2 { f, d }
    }
    fn add(&self, x: F2, y: F2) -> F2 { F2(self.f.add(x.0, y.0), self.f.add(x.1, y.1)) }
    fn sub(&self, x: F2, y: F2) -> F2 { F2(self.f.sub(x.0, y.0), self.f.sub(x.1, y.1)) }
    fn mul(&self, x: F2, y: F2) -> F2 {
        let f = &self.f;
        F2(f.add(f.mul(x.0, y.0), f.mul(self.d, f.mul(x.1, y.1))), f.add(f.mul(x.0, y.1), f.mul(x.1, y.0)))
    }
    fn scal(&self, c: u64, x: F2) -> F2 { F2(self.f.mul(c, x.0), self.f.mul(c, x.1)) }
    fn from(&self, c: u64) -> F2 { F2(c % self.f.p, 0) }
    fn is_zero(&self, x: F2) -> bool { x.0 == 0 && x.1 == 0 }
    fn pow(&self, mut a: F2, mut e: u64) -> F2 {
        let mut r = self.from(1);
        while e > 0 {
            if e & 1 == 1 { r = self.mul(r, a); }
            a = self.mul(a, a);
            e >>= 1;
        }
        r
    }
    fn inv(&self, x: F2) -> F2 {
        let f = &self.f;
        let n = f.sub(f.mul(x.0, x.0), f.mul(self.d, f.mul(x.1, x.1)));
        let ni = f.inv(n);
        F2(f.mul(x.0, ni), f.mul(f.neg(x.1), ni))
    }
    /// square root in F_{p^2} of an element of F_p
    fn sqrt_of_fp(&self, a: u64) -> F2 {
        let f = &self.f;
        let a = a % f.p;
        if a == 0 { return F2(0, 0); }
        if let Some(r) = f.sqrt(a) { return F2(r, 0); }
        let r = f.sqrt(f.mul(a, f.inv(self.d))).unwrap();
        F2(0, r)
    }
    /// roots of lead*X^2 + mid*X + c0 (coefficients in F_p, lead != 0) in F_{p^2}
    fn quad_roots(&self, lead: u64, mid: u64, c0: u64) -> [F2; 2] {
        let f = &self.f;
        let disc = f.sub(f.mul(mid, mid), f.mul(4, f.mul(lead, c0)));
        let s = self.sqrt_of_fp(disc);
        let inv2l = self.from(f.inv(f.mul(2, lead)));
        let mb = self.from(f.neg(mid));
        [self.mul(self.add(mb, s), inv2l), self.mul(self.sub(mb, s), inv2l)]
    }
}

// ---------------------------------------------------------------- instance
#[derive(Clone, Copy, Debug)]
struct Inst {
    p: u64,
    k: u32,
    a: u64,
    b: u64,
    t: u64,
}

fn s3_coeffs(f: &Fp, a: u64, b: u64, t: u64) -> Vec<((u32, u32), u64)> {
    // S_3(X1,X2,t) = (X1-X2)^2 t^2 - 2((X1+X2)(X1 X2 + a) + 2b) t + (X1 X2 - a)^2 - 4 b (X1 + X2)
    let mut c: HashMap<(u32, u32), u64> = HashMap::new();
    let mut add = |e: (u32, u32), v: u64| {
        let cur = c.entry(e).or_insert(0);
        *cur = f.add(*cur, v % f.p);
    };
    let t2 = f.mul(t, t);
    add((2, 0), t2);
    add((1, 1), f.neg(f.mul(2, t2)));
    add((0, 2), t2);
    add((2, 1), f.neg(f.mul(2, t)));
    add((1, 2), f.neg(f.mul(2, t)));
    add((1, 0), f.neg(f.mul(2, f.mul(t, a))));
    add((0, 1), f.neg(f.mul(2, f.mul(t, a))));
    add((0, 0), f.neg(f.mul(4, f.mul(t, b))));
    add((2, 2), 1);
    add((1, 1), f.neg(f.mul(2, a)));
    add((0, 0), f.mul(a, a));
    add((1, 0), f.neg(f.mul(4, b)));
    add((0, 1), f.neg(f.mul(4, b)));
    let mut v: Vec<_> = c.into_iter().filter(|(_, v)| *v != 0).collect();
    v.sort();
    v
}

/// S_3(x1, x2, t) evaluated directly in F_p.
fn s3_eval(f: &Fp, x1: u64, x2: u64, t: u64, a: u64, b: u64) -> u64 {
    let d = f.sub(x1, x2);
    let s = f.add(x1, x2);
    let pr = f.mul(x1, x2);
    let term1 = f.mul(f.mul(d, d), f.mul(t, t));
    let term2 = f.mul(2, f.mul(f.add(f.mul(s, f.add(pr, a)), f.mul(2, b)), t));
    let q = f.sub(pr, a);
    let term3 = f.mul(q, q);
    let term4 = f.mul(4, f.mul(b, s));
    f.sub(f.add(f.sub(term1, term2), term3), term4)
}

fn zeta_of_order(f: &Fp, k: u32) -> u64 {
    // element of exact order 2^k (requires 2^k | p-1)
    assert!((f.p - 1) % (1u64 << k) == 0, "2^k does not divide p-1");
    if k == 0 { return 1; }
    let mut h = 2;
    loop {
        let z = f.pow(h, (f.p - 1) >> k);
        if f.pow(z, 1u64 << (k - 1)) == f.p - 1 { return z; }
        h += 1;
    }
}

/// Grid zeros of S_3 on mu_N x mu_N, listed as (i, j) with x1 = zeta^i, x2 = zeta^j.
/// Uses the quadratic-in-x2 structure: O(N) field operations.
fn grid_zeros(inst: &Inst) -> Vec<(u32, u32)> {
    let f = Fp { p: inst.p };
    let k = inst.k;
    let n = 1u64 << k;
    let zeta = zeta_of_order(&f, k);
    // discrete log table in mu_N
    let mut dlog: HashMap<u64, u32> = HashMap::with_capacity(n as usize);
    let mut z = 1u64;
    for i in 0..n {
        dlog.insert(z, i as u32);
        z = f.mul(z, zeta);
    }
    let (a, b, t) = (inst.a, inst.b, inst.t);
    let mut zeros = Vec::new();
    let mut x1 = 1u64;
    for i in 0..n {
        // S_3(x1, X, t) = (x1-t)^2 X^2 - 2((x1+t)(x1 t + a) + 2b) X + (x1 t - a)^2 - 4b(x1+t)
        let lead = { let d = f.sub(x1, t); f.mul(d, d) };
        let mid = f.neg(f.mul(2, f.add(f.mul(f.add(x1, t), f.add(f.mul(x1, t), a)), f.mul(2, b))));
        let c0 = { let q = f.sub(f.mul(x1, t), a); f.sub(f.mul(q, q), f.mul(4, f.mul(b, f.add(x1, t)))) };
        let mut roots: Vec<u64> = Vec::new();
        if lead == 0 {
            if mid != 0 {
                roots.push(f.mul(f.neg(c0), f.inv(mid)));
            } else if c0 == 0 {
                // identically zero: every X is a root
                for j in 0..n { zeros.push((i as u32, j as u32)); }
                x1 = f.mul(x1, zeta);
                continue;
            }
        } else {
            let disc = f.sub(f.mul(mid, mid), f.mul(4, f.mul(lead, c0)));
            if let Some(s) = f.sqrt(disc) {
                let inv2l = f.inv(f.mul(2, lead));
                roots.push(f.mul(f.add(f.neg(mid), s), inv2l));
                roots.push(f.mul(f.sub(f.neg(mid), s), inv2l));
            }
        }
        for r in roots {
            if let Some(&j) = dlog.get(&r) {
                debug_assert_eq!(s3_eval(&f, x1, r, t, a, b), 0);
                zeros.push((i as u32, j));
            }
        }
        x1 = f.mul(x1, zeta);
    }
    zeros.sort();
    zeros.dedup();
    zeros
}

/// Independent brute-force grid zero count (O(N^2)), for cross-checking small k.
fn grid_zeros_brute(inst: &Inst) -> Vec<(u32, u32)> {
    let f = Fp { p: inst.p };
    let n = 1u64 << inst.k;
    let zeta = zeta_of_order(&f, inst.k);
    let mut pw = Vec::with_capacity(n as usize);
    let mut z = 1u64;
    for _ in 0..n { pw.push(z); z = f.mul(z, zeta); }
    let mut zeros = Vec::new();
    for i in 0..n as usize {
        for j in 0..n as usize {
            if s3_eval(&f, pw[i], pw[j], inst.t, inst.a, inst.b) == 0 { zeros.push((i as u32, j as u32)); }
        }
    }
    zeros
}

// ---------------------------------------------------------------- layout
struct Layout {
    k: u32,
    kshift: u32,
    uni: bool,    // one-variable model: only coordinate 1 is shifted  // shifts use 2^j for j < kshift (kshift <= k); kshift < k gives the no-wrap Z model
    d: u32,
    masks: Vec<u64>,      // weight-descending, then ascending mask
    idx: HashMap<u64, u32>,
    wt: Vec<u8>,
    n_top: usize,         // number of columns of weight exactly D (they come first)
}

fn popcnt(m: u64) -> u32 { m.count_ones() }

impl Layout {
    fn new(k: u32, d: u32) -> Layout {
        let nv = 2 * k;
        let mut by_w: Vec<Vec<u64>> = vec![Vec::new(); d as usize + 1];
        // enumerate all masks of popcount <= d over nv bits (Gosper's hack per weight)
        for w in 0..=d {
            if w == 0 { by_w[0].push(0); continue; }
            if w > nv { break; }
            let mut m: u64 = (1u64 << w) - 1;
            let limit: u64 = 1u64 << nv;
            while m < limit {
                by_w[w as usize].push(m);
                // next permutation of bits
                let c = m & m.wrapping_neg();
                let r = m + c;
                if r >= limit { break; }
                m = (((r ^ m) >> 2) / c) | r;
            }
        }
        let mut masks = Vec::new();
        for w in (0..=d).rev() {
            masks.extend_from_slice(&by_w[w as usize]);
        }
        let n_top = by_w[d as usize].len();
        let mut idx = HashMap::with_capacity(masks.len());
        for (i, &m) in masks.iter().enumerate() { idx.insert(m, i as u32); }
        let wt = masks.iter().map(|&m| popcnt(m) as u8).collect();
        Layout { k, kshift: k, uni: false, d, masks, idx, wt, n_top }
    }
    fn n(&self) -> usize { self.masks.len() }
    /// shift mask by variable v (0..2k): add 2^j to the corresponding exponent mod N
    fn shift(&self, mask: u64, v: u32) -> u64 {
        let k = self.k;
        let ks = self.kshift;
        let nmask = (1u64 << k) - 1;
        let a = mask & nmask;
        let b = mask >> k;
        if v < ks {
            let a2 = (a + (1u64 << v)) & nmask;
            a2 | (b << k)
        } else {
            let b2 = (b + (1u64 << (v - ks))) & nmask;
            a | (b2 << k)
        }
    }
}

// ---------------------------------------------------------------- closure engine
struct Basis {
    f: Fp,
    n: usize,
    track: bool,
    used: Vec<Vec<u32>>,          // per row: basis rows subtracted during its reduction (when track)
    last_used: Vec<u32>,          // scratch: rows used by the most recent reduce()
    source: Vec<(u32, u32)>,      // per row: (variable, source row); generator = (u32::MAX, u32::MAX)
    rows: Vec<Vec<u64>>,          // lazily reduced entries (< 2^63), pivot entry normalized to 1 exactly
    pivcol: Vec<usize>,           // pivot column of each row
    pivot_row: Vec<Option<u32>>,  // column -> row index
    lazy_count: Vec<u32>,         // number of unreduced row operations applied to each row
}

const LAZY_MAX: u32 = 1 << 16; // p < 2^23, products < 2^46, 2^16 additions stay < 2^62

impl Basis {
    fn new(f: Fp, n: usize) -> Basis {
        Basis { f, n, track: false, used: Vec::new(), last_used: Vec::new(), source: Vec::new(), rows: Vec::new(), pivcol: Vec::new(), pivot_row: vec![None; n], lazy_count: Vec::new() }
    }
    fn normalize_row(&self, row: &mut [u64]) {
        let p = self.f.p;
        for x in row.iter_mut() { *x %= p; }
    }
    /// Reduce a (lazily represented) row against the basis; returns Some(pivot col) if nonzero after reduction.
    /// On return the row is fully reduced mod p.
    fn reduce(&mut self, row: &mut Vec<u64>) -> Option<usize> {
        let p = self.f.p;
        let n = self.n;
        let mut ops = 0u32;
        let mut first_nz: Option<usize> = None;
        if self.track { self.last_used.clear(); }
        for col in 0..n {
            let v = row[col] % p;
            if v == 0 { row[col] = 0; continue; }
            if let Some(r) = self.pivot_row[col] {
                if self.track { self.last_used.push(r); }
                let br = &self.rows[r as usize];
                let c = p - v; // row += c * br  (br[col] == 1 exactly, so row[col] becomes ≡ 0)
                for j in col..n {
                    row[j] += c * br[j];
                }
                row[col] = 0;
                ops += 1;
                if ops >= LAZY_MAX { self.normalize_row(&mut row[col..]); ops = 0; }
            } else {
                row[col] = v;
                if first_nz.is_none() { first_nz = Some(col); }
                // entries after col keep accumulating; they are reduced when visited
            }
        }
        first_nz
    }
    /// Insert a reduced row with given pivot; normalizes pivot to 1. Returns row index.
    fn insert(&mut self, mut row: Vec<u64>, piv: usize) -> usize {
        let p = self.f.p;
        let inv = self.f.inv(row[piv] % p);
        for j in piv..self.n {
            row[j] = (row[j] % p) * inv % p;
        }
        debug_assert_eq!(row[piv], 1);
        let r = self.rows.len();
        self.rows.push(row);
        self.pivcol.push(piv);
        self.pivot_row[piv] = Some(r as u32);
        self.lazy_count.push(0);
        if self.track { let u = std::mem::take(&mut self.last_used); self.used.push(u); self.source.push((u32::MAX, u32::MAX)); }
        r
    }
}

#[derive(Debug, Clone)]
struct ClosureResult {
    k: u32,
    d: u32,
    columns: usize,
    rank: usize,
    one_in_v: bool,
    profile: Vec<usize>, // number of pivots of each weight 0..=D
    low_processed: usize,
    seconds: f64,
    early_exit: bool,
}

fn closure(inst: &Inst, d: u32, early: bool, verify: bool, log: &mut dyn Write) -> (ClosureResult, Layout, Basis) {
    closure_gen(inst, d, early, verify, log, inst.k, None)
}

/// General closure: exponent width `inst.k` bits, shifts by 2^j for j < kshift, optional explicit generator
/// (sparse list of (a, b, coef)); None = NF(S_3).
fn closure_gen(inst: &Inst, d: u32, early: bool, verify: bool, log: &mut dyn Write, kshift: u32, gen: Option<&[(u64, u64, u64)]>) -> (ClosureResult, Layout, Basis) { closure_full(inst, d, early, verify, log, kshift, gen, false) }

fn closure_full(inst: &Inst, d: u32, early: bool, verify: bool, log: &mut dyn Write, kshift: u32, gen: Option<&[(u64, u64, u64)]>, uni: bool) -> (ClosureResult, Layout, Basis) {
    let t0 = Instant::now();
    let f = Fp { p: inst.p };
    let mut lay = Layout::new(inst.k, d);
    lay.kshift = kshift;
    lay.uni = uni;
    let n = lay.n();
    let mut basis = Basis::new(f, n);
    basis.track = std::env::var("CHAINSD_TRACK").is_ok();
    // generator
    let mut g = vec![0u64; n];
    let nm = (1u64 << inst.k) - 1;
    let terms: Vec<(u64, u64, u64)> = match gen {
        Some(t) => t.to_vec(),
        None => s3_coeffs(&f, inst.a, inst.b, inst.t).into_iter().map(|((e1, e2), c)| (e1 as u64, e2 as u64, c)).collect(),
    };
    let terms_for_cert: Vec<(u64, u64, u64)> = terms.iter().map(|&(a, b, c)| (a & nm, b & nm, c % f.p)).collect();
    for (e1, e2, c) in terms {
        let mask = (e1 & nm) | ((e2 & nm) << inst.k);
        let w = popcnt(mask);
        if w > d { panic!("generator weight {} exceeds D = {}", w, d); }
        let i = lay.idx[&mask] as usize;
        g[i] = f.add(g[i], c % f.p);
    }
    let piv = basis.reduce(&mut g).expect("generator is zero");
    basis.insert(g, piv);
    let mut cert: Option<std::io::BufWriter<std::fs::File>> = std::env::var("CHAINSD_CERT").ok().map(|path| {
        let mut w = std::io::BufWriter::new(std::fs::File::create(path).expect("cert file"));
        let _ = writeln!(w, "chainsd-cert/v1");
        let _ = writeln!(w, "p {} k {} kshift {} uni {} D {}", inst.p, inst.k, kshift, uni as u8, d);
        let gt: Vec<String> = terms_for_cert.iter().map(|(a, b, c)| format!("{}:{}:{}", a, b, c)).collect();
        let _ = writeln!(w, "generator {}", gt.join(" "));
        w
    });
    let one_col = n - 1; // mask 0 is last (weight 0)
    debug_assert_eq!(lay.masks[one_col], 0);
    let mut queue: Vec<usize> = Vec::new(); // low rows to process
    if (lay.wt[piv] as u32) < d { queue.push(0); }
    let mut low_processed = 0usize;
    let mut early_exit = false;
    let nv = if lay.uni { kshift } else { 2 * kshift };
    let mut shifted = vec![0u64; n];
    'outer: while let Some(r) = queue.pop() {
        low_processed += 1;
        // gather sparse entries of row r
        let entries: Vec<(usize, u64)> = {
            let row = &basis.rows[r];
            let p = f.p;
            row.iter().enumerate().filter_map(|(c, &v)| { let v = v % p; if v != 0 { Some((c, v)) } else { None } }).collect()
        };
        for v in 0..nv {
            for x in shifted.iter_mut() { *x = 0; }
            for &(c, val) in &entries {
                let m2 = lay.shift(lay.masks[c], v);
                let j = *lay.idx.get(&m2).expect("shift left the weight window") as usize;
                shifted[j] = f.add(shifted[j], val);
            }
            let mut row = shifted.clone();
            if let Some(piv) = basis.reduce(&mut row) {
                let rr = basis.insert(row, piv);
                if basis.track { basis.source[rr] = (v, r as u32); }
                if let Some(w) = cert.as_mut() { let _ = writeln!(w, "s {} {}", v, r); }
                if (lay.wt[piv] as u32) < d { queue.push(rr); }
                if piv == one_col {
                    if early { early_exit = true; break 'outer; }
                }
            }
        }
        if low_processed % 500 == 0 {
            let _ = writeln!(log, "  [k={} D={}] processed {} low rows, rank {}, queue {}, {:.1}s", inst.k, d, low_processed, basis.rows.len(), queue.len(), t0.elapsed().as_secs_f64());
        }
    }
    let one_in_v = basis.pivot_row[one_col].is_some();
    if basis.track && one_in_v {
        // transitive dependency set of the row with pivot e_0
        let start = basis.pivot_row[one_col].unwrap() as usize;
        let mut mark = vec![false; basis.rows.len()];
        let mut stack = vec![start];
        while let Some(r) = stack.pop() {
            if mark[r] { continue; }
            mark[r] = true;
            let (v, src) = basis.source[r];
            if v != u32::MAX { stack.push(src as usize); }
            for &u in &basis.used[r] { stack.push(u as usize); }
        }
        let cnt = mark.iter().filter(|&&m| m).count();
        let mut by_w = vec![0usize; d as usize + 1];
        for (r, &m) in mark.iter().enumerate() { if m { by_w[lay.wt[basis.pivcol[r]] as usize] += 1; } }
        let _ = writeln!(log, "  derivation of 1: {} of {} rows used (by pivot weight {:?})", cnt, basis.rows.len(), by_w);
    }
    if let Some(w) = cert.as_mut() { let _ = writeln!(w, "end rank {} one_in_V {}", basis.rows.len(), one_in_v as u8); }
    drop(cert);
    let mut profile = vec![0usize; d as usize + 1];
    for &pc in &basis.pivcol { profile[lay.wt[pc] as usize] += 1; }
    if verify && !early_exit {
        // full closure re-check: every low row times every variable reduces to zero
        let mut bad = 0usize;
        for r in 0..basis.rows.len() {
            if (lay.wt[basis.pivcol[r]] as u32) >= d { continue; }
            let p = f.p;
            let entries: Vec<(usize, u64)> = basis.rows[r].iter().enumerate().filter_map(|(c, &v)| { let v = v % p; if v != 0 { Some((c, v)) } else { None } }).collect();
            for v in 0..nv {
                let mut row = vec![0u64; n];
                for &(c, val) in &entries {
                    let m2 = lay.shift(lay.masks[c], v);
                    let j = lay.idx[&m2] as usize;
                    row[j] = f.add(row[j], val);
                }
                if basis.reduce(&mut row).is_some() { bad += 1; }
            }
        }
        let _ = writeln!(log, "  verify_closed: bad = {}", bad);
        assert_eq!(bad, 0, "closure verification failed");
    }
    let res = ClosureResult { k: inst.k, d, columns: n, rank: basis.rows.len(), one_in_v, profile, low_processed, seconds: t0.elapsed().as_secs_f64(), early_exit };
    (res, lay, basis)
}

// ---------------------------------------------------------------- evaluation-functional test
/// Points (xi1, xi2) on S_3 = 0 over F_{p^2} with xi1 in mu_N (first coordinate in the base).
fn boundary_points(inst: &Inst) -> Vec<(F2, F2)> {
    let f = Fp { p: inst.p };
    let f2 = Fp2::new(f);
    let n = 1u64 << inst.k;
    let zeta = zeta_of_order(&f, inst.k);
    let (a, b, t) = (inst.a, inst.b, inst.t);
    let mut pts = Vec::new();
    let mut x1 = 1u64;
    for _ in 0..n {
        let lead = { let d = f.sub(x1, t); f.mul(d, d) };
        let mid = f.neg(f.mul(2, f.add(f.mul(f.add(x1, t), f.add(f.mul(x1, t), a)), f.mul(2, b))));
        let c0 = { let q = f.sub(f.mul(x1, t), a); f.sub(f.mul(q, q), f.mul(4, f.mul(b, f.add(x1, t)))) };
        if lead != 0 {
            for r in f2.quad_roots(lead, mid, c0) { pts.push((f2.from(x1), r)); }
        } else if mid != 0 {
            pts.push((f2.from(x1), f2.from(f.mul(f.neg(c0), f.inv(mid)))));
        }
        x1 = f.mul(x1, zeta);
    }
    pts
}

/// Evaluate every basis row at the boundary points with the standard lift [0,N); count rows that
/// fail to vanish at some point. Returns (rows_tested, rows_nonvanishing, points).
fn eval_test(inst: &Inst, lay: &Layout, basis: &Basis) -> (usize, usize, usize) { eval_test_lift(inst, lay, basis, 0) }

/// lift mode: 0 = standard [0,N) in both coordinates; 1 = balanced (-N/2, N/2] in both; 2 = balanced only in the
/// coordinate that is not in mu_N (the other is periodic anyway).
fn eval_test_lift(inst: &Inst, lay: &Layout, basis: &Basis, lift: u32) -> (usize, usize, usize) {
    let f = Fp { p: inst.p };
    let f2 = Fp2::new(f);
    let n = 1u64 << inst.k;
    let pts = boundary_points(inst);
    // also the swapped points (xi2 in mu_N)
    // (point, which coordinate is the mu_N one: 0 = first, 1 = second)
    let mut all: Vec<(F2, F2, u8)> = pts.iter().map(|(x, y)| (*x, *y, 0u8)).collect();
    for (x, y) in &pts { all.push((*y, *x, 1)); }
    let half = n / 2;
    let f2inv = |z: F2| f2.inv(z);
    let liftpow = |base: F2, e: u64, balanced: bool| -> F2 {
        if balanced && e > half { f2.pow(f2inv(base), n - e) } else { f2.pow(base, e) }
    };
    // power tables per point
    let nm = (1u64 << inst.k) - 1;
    let mut bad_rows = 0usize;
    let p = f.p;
    for row in &basis.rows {
        let entries: Vec<(u64, u64)> = row.iter().enumerate().filter_map(|(c, &v)| { let v = v % p; if v != 0 { Some((lay.masks[c], v)) } else { None } }).collect();
        let mut fails = false;
        for (x1, x2, which) in &all {
            let mut acc = F2(0, 0);
            let bal_a = lift == 1 || (lift == 2 && *which == 1);
            let bal_b = lift == 1 || (lift == 2 && *which == 0);
            for &(m, c) in &entries {
                let a = m & nm;
                let b = m >> inst.k;
                let term = f2.mul(liftpow(*x1, a, bal_a), liftpow(*x2, b, bal_b));
                acc = f2.add(acc, f2.scal(c, term));
            }
            if !f2.is_zero(acc) { fails = true; break; }
        }
        if fails { bad_rows += 1; }
    }
    let _ = n;
    (basis.rows.len(), bad_rows, all.len())
}

// ---------------------------------------------------------------- instance search
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        // splitmix64
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: u64) -> u64 { self.next() % n }
}

fn is_nonsingular(f: &Fp, a: u64, b: u64) -> bool {
    let d = f.add(f.mul(4, f.pow(a, 3)), f.mul(27, f.mul(b, b)));
    d != 0
}

fn parse(args: &[String], i: usize) -> u64 { args[i].parse::<u64>().unwrap_or_else(|_| panic!("bad integer argument {}", args[i])) }

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("usage:\n  chainsd unit p k A B t\n  chainsd closure p k A B t D [early] [verify] [evaltest]\n  chainsd lfd p k A B t Dmax [evaltest]\n  chainsd find-unit p k seed count [j1728]");
        std::process::exit(2);
    }
    let mut log = std::io::stderr();
    match args[1].as_str() {
        "unit" => {
            let inst = Inst { p: parse(&args, 2), k: parse(&args, 3) as u32, a: parse(&args, 4), b: parse(&args, 5), t: parse(&args, 6) };
            let f = Fp { p: inst.p };
            let z = grid_zeros(&inst);
            let brute = if inst.k <= 10 { Some(grid_zeros_brute(&inst)) } else { None };
            let ok = brute.as_ref().map(|b| *b == z);
            let const_term = f.sub(f.mul(inst.a, inst.a), f.mul(4, f.mul(inst.b, inst.t)));
            let rational = f.legendre(f.add(f.add(f.pow(inst.t, 3), f.mul(inst.a, inst.t)), inst.b));
            println!("{{\"p\":{},\"k\":{},\"A\":{},\"B\":{},\"t\":{},\"two_k1_divides_pm1\":{},\"nonsingular\":{},\"grid_zeros\":{},\"unit\":{},\"brute_agrees\":{:?},\"t_nonzero\":{},\"const_term_nonzero\":{},\"target_rational\":{}}}",
                inst.p, inst.k, inst.a, inst.b, inst.t, (inst.p - 1) % (1u64 << (inst.k + 1)) == 0, is_nonsingular(&f, inst.a, inst.b), z.len(), z.is_empty(), ok, inst.t != 0, const_term != 0, rational == 1);
        }
        "closure" | "lfd" => {
            let inst = Inst { p: parse(&args, 2), k: parse(&args, 3) as u32, a: parse(&args, 4), b: parse(&args, 5), t: parse(&args, 6) };
            let dmax = parse(&args, 7) as u32;
            let flags: Vec<&str> = args[8..].iter().map(|s| s.as_str()).collect();
            let early = flags.contains(&"early");
            let verify = flags.contains(&"verify");
            let evaltest = flags.contains(&"evaltest");
            let zeros = grid_zeros(&inst);
            let dstart = if args[1] == "lfd" { 2 } else { dmax };
            for d in dstart..=dmax {
                let (res, lay, basis) = closure(&inst, d, early, verify, &mut log);
                let ev = if evaltest && !res.early_exit { Some((eval_test_lift(&inst, &lay, &basis, 0), eval_test_lift(&inst, &lay, &basis, 1), eval_test_lift(&inst, &lay, &basis, 2))) } else { None };
                println!("{{\"p\":{},\"k\":{},\"A\":{},\"B\":{},\"t\":{},\"grid_zeros\":{},\"D\":{},\"columns\":{},\"rank\":{},\"one_in_V\":{},\"profile\":{:?},\"low_processed\":{},\"seconds\":{:.2},\"early_exit\":{},\"evaltest\":{:?}}}",
                    inst.p, inst.k, inst.a, inst.b, inst.t, zeros.len(), res.d, res.columns, res.rank, res.one_in_v, res.profile, res.low_processed, res.seconds, res.early_exit, ev);
                if res.one_in_v { break; }
            }
        }
        "find-unit" => {
            let p = parse(&args, 2);
            let k = parse(&args, 3) as u32;
            let seed = parse(&args, 4);
            let count = parse(&args, 5);
            let j1728 = args.len() > 6 && args[6] == "j1728";
            let f = Fp { p };
            let mut rng = Rng(seed);
            let mut found = 0;
            let mut tried = 0;
            while found < count {
                tried += 1;
                let (a, b) = if j1728 { (rng.below(p - 1) + 1, 0) } else { (rng.below(p), rng.below(p)) };
                if !is_nonsingular(&f, a, b) { continue; }
                let t = rng.below(p - 1) + 1;
                let rhs = f.add(f.add(f.pow(t, 3), f.mul(a, t)), b);
                if f.legendre(rhs) != 1 { continue; }
                let inst = Inst { p, k, a, b, t };
                let z = grid_zeros(&inst);
                let const_term = f.sub(f.mul(a, a), f.mul(4, f.mul(b, t)));
                println!("{{\"p\":{},\"k\":{},\"A\":{},\"B\":{},\"t\":{},\"grid_zeros\":{},\"unit\":{},\"t_square\":{},\"const_term_nonzero\":{},\"tried\":{}}}", p, k, a, b, t, z.len(), z.is_empty(), f.legendre(t) == 1, const_term != 0, tried);
                if z.is_empty() { found += 1; }
            }
        }

        "dump" => {
            let inst = Inst { p: parse(&args, 2), k: parse(&args, 3) as u32, a: parse(&args, 4), b: parse(&args, 5), t: parse(&args, 6) };
            let d = parse(&args, 7) as u32;
            let wmax = parse(&args, 8) as u32;
            let (res, lay, basis) = closure(&inst, d, false, false, &mut log);
            let p = inst.p;
            let nm = (1u64 << inst.k) - 1;
            let mut tot_nnz = 0usize;
            for (r, row) in basis.rows.iter().enumerate() {
                let entries: Vec<(u64, u64, u64)> = row.iter().enumerate().filter_map(|(c, &v)| { let v = v % p; if v != 0 { let m = lay.masks[c]; Some((m & nm, m >> inst.k, v)) } else { None } }).collect();
                tot_nnz += entries.len();
                if (lay.wt[basis.pivcol[r]] as u32) <= wmax {
                    let s: Vec<String> = entries.iter().map(|(a, b, c)| format!("{}:{}:{}", a, b, c)).collect();
                    println!("row {} pivw {} nnz {} :: {}", r, lay.wt[basis.pivcol[r]], entries.len(), s.join(" "));
                }
            }
            println!("# rank {} columns {} one_in_V {} avg_nnz {:.1}", res.rank, res.columns, res.one_in_v, tot_nnz as f64 / res.rank as f64);
        }

        "find-twist" => {
            // admissible family: cubic x^3+Ax+B with no root in mu_N (checked), target t with t^3+At+B a nonsquare
            let p = parse(&args, 2);
            let k = parse(&args, 3) as u32;
            let seed = parse(&args, 4);
            let count = parse(&args, 5);
            let f = Fp { p };
            let n = 1u64 << k;
            let zeta = zeta_of_order(&f, k);
            let mut rng = Rng(seed);
            let mut found = 0;
            while found < count {
                let (a, b) = (rng.below(p), rng.below(p));
                if !is_nonsingular(&f, a, b) { continue; }
                // no root in mu_N
                let mut z = 1u64; let mut root_in_mu = false;
                for _ in 0..n { if f.add(f.add(f.pow(z, 3), f.mul(a, z)), b) == 0 { root_in_mu = true; break; } z = f.mul(z, zeta); }
                if root_in_mu { continue; }
                let mut irreducible = true; // cubic has no root in F_p at all?
                for x in 0..p { if f.add(f.add(f.pow(x, 3), f.mul(a, x)), b) == 0 { irreducible = false; break; } }
                let t = loop { let t = rng.below(p); if f.legendre(f.add(f.add(f.pow(t, 3), f.mul(a, t)), b)) == -1 { break t; } };
                let inst = Inst { p, k, a, b, t };
                let z = grid_zeros(&inst);
                let const_term = f.sub(f.mul(a, a), f.mul(4, f.mul(b, t)));
                println!("{{\"p\":{},\"k\":{},\"A\":{},\"B\":{},\"t\":{},\"grid_zeros\":{},\"unit\":{},\"cubic_irreducible\":{},\"t_nonzero\":{},\"const_term_nonzero\":{},\"tangent_3t2_plus_A_nonzero\":{}}}", p, k, a, b, t, z.len(), z.is_empty(), irreducible, t != 0, const_term != 0, f.add(f.mul(3, f.mul(t, t)), a) != 0);
                found += 1;
            }
        }

        "family" => {
            // admissible CM family: n a non-square in F_p, a = n^2 (n-1), B = 0, t = n. Prints the instance and its unit status.
            let p = parse(&args, 2);
            let k = parse(&args, 3) as u32;
            let n = parse(&args, 4) % p;
            let f = Fp { p };
            assert!(p < (1u64 << 23), "p must be below 2^23");
            assert!((p - 1) % (1u64 << (k + 1)) == 0, "2^(k+1) must divide p-1");
            assert!(f.legendre(n) == -1, "n must be a non-square");
            let a = f.mul(f.mul(n, n), f.sub(n, 1));
            let inst = Inst { p, k, a, b: 0, t: n };
            let z = grid_zeros(&inst);
            let brute = if k <= 9 { Some(grid_zeros_brute(&inst).len()) } else { None };
            println!("{{\"p\":{},\"k\":{},\"n\":{},\"A\":{},\"B\":0,\"t\":{},\"grid_zeros\":{},\"brute\":{:?},\"unit\":{},\"f_t\":{},\"f_t_square\":{},\"nonsingular\":{}}}", p, k, n, a, n, z.len(), brute, z.is_empty(), f.add(f.pow(n, 3), f.mul(a, n)), f.legendre(f.add(f.pow(n, 3), f.mul(a, n))) == 1, is_nonsingular(&f, a, 0));
        }

        "nowrap" => {
            // Z-exponent model: exponent width k + D + 1 bits, shifts only by 2^j with j < k. Reports the rank of the
            // closure and the rank of its projection mod 2^k (reduction of exponents), to compare with the Q_k closure.
            let inst = Inst { p: parse(&args, 2), k: parse(&args, 3) as u32, a: parse(&args, 4), b: parse(&args, 5), t: parse(&args, 6) };
            let d = parse(&args, 7) as u32;
            let kk = inst.k + 2;
            let wide = Inst { k: kk, ..inst };
            let (res, lay, basis) = closure_gen(&wide, d, false, true, &mut log, inst.k, None);
            // project rows mod 2^k into the Q_k layout and compute the rank
            let lay_k = Layout::new(inst.k, d);
            let f = Fp { p: inst.p };
            let mut bk = Basis::new(f, lay_k.n());
            let nmw = (1u64 << kk) - 1;
            let nmk = (1u64 << inst.k) - 1;
            let mut maxexp = 0u64;
            for row in &basis.rows {
                let mut v = vec![0u64; lay_k.n()];
                for (c, &x) in row.iter().enumerate() { let x = x % f.p; if x == 0 { continue; }
                    let m = lay.masks[c]; let a = m & nmw; let b = m >> kk; maxexp = maxexp.max(a).max(b);
                    let mk = (a & nmk) | ((b & nmk) << inst.k);
                    let j = lay_k.idx[&mk] as usize; v[j] = f.add(v[j], x); }
                if let Some(pv) = bk.reduce(&mut v) { bk.insert(v, pv); }
            }
            let (res_k, _, _) = closure(&inst, d, false, false, &mut log);
            println!("{{\"k\":{},\"D\":{},\"zrank\":{},\"zprofile\":{:?},\"proj_rank\":{},\"q_rank\":{},\"q_one_in_V\":{},\"max_exponent_seen\":{},\"N\":{}}}", inst.k, d, res.rank, res.profile, bk.rows.len(), res_k.rank, res_k.one_in_v, maxexp, 1u64 << inst.k);
        }
        "uni" => {
            // one-variable toy: Q = F_p[x]/(x^N - 1), generator c2 x^2 + c1 x + c0, shifts x^(2^j)
            let p = parse(&args, 2); let k = parse(&args, 3) as u32;
            let c0 = parse(&args, 4); let c1 = parse(&args, 5); let c2 = parse(&args, 6); let dmax = parse(&args, 7) as u32;
            let inst = Inst { p, k, a: 0, b: 0, t: 0 };
            let f = Fp { p };
            // unit check: no root in mu_N
            let zeta = zeta_of_order(&f, k); let mut z = 1u64; let mut roots = 0;
            for _ in 0..(1u64 << k) { if f.add(f.add(f.mul(c2, f.mul(z, z)), f.mul(c1, z)), c0) == 0 { roots += 1; } z = f.mul(z, zeta); }
            let gen = [(0u64, 0u64, c0), (1, 0, c1), (2, 0, c2)];
            for d in 2..=dmax {
                let (res, _, _) = closure_full(&inst, d, false, true, &mut log, k, Some(&gen), true);
                println!("{{\"uni\":true,\"p\":{},\"k\":{},\"q\":[{},{},{}],\"roots_in_muN\":{},\"D\":{},\"columns\":{},\"rank\":{},\"one_in_V\":{},\"profile\":{:?},\"seconds\":{:.2}}}", p, k, c0, c1, c2, roots, d, res.columns, res.rank, res.one_in_v, res.profile, res.seconds);
                if res.one_in_v { break; }
            }
        }
        _ => { eprintln!("unknown command"); std::process::exit(2); }
    }
}
