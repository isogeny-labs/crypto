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


        "bnddetail" => {
            let inst = Inst { p: parse(&args, 2), k: parse(&args, 3) as u32, a: parse(&args, 4), b: parse(&args, 5), t: parse(&args, 6) };
            bnd_detail(&inst, parse(&args, 7) as u32, parse(&args, 8) as u32, &mut log);
        }

        "gen2" => {
            // LFD of a custom two-variable generator: gen2 p k Dmax a:b:c ...  (exponents < 4); brute-force unit check
            let p = parse(&args, 2); let k = parse(&args, 3) as u32; let dmax = parse(&args, 4) as u32;
            let terms: Vec<(u64, u64, u64)> = args[5..].iter().map(|s| { let v: Vec<u64> = s.split(':').map(|x| x.parse().unwrap()).collect(); (v[0], v[1], v[2] % p) }).collect();
            let f = Fp { p };
            let n = 1u64 << k; let zeta = zeta_of_order(&f, k);
            let mut pw = vec![1u64; n as usize]; for i in 1..n as usize { pw[i] = f.mul(pw[i-1], zeta); }
            let mut zeros = 0u64;
            for i in 0..n as usize { for j in 0..n as usize {
                let mut acc = 0u64; for &(a, b, c) in &terms { acc = f.add(acc, f.mul(c, f.mul(f.pow(pw[i], a), f.pow(pw[j], b)))); }
                if acc == 0 { zeros += 1; } } }
            let inst = Inst { p, k, a: 0, b: 0, t: 0 };
            let mut lfd = None;
            for d in 2..=dmax {
                let (res, _, _) = closure_gen(&inst, d, true, false, &mut log, k, Some(&terms));
                if res.one_in_v { lfd = Some(d); break; }
            }
            println!("{{\"gen2\":true,\"p\":{},\"k\":{},\"grid_zeros\":{},\"terms\":{},\"lfd\":{:?}}}", p, k, zeros, terms.len(), lfd);
        }

        "cofactor" => {
            // cofactor p k D a:b:c ... : closure of the given generator, then support statistics of g^{-1} U_D
            let p = parse(&args, 2); let k = parse(&args, 3) as u32; let d = parse(&args, 4) as u32;
            let terms: Vec<(u64, u64, u64)> = args[5..].iter().map(|s| { let v: Vec<u64> = s.split(':').map(|x| x.parse().unwrap()).collect(); (v[0], v[1], v[2] % p) }).collect();
            let inst = Inst { p, k, a: 0, b: 0, t: 0 };
            let (res, lay, basis) = closure_gen(&inst, d, false, false, &mut log, k, Some(&terms));
            let mut rng = Rng(5);
            let all: Vec<usize> = (0..basis.rows.len()).collect();
            let low: Vec<usize> = (0..basis.rows.len()).filter(|&r| (lay.wt[basis.pivcol[r]] as u32) < d).collect();
            let sa = cofactor_stats(&inst, &lay, &basis, &all, &terms, &mut rng);
            let sl = cofactor_stats(&inst, &lay, &basis, &low, &terms, &mut rng);
            println!("{{\"k\":{},\"D\":{},\"rank\":{},\"one_in_V\":{},\"all:max_pop1,max_pop2,max_total,touchesTop2\":[{},{},{},{}],\"low\":[{},{},{},{}],\"low_shift_hits_top\":\"{}\"}}", k, d, res.rank, res.one_in_v, sa.0, sa.1, sa.2, sa.3, sl.0, sl.1, sl.2, sl.3,
                sl.4.map_or("none".to_string(), |(a, b, j)| format!("({},{}) j={}", a, b, j)));
        }

        "lwm" => {
            // lwm p X w a:b:c ... : low-weight multiples of the generator in F[x1,x2] inside the box [0,X)^2
            let p = parse(&args, 2); let xbox = parse(&args, 3) as usize; let w = parse(&args, 4) as u32;
            let terms: Vec<(u64, u64, u64)> = args[5..].iter().map(|s| { let v: Vec<u64> = s.split(':').map(|x| x.parse().unwrap()).collect(); (v[0], v[1], v[2] % p) }).collect();
            let (dim, m1, m2, mt, md, sup) = low_weight_multiples(p, xbox, w, &terms, 9);
            let hi: Vec<String> = sup.iter().filter(|(a, b)| (*a as u64).count_ones() >= m1 || (*b as u64).count_ones() >= m2).take(8).map(|(a, b)| format!("({},{})", a, b)).collect();
            println!("{{\"X\":{},\"w\":{},\"kernel_dim\":{},\"max_pop_x1\":{},\"max_pop_x2\":{},\"max_total\":{},\"max_deg\":{},\"extreme_monomials\":\"{}\"}}", xbox, w, dim, m1, m2, mt, md, hi.join(" "));
        }

        "lwm2" => {
            let p = parse(&args, 2); let xbox = parse(&args, 3) as usize; let w = parse(&args, 4) as u32;
            let terms: Vec<(u64, u64, u64)> = args[5..].iter().map(|s| { let v: Vec<u64> = s.split(':').map(|x| x.parse().unwrap()).collect(); (v[0], v[1], v[2] % p) }).collect();
            let (dim, m1, m2, mt, md, nl) = lwm_eval(p, xbox, w, &terms, 11);
            println!("{{\"X\":{},\"w\":{},\"unknowns\":{},\"kernel_dim\":{},\"max_pop_x1\":{},\"max_pop_x2\":{},\"max_total\":{},\"max_deg_h\":{}}}", xbox, w, nl, dim, m1, m2, mt, md);
        }
        "bnd" => {
            let inst = Inst { p: parse(&args, 2), k: parse(&args, 3) as u32, a: parse(&args, 4), b: parse(&args, 5), t: parse(&args, 6) };
            let d = parse(&args, 7) as u32;
            let seed = if args.len() > 8 { parse(&args, 8) } else { 1 };
            bnd_test(&inst, d, seed, &mut log);
        }
        _ => { eprintln!("unknown command"); std::process::exit(2); }
    }
}

