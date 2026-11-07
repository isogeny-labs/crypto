// ecld_toy.rs : toy test of the elliptic-code list-decoding lead (slug ecld).
//
// Std-only Rust.  Build:  rustc -O -o <out>/ecld_toy ecld_toy.rs
// Modes:  check | bound | plan | panel
//
// Construction (Cheng, arXiv cs/0507026, Lemma 3).  E: y^2 = x^3 + a x + b over F_p of prime
// order N.  D = {P_1..P_B} a factor base (random subgroup points).  Target Q, x_Q not in x(D),
// y_Q != 0.  Code C = { ev_D(f) : f in L((k-1)O) }, dimension m = k-1, basis
// 1, x, y, x^2, xy, x^3, ... (pole orders 0,2,3,4,5,6,...).  Received word
// R_i = f'(P_i), f' = (y + y_Q)/(x - x_Q) in L(Q + O) \ L((k-1)O).
// A codeword ev(f) agrees with R on a set Z iff f' - f vanishes on Z; |Z| <= k, and |Z| = k
// iff the points of Z sum to Q.  So the codewords at agreement k are exactly the relations.
//
// Arms (all exact, all return the full set of k-subsets of D summing to Q):
//   E1  enumerate (k-1)-subsets incrementally, one lookup of Q - S in a hash of D
//   E2  meet in the middle: table of floor(k/2)-subset sums, enumerate ceil(k/2)-subsets
//   D1  exact list decoding at agreement k by information-set decoding: for each (k-1)-subset S
//       interpolate the unique codeword agreeing with R on S, scan the later positions
//   D0  (check mode only) brute force over all p^(k-1) codewords
//
// Counters: gae (priced, one affine group addition each).  Everything else is an unpriced
// native operation, listed with its count and priced at w in {0, 0.1, 1} GAE (AGENTS.md 5a).
use std::collections::{BTreeSet, HashMap};
use std::hash::{BuildHasherDefault, Hasher};
use std::time::Instant;

#[derive(Default)]
struct Fx(u64);
impl Hasher for Fx {
    fn finish(&self) -> u64 {
        self.0
    }
    fn write(&mut self, b: &[u8]) {
        for &x in b {
            self.0 = (self.0.rotate_left(5) ^ x as u64).wrapping_mul(0x517cc1b727220a95);
        }
    }
    fn write_u64(&mut self, v: u64) {
        self.0 = (self.0.rotate_left(5) ^ v).wrapping_mul(0x517cc1b727220a95);
    }
}
type FxMap<K, V> = HashMap<K, V, BuildHasherDefault<Fx>>;

type Pt = (u64, u64);
const INF: Pt = (u64::MAX, 0);

#[derive(Default, Clone, Copy, Debug)]
struct Cnt {
    gae: u64,
    fmul: u64,
    finv: u64,
    fadd: u64,
    tins: u64,
    tlook: u64,
    cmp: u64,
}
impl Cnt {
    fn native(&self) -> u64 {
        self.fmul + self.finv + self.fadd + self.tins + self.tlook + self.cmp
    }
    fn priced(&self, w: f64) -> f64 {
        self.gae as f64 + w * self.native() as f64
    }
    fn json(&self) -> String {
        format!(
            "{{\"gae\":{},\"fmul\":{},\"finv\":{},\"fadd\":{},\"tins\":{},\"tlook\":{},\"cmp\":{},\"native\":{}}}",
            self.gae, self.fmul, self.finv, self.fadd, self.tins, self.tlook, self.cmp, self.native()
        )
    }
}

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
fn derive(base: u64, tags: &[u64]) -> u64 {
    let mut r = Rng(base ^ 0xA5A5_5A5A_1234_5678);
    let mut s = r.next();
    for &t in tags {
        r = Rng(s ^ t.wrapping_mul(0x9E3779B97F4A7C15));
        s = r.next();
    }
    s
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}

