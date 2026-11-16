//! F1 (ECC2K-130 follow-ups 2026-11-16): cold SAT solve with the target R folded in as
//! constants, seeded with warm heuristic state (saved phases, optionally target-free learnt
//! clauses) carried from previous targets.  Lines up to `new_solver` are copied verbatim from
//! research/ecc2k130_open7_20261114/L3-incremental-sat/tool/src/main.rs (instance generator,
//! field, curve, CNF builder, ground truth, learner hook); everything after is F1 code.
//! See ../PROTOCOL.md.
//!
//! Arms: A0 cold folded (current practice; its final saved phases are read, read-only);
//! WS positive control (same target re-solved cold, seeded with A0's final saved phases);
//! AA = A0 with shuffled clauses (A/A); WP cold folded, saved phases seeded from the previous
//! target's final saved phases (primary); WF = WP via CaDiCaL's forced `phase()` API;
//! WR negative control (random saved phases on the same variables WP would seed);
//! WT = WP with an LRAT proof tracer attached (tracer control); WL = WT plus the carried
//! target-free learnt clauses; B persistent solver with R as assumptions (L3 reference).

use cadical_sys::{CaDiCal, Learner, Status};
use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt::Write as _;
use std::io::{Read, Seek, SeekFrom, Write};
use std::rc::Rc;
use std::time::Instant;

// ---------------------------------------------------------------- rng
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}
fn seed_of(s: &str) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in s.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

// ---------------------------------------------------------------- field: type-II ONB
// element: m-bit u64, bit (i-1) = coefficient of gamma_i = g^i + g^-i, g a primitive n-th root, n=2m+1
#[derive(Clone)]
struct Onb {
    m: usize,
    terms: Vec<Vec<(usize, usize)>>, // output k (0-based) <- xor of a_i b_j, 0-based
    sqperm: Vec<usize>,              // coord i lands at sqperm[i] under squaring
}
fn is_prime(n: usize) -> bool {
    n >= 2 && (2..n).take_while(|d| d * d <= n).all(|d| n % d != 0)
}
fn ord2(n: usize) -> usize {
    let mut x = 2 % n;
    let mut k = 1;
    while x != 1 {
        x = x * 2 % n;
        k += 1;
    }
    k
}
impl Onb {
    fn new(m: usize) -> Onb {
        let n = 2 * m + 1;
        assert!(is_prime(n), "2m+1 not prime");
        let o = ord2(n);
        assert!(o == m || o == 2 * m, "no type-II ONB");
        let fold = |e: i64| -> usize {
            let e = e.rem_euclid(n as i64) as usize;
            if e <= m { e } else { n - e }
        };
        let mut terms = vec![Vec::new(); m];
        for i in 1..=m {
            for j in 1..=m {
                // gamma_i gamma_j = gamma_{fold(i+j)} + gamma_{fold(i-j)}; fold 0 means 2*g^0 = 0
                let a = fold(i as i64 + j as i64);
                let b = fold(i as i64 - j as i64);
                if a != 0 {
                    terms[a - 1].push((i - 1, j - 1));
                }
                if b != 0 {
                    terms[b - 1].push((i - 1, j - 1));
                }
            }
        }
        let sqperm = (1..=m).map(|i| fold(2 * i as i64) - 1).collect();
        Onb { m, terms, sqperm }
    }
    fn one(&self) -> u64 {
        (1u64 << self.m) - 1
    }
    /// fast multiply: symmetric representation in Z2[x]/(x^n - 1), n = 2m+1 <= 127
    fn mul(&self, a: u64, b: u64) -> u64 {
        let n = 2 * self.m + 1;
        let sym = |a: u64| -> u128 {
            let mut u = 0u128;
            for i in 1..=self.m {
                if (a >> (i - 1)) & 1 == 1 {
                    u |= (1u128 << i) | (1u128 << (n - i));
                }
            }
            u
        };
        let (sa, sb) = (sym(a), sym(b));
        let full: u128 = if n == 128 { !0 } else { (1u128 << n) - 1 };
        let mut c = 0u128;
        let mut x = sa;
        while x != 0 {
            let i = x.trailing_zeros() as usize;
            x &= x - 1;
            let rot = if i == 0 { sb } else { ((sb << i) | (sb >> (n - i))) & full };
            c ^= rot;
        }
        let mut out = 0u64;
        for i in 1..=self.m {
            if (c >> i) & 1 == 1 {
                out |= 1 << (i - 1);
            }
        }
        out
    }
    fn mul_terms(&self, a: u64, b: u64) -> u64 {
        let mut c = 0u64;
        for (k, ts) in self.terms.iter().enumerate() {
            let mut bit = 0u64;
            for &(i, j) in ts {
                bit ^= (a >> i) & (b >> j) & 1;
            }
            c |= bit << k;
        }
        c
    }
    fn sqr(&self, a: u64) -> u64 {
        let mut c = 0;
        for i in 0..self.m {
            if (a >> i) & 1 == 1 {
                c |= 1 << self.sqperm[i];
            }
        }
        c
    }
    fn pow2k(&self, mut a: u64, k: usize) -> u64 {
        for _ in 0..k {
            a = self.sqr(a);
        }
        a
    }
    fn inv(&self, a: u64) -> u64 {
        assert!(a != 0);
        // a^(2^m - 2) = (a^(2^(m-1) - 1))^2
        let mut r = self.one();
        let mut b = a;
        for _ in 1..self.m {
            b = self.sqr(b);
            r = self.mul(r, b);
        }
        r
    }
    fn trace(&self, a: u64) -> u64 {
        (a.count_ones() & 1) as u64
    }
    fn half_trace(&self, c: u64) -> u64 {
        let mut z = 0;
        let mut t = c;
        for _ in 0..=(self.m - 1) / 2 {
            z ^= t;
            t = self.sqr(self.sqr(t));
        }
        z
    }
    fn sqrt(&self, a: u64) -> u64 {
        self.pow2k(a, self.m - 1)
    }
    fn s3(&self, u: u64, v: u64, w: u64) -> u64 {
        let s = self.mul(u, v) ^ self.mul(u, w) ^ self.mul(v, w);
        self.sqr(s) ^ self.mul(self.mul(u, v), w) ^ self.one()
    }
    /// all w with S3(u,v,w) = 0: (u+v)^2 w^2 + uv w + (uv)^2 + 1 = 0
    fn s3_roots(&self, u: u64, v: u64) -> Vec<u64> {
        let a = self.sqr(u ^ v);
        let b = self.mul(u, v);
        let c = self.sqr(b) ^ self.one();
        if a == 0 {
            if b == 0 {
                return vec![]; // c = 1 != 0
            }
            return vec![self.mul(c, self.inv(b))];
        }
        if b == 0 {
            return vec![self.sqrt(self.mul(c, self.inv(a)))];
        }
        let alpha = self.mul(b, self.inv(a));
        let beta = self.mul(self.mul(c, self.inv(a)), self.inv(self.sqr(alpha)));
        if self.trace(beta) == 1 {
            return vec![];
        }
        let z = self.half_trace(beta);
        let mut r = vec![self.mul(alpha, z), self.mul(alpha, z ^ self.one())];
        r.sort();
        r.dedup();
        r
    }
}