// ---------------------------------------------------------------- boundary-functional invariant tests
/// Boundary families: X1 = points (zeta, xi) with zeta in mu_N and S_3(zeta, xi, t) = 0 (xi in F_{p^2});
/// X2 = the swapped points (xi, zeta).  Exponents are lifted to [0, N) in both coordinates.
fn boundary_families(inst: &Inst) -> (Vec<(F2, F2)>, Vec<(F2, F2)>) {
    let x1fam = boundary_points(inst);
    let x2fam: Vec<(F2, F2)> = x1fam.iter().map(|(a, b)| (*b, *a)).collect();
    (x1fam, x2fam)
}

/// Evaluate a dense column vector at each point (standard lift); returns the number of points with a
/// nonzero value.
fn count_nonzero_evals(inst: &Inst, lay: &Layout, v: &[u64], pts: &[(F2, F2)]) -> usize {
    let f = Fp { p: inst.p };
    let f2 = Fp2::new(f);
    let n = 1usize << inst.k;
    let nm = (n as u64) - 1;
    let support: Vec<(usize, usize, u64)> = v.iter().enumerate()
        .filter_map(|(c, &x)| { let x = x % f.p; if x != 0 { let m = lay.masks[c]; Some(((m & nm) as usize, (m >> inst.k) as usize, x)) } else { None } })
        .collect();
    if support.is_empty() { return 0; }
    let mut bad = 0;
    let mut p1 = vec![F2(0, 0); n];
    let mut p2 = vec![F2(0, 0); n];
    for (x1, x2) in pts {
        let mut z = f2.from(1);
        for e in 0..n { p1[e] = z; z = f2.mul(z, *x1); }
        let mut z = f2.from(1);
        for e in 0..n { p2[e] = z; z = f2.mul(z, *x2); }
        let mut acc = F2(0, 0);
        for &(a, b, c) in &support {
            acc = f2.add(acc, f2.scal(c, f2.mul(p1[a], p2[b])));
        }
        if !f2.is_zero(acc) { bad += 1; }
    }
    bad
}

/// H^{(s)}_j: extract the terms whose coordinate-s exponent has bits j..k-1 all set, and clear those bits.
fn extract_top(inst: &Inst, lay: &Layout, v: &[u64], s: u32, j: u32) -> Vec<u64> {
    let p = inst.p;
    let k = inst.k;
    let nm = (1u64 << k) - 1;
    let top = nm & !((1u64 << j) - 1); // bits j..k-1
    let mut out = vec![0u64; v.len()];
    for (c, &x) in v.iter().enumerate() {
        let x = x % p;
        if x == 0 { continue; }
        let m = lay.masks[c];
        let (a, b) = (m & nm, m >> k);
        let (a2, b2) = if s == 1 {
            if a & top != top { continue; }
            (a & !top, b)
        } else {
            if b & top != top { continue; }
            (a, b & !top)
        };
        let j2 = lay.idx[&(a2 | (b2 << k))] as usize;
        out[j2] = (out[j2] + x) % p;
    }
    out
}