fn modinv(a: u64, p: u64) -> u64 {
    let (mut t, mut nt) = (0i64, 1i64);
    let (mut r, mut nr) = (p as i64, a as i64);
    while nr != 0 {
        let q = r / nr;
        let tmp = t - q * nt;
        t = nt;
        nt = tmp;
        let tmp = r - q * nr;
        r = nr;
        nr = tmp;
    }
    if t < 0 {
        t += p as i64;
    }
    t as u64
}

struct Curve {
    p: u64,
    a: u64,
    b: u64,
    n: u64, // prime group order
    g: Pt,
}
impl Curve {
    fn add(&self, c: &mut Cnt, p1: Pt, p2: Pt) -> Pt {
        c.gae += 1;
        self.add_raw(p1, p2)
    }
    fn add_raw(&self, p1: Pt, p2: Pt) -> Pt {
        let p = self.p;
        if p1 == INF {
            return p2;
        }
        if p2 == INF {
            return p1;
        }
        let lam;
        if p1.0 == p2.0 {
            if (p1.1 + p2.1) % p == 0 {
                return INF;
            }
            let num = (3 * p1.0 % p * p1.0 + self.a) % p;
            lam = num * modinv(2 * p1.1 % p, p) % p;
        } else {
            let num = (p2.1 + p - p1.1) % p;
            let den = (p2.0 + p - p1.0) % p;
            lam = num * modinv(den, p) % p;
        }
        let x3 = (lam * lam % p + 2 * p - p1.0 - p2.0) % p;
        let y3 = (lam * ((p1.0 + p - x3) % p) % p + p - p1.1) % p;
        (x3, y3)
    }
    fn neg(&self, a: Pt) -> Pt {
        if a == INF {
            INF
        } else {
            (a.0, (self.p - a.1) % self.p)
        }
    }
    fn mul_raw(&self, mut k: u64, pt: Pt) -> Pt {
        let mut acc = INF;
        let mut base = pt;
        while k > 0 {
            if k & 1 == 1 {
                acc = self.add_raw(acc, base);
            }
            base = self.add_raw(base, base);
            k >>= 1;
        }
        acc
    }
    fn on_curve(&self, a: Pt) -> bool {
        let p = self.p;
        a.1 * a.1 % p == (a.0 * a.0 % p * a.0 + self.a * a.0 + self.b) % p
    }
}

/// Deterministic prime p in [lo, hi) from a seed, then a curve of prime order.
fn make_curve(seed: u64, lo: u64, hi: u64) -> Curve {
    let mut r = Rng(seed);
    let mut p;
    loop {
        p = lo + r.below(hi - lo);
        if p > 5 && is_prime(p) {
            break;
        }
    }
    let mut sqrt = vec![u64::MAX; p as usize];
    let mut chi = vec![0i8; p as usize];
    for y in 0..p {
        let s = (y * y % p) as usize;
        if sqrt[s] == u64::MAX {
            sqrt[s] = y;
        }
        chi[s] = 1;
    }
    loop {
        let a = r.below(p);
        let b = r.below(p);
        if (4 * a % p * a % p * a % p + 27 * b % p * b % p) % p == 0 {
            continue;
        }
        let mut n = 1u64;
        for x in 0..p {
            let v = ((x * x % p + a) % p * x % p + b) % p;
            n += if v == 0 { 1 } else if chi[v as usize] == 1 { 2 } else { 0 };
        }
        if n == p || n < 50 || !is_prime(n) {
            continue;
        }
        // generator: first point found from a random x
        loop {
            let x = r.below(p);
            let v = ((x * x % p + a) % p * x % p + b) % p;
            if sqrt[v as usize] != u64::MAX && v != 0 {
                return Curve { p, a, b, n, g: (x, sqrt[v as usize]) };
            }
        }
    }
}

struct Inst {
    d: Vec<Pt>,
    q: Pt,
    planted: Option<Vec<usize>>,
}

