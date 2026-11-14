//! L3 (ECC2K-130 open-items sweep 2026-11-14): incremental SAT across point-decomposition
//! targets on the Koblitz curve y^2 + xy = x^3 + 1 over GF(2^m), type-II optimal normal basis,
//! factor base {x : HW(x) <= w} (the goal22 codegen convention), chained Semaev S3 links.
//! See ../PROTOCOL.md.  Every target is fully enumerated (all decompositions, then one UNSAT
//! call), and the model set of every arm is compared with a brute-force ground truth.
//!
//! Arms: A0 cold + R folded in as constants (current practice); AA = A0 with the clause list
//! shuffled (A/A); A1 cold, template CNF with R as assumptions; B persistent solver, template
//! loaded once, R as assumptions; BS = B with shuffled template clauses (A/A for B);
//! CI rebuild after each target from template + live derived clauses that mention neither R
//! nor an activation literal; CA rebuild with every live derived clause that mentions no
//! activation literal; CK = B flushed (rebuilt empty) every K targets.

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
    fn mul(&self, a: u64, b: u64) -> u64 {
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

struct Arm {
    name: &'static str,
    s: Option<CaDiCal>,
    next_act: i32,
    n: usize,
    k_flush: usize,
    lst: Rc<RefCell<LStats>>,
    proof: Option<(String, u64)>, // path, read offset
    live: HashMap<Vec<i32>, u32>,
    blocking: HashSet<Vec<i32>>,
    shuffle: Option<u64>,
}

struct Rec {
    models: BTreeSet<Vec<u64>>,
    calls: Vec<(String, [i64; 3])>,
    tot: [i64; 3],
    first: [i64; 3],
    censored: bool,
    wall_ms: f64,
    lstats: Option<LStats>,
    carried: Option<usize>,
}

impl Arm {
    fn new(name: &'static str, ctx: &Ctx, k_flush: usize, shuffle: Option<u64>, scratch: &str) -> Arm {
        let mut a = Arm {
            name,
            s: None,
            next_act: ctx.tnv + 1,
            n: 0,
            k_flush,
            lst: Rc::new(RefCell::new(LStats::default())),
            proof: None,
            live: HashMap::new(),
            blocking: HashSet::new(),
            shuffle,
        };
        if matches!(name, "B" | "BS" | "CI" | "CA" | "CK") {
            let extra = vec![];
            a.fresh(ctx, &extra, scratch);
        }
        a
    }
    fn fresh(&mut self, ctx: &Ctx, extra: &[Vec<i32>], scratch: &str) {
        let mut s = new_solver(ctx.tnv);
        if matches!(self.name, "CI" | "CA") {
            let path = format!("{}/{}-{}-{}.drat", scratch, ctx.tag, self.name, self.n);
            let _ = std::fs::remove_file(&path);
            assert!(s.set("binary", 0));
            assert!(s.trace_proof2(&path));
            self.proof = Some((path, 0));
            self.live.clear();
        }
        let mut cls = ctx.tmpl.clone();
        if let Some(seed) = self.shuffle {
            let mut rng = Rng(seed);
            for i in (1..cls.len()).rev() {
                let j = rng.below(i as u64 + 1) as usize;
                cls.swap(i, j);
            }
        }
        for c in cls.iter().chain(extra.iter()) {
            s.clause6(c);
        }
        let rmin = *ctx.rvars.iter().min().unwrap();
        let rmax = *ctx.rvars.iter().max().unwrap();
        s.connect_learner(L { cur: vec![], st: self.lst.clone(), rmin, rmax, tnv: ctx.tnv });
        self.s = Some(s);
    }
    fn absorb_proof(&mut self) {
        let s = self.s.as_mut().unwrap();
        s.flush_proof_trace(false);
        let (path, off) = self.proof.as_mut().unwrap();
        let mut fh = std::fs::File::open(&*path).unwrap();
        fh.seek(SeekFrom::Start(*off)).unwrap();
        let mut buf = String::new();
        fh.read_to_string(&mut buf).unwrap();
        // consume only complete lines
        let upto = buf.rfind('\n').map(|i| i + 1).unwrap_or(0);
        *off += upto as u64;
        for line in buf[..upto].lines() {
            let mut it = line.split_whitespace().peekable();
            let del = it.peek() == Some(&"d");
            if del {
                it.next();
            }
            let mut key: Vec<i32> = it.map(|x| x.parse::<i32>().unwrap()).filter(|&x| x != 0).collect();
            key.sort();
            if del {
                if let Some(c) = self.live.get_mut(&key) {
                    *c -= 1;
                    if *c == 0 {
                        self.live.remove(&key);
                    }
                }
            } else {
                *self.live.entry(key).or_insert(0) += 1;
            }
        }
    }

    fn solve_target(&mut self, ctx: &Ctx, xr: u64, scratch: &str) -> Rec {
        let t0 = Instant::now();
        let f = &ctx.f;
        let m = f.m;
        let lbefore = self.lst.borrow().clone();
        let mut owned: Option<CaDiCal> = None;
        let mut proj = ctx.proj.clone();
        let mut assumps: Vec<i32> = Vec::new();
        let mut act: Option<i32> = None;
        match self.name {
            "A0" | "AA" => {
                let sys = build_system(f, ctx.k, ctx.w, Some(xr));
                let mut cls = sys.cnf.cls;
                if self.name == "AA" {
                    let mut rng = Rng(seed_of(&format!("{}/AA/{}", ctx.tag, self.n)));
                    for i in (1..cls.len()).rev() {
                        let j = rng.below(i as u64 + 1) as usize;
                        cls.swap(i, j);
                    }
                }
                let mut s = new_solver(sys.cnf.nv);
                for c in &cls {
                    s.clause6(c);
                }
                proj = sys.pvars.iter().chain(sys.tvars.iter()).cloned().collect();
                owned = Some(s);
            }
            "A1" => {
                let mut s = new_solver(ctx.tnv);
                for c in &ctx.tmpl {
                    s.clause6(c);
                }
                owned = Some(s);
                act = Some(ctx.tnv + 1);
            }
            _ => {
                act = Some(self.next_act);
                self.next_act += 1;
            }
        }
        if !ctx.rvars.is_empty() && self.name != "A0" && self.name != "AA" {
            for j in 0..m {
                let v = ctx.rvars[j];
                assumps.push(if (xr >> j) & 1 == 1 { v } else { -v });
            }
        }
        if let Some(a) = act {
            assumps.push(a);
        }
        let is_owned = owned.is_some();
        let s: &mut CaDiCal = match owned.as_mut() {
            Some(s) => s,
            None => self.s.as_mut().unwrap(),
        };
        let base = stat3(s);
        let mut prev = base;
        let mut models = BTreeSet::new();
        let mut calls = Vec::new();
        let mut censored = false;
        loop {
            for &a in &assumps {
                s.assume(a);
            }
            s.limit("conflicts", ctx.budget);
            let st = s.solve();
            let now = stat3(s);
            let d = [now[0] - prev[0], now[1] - prev[1], now[2] - prev[2]];
            prev = now;
            match st {
                Status::SATISFIABLE => {
                    calls.push(("sat".to_string(), d));
                    let mut blk = Vec::new();
                    if let Some(a) = act {
                        blk.push(-a);
                    }
                    let mut mdl = Vec::new();
                    for vec in &proj {
                        let mut c = 0u64;
                        for (j, &v) in vec.iter().enumerate() {
                            let val = s.val(v);
                            if val > 0 {
                                c |= 1 << j;
                                blk.push(-v);
                            } else {
                                blk.push(v);
                            }
                        }
                        mdl.push(c);
                    }
                    assert!(models.insert(mdl), "duplicate model");
                    blk.sort();
                    self.blocking.insert(blk.clone());
                    s.clause6(&blk);
                }
                Status::UNSATISFIABLE => {
                    calls.push(("unsat".to_string(), d));
                    break;
                }
                Status::UNKNOWN => {
                    calls.push(("budget".to_string(), d));
                    censored = true;
                    break;
                }
            }
        }
        let tot = [prev[0] - base[0], prev[1] - base[1], prev[2] - base[2]];
        let first = calls[0].1;
        if let (Some(a), false) = (act, is_owned) {
            s.clause1(-a);
        }
        drop(owned);
        self.n += 1;
        let lafter = self.lst.borrow().clone();
        let mut lst = None;
        if matches!(self.name, "B" | "BS" | "CI" | "CA" | "CK") {
            lst = Some(LStats {
                learned: lafter.learned - lbefore.learned,
                indep: lafter.indep - lbefore.indep,
                r_only: lafter.r_only - lbefore.r_only,
                act: lafter.act - lbefore.act,
                applic: lafter.applic - lbefore.applic,
                rhist: std::array::from_fn(|i| lafter.rhist[i] - lbefore.rhist[i]),
            });
        }
        let mut carried = None;
        if matches!(self.name, "CI" | "CA") {
            self.absorb_proof();
            let rmin = *ctx.rvars.iter().min().unwrap();
            let rmax = *ctx.rvars.iter().max().unwrap();
            let keep: Vec<Vec<i32>> = self
                .live
                .keys()
                .filter(|c| !ctx.tmpl_set.contains(*c) && !self.blocking.contains(*c))
                .filter(|c| c.iter().all(|l| l.abs() <= ctx.tnv))
                .filter(|c| self.name == "CA" || c.iter().all(|l| l.abs() < rmin || l.abs() > rmax))
                .cloned()
                .collect();
            carried = Some(keep.len());
            let mut keep = keep;
            keep.sort();
            if let Some((p, _)) = &self.proof {
                let _ = std::fs::remove_file(p);
            }
            self.fresh(ctx, &keep, scratch);
        }
        if self.name == "CK" && self.n % self.k_flush == 0 {
            self.fresh(ctx, &[], scratch);
        }
        Rec { models, calls, tot, first, censored, wall_ms: t0.elapsed().as_secs_f64() * 1e3, lstats: lst, carried }
    }
}

// ---------------------------------------------------------------- self tests
fn selftest() {
    for &m in &[5usize, 11, 29, 53] {
        let f = Onb::new(m);
        let mut rng = Rng(seed_of(&format!("selftest/{}", m)));
        let mask = f.one();
        for _ in 0..200 {
            let (a, b, c) = (rng.next() & mask, rng.next() & mask, rng.next() & mask);
            assert_eq!(f.mul(a, b), f.mul(b, a));
            assert_eq!(f.mul(f.mul(a, b), c), f.mul(a, f.mul(b, c)));
            assert_eq!(f.mul(a, b ^ c), f.mul(a, b) ^ f.mul(a, c));
            assert_eq!(f.mul(a, f.one()), a);
            assert_eq!(f.sqr(a), f.mul(a, a));
            if a != 0 {
                assert_eq!(f.mul(a, f.inv(a)), f.one());
            }
            let s = f.half_trace(a);
            if f.trace(a) == 0 {
                assert_eq!(f.sqr(s) ^ s, a);
            }
            let (u, v) = (a, b);
            for w in f.s3_roots(u, v) {
                assert_eq!(f.s3(u, v, w), 0);
            }
        }
        // S3 vanishes on x-coordinates of P, Q, P+Q
        let cv = Curve { f: f.clone() };
        let mut pts = Vec::new();
        while pts.len() < 20 {
            if let Some(p) = cv.from_x(rng.next() & mask) {
                pts.push(Some(p));
            }
        }
        for i in 0..10 {
            let (p, q) = (pts[i], pts[i + 10]);
            if let Some((x3, _)) = cv.add(p, q) {
                let r = f.s3_roots(p.unwrap().0, q.unwrap().0);
                assert!(r.contains(&x3), "S3 root missing m={m}");
            }
        }
        if m <= 11 {
            let mut cnt: i128 = 1;
            for x in 0..(1u64 << m) {
                if x == 0 {
                    cnt += 1; // (0, 1)
                    continue;
                }
                let c = x ^ f.inv(f.sqr(x));
                if f.trace(c) == 0 {
                    cnt += 2;
                }
            }
            assert_eq!(cnt, Curve::order_formula(m), "curve order m={m}");
        }
        // CNF S3 circuit agrees with field arithmetic (fixed inputs via units)
        if m <= 29 {
            for trial in 0..4 {
                let mut c = Cnf::new();
                let u: Vec<i32> = (0..m).map(|_| c.var()).collect();
                let v: Vec<i32> = (0..m).map(|_| c.var()).collect();
                let w: Vec<i32> = (0..m).map(|_| c.var()).collect();
                let out = c.s3(&f, &u, &v, &w);
                let (a, b, d) = (rng.next() & mask, rng.next() & mask, rng.next() & mask);
                let mut s = new_solver(c.nv);
                for cl in &c.cls {
                    s.clause6(cl);
                }
                for j in 0..m {
                    s.clause1(if (a >> j) & 1 == 1 { u[j] } else { -u[j] });
                    s.clause1(if (b >> j) & 1 == 1 { v[j] } else { -v[j] });
                    s.clause1(if (d >> j) & 1 == 1 { w[j] } else { -w[j] });
                }
                assert!(matches!(s.solve(), Status::SATISFIABLE));
                let mut got = 0u64;
                for j in 0..m {
                    let l = out[j];
                    let val = if l == T { true } else if l == F { false } else if l > 0 { s.val(l) > 0 } else { s.val(-l) < 0 };
                    if val {
                        got |= 1 << j;
                    }
                }
                assert_eq!(got, f.s3(a, b, d), "cnf s3 m={m} trial={trial}");
            }
        }
    }
    println!("selftest ok");
}

// ---------------------------------------------------------------- main run
fn jarr3(a: &[i64; 3]) -> String {
    format!("{{\"conflicts\":{},\"decisions\":{},\"propagations\":{}}}", a[0], a[1], a[2])
}

fn run(m: usize, k: usize, w: usize, ntargets: usize, nrepeat: usize, kflush: usize, budget: i32, arms: &[&'static str], out: &str, scratch: &str) {
    let tag = format!("m{}k{}w{}", m, k, w);
    let f = Onb::new(m);
    let cv = Curve { f: f.clone() };
    let tsys = build_system(&f, k, w, None);
    let tmpl = tsys.cnf.cls.clone();
    let tnv = tsys.cnf.nv;
    let mut tmpl_set = HashSet::new();
    for c in &tmpl {
        let mut c = c.clone();
        c.sort();
        tmpl_set.insert(c);
    }
    let ctx = Ctx {
        f: f.clone(),
        k,
        w,
        tmpl,
        tnv,
        proj: tsys.pvars.iter().chain(tsys.tvars.iter()).cloned().collect(),
        rvars: tsys.rvars.clone(),
        tmpl_set,
        budget,
        tag: tag.clone(),
    };
    // target stream: x of uniformly random curve points (x != 0), seeded; then repeats
    let mut rng = Rng(seed_of(&format!("L3/targets/{}", tag)));
    let mut stream = Vec::new();
    while stream.len() < ntargets {
        let x = rng.next() & f.one();
        if cv.from_x(x).is_some() {
            stream.push(x);
        }
    }
    for i in 0..nrepeat {
        stream.push(stream[i]);
    }
    let tset: HashSet<u64> = stream.iter().cloned().collect();
    let gt0 = Instant::now();
    let truth = ground_truth(&f, k, w, &tset);
    let gt_ms = gt0.elapsed().as_secs_f64() * 1e3;
    eprintln!("[{tag}] template vars {} clauses {}; ground truth {:.0} ms", tnv, ctx.tmpl.len(), gt_ms);
    let mut armv: Vec<Arm> = arms
        .iter()
        .map(|&a| {
            let sh = if a == "BS" { Some(seed_of(&format!("L3/BS/{}", tag))) } else { None };
            Arm::new(a, &ctx, kflush, sh, scratch)
        })
        .collect();
    let mut fh = std::fs::File::create(out).unwrap();
    let mut header = String::new();
    write!(header, "{{\"type\":\"header\",\"tag\":\"{}\",\"m\":{},\"k\":{},\"w\":{},\"template_vars\":{},\"template_clauses\":{},\"ntargets\":{},\"nrepeat\":{},\"kflush\":{},\"budget\":{},\"solver\":\"CaDiCaL {} via cadical-sys 0.8.0, factor=0 factorcheck=0, else defaults\",\"ground_truth_ms\":{:.1}}}", tag, m, k, w, tnv, ctx.tmpl.len(), ntargets, nrepeat, kflush, budget, CaDiCal::version(), gt_ms).unwrap();
    writeln!(fh, "{}", header).unwrap();
    for (ti, &xr) in stream.iter().enumerate() {
        let tr = &truth[&xr];
        let ncurve = tr.iter().filter(|mdl| curve_verified(&cv, mdl, k, xr)).count();
        // interleave: rotate the arm order with the target index
        let na = armv.len();
        for off in 0..na {
            let ai = (ti + off) % na;
            let arm = &mut armv[ai];
            let rec = arm.solve_target(&ctx, xr, scratch);
            let matches = rec.censored || rec.models == *tr;
            if !matches {
                panic!("[{tag}] arm {} target {} model set differs from ground truth ({} vs {})", arm.name, ti, rec.models.len(), tr.len());
            }
            let mut line = String::new();
            write!(line, "{{\"type\":\"row\",\"tag\":\"{}\",\"arm\":\"{}\",\"pos\":{},\"repeat_of\":{},\"xr\":{},\"truth_models\":{},\"curve_verified_models\":{},\"models\":{},\"match_truth\":{},\"censored\":{},\"decomposable\":{},\"total\":{},\"first_call\":{},\"ncalls\":{},\"last_call\":{},\"wall_ms\":{:.3}",
                tag, arm.name, ti, if ti >= ntargets { (ti - ntargets).to_string() } else { "null".into() }, xr, tr.len(), ncurve, rec.models.len(), rec.models == *tr, rec.censored, !tr.is_empty(), jarr3(&rec.tot), jarr3(&rec.first), rec.calls.len(), jarr3(&rec.calls.last().unwrap().1), rec.wall_ms).unwrap();
            if let Some(l) = &rec.lstats {
                write!(line, ",\"learned\":{{\"n\":{},\"indep\":{},\"r_only\":{},\"act\":{},\"applic_sum\":{:.4},\"r_lits_hist\":{:?}}}", l.learned, l.indep, l.r_only, l.act, l.applic, l.rhist).unwrap();
            }
            if let Some(c) = rec.carried {
                write!(line, ",\"carried\":{}", c).unwrap();
            }
            line.push('}');
            writeln!(fh, "{}", line).unwrap();
        }
        fh.flush().unwrap();
    }
    eprintln!("[{tag}] done");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let get = |k: &str, d: &str| -> String {
        args.iter().position(|a| a == k).map(|i| args[i + 1].clone()).unwrap_or(d.to_string())
    };
    if args.iter().any(|a| a == "--selftest") {
        selftest();
        return;
    }
    let m: usize = get("--m", "11").parse().unwrap();
    let k: usize = get("--k", "2").parse().unwrap();
    let w: usize = get("--w", "2").parse().unwrap();
    let nt: usize = get("--targets", "40").parse().unwrap();
    let nrep: usize = get("--repeat", "5").parse().unwrap();
    let kf: usize = get("--kflush", "4").parse().unwrap();
    let budget: i32 = get("--budget", "2000000").parse().unwrap();
    let out = get("--out", "rows.jsonl");
    let scratch = get("--scratch", "/tmp");
    let armlist = get("--arms", "A0,AA,A1,B,BS,CI,CA,CK");
    let arms: Vec<&'static str> = armlist
        .split(',')
        .map(|a| match a {
            "A0" => "A0", "AA" => "AA", "A1" => "A1", "B" => "B", "BS" => "BS", "CI" => "CI", "CA" => "CA", "CK" => "CK",
            _ => panic!("unknown arm {a}"),
        })
        .collect();
    run(m, k, w, nt, nrep, kf, budget, &arms, &out, &scratch);
}