fn random_combo(basis: &Basis, rows: &[usize], rng: &mut Rng) -> Vec<u64> {
    let p = basis.f.p;
    let mut acc = vec![0u64; basis.n];
    for &r in rows {
        let c = rng.below(p - 1) + 1;
        for (j, &x) in basis.rows[r].iter().enumerate() {
            let x = x % p;
            if x != 0 { acc[j] = (acc[j] + c * x) % p; }
        }
    }
    acc
}


fn bnd_detail(inst: &Inst, d: u32, j: u32, log: &mut dyn Write) {
    // print low rows whose coordinate-2 top extraction H_j fails on the X1 family, smallest support first
    let (_res, lay, basis) = closure(inst, d, false, false, log);
    let (x1fam, _x2fam) = boundary_families(inst);
    let p = inst.p;
    let nm = (1u64 << inst.k) - 1;
    let mut fails: Vec<(usize, usize)> = Vec::new();
    for r in 0..basis.rows.len() {
        if (lay.wt[basis.pivcol[r]] as u32) >= d { continue; }
        let h = extract_top(inst, &lay, &basis.rows[r], 2, j);
        if count_nonzero_evals(inst, &lay, &h, &x1fam) > 0 {
            let nnz = basis.rows[r].iter().filter(|&&x| x % p != 0).count();
            fails.push((nnz, r));
        }
    }
    fails.sort();
    println!("# failing low rows for H2_{}: {}", j, fails.len());
    for &(nnz, r) in fails.iter().take(3) {
        let row = &basis.rows[r];
        let mut terms: Vec<String> = Vec::new();
        for (c, &x) in row.iter().enumerate() {
            let x = x % p; if x == 0 { continue; }
            let m = lay.masks[c];
            terms.push(format!("({},{})", m & nm, m >> inst.k));
        }
        println!("row {} pivw {} nnz {} :: {}", r, lay.wt[basis.pivcol[r]], nnz, terms.join(" "));
    }
}


/// 2D DFT over Z/N x Z/N: F(i,j) = sum_{a,b} u[a][b] z^{a i + b j}; inverse uses z^{-1} and N^{-2}.
fn dft2(f: &Fp, n: usize, z: u64, u: &[u64], inverse: bool) -> Vec<u64> {
    let zz = if inverse { f.inv(z) } else { z };
    let mut pw = vec![1u64; n];
    for i in 1..n { pw[i] = f.mul(pw[i - 1], zz); }
    // transform along b (inner index), then along a
    let mut tmp = vec![0u64; n * n];
    for a in 0..n {
        for j in 0..n {
            let mut acc = 0u64;
            for b in 0..n { let x = u[a * n + b]; if x != 0 { acc = (acc + x * pw[(b * j) % n]) % f.p; } }
            tmp[a * n + j] = acc;
        }
    }
    let mut out = vec![0u64; n * n];
    for i in 0..n {
        for j in 0..n {
            let mut acc = 0u64;
            for a in 0..n { let x = tmp[a * n + j]; if x != 0 { acc = (acc + x * pw[(a * i) % n]) % f.p; } }
            out[i * n + j] = acc;
        }
    }
    if inverse {
        let ninv2 = f.inv(f.mul(n as u64 % f.p, n as u64 % f.p));
        for x in out.iter_mut() { *x = f.mul(*x, ninv2); }
    }
    out
}

/// cofactor support statistics: kappa(u) = g^{-1} u in Q_k for a random combination of the given rows
/// Smallest j < k with (a + 2^j) mod n in {n-2, n-1} (the shift by x^(2^j) lands on a top residue).
fn shift_hits_top(a: usize, n: usize, k: u32) -> Option<u32> {
    (0..k).find(|&j| { let t = (a + (1usize << j)) % n; t >= n - 2 })
}