fn make_inst(c: &Curve, seed: u64, b: usize, k: usize, plant: bool) -> Inst {
    let mut r = Rng(seed);
    let mut d: Vec<Pt> = Vec::new();
    while d.len() < b {
        let e = 1 + r.below(c.n - 1);
        let pt = c.mul_raw(e, c.g);
        if pt != INF && !d.contains(&pt) {
            d.push(pt);
        }
    }
    let mut planted = None;
    let q;
    loop {
        let cand = if plant {
            let mut idx: Vec<usize> = Vec::new();
            while idx.len() < k {
                let i = r.below(b as u64) as usize;
                if !idx.contains(&i) {
                    idx.push(i);
                }
            }
            idx.sort();
            let mut s = INF;
            for &i in &idx {
                s = c.add_raw(s, d[i]);
            }
            planted = Some(idx);
            s
        } else {
            c.mul_raw(1 + r.below(c.n - 1), c.g)
        };
        if cand != INF && cand.1 != 0 && !d.iter().any(|t| t.0 == cand.0) {
            q = cand;
            break;
        }
        if !plant {
            continue;
        }
        // planted sum collided with x(D) (probability ~ B/p): redraw whole set
        d.clear();
        while d.len() < b {
            let e = 1 + r.below(c.n - 1);
            let pt = c.mul_raw(e, c.g);
            if pt != INF && !d.contains(&pt) {
                d.push(pt);
            }
        }
    }
    Inst { d, q, planted }
}

fn key(a: Pt) -> u64 {
    (a.0 << 32) | a.1
}

// ---------------------------------------------------------------- exact checker
fn verify(c: &Curve, inst: &Inst, set: &[usize]) -> bool {
    let mut s = INF;
    for (j, &i) in set.iter().enumerate() {
        if j > 0 && set[j - 1] >= i {
            return false;
        }
        s = c.add_raw(s, inst.d[i]);
    }
    s == inst.q
}

fn set_hash(sets: &BTreeSet<Vec<usize>>) -> u64 {
    let mut h = 0xcbf29ce484222325u64;
    for s in sets {
        for &i in s {
            h = (h ^ (i as u64 + 1)).wrapping_mul(0x100000001b3);
        }
        h = (h ^ 0xFFFF).wrapping_mul(0x100000001b3);
    }
    h
}

// ---------------------------------------------------------------- E1
struct E1<'a> {
    c: &'a Curve,
    d: &'a [Pt],
    q: Pt,
    m: usize,
    map: &'a FxMap<u64, usize>,
    cnt: Cnt,
    out: BTreeSet<Vec<usize>>,
    stack: Vec<usize>,
}
impl<'a> E1<'a> {
    fn rec(&mut self, start: usize, part: Pt) {
        let depth = self.stack.len();
        if depth == self.m {
            let t = self.c.add(&mut self.cnt, self.q, self.c.neg(part));
            if t != INF {
                self.cnt.tlook += 1;
                if let Some(&j) = self.map.get(&key(t)) {
                    if j > *self.stack.last().unwrap() {
                        let mut s = self.stack.clone();
                        s.push(j);
                        self.out.insert(s);
                    }
                }
            }
            return;
        }
        for i in start..self.d.len() {
            let np = if depth == 0 { self.d[i] } else { self.c.add(&mut self.cnt, part, self.d[i]) };
            self.stack.push(i);
            self.rec(i + 1, np);
            self.stack.pop();
        }
    }
}
fn run_e1(c: &Curve, inst: &Inst, k: usize) -> (BTreeSet<Vec<usize>>, Cnt) {
    let mut map: FxMap<u64, usize> = FxMap::default();
    let mut cnt = Cnt::default();
    for (i, &pt) in inst.d.iter().enumerate() {
        map.insert(key(pt), i);
        cnt.tins += 1;
    }
    let mut e = E1 { c, d: &inst.d, q: inst.q, m: k - 1, map: &map, cnt, out: BTreeSet::new(), stack: vec![] };
    e.rec(0, INF);
    (e.out, e.cnt)
}