// ---------------------------------------------------------------- curve y^2+xy=x^3+1
type Pt = Option<(u64, u64)>;
struct Curve {
    f: Onb,
}
impl Curve {
    fn add(&self, p: Pt, q: Pt) -> Pt {
        let f = &self.f;
        let (x1, y1) = match p { None => return q, Some(v) => v };
        let (x2, y2) = match q { None => return p, Some(v) => v };
        if x1 == x2 {
            if y1 == y2 {
                if x1 == 0 {
                    return None;
                }
                let l = x1 ^ f.mul(y1, f.inv(x1));
                let x3 = f.sqr(l) ^ l;
                let y3 = f.sqr(x1) ^ f.mul(l ^ f.one(), x3);
                return Some((x3, y3));
            }
            return None; // q = -p (y2 = x1 + y1)
        }
        let l = f.mul(y1 ^ y2, f.inv(x1 ^ x2));
        let x3 = f.sqr(l) ^ l ^ x1 ^ x2;
        let y3 = f.mul(l, x1 ^ x3) ^ x3 ^ y1;
        Some((x3, y3))
    }
    fn neg(&self, p: Pt) -> Pt {
        p.map(|(x, y)| (x, x ^ y))
    }
    fn on_curve(&self, p: Pt) -> bool {
        let f = &self.f;
        match p {
            None => true,
            Some((x, y)) => f.sqr(y) ^ f.mul(x, y) == f.mul(f.sqr(x), x) ^ f.one(),
        }
    }
    fn from_x(&self, x: u64) -> Pt {
        let f = &self.f;
        if x == 0 {
            return None;
        }
        let c = x ^ f.inv(f.sqr(x));
        if f.trace(c) == 1 {
            return None;
        }
        let z = f.half_trace(c);
        let p = Some((x, f.mul(x, z)));
        assert!(self.on_curve(p));
        p
    }
    fn order_formula(m: usize) -> i128 {
        let (mut v0, mut v1): (i128, i128) = (2, -1);
        for _ in 0..m - 1 {
            let t = -v1 - 2 * v0;
            v0 = v1;
            v1 = t;
        }
        (1i128 << m) + 1 - v1
    }
}

// ---------------------------------------------------------------- CNF builder
struct Cnf {
    nv: i32,
    cls: Vec<Vec<i32>>,
}
const T: i32 = 1;
const F: i32 = -1;
impl Cnf {
    fn new() -> Cnf {
        Cnf { nv: 1, cls: vec![vec![1]] }
    }
    fn var(&mut self) -> i32 {
        self.nv += 1;
        self.nv
    }
    fn xor(&mut self, a: i32, b: i32) -> i32 {
        if a == F { return b; }
        if b == F { return a; }
        if a == T { return -b; }
        if b == T { return -a; }
        if a == b { return F; }
        if a == -b { return T; }
        let z = self.var();
        // z = a xor b
        self.cls.push(vec![-z, a, b]);
        self.cls.push(vec![-z, -a, -b]);
        self.cls.push(vec![z, -a, b]);
        self.cls.push(vec![z, a, -b]);
        z
    }
    fn and(&mut self, a: i32, b: i32) -> i32 {
        if a == F || b == F { return F; }
        if a == T { return b; }
        if b == T { return a; }
        if a == b { return a; }
        if a == -b { return F; }
        let z = self.var();
        self.cls.push(vec![-z, a]);
        self.cls.push(vec![-z, b]);
        self.cls.push(vec![z, -a, -b]);
        z
    }
    fn xor_list(&mut self, mut v: Vec<i32>) -> i32 {
        // balanced tree, as ir.xorList
        if v.is_empty() {
            return F;
        }
        while v.len() > 1 {
            let mut nx = Vec::new();
            let mut i = 0;
            while i + 1 < v.len() {
                nx.push(self.xor(v[i], v[i + 1]));
                i += 2;
            }
            if i < v.len() {
                nx.push(v[i]);
            }
            v = nx;
        }
        v[0]
    }
    fn assert_zero(&mut self, l: i32) {
        assert!(l != T, "unsat by construction");
        if l != F {
            self.cls.push(vec![-l]);
        }
    }
    fn at_most(&mut self, lits: &[i32], k: usize) {
        // Sinz sequential counter, as cnf.py atMost
        let n = lits.len();
        if k >= n {
            return;
        }
        if k == 0 {
            for &l in lits {
                self.cls.push(vec![-l]);
            }
            return;
        }
        let mut s: Vec<Vec<i32>> = Vec::new();
        for _ in 0..n - 1 {
            let row = (0..k).map(|_| self.var()).collect();
            s.push(row);
        }
        self.cls.push(vec![-lits[0], s[0][0]]);
        for j in 1..k {
            self.cls.push(vec![-s[0][j]]);
        }
        for i in 1..n - 1 {
            self.cls.push(vec![-lits[i], s[i][0]]);
            self.cls.push(vec![-s[i - 1][0], s[i][0]]);
            for j in 1..k {
                self.cls.push(vec![-lits[i], -s[i - 1][j - 1], s[i][j]]);
                self.cls.push(vec![-s[i - 1][j], s[i][j]]);
            }
            self.cls.push(vec![-lits[i], -s[i - 1][k - 1]]);
        }
        self.cls.push(vec![-lits[n - 1], -s[n - 2][k - 1]]);
    }
    fn lex_leq(&mut self, a: &[i32], b: &[i32]) {
        // as cnf.py lexLeq: index 0 most significant
        let mut eq = T;
        for i in 0..a.len() {
            if eq != F {
                let mut cl = vec![-a[i], b[i]];
                if eq != T {
                    cl.push(-eq);
                }
                self.cls.push(cl);
            }
            let x = self.xor(a[i], b[i]);
            eq = self.and(eq, -x);
        }
    }
    fn onb_mul(&mut self, f: &Onb, a: &[i32], b: &[i32]) -> Vec<i32> {
        let m = f.m;
        let mut andc: HashMap<(usize, usize), i32> = HashMap::new();
        let mut out = Vec::with_capacity(m);
        for k in 0..m {
            let mut ts = Vec::new();
            for &(i, j) in &f.terms[k] {
                let key = (i, j);
                let l = match andc.get(&key) {
                    Some(&l) => l,
                    None => {
                        let l = self.and(a[i], b[j]);
                        andc.insert(key, l);
                        l
                    }
                };
                ts.push(l);
            }
            out.push(self.xor_list(ts));
        }
        out
    }
    fn s3(&mut self, f: &Onb, u: &[i32], v: &[i32], w: &[i32]) -> Vec<i32> {
        let m = f.m;
        let uv = self.onb_mul(f, u, v);
        let uw = self.onb_mul(f, u, w);
        let vw = self.onb_mul(f, v, w);
        let s: Vec<i32> = (0..m).map(|i| { let t = self.xor(uv[i], uw[i]); self.xor(t, vw[i]) }).collect();
        let mut sq = vec![F; m];
        for i in 0..m {
            sq[f.sqperm[i]] = s[i];
        }
        let uvw = self.onb_mul(f, &uv, w);
        (0..m).map(|i| { let t = self.xor(sq[i], uvw[i]); self.xor(t, T) }).collect()
    }
}