fn cofactor_stats(inst: &Inst, lay: &Layout, basis: &Basis, rows: &[usize], gen: &[(u64, u64, u64)], rng: &mut Rng) -> (u32, u32, u32, bool, Option<(usize, usize, u32)>) {
    let f = Fp { p: inst.p };
    let n = 1usize << inst.k;
    let nm = (n as u64) - 1;
    let z = zeta_of_order(&f, inst.k);
    let mut gv = vec![0u64; n * n];
    for &(a, b, c) in gen { let i = ((a as usize) % n) * n + (b as usize) % n; gv[i] = (gv[i] + c) % f.p; }
    let gh = dft2(&f, n, z, &gv, false);
    let r = random_combo(basis, rows, rng);
    let mut uv = vec![0u64; n * n];
    for (c, &x) in r.iter().enumerate() { let x = x % f.p; if x != 0 { let m = lay.masks[c]; uv[((m & nm) as usize) * n + (m >> inst.k) as usize] = x; } }
    let uh = dft2(&f, n, z, &uv, false);
    let qh: Vec<u64> = uh.iter().zip(gh.iter()).map(|(&a, &b)| { assert!(b != 0, "g vanishes on the grid"); f.mul(a, f.inv(b)) }).collect();
    let kap = dft2(&f, n, z, &qh, true);
    let (mut m1, mut m2, mut mt) = (0u32, 0u32, 0u32);
    let mut touches = false;
    let mut shift_hit: Option<(usize, usize, u32)> = None;
    for a in 0..n { for b in 0..n {
        if kap[a * n + b] == 0 { continue; }
        if shift_hit.is_none() { if let Some(j) = shift_hits_top(a, n, inst.k).or(shift_hits_top(b, n, inst.k)) { shift_hit = Some((a, b, j)); } }
        let (pa, pb) = ((a as u64).count_ones(), (b as u64).count_ones());
        m1 = m1.max(pa); m2 = m2.max(pb); mt = mt.max(pa + pb);
        if a >= n - 2 || b >= n - 2 { touches = true; }
    } }
    (m1, m2, mt, touches, shift_hit)
}