// ---------------------------------------------------------------- E2 (meet in the middle)
fn subsets_sums(c: &Curve, d: &[Pt], h: usize, cnt: &mut Cnt, f: &mut dyn FnMut(&[usize], Pt, &mut Cnt)) {
    fn rec(c: &Curve, d: &[Pt], h: usize, start: usize, st: &mut Vec<usize>, part: Pt, cnt: &mut Cnt, f: &mut dyn FnMut(&[usize], Pt, &mut Cnt)) {
        if st.len() == h {
            f(st, part, cnt);
            return;
        }
        for i in start..d.len() {
            let np = if st.is_empty() { d[i] } else { c.add(cnt, part, d[i]) };
            st.push(i);
            rec(c, d, h, i + 1, st, np, cnt, f);
            st.pop();
        }
    }
    let mut st = Vec::new();
    rec(c, d, h, 0, &mut st, INF, cnt, f);
}
fn pack(s: &[usize]) -> u32 {
    let mut v = 0u32;
    for &i in s {
        v = (v << 9) | i as u32;
    }
    v
}
fn unpack(mut v: u32, h: usize) -> Vec<usize> {
    let mut o = vec![0; h];
    for j in (0..h).rev() {
        o[j] = (v & 511) as usize;
        v >>= 9;
    }
    o
}
fn run_e2(c: &Curve, inst: &Inst, k: usize) -> (BTreeSet<Vec<usize>>, Cnt) {
    assert!(inst.d.len() <= 512);
    let h = k / 2;
    let l = k - h;
    let mut cnt = Cnt::default();
    let mut head: FxMap<u64, u32> = FxMap::default();
    let mut ent: Vec<(u32, u32)> = Vec::new();
    subsets_sums(c, &inst.d, h, &mut cnt, &mut |s, sum, cn| {
        cn.tins += 1;
        if sum == INF {
            return;
        }
        let id = ent.len() as u32;
        let nxt = head.insert(key(sum), id).unwrap_or(u32::MAX);
        ent.push((pack(s), nxt));
    });
    let mut out = BTreeSet::new();
    let q = inst.q;
    subsets_sums(c, &inst.d, l, &mut cnt, &mut |s, sum, cn| {
        let t = c.add(cn, q, c.neg(sum));
        if t == INF {
            return;
        }
        cn.tlook += 1;
        if let Some(&first) = head.get(&key(t)) {
            let mut id = first;
            while id != u32::MAX {
                let (pk, nx) = ent[id as usize];
                let hs = unpack(pk, h);
                if !hs.iter().any(|x| s.contains(x)) {
                    let mut u: Vec<usize> = hs;
                    u.extend_from_slice(s);
                    u.sort();
                    out.insert(u);
                }
                id = nx;
            }
        }
    });
    (out, cnt)
}

// ---------------------------------------------------------------- code construction + D1
struct Code {
    m: usize,
    phi: Vec<Vec<u64>>, // phi[i][j] = basis j at P_i
    r: Vec<u64>,        // received word
    setup: Cnt,
}
fn basis_at(p: u64, x: u64, y: u64, j: usize, cnt: &mut Cnt) -> u64 {
    if j == 0 {
        return 1;
    }
    let o = j + 1; // pole order
    let (e, uy) = if o % 2 == 0 { (o / 2, false) } else { ((o - 3) / 2, true) };
    let mut v = 1u64;
    for _ in 0..e {
        v = v * x % p;
        cnt.fmul += 1;
    }
    if uy {
        v = v * y % p;
        cnt.fmul += 1;
    }
    v
}
fn build_code(c: &Curve, inst: &Inst, k: usize) -> Code {
    let p = c.p;
    let m = k - 1;
    let mut setup = Cnt::default();
    let mut phi = Vec::new();
    let mut r = Vec::new();
    for &(x, y) in &inst.d {
        phi.push((0..m).map(|j| basis_at(p, x, y, j, &mut setup)).collect());
        // f' = (y + y_Q) / (x - x_Q)
        let den = (x + p - inst.q.0) % p;
        r.push((y + inst.q.1) % p * modinv(den, p) % p);
        setup.finv += 1;
        setup.fmul += 1;
    }
    Code { m, phi, r, setup }
}