struct System {
    cnf: Cnf,
    pvars: Vec<Vec<i32>>,
    tvars: Vec<Vec<i32>>,
    rvars: Vec<i32>, // empty when R is constant
}
fn build_system(f: &Onb, k: usize, w: usize, rconst: Option<u64>) -> System {
    let m = f.m;
    let mut c = Cnf::new();
    let mut pvars = Vec::new();
    for _ in 0..k {
        let v: Vec<i32> = (0..m).map(|_| c.var()).collect();
        c.at_most(&v, w);
        pvars.push(v);
    }
    let mut tvars = Vec::new();
    for _ in 0..k.saturating_sub(2) {
        tvars.push((0..m).map(|_| c.var()).collect::<Vec<i32>>());
    }
    let (r, rvars): (Vec<i32>, Vec<i32>) = match rconst {
        Some(b) => ((0..m).map(|j| if (b >> j) & 1 == 1 { T } else { F }).collect(), vec![]),
        None => {
            let v: Vec<i32> = (0..m).map(|_| c.var()).collect();
            (v.clone(), v)
        }
    };
    let mut chain: Vec<Vec<i32>> = tvars.clone();
    chain.push(r);
    let mut left = pvars[0].clone();
    for i in 0..k - 1 {
        let out = chain[i].clone();
        let roots = c.s3(f, &left, &pvars[i + 1], &out);
        for l in roots {
            c.assert_zero(l);
        }
        left = out;
    }
    for i in 0..k - 1 {
        let (a, b) = (pvars[i].clone(), pvars[i + 1].clone());
        c.lex_leq(&a, &b);
    }
    System { cnf: c, pvars, tvars, rvars }
}

// ---------------------------------------------------------------- ground truth
fn lex_key(c: u64, m: usize) -> u64 {
    // lexLeq compares bit 0 first (most significant)
    let mut r = 0;
    for j in 0..m {
        r = (r << 1) | ((c >> j) & 1);
    }
    r
}
fn low_weight(m: usize, w: usize) -> Vec<u64> {
    let mut v: Vec<u64> = (0..(1u64 << m)).filter(|x| x.count_ones() as usize <= w).collect();
    if m > 24 {
        unreachable!()
    }
    v.sort_by_key(|&c| lex_key(c, m));
    v
}
fn low_weight_big(m: usize, w: usize) -> Vec<u64> {
    let mut out = vec![0u64];
    let mut cur: Vec<Vec<usize>> = vec![vec![]];
    for _ in 0..w {
        let mut nx = Vec::new();
        for s in &cur {
            let st = s.last().map(|&x| x + 1).unwrap_or(0);
            for i in st..m {
                let mut t = s.clone();
                t.push(i);
                nx.push(t);
            }
        }
        for s in &nx {
            out.push(s.iter().fold(0u64, |a, &i| a | (1 << i)));
        }
        cur = nx;
    }
    out.sort_by_key(|&c| lex_key(c, m));
    out
}
/// target x -> set of projected models, for every target in `targets`
fn ground_truth(f: &Onb, k: usize, w: usize, targets: &HashSet<u64>) -> HashMap<u64, BTreeSet<Vec<u64>>> {
    let m = f.m;
    let wl = if m <= 24 { low_weight(m, w) } else { low_weight_big(m, w) };
    let mut out: HashMap<u64, BTreeSet<Vec<u64>>> = HashMap::new();
    for &t in targets {
        out.insert(t, BTreeSet::new());
    }
    if k == 2 {
        // per target: S3 is symmetric, so for each u in the base the partners v are the roots of S3(u, ., r)
        let inw: HashSet<u64> = wl.iter().cloned().collect();
        for &r in targets {
            let s = out.get_mut(&r).unwrap();
            for &u in &wl {
                for v in f.s3_roots(u, r) {
                    if inw.contains(&v) && lex_key(u, m) <= lex_key(v, m) {
                        s.insert(vec![u, v]);
                    }
                }
            }
        }
        if m > 11 {
            return out;
        }
        // small m: cross-check with the pair enumeration
        let mut out2: HashMap<u64, BTreeSet<Vec<u64>>> = HashMap::new();
        for &t in targets {
            out2.insert(t, BTreeSet::new());
        }
        for i in 0..wl.len() {
            for j in i..wl.len() {
                for r in f.s3_roots(wl[i], wl[j]) {
                    if let Some(s) = out2.get_mut(&r) {
                        s.insert(vec![wl[i], wl[j]]);
                    }
                }
            }
        }
        assert_eq!(out, out2, "ground truth cross-check");
        return out;
    }
    if k == 2 {
        for i in 0..wl.len() {
            for j in i..wl.len() {
                for r in f.s3_roots(wl[i], wl[j]) {
                    if let Some(s) = out.get_mut(&r) {
                        s.insert(vec![wl[i], wl[j]]);
                    }
                }
            }
        }
    } else if k == 3 {
        for i in 0..wl.len() {
            for j in i..wl.len() {
                for t in f.s3_roots(wl[i], wl[j]) {
                    for l in j..wl.len() {
                        for r in f.s3_roots(t, wl[l]) {
                            if let Some(s) = out.get_mut(&r) {
                                s.insert(vec![wl[i], wl[j], wl[l], t]);
                            }
                        }
                    }
                }
            }
        }
    } else {
        panic!("k in {{2,3}}");
    }
    out
}
fn curve_verified(cv: &Curve, model: &[u64], k: usize, xr: u64) -> bool {
    let r = cv.from_x(xr);
    let mut pts = Vec::new();
    for &x in &model[..k] {
        let p = cv.from_x(x);
        if p.is_none() {
            return false;
        }
        pts.push(p);
    }
    for mask in 0..(1u32 << k) {
        let mut acc: Pt = None;
        for i in 0..k {
            let q = if (mask >> i) & 1 == 1 { cv.neg(pts[i]) } else { pts[i] };
            acc = cv.add(acc, q);
        }
        if acc.is_some() && (acc == r || acc == cv.neg(r)) {
            return true;
        }
    }
    false
}

// ---------------------------------------------------------------- learner
#[derive(Default, Clone)]
struct LStats {
    learned: u64,
    indep: u64,     // mentions no R var and no activation var
    r_only: u64,    // mentions R, no activation
    act: u64,       // mentions an activation var (dead after its target)
    applic: f64,    // sum over non-act clauses of 2^-(#R lits)
    rhist: [u64; 8],
}
struct L {
    cur: Vec<i32>,
    st: Rc<RefCell<LStats>>,
    rmax: i32,
    rmin: i32,
    tnv: i32,
}
impl Learner for L {
    fn learning(&mut self, _size: i32) -> bool {
        true
    }
    fn learn(&mut self, lit: i32) {
        if lit != 0 {
            self.cur.push(lit);
            return;
        }
        let mut s = self.st.borrow_mut();
        s.learned += 1;
        let act = self.cur.iter().any(|l| l.abs() > self.tnv);
        let nr = self.cur.iter().filter(|l| l.abs() >= self.rmin && l.abs() <= self.rmax).count();
        if act {
            s.act += 1;
        } else {
            if nr == 0 { s.indep += 1 } else { s.r_only += 1 }
            s.applic += 0.5f64.powi(nr as i32);
            s.rhist[nr.min(7)] += 1;
        }
        self.cur.clear();
    }
}

// ---------------------------------------------------------------- arms
struct Ctx {
    f: Onb,
    k: usize,
    w: usize,
    tmpl: Vec<Vec<i32>>,
    tnv: i32,
    proj: Vec<Vec<i32>>,
    rvars: Vec<i32>,
    tmpl_set: HashSet<Vec<i32>>,
    budget: i32,
    tag: String,
}
fn new_solver(nv: i32) -> CaDiCal {
    let mut s = CaDiCal::new();
    assert!(s.set("factor", 0));
    assert!(s.set("factorcheck", 0));
    let _ = nv;
    s
}
fn stat3(s: &CaDiCal) -> [i64; 3] {
    [s.get_statistic_value("conflicts"), s.get_statistic_value("decisions"), s.get_statistic_value("propagations")]
}

// ================================================================ F1 code starts here
use cadical_sys::ProofTracer;