/// Low-weight multiples in the polynomial ring F[x1,x2] (integer exponents, binary weight = popcount sum):
/// K = { h : deg_{x_i} h < X, every monomial of S*h has weight <= w }.  Returns (dim K, max popcount of x1
/// exponents, of x2 exponents, max total, max degree) over the support of a random element of K.
fn low_weight_multiples(p: u64, xbox: usize, w: u32, gen: &[(u64, u64, u64)], seed: u64) -> (usize, u32, u32, u32, usize, Vec<(usize, usize)>) {
    let f = Fp { p };
    let gx = gen.iter().map(|t| t.0 as usize).max().unwrap();
    let gy = gen.iter().map(|t| t.1 as usize).max().unwrap();
    let ox = xbox + gx; let oy = xbox + gy;
    let nunk = xbox * xbox;
    // constraint rows: one per high-weight output monomial (A,B), as sparse lists over unknowns
    let mut rows: Vec<Vec<(usize, u64)>> = Vec::new();
    for aa in 0..ox { for bb in 0..oy {
        if (aa as u64).count_ones() + (bb as u64).count_ones() <= w { continue; }
        let mut r = Vec::new();
        for &(ga, gb, c) in gen {
            let (ga, gb) = (ga as usize, gb as usize);
            if aa >= ga && bb >= gb && aa - ga < xbox && bb - gb < xbox { r.push(((aa - ga) * xbox + (bb - gb), c)); }
        }
        if !r.is_empty() { rows.push(r); }
    } }
    // dense Gaussian elimination to RREF over unknowns
    let mut piv_of_col: Vec<Option<usize>> = vec![None; nunk];
    let mut basis: Vec<Vec<u64>> = Vec::new();
    let mut pivcols: Vec<usize> = Vec::new();
    for r in rows {
        let mut v = vec![0u64; nunk];
        for (c, x) in r { v[c] = (v[c] + x) % p; }
        // reduce
        for col in 0..nunk {
            let x = v[col] % p;
            if x == 0 { v[col] = 0; continue; }
            if let Some(bi) = piv_of_col[col] {
                let b = &basis[bi]; let cc = p - x;
                for j in col..nunk { if b[j] != 0 { v[j] = (v[j] + cc * b[j]) % p; } }
            } else {
                let inv = f.inv(x);
                for j in col..nunk { v[j] = v[j] * inv % p; }
                piv_of_col[col] = Some(basis.len()); pivcols.push(col); basis.push(v); break;
            }
        }
    }
    // back-substitute to RREF: pivot columns in decreasing order, eliminated from every other row
    let nb = basis.len();
    let mut order: Vec<usize> = (0..nb).collect();
    order.sort_by(|&x, &y| pivcols[y].cmp(&pivcols[x]));
    for &i in &order {
        let pc = pivcols[i];
        let bi = basis[i].clone();
        for i2 in 0..nb {
            if i2 == i { continue; }
            let x = basis[i2][pc];
            if x != 0 { let cc = p - x; for j in pc..nunk { if bi[j] != 0 { basis[i2][j] = (basis[i2][j] + cc * bi[j]) % p; } } }
        }
    }
    // self-check: every RREF row has zeros at all other pivot columns
    for i in 0..nb { for i2 in 0..nb { if i != i2 { assert!(basis[i][pivcols[i2]] == 0, "RREF check failed"); } } }
    if let Ok(spec) = std::env::var("LWM_SHOW") {
        let v: Vec<usize> = spec.split(',').map(|x| x.parse().unwrap()).collect();
        let fc = v[0] * xbox + v[1];
        if piv_of_col[fc].is_none() {
            let mut hv = vec![0u64; nunk]; hv[fc] = 1;
            for i in 0..nb { let x = basis[i][fc]; if x != 0 { hv[pivcols[i]] = (p - x) % p; } }
            let hs: Vec<String> = (0..nunk).filter(|&c| hv[c] != 0).map(|c| format!("({},{})", c / xbox, c % xbox)).collect();
            // product S*h
            let mut prod = std::collections::BTreeMap::new();
            for c in 0..nunk { if hv[c] == 0 { continue; } for &(ga, gb, gc) in gen { let key = (c / xbox + ga as usize, c % xbox + gb as usize); let e = prod.entry(key).or_insert(0u64); *e = (*e + gc * hv[c]) % p; } }
            let ps: Vec<String> = prod.iter().filter(|(_, &x)| x != 0).map(|((a, b), _)| format!("({},{})", a, b)).collect();
            eprintln!("h support ({} terms): {}", hs.len(), hs.join(" "));
            eprintln!("S*h support ({} terms): {}", ps.len(), ps.join(" "));
        } else { eprintln!("column ({},{}) is a pivot (not free)", v[0], v[1]); }
    }
    if let Ok(spec) = std::env::var("LWM_TOUCH") {
        // smallest RREF kernel basis vector whose support has an x2-exponent with popcount >= spec
        let thr: u32 = spec.parse().unwrap();
        let mut best: Option<(usize, Vec<u64>)> = None;
        for fc in 0..nunk { if piv_of_col[fc].is_some() { continue; }
            let mut hv = vec![0u64; nunk]; hv[fc] = 1;
            for i in 0..nb { let x = basis[i][fc]; if x != 0 { hv[pivcols[i]] = (p - x) % p; } }
            let touch = (0..nunk).any(|c| hv[c] != 0 && ((c % xbox) as u64).count_ones() >= thr);
            if touch { let nz = hv.iter().filter(|&&x| x != 0).count(); if best.as_ref().map_or(true, |(b, _)| nz < *b) { best = Some((nz, hv)); } }
        }
        if let Some((nz, hv)) = best {
            let hs: Vec<String> = (0..nunk).filter(|&c| hv[c] != 0).map(|c| format!("({},{})", c / xbox, c % xbox)).collect();
            let mut prod = std::collections::BTreeMap::new();
            for c in 0..nunk { if hv[c] == 0 { continue; } for &(ga, gb, gc) in gen { let key = (c / xbox + ga as usize, c % xbox + gb as usize); let e = prod.entry(key).or_insert(0u64); *e = (*e + gc * hv[c]) % p; } }
            let ps: Vec<String> = prod.iter().filter(|(_, &x)| x != 0).map(|((a, b), _)| format!("({},{})", a, b)).collect();
            eprintln!("h ({} terms): {}", nz, hs.join(" "));
            eprintln!("S*h ({} terms): {}", ps.len(), ps.join(" "));
        } else { eprintln!("no basis vector touches popcount {}", thr); }
    }
    // random kernel element: free variables random, pivots solved
    let mut rng = Rng(seed);
    let mut h = vec![0u64; nunk];
    for col in 0..nunk { if piv_of_col[col].is_none() { h[col] = rng.below(p - 1) + 1; } }
    for i in 0..nb {
        let pc = pivcols[i]; let mut acc = 0u64;
        for j in (pc + 1)..nunk { if basis[i][j] != 0 && piv_of_col[j].is_none() { acc = (acc + basis[i][j] * h[j]) % p; } }
        h[pc] = (p - acc) % p;
    }
    {
        let mut prod = std::collections::HashMap::new();
        for c in 0..nunk { if h[c] == 0 { continue; } for &(ga, gb, gc) in gen { let key = (c / xbox + ga as usize, c % xbox + gb as usize); let e = prod.entry(key).or_insert(0u64); *e = (*e + gc * h[c]) % p; } }
        for ((a, b), x) in prod { if x != 0 { assert!((a as u64).count_ones() + (b as u64).count_ones() <= w, "kernel element violates the weight bound"); } }
    }
    let (mut m1, mut m2, mut mt, mut md) = (0u32, 0u32, 0u32, 0usize);
    let mut sup = Vec::new();
    for a in 0..xbox { for b in 0..xbox { if h[a * xbox + b] != 0 {
        let (pa, pb) = ((a as u64).count_ones(), (b as u64).count_ones());
        m1 = m1.max(pa); m2 = m2.max(pb); mt = mt.max(pa + pb); md = md.max(a.max(b)); sup.push((a, b));
    } } }
    (nunk - nb, m1, m2, mt, md, sup)
}