/// Solve M c = v (m x m) by Gauss-Jordan; returns None if singular. Counts native ops.
fn solve(p: u64, mat: &mut [[u64; 9]; 8], m: usize, cn: &mut Cnt) -> Option<[u64; 8]> {
    for col in 0..m {
        let mut piv = None;
        for r in col..m {
            cn.cmp += 1;
            if mat[r][col] != 0 {
                piv = Some(r);
                break;
            }
        }
        let pr = piv?;
        mat.swap(col, pr);
        let inv = modinv(mat[col][col], p);
        cn.finv += 1;
        for c2 in col..=m {
            mat[col][c2] = mat[col][c2] * inv % p;
            cn.fmul += 1;
        }
        for r in 0..m {
            if r != col && mat[r][col] != 0 {
                let f = mat[r][col];
                for c2 in col..=m {
                    mat[r][c2] = (mat[r][c2] + p - f * mat[col][c2] % p) % p;
                    cn.fmul += 1;
                    cn.fadd += 1;
                }
            }
        }
    }
    let mut out = [0u64; 8];
    for r in 0..m {
        out[r] = mat[r][m];
    }
    Some(out)
}

fn run_d1(c: &Curve, code: &Code, b: usize) -> (BTreeSet<Vec<usize>>, Cnt, u64) {
    let p = c.p;
    let m = code.m;
    let mut cnt = code.setup; // setup (basis tables, received word) is charged to the arm
    let mut out = BTreeSet::new();
    let mut singular = 0u64;
    let mut s: Vec<usize> = Vec::with_capacity(m);
    // iterative enumeration of m-subsets
    let mut idx: Vec<usize> = (0..m).collect();
    if m > b {
        return (out, cnt, 0);
    }
    loop {
        s.clear();
        s.extend_from_slice(&idx);
        let mut mat = [[0u64; 9]; 8];
        for (r, &i) in s.iter().enumerate() {
            for j in 0..m {
                mat[r][j] = code.phi[i][j];
            }
            mat[r][m] = code.r[i];
        }
        match solve(p, &mut mat, m, &mut cnt) {
            None => singular += 1,
            Some(co) => {
                let last = s[m - 1];
                for j in (last + 1)..b {
                    let mut acc = 0u64;
                    for t in 0..m {
                        acc += co[t] * code.phi[j][t];
                    }
                    cnt.fmul += m as u64;
                    cnt.fadd += m as u64;
                    cnt.cmp += 1;
                    if acc % p == code.r[j] {
                        let mut u = s.clone();
                        u.push(j);
                        out.insert(u);
                    }
                }
            }
        }
        // next combination
        let mut i = m;
        loop {
            if i == 0 {
                return (out, cnt, singular);
            }
            i -= 1;
            if idx[i] != i + b - m {
                break;
            }
            if i == 0 {
                return (out, cnt, singular);
            }
        }
        idx[i] += 1;
        for j in (i + 1)..m {
            idx[j] = idx[j - 1] + 1;
        }
    }
}

// ---------------------------------------------------------------- D0 brute force over the code
/// Histogram of agreement counts over all p^m codewords; returns (hist, sets with agreement>=k)
fn run_d0(c: &Curve, code: &Code, b: usize, k: usize) -> (Vec<u64>, BTreeSet<Vec<usize>>) {
    let p = c.p;
    let m = code.m;
    let mut hist = vec![0u64; b + 1];
    let mut sets = BTreeSet::new();
    let total = p.pow(m as u32);
    let mut co = vec![0u64; m];
    for mut w in 0..total {
        for t in 0..m {
            co[t] = w % p;
            w /= p;
        }
        let mut ag = 0usize;
        let mut pos = Vec::new();
        for i in 0..b {
            let mut acc = 0u64;
            for t in 0..m {
                acc += co[t] * code.phi[i][t];
            }
            if acc % p == code.r[i] {
                ag += 1;
                if ag <= k {
                    pos.push(i);
                }
            }
        }
        hist[ag] += 1;
        if ag >= k {
            sets.insert(pos);
        }
    }
    (hist, sets)
}