// ---------------------------------------------------------------- dual builder
// Every wire carries its folded literal (R replaced by constants, exactly as build_system(Some(R)))
// and its template literal (R as variables, exactly as build_system(None)).  A folded gate that
// needs a fresh variable takes the variable index of the template gate built at the same step,
// so the folded CNF is written over template indices: the same variable means the same gate for
// every target.  Compacting these indices in increasing order reproduces A0's CNF exactly
// (asserted per target).
#[derive(Clone, Copy, Debug)]
struct Wr {
    f: i32,
    t: i32,
}
fn wneg(a: Wr) -> Wr {
    Wr { f: -a.f, t: -a.t }
}
const WTRUE: Wr = Wr { f: T, t: T };
const WFALSE: Wr = Wr { f: F, t: F };

struct Dual {
    tc: Cnf,
    rdep: Vec<bool>, // template var depends on R
    fcls: Vec<Vec<i32>>,
    fcore: Vec<bool>, // folded clause is target-independent (defines/constrains R-free wires only)
    falloc: Vec<i32>,
}
impl Dual {
    fn new() -> Dual {
        Dual { tc: Cnf::new(), rdep: vec![false, false], fcls: vec![vec![1]], fcore: vec![true], falloc: vec![1] }
    }
    fn rd(&self, t: i32) -> bool {
        if t == T || t == F { false } else { self.rdep[t.unsigned_abs() as usize] }
    }
    fn grow(&mut self, nvb: i32, dep: bool) {
        assert_eq!(self.rdep.len() as i32, nvb + 1);
        while (self.rdep.len() as i32) <= self.tc.nv {
            self.rdep.push(dep);
        }
    }
    fn var(&mut self) -> Wr {
        let nvb = self.tc.nv;
        let v = self.tc.var();
        self.grow(nvb, false);
        self.falloc.push(v);
        Wr { f: v, t: v }
    }
    fn rvar(&mut self, bit: bool) -> Wr {
        let nvb = self.tc.nv;
        let v = self.tc.var();
        self.grow(nvb, true);
        Wr { f: if bit { T } else { F }, t: v }
    }
    fn push(&mut self, c: Vec<i32>, core: bool) {
        self.fcls.push(c);
        self.fcore.push(core);
    }
    fn fresh(&mut self, t: i32, nvb: i32) -> i32 {
        assert!(t > 0 && t == self.tc.nv && t == nvb + 1, "folded gate without a template gate");
        self.falloc.push(t);
        t
    }
    fn xor(&mut self, a: Wr, b: Wr) -> Wr {
        let nvb = self.tc.nv;
        let t = self.tc.xor(a.t, b.t);
        let dep = self.rd(a.t) || self.rd(b.t);
        self.grow(nvb, dep);
        let (x, y) = (a.f, b.f);
        let f = if x == F {
            y
        } else if y == F {
            x
        } else if x == T {
            -y
        } else if y == T {
            -x
        } else if x == y {
            F
        } else if x == -y {
            T
        } else {
            let z = self.fresh(t, nvb);
            let core = !self.rdep[z as usize];
            self.push(vec![-z, x, y], core);
            self.push(vec![-z, -x, -y], core);
            self.push(vec![z, -x, y], core);
            self.push(vec![z, x, -y], core);
            z
        };
        Wr { f, t }
    }
    fn and(&mut self, a: Wr, b: Wr) -> Wr {
        let nvb = self.tc.nv;
        let t = self.tc.and(a.t, b.t);
        let dep = self.rd(a.t) || self.rd(b.t);
        self.grow(nvb, dep);
        let (x, y) = (a.f, b.f);
        let f = if x == F || y == F {
            F
        } else if x == T {
            y
        } else if y == T {
            x
        } else if x == y {
            x
        } else if x == -y {
            F
        } else {
            let z = self.fresh(t, nvb);
            let core = !self.rdep[z as usize];
            self.push(vec![-z, x], core);
            self.push(vec![-z, y], core);
            self.push(vec![z, -x, -y], core);
            z
        };
        Wr { f, t }
    }
    fn xor_list(&mut self, mut v: Vec<Wr>) -> Wr {
        if v.is_empty() {
            return WFALSE;
        }
        while v.len() > 1 {
            let mut nx = Vec::new();
            let mut i = 0;
            while i + 1 < v.len() {
                nx.push(self.xor(v[i], v[i + 1]));
                i += 2;
            }
            if i < v.len() {
                nx.push(v[i]);
            }
            v = nx;
        }
        v[0]
    }
    fn assert_zero(&mut self, l: Wr) {
        self.tc.assert_zero(l.t);
        assert!(l.f != T, "unsat by construction");
        if l.f != F {
            let core = !self.rd(l.t);
            self.push(vec![-l.f], core);
        }
    }
    fn at_most(&mut self, lits: &[Wr], k: usize) {
        let tl: Vec<i32> = lits.iter().map(|l| { assert_eq!(l.f, l.t); l.t }).collect();
        let nvb = self.tc.nv;
        let cb = self.tc.cls.len();
        self.tc.at_most(&tl, k);
        self.grow(nvb, false);
        for v in nvb + 1..=self.tc.nv {
            self.falloc.push(v);
        }
        let new: Vec<Vec<i32>> = self.tc.cls[cb..].to_vec();
        for c in new {
            self.push(c, true);
        }
    }
    fn lex_leq(&mut self, a: &[Wr], b: &[Wr]) {
        let mut eq = WTRUE;
        for i in 0..a.len() {
            assert_eq!(eq.f == F, eq.t == F);
            if eq.f != F {
                let mut cl = vec![-a[i].f, b[i].f];
                if eq.f != T {
                    cl.push(-eq.f);
                }
                let core = !(self.rd(a[i].t) || self.rd(b[i].t) || self.rd(eq.t));
                self.push(cl, core);
                // the same clause on the template side, so that tc.cls equals build_system(None)
                let mut tcl = vec![-a[i].t, b[i].t];
                if eq.t != T {
                    tcl.push(-eq.t);
                }
                self.tc.cls.push(tcl);
            }
            let x = self.xor(a[i], b[i]);
            eq = self.and(eq, wneg(x));
        }
    }
    fn onb_mul(&mut self, f: &Onb, a: &[Wr], b: &[Wr]) -> Vec<Wr> {
        let m = f.m;
        let mut andc: HashMap<(usize, usize), Wr> = HashMap::new();
        let mut out = Vec::with_capacity(m);
        for k in 0..m {
            let mut ts = Vec::new();
            for &(i, j) in &f.terms[k] {
                let l = match andc.get(&(i, j)) {
                    Some(&l) => l,
                    None => {
                        let l = self.and(a[i], b[j]);
                        andc.insert((i, j), l);
                        l
                    }
                };
                ts.push(l);
            }
            out.push(self.xor_list(ts));
        }
        out
    }
    fn s3(&mut self, f: &Onb, u: &[Wr], v: &[Wr], w: &[Wr]) -> Vec<Wr> {
        let m = f.m;
        let uv = self.onb_mul(f, u, v);
        let uw = self.onb_mul(f, u, w);
        let vw = self.onb_mul(f, v, w);
        let mut s = Vec::with_capacity(m);
        for i in 0..m {
            let t = self.xor(uv[i], uw[i]);
            s.push(self.xor(t, vw[i]));
        }
        let mut sq = vec![WFALSE; m];
        for i in 0..m {
            sq[f.sqperm[i]] = s[i];
        }
        let uvw = self.onb_mul(f, &uv, w);
        let mut out = Vec::with_capacity(m);
        for i in 0..m {
            let t = self.xor(sq[i], uvw[i]);
            out.push(self.xor(t, WTRUE));
        }
        out
    }
}