/// Low-weight multiples via evaluation: unknowns f_{AB} on L = {(A,B) in [0,X+2)^2 : pop(A)+pop(B) <= w};
/// f must vanish at >= 4X+50 points of S = 0 with x1, x2 in F_p.  Then h = f / S by exact division.
/// Returns (kernel dim, max pop x1, max pop x2, max total, max deg of h, |L|).
fn lwm_eval(p: u64, xbox: usize, w: u32, gen: &[(u64, u64, u64)], seed: u64) -> (usize, u32, u32, u32, usize, usize) {
    let f = Fp { p };
    let lim = xbox + 2;
    let mut lmon: Vec<(usize, usize)> = Vec::new();
    for a in 0..lim { for b in 0..lim { if (a as u64).count_ones() + (b as u64).count_ones() <= w { lmon.push((a, b)); } } }
    let nunk = lmon.len();
    // generator as polynomial in x2 with F[x1] coefficients (degree <= 2 in each)
    let mut s = vec![vec![0u64; 3]; 3]; // s[b][a]
    for &(a, b, c) in gen { s[b as usize][a as usize] = (s[b as usize][a as usize] + c) % p; }
    // points: x1 random in F_p, x2 root of the quadratic in x2 when it lies in F_p
    let npts = 4 * xbox + 200;
    let mut pts: Vec<(u64, u64)> = Vec::new();
    let mut rng = Rng(seed);
    let mut used = std::collections::HashSet::new();
    while pts.len() < npts {
        let x1 = rng.below(p);
        if !used.insert(x1) { continue; } // distinct abscissae, hence distinct points
        let ev = |row: &Vec<u64>| -> u64 { f.add(f.add(row[0], f.mul(row[1], x1)), f.mul(row[2], f.mul(x1, x1))) };
        let (c0, c1, c2) = (ev(&s[0]), ev(&s[1]), ev(&s[2]));
        if s[2].iter().all(|&x| x == 0) {
            // generator linear in x2: x2 = -c0 / c1
            if c1 == 0 { continue; }
            pts.push((x1, f.mul(f.neg(c0), f.inv(c1))));
            continue;
        }
        if c2 == 0 { continue; }
        let disc = f.sub(f.mul(c1, c1), f.mul(4, f.mul(c2, c0)));
        if let Some(r) = f.sqrt(disc) {
            let inv = f.inv(f.mul(2, c2));
            for x2 in [f.mul(f.add(f.neg(c1), r), inv), f.mul(f.sub(f.neg(c1), r), inv)] { pts.push((x1, x2)); }
        }
    }
    // evaluation matrix rows = points, columns = unknowns; RREF via column pivots
    let mut piv_of_col: Vec<Option<usize>> = vec![None; nunk];
    let mut rows: Vec<Vec<u64>> = Vec::new(); let mut pivcols: Vec<usize> = Vec::new();
    for &(x1, x2) in &pts {
        let mut pw1 = vec![1u64; lim]; for i in 1..lim { pw1[i] = f.mul(pw1[i - 1], x1); }
        let mut pw2 = vec![1u64; lim]; for i in 1..lim { pw2[i] = f.mul(pw2[i - 1], x2); }
        let mut v: Vec<u64> = lmon.iter().map(|&(a, b)| f.mul(pw1[a], pw2[b])).collect();
        for col in 0..nunk {
            let x = v[col] % p; if x == 0 { v[col] = 0; continue; }
            if let Some(bi) = piv_of_col[col] { let b = &rows[bi]; let cc = p - x; for j in col..nunk { if b[j] != 0 { v[j] = (v[j] + cc * b[j]) % p; } } }
            else { let inv = f.inv(x); for j in col..nunk { v[j] = v[j] * inv % p; } piv_of_col[col] = Some(rows.len()); pivcols.push(col); rows.push(v); break; }
        }
    }
    let nb = rows.len();
    let mut order: Vec<usize> = (0..nb).collect(); order.sort_by(|&x, &y| pivcols[y].cmp(&pivcols[x]));
    for &i in &order { let pc = pivcols[i]; let bi = rows[i].clone(); for i2 in 0..nb { if i2 == i { continue; } let x = rows[i2][pc]; if x != 0 { let cc = p - x; for j in pc..nunk { if bi[j] != 0 { rows[i2][j] = (rows[i2][j] + cc * bi[j]) % p; } } } } }
    // random kernel element
    let mut fv = vec![0u64; nunk];
    for col in 0..nunk { if piv_of_col[col].is_none() { fv[col] = rng.below(p - 1) + 1; } }
    for i in 0..nb { let pc = pivcols[i]; let mut acc = 0u64; for j in (pc + 1)..nunk { if rows[i][j] != 0 && piv_of_col[j].is_none() { acc = (acc + rows[i][j] * fv[j]) % p; } } fv[pc] = (p - acc) % p; }
    // f as poly in x2 with F[x1] coefficients
    let mut fp = vec![vec![0u64; lim]; lim]; // fp[b][a]
    for (c, &(a, b)) in lmon.iter().enumerate() { fp[b][a] = fv[c]; }
    // exact division by S in F[x1][x2]
    let mut h = vec![vec![0u64; lim]; lim];
    let dy = if s[2].iter().any(|&x| x != 0) { 2 } else { 1 };
    let s2 = &s[dy];
    // divide polynomial (in x1) by s2 exactly
    let divx1 = |num: &Vec<u64>| -> Vec<u64> {
        let mut r = num.clone(); let mut q = vec![0u64; num.len()];
        let ds = (0..3).rev().find(|&i| s2[i] != 0).unwrap(); let linv = f.inv(s2[ds]);
        for d in (ds..r.len()).rev() { let c = r[d]; if c == 0 { continue; } let qc = f.mul(c, linv); q[d - ds] = qc; for i in 0..=ds { r[d - ds + i] = f.sub(r[d - ds + i], f.mul(qc, s2[i])); } }
        assert!(r.iter().all(|&x| x == 0), "non-exact division in x1"); q
    };
    for d in (dy..lim).rev() {
        if fp[d].iter().all(|&x| x == 0) { continue; }
        let q = divx1(&fp[d]);
        // subtract q * S * x2^{d-dy}
        for bb in 0..=dy { for aa in 0..3 { if s[bb][aa] == 0 { continue; } for (i, &qc) in q.iter().enumerate() { if qc == 0 { continue; } let ai = i + aa; if ai < lim { fp[d - dy + bb][ai] = f.sub(fp[d - dy + bb][ai], f.mul(qc, s[bb][aa])); } } } }
        h[d - dy] = q;
    }
    assert!(fp.iter().all(|r| r.iter().all(|&x| x == 0)), "f is not divisible by S");
    if std::env::var("LWM2_SHOW").is_ok() {
        // smallest RREF kernel vector (over f-unknowns) whose cofactor contains a monomial of max x2-popcount
        let mut fsup: Vec<String> = Vec::new();
        for (c, &(a, b)) in lmon.iter().enumerate() { if fv[c] != 0 { fsup.push(format!("({},{})", a, b)); } }
        let mut hist = std::collections::BTreeMap::new();
        for b in 0..lim { for a in 0..lim { if h[b][a] != 0 { *hist.entry(((a as u64).count_ones(), (b as u64).count_ones())).or_insert(0usize) += 1; } } }
        eprintln!("random kernel element: f has {} terms; h popcount histogram (pop1,pop2)->count: {:?}", fsup.len(), hist);
        let mx2 = (0..lim).filter(|&b| h[b].iter().any(|&x| x != 0)).map(|b| (b as u64).count_ones()).max().unwrap_or(0);
        let cols: Vec<usize> = (0..lim).filter(|&b| (b as u64).count_ones() == mx2 && h[b].iter().any(|&x| x != 0)).collect();
        eprintln!("x2-exponents of h with popcount {}: {:?}", mx2, cols);
        for &b in cols.iter().take(2) { let a_s: Vec<usize> = (0..lim).filter(|&a| h[b][a] != 0).collect(); eprintln!("  column x2^{}: x1-exponents {:?}", b, &a_s[..a_s.len().min(40)]); }
    }
    let (mut m1, mut m2, mut mt, mut md) = (0u32, 0u32, 0u32, 0usize);
    // when X + 2 = N = 2^k, h is a cofactor in Q_k off the top residues; record the first exponent whose
    // shift by some x_s^(2^j) lands on a top residue, and how many support points do so
    let kq = if lim.is_power_of_two() { Some(lim.trailing_zeros()) } else { None };
    let (mut nhit, mut first): (usize, Option<(usize, usize, u32)>) = (0, None);
    for b in 0..lim { for a in 0..lim { if h[b][a] != 0 {
        let (pa, pb) = ((a as u64).count_ones(), (b as u64).count_ones()); m1 = m1.max(pa); m2 = m2.max(pb); mt = mt.max(pa + pb); md = md.max(a.max(b));
        if let Some(kk) = kq { if let Some(j) = shift_hits_top(a, lim, kk).or(shift_hits_top(b, lim, kk)) { nhit += 1; if first.is_none() { first = Some((a, b, j)); } } }
    } } }
    if let Some(kk) = kq {
        println!("{{\"k\":{},\"shift_test\":true,\"support_points_hitting_top\":{},\"first\":\"{}\"}}", kk, nhit,
                 first.map_or("none".to_string(), |(a, b, j)| format!("({},{}) j={}", a, b, j)));
    }
    (nunk - nb, m1, m2, mt, md, nunk)
}