// ---------------------------------------------------------------- bounds
fn lg_binom(n: usize, r: usize) -> f64 {
    let mut s = 0.0;
    for i in 0..r {
        s += ((n - i) as f64).log2() - ((i + 1) as f64).log2();
    }
    s
}
fn binom_f(n: usize, r: usize) -> f64 {
    lg_binom(n, r).exp2()
}
fn t_gs(b: usize, m: usize) -> usize {
    ((b * m) as f64).sqrt().floor() as usize + 1
}

fn cmd_bound() {
    println!("# GS agreement radius t_GS(m) = floor(sqrt(B m)) + 1 for C_L(D, mO) on genus 1 (cited bound, not re-derived here)");
    println!("# Structured fact: for received word f' with poles Q + O-part, any f' - f has <= m+1 zeros (Cheng Lemma 3 with m = k-1), so agreement <= m+1 = k.");
    println!("# Random-word model: expected number of codewords with agreement >= t is C(B,t) p^(m-t); log2 shown.");
    for &(b, lp) in &[(64usize, 16.0f64), (128, 16.0), (256, 16.0), (256, 20.0)] {
        println!("\nB={} log2(p)={}", b, lp);
        let mut best = (f64::NEG_INFINITY, 0usize, 0usize);
        let mut first_cheng = None;
        let mut first_rand = None;
        for m in 1..b {
            let t = t_gs(b, m);
            if t <= b {
                let l2 = lg_binom(b, t) + (m as f64 - t as f64) * lp;
                if l2 > best.0 {
                    best = (l2, m, t);
                }
                if l2 >= 0.0 && first_rand.is_none() {
                    first_rand = Some((m, t, l2));
                }
            }
            if first_cheng.is_none() && (m + 1) as f64 >= ((b * m) as f64).sqrt() {
                first_cheng = Some(m);
            }
        }
        println!("  max over m of log2 E[list at GS radius] = {:.1} at m={} t={}", best.0, best.1, best.2);
        match first_rand {
            Some((m, t, l)) => println!("  first m with E>=1 in random model: m={} t={} log2E={:.1}", m, t, l),
            None => println!("  no m<B with E>=1 at the GS radius in the random model"),
        }
        match first_cheng {
            Some(m) => println!("  Cheng word (agreement <= m+1): GS radius <= m+1 first possible at m={} (k={}), i.e. k >= ~B-1", m, m + 1),
            None => println!("  Cheng word: never"),
        }
        for k in 2..=6usize {
            let m = k - 1;
            println!("  k={}: t_GS={} > k={}  (GS list on Cheng word provably empty: {})", k, t_gs(b, m), k, t_gs(b, m) > k);
        }
    }
}

// ---------------------------------------------------------------- plan (cost estimate)
fn cells_full() -> Vec<(usize, usize, usize, usize)> {
    // (k, B, curves, targets per curve incl. planted)
    let mut v = vec![];
    for &k in &[2usize, 3, 4, 5] {
        for &b in &[64usize, 128, 256] {
            let curves = if k == 5 && b == 256 { 2 } else { 4 };
            v.push((k, b, curves, 4));
        }
    }
    for &b in &[32usize, 48, 64] {
        v.push((6, b, 4, 4));
    }
    v
}
fn cmd_plan() {
    println!("cell k B curves targets | E1 gae | E2 gae | D1 native(mul-ish) | E[relations] per random target");
    let mut tot_e1 = 0.0;
    let mut tot_e2 = 0.0;
    let mut tot_d1 = 0.0;
    for (k, b, cu, ta) in cells_full() {
        let m = k - 1;
        let e1 = binom_f(b, m) + (2..=m).map(|d| binom_f(b, d)).sum::<f64>();
        let (h, l) = (k / 2, k - k / 2);
        let e2 = (if h >= 2 { binom_f(b, h) } else { 0.0 }) + (if l >= 2 { binom_f(b, l) } else { 0.0 }) + binom_f(b, l);
        let scan = (b - m) as f64 / (m as f64 + 1.0) * (2.0 * m as f64 + 1.0);
        let d1 = binom_f(b, m) * (scan + (m * m * (m + 1)) as f64 + 3.0 * m as f64);
        let n = (cu * ta) as f64;
        println!("{} {} {} {} | {:.3e} | {:.3e} | {:.3e} | {:.2e}", k, b, cu, ta, e1, e2, d1, binom_f(b, k) / 65000.0);
        tot_e1 += e1 * n;
        tot_e2 += e2 * n;
        tot_d1 += d1 * n;
    }
    println!("totals over panel: E1 gae {:.3e}, E2 gae {:.3e}, D1 native {:.3e}", tot_e1, tot_e2, tot_d1);
    println!("(plus k=6 B=256 anchor: E2 only, {:.3e} gae per instance)", binom_f(256, 3) * 3.0);
}