struct FoldInst {
    cls: Vec<Vec<i32>>, // dense numbering; equals build_system(Some(R)).cnf.cls
    core: Vec<bool>,
    d2t: Vec<i32>,
    t2d: Vec<i32>,
    nv: i32,
    proj: Vec<Vec<i32>>,
    core_t: Vec<Vec<i32>>, // target-independent clauses, template numbering, each sorted
    rdep: Vec<bool>,
    tmpl_nv: i32,
    tmpl_cls: Vec<Vec<i32>>,
}
fn fold_instance(f: &Onb, k: usize, w: usize, xr: u64) -> FoldInst {
    let m = f.m;
    let mut d = Dual::new();
    let mut pvars = Vec::new();
    for _ in 0..k {
        let v: Vec<Wr> = (0..m).map(|_| d.var()).collect();
        d.at_most(&v, w);
        pvars.push(v);
    }
    let mut tvars = Vec::new();
    for _ in 0..k.saturating_sub(2) {
        tvars.push((0..m).map(|_| d.var()).collect::<Vec<Wr>>());
    }
    let r: Vec<Wr> = (0..m).map(|j| d.rvar((xr >> j) & 1 == 1)).collect();
    let mut chain: Vec<Vec<Wr>> = tvars.clone();
    chain.push(r);
    let mut left = pvars[0].clone();
    for i in 0..k - 1 {
        let out = chain[i].clone();
        let roots = d.s3(f, &left, &pvars[i + 1], &out);
        for l in roots {
            d.assert_zero(l);
        }
        left = out;
    }
    for i in 0..k - 1 {
        let (a, b) = (pvars[i].clone(), pvars[i + 1].clone());
        d.lex_leq(&a, &b);
    }
    let mut t2d = vec![0i32; d.tc.nv as usize + 1];
    let mut d2t = vec![0i32];
    for (i, &t) in d.falloc.iter().enumerate() {
        if i > 0 {
            assert!(t > d.falloc[i - 1]);
        }
        t2d[t as usize] = i as i32 + 1;
        d2t.push(t);
    }
    let map = |l: i32| -> i32 {
        let x = t2d[l.unsigned_abs() as usize];
        assert!(x != 0, "folded clause uses an unallocated variable");
        if l > 0 { x } else { -x }
    };
    let cls: Vec<Vec<i32>> = d.fcls.iter().map(|c| c.iter().map(|&l| map(l)).collect()).collect();
    let proj: Vec<Vec<i32>> = pvars.iter().chain(tvars.iter()).map(|v| v.iter().map(|w| map(w.f)).collect()).collect();
    let core_t: Vec<Vec<i32>> = d
        .fcls
        .iter()
        .zip(d.fcore.iter())
        .filter(|(_, &c)| c)
        .map(|(c, _)| {
            let mut c = c.clone();
            c.sort();
            c
        })
        .collect();
    FoldInst { cls, core: d.fcore.clone(), d2t, t2d, nv: d.falloc.len() as i32, proj, core_t, rdep: d.rdep.clone(), tmpl_nv: d.tc.nv, tmpl_cls: d.tc.cls.clone() }
}

// ---------------------------------------------------------------- taint tracer (target-free learnts)
#[derive(Default)]
struct Taint {
    core: HashSet<Vec<i32>>, // dense, sorted: clauses implied by the target-independent core
    taint: HashMap<i64, bool>,
    live: HashMap<i64, Vec<i32>>, // untainted derived clauses still alive, dense, sorted
    n_orig: u64,
    n_orig_core: u64,
    n_derived: u64,
    n_derived_clean: u64,
    n_rat: u64,
    n_empty_chain: u64,
}
impl ProofTracer for Taint {
    fn add_original_clause(&mut self, id: i64, _redundant: bool, clause: &[i32], restored: bool) {
        let mut c = clause.to_vec();
        c.sort();
        c.dedup();
        let clean = match (restored, self.taint.get(&id)) {
            (true, Some(&t)) => !t,
            _ => self.core.contains(&c),
        };
        self.n_orig += 1;
        if clean {
            self.n_orig_core += 1;
        }
        self.taint.insert(id, !clean);
    }
    fn add_derived_clause(&mut self, id: i64, _redundant: bool, witness: i32, clause: &[i32], antecedents: &[i64]) {
        self.n_derived += 1;
        if witness != 0 {
            self.n_rat += 1;
        }
        if antecedents.is_empty() {
            self.n_empty_chain += 1;
        }
        let tainted = witness != 0 || antecedents.is_empty() || antecedents.iter().any(|a| *self.taint.get(a).unwrap_or(&true));
        self.taint.insert(id, tainted);
        if !tainted {
            self.n_derived_clean += 1;
            let mut c = clause.to_vec();
            c.sort();
            c.dedup();
            self.live.insert(id, c);
        }
    }
    fn delete_clause(&mut self, id: i64, _redundant: bool, _clause: &[i32]) {
        self.live.remove(&id);
        self.taint.remove(&id);
    }
    fn weaken_minus(&mut self, _id: i64, _clause: &[i32]) {}
    fn strengthen(&mut self, _id: i64) {}
    fn finalize_clause(&mut self, _id: i64, _clause: &[i32]) {}
    fn add_assumption(&mut self, _lit: i32) {}
    fn add_constraint(&mut self, _clause: &[i32]) {}
    fn reset_assumptions(&mut self) {}
    fn add_assumption_clause(&mut self, id: i64, _clause: &[i32], _antecedents: &[i64]) {
        self.taint.insert(id, true);
    }
    fn conclude_sat(&mut self, _conclusion_type: i32, _model: &[i32]) {}
    fn conclude_unsat(&mut self, _conclusion_type: i32, _clause_ids: &[i64]) {}
    fn conclude_unknown(&mut self, _trail: &[i32]) {}
}

// ---------------------------------------------------------------- enumeration (as L3)
struct Enum {
    models: BTreeSet<Vec<u64>>,
    ncalls: usize,
    tot: [i64; 3],
    first: [i64; 3],
    last: [i64; 3],
    censored: bool,
    first_model: Option<Vec<i32>>, // values of vars 1..=nv at the first SAT answer (index = var)
}
fn enumerate(s: &mut CaDiCal, assumps: &[i32], act: Option<i32>, proj: &[Vec<i32>], budget: i32, model_nv: Option<i32>) -> Enum {
    let base = stat3(s);
    let mut prev = base;
    let mut models = BTreeSet::new();
    let mut calls: Vec<[i64; 3]> = Vec::new();
    let mut censored = false;
    let mut first_model = None;
    loop {
        for &a in assumps {
            s.assume(a);
        }
        s.limit("conflicts", budget);
        let st = s.solve();
        let now = stat3(s);
        let d = [now[0] - prev[0], now[1] - prev[1], now[2] - prev[2]];
        prev = now;
        calls.push(d);
        match st {
            Status::SATISFIABLE => {
                let mut blk = Vec::new();
                if let Some(a) = act {
                    blk.push(-a);
                }
                let mut mdl = Vec::new();
                for vec in proj {
                    let mut c = 0u64;
                    for (j, &v) in vec.iter().enumerate() {
                        if s.val(v) > 0 {
                            c |= 1 << j;
                            blk.push(-v);
                        } else {
                            blk.push(v);
                        }
                    }
                    mdl.push(c);
                }
                if first_model.is_none() {
                    if let Some(nv) = model_nv {
                        let mut fm = vec![0i32; nv as usize + 1];
                        for v in 1..=nv {
                            fm[v as usize] = if s.val(v) > 0 { 1 } else { -1 };
                        }
                        first_model = Some(fm);
                    }
                }
                assert!(models.insert(mdl), "duplicate model");
                blk.sort();
                s.clause6(&blk);
            }
            Status::UNSATISFIABLE => break,
            Status::UNKNOWN => {
                censored = true;
                break;
            }
        }
    }
    let tot = [prev[0] - base[0], prev[1] - base[1], prev[2] - base[2]];
    Enum { models, ncalls: calls.len(), first: calls[0], last: *calls.last().unwrap(), tot, censored, first_model }
}