fn bnd_test(inst: &Inst, d: u32, seed: u64, log: &mut dyn Write) {
    let (res, lay, basis) = closure(inst, d, false, false, log);
    let (x1fam, x2fam) = boundary_families(inst);
    let mut rng = Rng(seed);
    let all: Vec<usize> = (0..basis.rows.len()).collect();
    let low: Vec<usize> = (0..basis.rows.len()).filter(|&r| (lay.wt[basis.pivcol[r]] as u32) < d).collect();
    let mut out = String::new();
    for trial in 0..2 {
        let r_all = random_combo(&basis, &all, &mut rng);
        let b1 = count_nonzero_evals(inst, &lay, &r_all, &x1fam);
        let b2 = count_nonzero_evals(inst, &lay, &r_all, &x2fam);
        let r_low = random_combo(&basis, &low, &mut rng);
        let mut hs = Vec::new();
        for j in 0..inst.k {
            let h2 = extract_top(inst, &lay, &r_low, 2, j);
            let h1 = extract_top(inst, &lay, &r_low, 1, j);
            let f_h2 = count_nonzero_evals(inst, &lay, &h2, &x1fam);
            let f_h1 = count_nonzero_evals(inst, &lay, &h1, &x2fam);
            if f_h2 > 0 || f_h1 > 0 { hs.push(format!("{}:{}/{}", j, f_h2, f_h1)); }
        }
        out.push_str(&format!("{{\"trial\":{},\"all_x1\":{},\"all_x2\":{},\"points_per_family\":{},\"H_fail_j:x1fam/x2fam\":\"{}\"}}", trial, b1, b2, x1fam.len(), hs.join(" ")));
        if trial == 0 { out.push(','); }
    }
    println!("{{\"p\":{},\"k\":{},\"A\":{},\"B\":{},\"t\":{},\"D\":{},\"rank\":{},\"columns\":{},\"one_in_V\":{},\"low_rows\":{},\"tests\":[{}]}}",
        inst.p, inst.k, inst.a, inst.b, inst.t, d, res.rank, res.columns, res.one_in_v, low.len(), out);
}