// ---------------------------------------------------------------- check (development)
fn cmd_check() {
    println!("# DEVELOPMENT CHECK ONLY (tiny cases, p in [200,1100)); not a measurement  [pcc:unverified]");
    let mut ok = true;
    for (ci, &(k, b)) in [(2usize, 40usize), (3, 36), (4, 30)].iter().enumerate() {
        let lo = if k == 4 { 200 } else { 700 };
        let hi = if k == 4 { 260 } else { 1100 };
        let c = make_curve(derive(7, &[ci as u64]), lo, hi);
        assert!(c.on_curve(c.g));
        assert_eq!(c.mul_raw(c.n, c.g), INF);
        println!("\nk={} B={} p={} N={} (prime order, G*N=O verified)", k, b, c.p, c.n);
        for plant in [false, true] {
            let inst = make_inst(&c, derive(11, &[ci as u64, plant as u64]), b, k, plant);
            let code = build_code(&c, &inst, k);
            // basis points: every phi value finite, R matches definition
            let (e1, c1) = run_e1(&c, &inst, k);
            let (e2, c2) = run_e2(&c, &inst, k);
            let (d1, c3, sing) = run_d1(&c, &code, b);
            let t0 = Instant::now();
            let (hist, d0) = run_d0(&c, &code, b, k);
            let maxag = (0..=b).rev().find(|&t| hist[t] > 0).unwrap();
            let all_ver = e1.iter().all(|s| verify(&c, &inst, s));
            let agree = e1 == e2 && e1 == d1 && e1 == d0;
            let pl = inst.planted.as_ref().map(|s| e1.contains(s)).unwrap_or(true);
            println!(
                "  plant={} relations={} E1==E2==D1==D0:{} all_verified:{} planted_recovered:{} max_agreement_over_code={} (<=k:{}) singular_S={} d0_time={:?}",
                plant, e1.len(), agree, all_ver, pl, maxag, maxag <= k, sing, t0.elapsed()
            );
            println!(
                "    gae: E1={} E2={} D1={} | D1 native={} (fmul {} finv {} cmp {})",
                c1.gae, c2.gae, c3.gae, c3.native(), c3.fmul, c3.finv, c3.cmp
            );
            let h: Vec<String> = (0..=k.min(b)).map(|t| format!("{}:{}", t, hist[t])).collect();
            println!("    agreement histogram (agreement:codewords) {}", h.join(" "));
            ok &= agree && all_ver && pl && maxag <= k && !e1.is_empty() == (plant || !e1.is_empty());
            if plant {
                ok &= !e1.is_empty();
            }
        }
    }
    // GS radius vs this size
    for k in 2..=6 {
        println!("t_GS(B=36,k={})={} > k", k, t_gs(36, k - 1));
    }
    println!("\nCHECK {}", if ok { "PASS" } else { "FAIL" });
    if !ok {
        std::process::exit(1);
    }
}