struct ColdOut {
    e: Enum,
    final_ph: Vec<i8>,            // template-indexed saved phases at the end (0 = var absent)
    first_model: Option<Vec<i8>>, // template-indexed
    seeded: usize,
    wall_ms: f64,
    lrat_opt: i32,
}
fn shuffle(cls: &mut Vec<Vec<i32>>, seed: u64) {
    let mut rng = Rng(seed);
    for i in (1..cls.len()).rev() {
        let j = rng.below(i as u64 + 1) as usize;
        cls.swap(i, j);
    }
}
fn cold(ctx: &Ctx, inst: &FoldInst, shuffle_seed: Option<u64>, extra: &[Vec<i32>], seed: &[i32], forced: bool, tracer: Option<Rc<RefCell<Taint>>>) -> ColdOut {
    let t0 = Instant::now();
    let mut s = new_solver(inst.nv);
    if let Some(tr) = tracer {
        s.connect_proof_tracer1(tr, true, false);
    }
    let lrat_opt = s.get("lrat");
    let mut cls = inst.cls.clone();
    if let Some(sd) = shuffle_seed {
        shuffle(&mut cls, sd);
    }
    for c in cls.iter().chain(extra.iter()) {
        s.clause6(c);
    }
    let mut seeded = 0;
    for &l in seed {
        if forced {
            s.phase(l);
            seeded += 1;
        } else if s.f1_seed_saved_phase(l) {
            seeded += 1;
        }
    }
    let e = enumerate(&mut s, &[], None, &inst.proj, ctx.budget, Some(inst.nv));
    let tn = ctx.tnv as usize + 1;
    let mut final_ph = vec![0i8; tn];
    for dv in 1..=inst.nv {
        final_ph[inst.d2t[dv as usize] as usize] = s.f1_saved_phase(dv) as i8;
    }
    let first_model = e.first_model.as_ref().map(|fm| {
        let mut v = vec![0i8; tn];
        for dv in 1..=inst.nv {
            v[inst.d2t[dv as usize] as usize] = fm[dv as usize] as i8;
        }
        v
    });
    ColdOut { e, final_ph, first_model, seeded, wall_ms: t0.elapsed().as_secs_f64() * 1e3, lrat_opt }
}
/// dense seed literals from a template-indexed phase vector (only variables present in both)
fn seed_from(inst: &FoldInst, ph: &[i8]) -> Vec<i32> {
    let mut out = Vec::new();
    for dv in 2..=inst.nv {
        let p = ph[inst.d2t[dv as usize] as usize];
        if p != 0 {
            out.push(if p > 0 { dv } else { -dv });
        }
    }
    out
}
fn phstr(v: &[i8]) -> String {
    v[1..].iter().map(|&p| match p { 1 => '+', -1 => '-', _ => '.' }).collect()
}

// ---------------------------------------------------------------- B (L3's persistent arm, same semantics)
struct BArm {
    s: CaDiCal,
    next_act: i32,
}
impl BArm {
    fn new(ctx: &Ctx) -> BArm {
        let mut s = new_solver(ctx.tnv);
        for c in &ctx.tmpl {
            s.clause6(c);
        }
        let rmin = *ctx.rvars.iter().min().unwrap();
        let rmax = *ctx.rvars.iter().max().unwrap();
        s.connect_learner(L { cur: vec![], st: Rc::new(RefCell::new(LStats::default())), rmin, rmax, tnv: ctx.tnv });
        BArm { s, next_act: ctx.tnv + 1 }
    }
    fn solve(&mut self, ctx: &Ctx, xr: u64) -> (Enum, f64) {
        let t0 = Instant::now();
        let a = self.next_act;
        self.next_act += 1;
        let mut assumps: Vec<i32> = (0..ctx.f.m).map(|j| if (xr >> j) & 1 == 1 { ctx.rvars[j] } else { -ctx.rvars[j] }).collect();
        assumps.push(a);
        let e = enumerate(&mut self.s, &assumps, Some(a), &ctx.proj, ctx.budget, None);
        self.s.clause1(-a);
        (e, t0.elapsed().as_secs_f64() * 1e3)
    }
}

// ---------------------------------------------------------------- self tests
fn selftest() {
    for &m in &[5usize, 11, 23] {
        let f = Onb::new(m);
        let mut rng = Rng(seed_of(&format!("F1/selftest/{}", m)));
        for _ in 0..200 {
            let (a, b) = (rng.next() & f.one(), rng.next() & f.one());
            assert_eq!(f.mul(a, b), f.mul_terms(a, b));
        }
        for k in [2usize, 3] {
            let tsys = build_system(&f, k, 2, None);
            for _ in 0..6 {
                let xr = rng.next() & f.one();
                let inst = fold_instance(&f, k, 2, xr);
                let a0 = build_system(&f, k, 2, Some(xr));
                assert_eq!(inst.cls, a0.cnf.cls, "fold != A0 m={m} k={k}");
                assert_eq!(inst.nv, a0.cnf.nv);
                assert_eq!(inst.tmpl_nv, tsys.cnf.nv);
                assert_eq!(inst.tmpl_cls, tsys.cnf.cls);
                let p0: Vec<Vec<i32>> = a0.pvars.iter().chain(a0.tvars.iter()).cloned().collect();
                assert_eq!(inst.proj, p0);
            }
            // core clause set independent of the target
            let c1 = fold_instance(&f, k, 2, 1).core_t;
            let c2 = fold_instance(&f, k, 2, f.one() ^ 5).core_t;
            assert_eq!(c1, c2, "core differs across targets m={m} k={k}");
        }
    }
    // phase getter/seeder: seed a model as saved phases, then the first call must need 0 conflicts
    let f = Onb::new(11);
    let cv = Curve { f: f.clone() };
    let mut rng = Rng(seed_of("F1/selftest/seed"));
    let mut done = 0;
    while done < 5 {
        let xr = rng.next() & f.one();
        if cv.from_x(xr).is_none() {
            continue;
        }
        let inst = fold_instance(&f, 2, 2, xr);
        let mut s = new_solver(inst.nv);
        for c in &inst.cls {
            s.clause6(c);
        }
        if !matches!(s.solve(), Status::SATISFIABLE) {
            continue;
        }
        let model: Vec<i32> = (1..=inst.nv).map(|v| if s.val(v) > 0 { v } else { -v }).collect();
        for forced in [false, true] {
            let mut s2 = new_solver(inst.nv);
            for c in &inst.cls {
                s2.clause6(c);
            }
            for &l in &model[1..] {
                if forced {
                    s2.phase(l);
                } else {
                    assert!(s2.f1_seed_saved_phase(l));
                }
            }
            if !forced {
                for &l in &model[1..] {
                    assert_eq!(s2.f1_saved_phase(l.abs()), l.signum(), "getter after seed");
                }
            }
            assert!(matches!(s2.solve(), Status::SATISFIABLE));
            let c = s2.get_statistic_value("conflicts");
            println!("seed-model check m=11 xr={xr} forced={forced}: conflicts {c}");
            assert_eq!(c, 0, "seeded model should be found without conflicts");
        }
        done += 1;
    }
    // taint tracer: every clean derived clause must be implied by the core alone
    let tsys = build_system(&f, 2, 2, None);
    let _ = tsys;
    let mut checked = 0;
    for i in 0..4u64 {
        let xr = (rng.next() & f.one()) | 1 | i;
        let inst = fold_instance(&f, 2, 2, xr);
        let tr = Rc::new(RefCell::new(Taint::default()));
        {
            let mut t = tr.borrow_mut();
            for (c, &k) in inst.cls.iter().zip(inst.core.iter()) {
                if k {
                    let mut c = c.clone();
                    c.sort();
                    c.dedup();
                    t.core.insert(c);
                }
            }
        }
        let mut s = new_solver(inst.nv);
        s.connect_proof_tracer1(tr.clone(), true, false);
        for c in &inst.cls {
            s.clause6(c);
        }
        let _ = enumerate(&mut s, &[], None, &inst.proj, 2_000_000, None);
        let t = tr.borrow();
        for c in t.live.values() {
            let mut chk = new_solver(inst.tmpl_nv);
            for cc in &inst.core_t {
                chk.clause6(cc);
            }
            for &l in c {
                let tl = inst.d2t[l.unsigned_abs() as usize] * l.signum();
                chk.clause1(-tl);
            }
            assert!(matches!(chk.solve(), Status::UNSATISFIABLE), "carried clause not implied by core");
            checked += 1;
        }
        println!("taint m=11 xr={xr}: orig {} core {} derived {} clean {} live-clean {} rat {} emptychain {}", t.n_orig, t.n_orig_core, t.n_derived, t.n_derived_clean, t.live.len(), t.n_rat, t.n_empty_chain);
    }
    println!("taint: {checked} live clean clauses verified against the core");
    println!("selftest ok");
}

// ---------------------------------------------------------------- main run
fn jarr3(a: &[i64; 3]) -> String {
    format!("{{\"conflicts\":{},\"decisions\":{},\"propagations\":{}}}", a[0], a[1], a[2])
}

struct WState {
    prev: Option<Vec<i8>>,
    carried_t: Vec<Vec<i32>>, // WL only, template numbering
}

#[allow(clippy::too_many_arguments)]
fn run(m: usize, k: usize, w: usize, ntargets: usize, budget: i32, arms: &[String], out: &str, check_per_target: usize, carry_cap: usize) {
    let tag = format!("m{}k{}w{}", m, k, w);
    let f = Onb::new(m);
    let cv = Curve { f: f.clone() };
    let tsys = build_system(&f, k, w, None);
    let tmpl = tsys.cnf.cls.clone();
    let tnv = tsys.cnf.nv;
    let ctx = Ctx {
        f: f.clone(),
        k,
        w,
        tmpl,
        tnv,
        proj: tsys.pvars.iter().chain(tsys.tvars.iter()).cloned().collect(),
        rvars: tsys.rvars.clone(),
        tmpl_set: HashSet::new(),
        budget,
        tag: tag.clone(),
    };
    // target stream: identical generator and seed string to L3
    let mut rng = Rng(seed_of(&format!("L3/targets/{}", tag)));
    let mut stream = Vec::new();
    while stream.len() < ntargets {
        let x = rng.next() & f.one();
        if cv.from_x(x).is_some() {
            stream.push(x);
        }
    }
    let tset: HashSet<u64> = stream.iter().cloned().collect();
    let gt0 = Instant::now();
    let truth = ground_truth(&f, k, w, &tset);
    let gt_ms = gt0.elapsed().as_secs_f64() * 1e3;
    let inst0 = fold_instance(&f, k, w, stream[0]);
    let core_ref = inst0.core_t.clone();
    let rdep_s: String = inst0.rdep[1..].iter().map(|&d| if d { 'r' } else { 'c' }).collect();
    eprintln!("[{tag}] template vars {} clauses {}; core clauses {}; ground truth {:.0} ms; arms {:?}", tnv, ctx.tmpl.len(), core_ref.len(), gt_ms, arms);
    let mut fh = std::fs::File::create(out).unwrap();
    writeln!(fh, "{{\"type\":\"header\",\"tag\":\"{}\",\"m\":{},\"k\":{},\"w\":{},\"template_vars\":{},\"template_clauses\":{},\"core_clauses\":{},\"ntargets\":{},\"budget\":{},\"arms\":{:?},\"solver\":\"CaDiCaL {} via cadical-sys 0.8.0 + F1 bridge patch (saved-phase getter/seeder), factor=0 factorcheck=0, else defaults\",\"ground_truth_ms\":{:.1},\"template_var_class\":\"{}\"}}",
        tag, m, k, w, tnv, ctx.tmpl.len(), core_ref.len(), ntargets, budget, arms, CaDiCal::version(), gt_ms, rdep_s).unwrap();
    let mut barm: Option<BArm> = if arms.iter().any(|a| a == "B") { Some(BArm::new(&ctx)) } else { None };
    let mut ws: HashMap<String, WState> = HashMap::new();
    for a in arms {
        ws.insert(a.clone(), WState { prev: None, carried_t: vec![] });
    }
    for (ti, &xr) in stream.iter().enumerate() {
        let tr = &truth[&xr];
        let ncurve = tr.iter().filter(|mdl| curve_verified(&cv, mdl, k, xr)).count();
        let inst = fold_instance(&f, k, w, xr);
        let a0sys = build_system(&f, k, w, Some(xr));
        assert_eq!(inst.cls, a0sys.cnf.cls, "folded instance differs from A0");
        assert_eq!(inst.nv, a0sys.cnf.nv);
        assert_eq!(inst.core_t, core_ref, "core differs across targets");
        assert_eq!(inst.tmpl_cls, ctx.tmpl);
        let na = arms.len();
        for off in 0..na {
            let arm = arms[(ti + off) % na].clone();
            // each entry: (arm name, Enum, wall, extra json)
            let mut outs: Vec<(String, Enum, f64, String)> = Vec::new();
            match arm.as_str() {
                "A0" => {
                    let o = cold(&ctx, &inst, None, &[], &[], false, None);
                    let mut ex = format!(",\"final_phases\":\"{}\"", phstr(&o.final_ph));
                    if let Some(fm) = &o.first_model {
                        write!(ex, ",\"first_model\":\"{}\"", phstr(fm)).unwrap();
                    }
                    let a0ph = o.final_ph.clone();
                    outs.push(("A0".into(), o.e, o.wall_ms, ex));
                    if arms.iter().any(|a| a == "WS") {
                        // positive control, run right after A0 on the same target
                        let sd = seed_from(&inst, &a0ph);
                        let o = cold(&ctx, &inst, None, &[], &sd, false, None);
                        let ex = format!(",\"seeded\":{},\"seed_source\":\"A0 same target\"", o.seeded);
                        outs.push(("WS".into(), o.e, o.wall_ms, ex));
                    }
                }
                "WS" => {} // runs inside the A0 unit
                "AA" => {
                    let o = cold(&ctx, &inst, Some(seed_of(&format!("{}/AA/{}", tag, ti))), &[], &[], false, None);
                    let ex = format!(",\"final_phases\":\"{}\"", phstr(&o.final_ph));
                    outs.push(("AA".into(), o.e, o.wall_ms, ex));
                }
                "WP" | "WF" | "WR" | "WT" | "WL" => {
                    let st = ws.get_mut(&arm).unwrap();
                    let mut sd = match &st.prev {
                        Some(p) => seed_from(&inst, p),
                        None => vec![],
                    };
                    if arm == "WR" {
                        let mut r = Rng(seed_of(&format!("F1/WR/{}/{}", tag, ti)));
                        for l in sd.iter_mut() {
                            *l = if r.next() & 1 == 1 { l.abs() } else { -l.abs() };
                        }
                    }
                    let seed_ph: Option<String> = if matches!(arm.as_str(), "WP" | "WR") {
                        let mut v = vec![0i8; tnv as usize + 1];
                        for &l in &sd {
                            v[inst.d2t[l.unsigned_abs() as usize] as usize] = l.signum() as i8;
                        }
                        Some(phstr(&v))
                    } else {
                        None
                    };
                    let tracer = if matches!(arm.as_str(), "WT" | "WL") {
                        let t = Rc::new(RefCell::new(Taint::default()));
                        {
                            let mut tb = t.borrow_mut();
                            for (c, &kk) in inst.cls.iter().zip(inst.core.iter()) {
                                if kk {
                                    let mut c = c.clone();
                                    c.sort();
                                    c.dedup();
                                    tb.core.insert(c);
                                }
                            }
                        }
                        Some(t)
                    } else {
                        None
                    };
                    let mut extra: Vec<Vec<i32>> = Vec::new();
                    if arm == "WL" {
                        for c in &st.carried_t {
                            let dc: Vec<i32> = c
                                .iter()
                                .map(|&l| {
                                    let d = inst.t2d[l.unsigned_abs() as usize];
                                    assert!(d != 0, "carried clause on a variable absent from this target");
                                    d * l.signum()
                                })
                                .collect();
                            extra.push(dc);
                        }
                        if let Some(t) = &tracer {
                            let mut tb = t.borrow_mut();
                            for c in &extra {
                                let mut c = c.clone();
                                c.sort();
                                c.dedup();
                                tb.core.insert(c);
                            }
                        }
                    }
                    let o = cold(&ctx, &inst, None, &extra, &sd, arm == "WF", tracer.clone());
                    let mut ex = format!(",\"seeded\":{},\"carried_in\":{},\"lrat\":{}", o.seeded, extra.len(), o.lrat_opt);
                    if arm == "WP" {
                        write!(ex, ",\"final_phases\":\"{}\"", phstr(&o.final_ph)).unwrap();
                    }
                    if let Some(sp) = seed_ph {
                        write!(ex, ",\"seed_phases\":\"{}\"", sp).unwrap();
                    }
                    if let Some(t) = &tracer {
                        let tb = t.borrow();
                        let mut newc: Vec<Vec<i32>> = tb
                            .live
                            .values()
                            .map(|c| {
                                let mut v: Vec<i32> = c.iter().map(|&l| inst.d2t[l.unsigned_abs() as usize] * l.signum()).collect();
                                v.sort();
                                v
                            })
                            .collect();
                        newc.sort();
                        newc.dedup();
                        // validity: sample of the new clean clauses must be implied by the core alone
                        let mut checked = 0;
                        if !newc.is_empty() && check_per_target > 0 {
                            let mut r = Rng(seed_of(&format!("F1/check/{}/{}/{}", tag, arm, ti)));
                            for _ in 0..check_per_target.min(newc.len()) {
                                let c = &newc[r.below(newc.len() as u64) as usize];
                                let mut chk = new_solver(tnv);
                                for cc in &core_ref {
                                    chk.clause6(cc);
                                }
                                for &l in c {
                                    chk.clause1(-l);
                                }
                                assert!(matches!(chk.solve(), Status::UNSATISFIABLE), "[{tag}] {arm} target {ti}: carried clause not implied by the core");
                                checked += 1;
                            }
                        }
                        write!(ex, ",\"taint\":{{\"orig\":{},\"orig_core\":{},\"derived\":{},\"derived_clean\":{},\"live_clean\":{},\"rat\":{},\"empty_chain\":{},\"checked\":{}}}", tb.n_orig, tb.n_orig_core, tb.n_derived, tb.n_derived_clean, newc.len(), tb.n_rat, tb.n_empty_chain, checked).unwrap();
                        if arm == "WL" {
                            let mut set: BTreeSet<Vec<i32>> = st.carried_t.iter().cloned().collect();
                            for c in newc {
                                if set.len() >= carry_cap {
                                    break;
                                }
                                set.insert(c);
                            }
                            st.carried_t = set.into_iter().collect();
                            write!(ex, ",\"carried_out\":{}", st.carried_t.len()).unwrap();
                        }
                    }
                    st.prev = Some(o.final_ph.clone());
                    outs.push((arm.clone(), o.e, o.wall_ms, ex));
                }
                "B" => {
                    let (e, wall) = barm.as_mut().unwrap().solve(&ctx, xr);
                    outs.push(("B".into(), e, wall, String::new()));
                }
                _ => panic!("unknown arm {arm}"),
            }
            for (name, e, wall, ex) in outs {
                let ok = e.censored || e.models == *tr;
                if !ok {
                    panic!("[{tag}] arm {} target {} model set differs from ground truth ({} vs {})", name, ti, e.models.len(), tr.len());
                }
                writeln!(fh, "{{\"type\":\"row\",\"tag\":\"{}\",\"arm\":\"{}\",\"pos\":{},\"xr\":{},\"truth_models\":{},\"curve_verified_models\":{},\"models\":{},\"match_truth\":{},\"censored\":{},\"decomposable\":{},\"total\":{},\"first_call\":{},\"ncalls\":{},\"last_call\":{},\"wall_ms\":{:.3}{}}}",
                    tag, name, ti, xr, tr.len(), ncurve, e.models.len(), e.models == *tr, e.censored, !tr.is_empty(), jarr3(&e.tot), jarr3(&e.first), e.ncalls, jarr3(&e.last), wall, ex).unwrap();
            }
        }
        fh.flush().unwrap();
        eprintln!("[{tag}] target {ti} done");
    }
    eprintln!("[{tag}] done");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let get = |k: &str, d: &str| -> String { args.iter().position(|a| a == k).map(|i| args[i + 1].clone()).unwrap_or(d.to_string()) };
    if args.iter().any(|a| a == "--selftest") {
        selftest();
        return;
    }
    let m: usize = get("--m", "11").parse().unwrap();
    let k: usize = get("--k", "2").parse().unwrap();
    let w: usize = get("--w", "2").parse().unwrap();
    let nt: usize = get("--targets", "40").parse().unwrap();
    let budget: i32 = get("--budget", "2000000").parse().unwrap();
    let out = get("--out", "rows.jsonl");
    let chk: usize = get("--check", "30").parse().unwrap();
    let cap: usize = get("--carry-cap", "200000").parse().unwrap();
    let arms: Vec<String> = get("--arms", "A0,WS,AA,WP,WF,WR,WT,WL,B").split(',').map(|s| s.to_string()).collect();
    for a in &arms {
        assert!(["A0", "WS", "AA", "WP", "WF", "WR", "WT", "WL", "B"].contains(&a.as_str()), "unknown arm {a}");
    }
    assert!(!arms.iter().any(|a| a == "WS") || arms.iter().any(|a| a == "A0"), "WS needs A0 in the same process");
    run(m, k, w, nt, budget, &arms, &out, chk, cap);
}