// ---------------------------------------------------------------- panel
fn cmd_panel(args: &[String]) {
    let tier = args.iter().position(|a| a == "--tier").map(|i| args[i + 1].clone()).unwrap_or("smoke".into());
    let master: u64 = args.iter().position(|a| a == "--seed").map(|i| args[i + 1].parse().unwrap()).unwrap_or(20261107);
    let only: Option<(usize, usize)> = args.iter().position(|a| a == "--cell").map(|i| {
        let v: Vec<usize> = args[i + 1].split(',').map(|x| x.parse().unwrap()).collect();
        (v[0], v[1])
    });
    let (lo, hi, cells): (u64, u64, Vec<(usize, usize, usize, usize)>) = if tier == "full" {
        (61000, 65521, cells_full())
    } else if tier == "calib" {
        (61000, 65521, vec![(4, 48, 1, 2)])
    } else {
        (1000, 1100, vec![(2, 24, 1, 2), (3, 24, 1, 2), (4, 20, 1, 2), (5, 16, 1, 2), (6, 14, 1, 2)])
    };
    for (k, b, curves, targets) in cells {
        if let Some((ok_, ob)) = only {
            if ok_ != k || ob != b {
                continue;
            }
        }
        for ci in 0..curves {
            let c = make_curve(derive(master, &[1, k as u64, b as u64, ci as u64]), lo, hi);
            for ti in 0..targets {
                let plant = ti == targets - 1; // last target of each curve is the planted positive control
                let inst = make_inst(&c, derive(master, &[2, k as u64, b as u64, ci as u64, ti as u64]), b, k, plant);
                // arm order interleaved by (ci+ti) parity per frozen protocol
                let order: Vec<&str> = if (ci + ti) % 2 == 0 { vec!["E1", "E2", "D1"] } else { vec!["D1", "E2", "E1"] };
                let mut res: Vec<(String, BTreeSet<Vec<usize>>, Cnt, f64, u64)> = vec![];
                for arm in order {
                    let t0 = Instant::now();
                    let (set, cnt, sing) = match arm {
                        "E1" => {
                            let (s, c1) = run_e1(&c, &inst, k);
                            (s, c1, 0)
                        }
                        "E2" => {
                            let (s, c1) = run_e2(&c, &inst, k);
                            (s, c1, 0)
                        }
                        _ => {
                            let code = build_code(&c, &inst, k);
                            run_d1(&c, &code, b)
                        }
                    };
                    res.push((arm.to_string(), set, cnt, t0.elapsed().as_secs_f64(), sing));
                }
                let hashes: Vec<u64> = res.iter().map(|r| set_hash(&r.1)).collect();
                let consistent = hashes.iter().all(|&h| h == hashes[0]);
                for (arm, set, cnt, secs, sing) in &res {
                    let ver = set.iter().filter(|s| verify(&c, &inst, s)).count();
                    let planted_rec = inst.planted.as_ref().map(|s| set.contains(s));
                    println!(
                        "{{\"k\":{},\"B\":{},\"p\":{},\"N\":{},\"curve\":{},\"target\":{},\"planted\":{},\"arm\":\"{}\",\"relations\":{},\"verified\":{},\"planted_recovered\":{},\"sets_consistent_across_arms\":{},\"set_hash\":{},\"singular_S\":{},\"t_gs\":{},\"counters\":{},\"priced_w0\":{:.1},\"priced_w0.1\":{:.1},\"priced_w1\":{:.1},\"wall_s\":{:.4}}}",
                        k, b, c.p, c.n, ci, ti, inst.planted.is_some(), arm, set.len(), ver,
                        match planted_rec { Some(v) => v.to_string(), None => "null".into() },
                        consistent, set_hash(set), sing, t_gs(b, k - 1), cnt.json(),
                        cnt.priced(0.0), cnt.priced(0.1), cnt.priced(1.0), secs
                    );
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("check") => cmd_check(),
        Some("bound") => cmd_bound(),
        Some("plan") => cmd_plan(),
        Some("panel") => cmd_panel(&args[2..]),
        _ => eprintln!("usage: ecld_toy check|bound|plan|panel [--tier smoke|full] [--cell k,B] [--seed S]"),
    }
}
