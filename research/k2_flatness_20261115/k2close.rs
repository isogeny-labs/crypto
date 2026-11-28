//! K2 flatness test: native GF(2) degree-bounded closure of Weil-descended binary
//! point-decomposition systems (LEAD-KOB-flat-degree-crossover-bar, research/leads_20261112).
//!
//! Standalone (no crate dependency, so concurrent edits elsewhere in `src/` cannot break it):
//!   rustc -C opt-level=3 -C target-cpu=native -o k2close k2close.rs
//!
//! System (m = 3): S_4(x1, x2, x3, xR) = 0, x_i = sum_j v_{i,j} e_j over a k-dimensional
//! F_2-subspace V = <e_1..e_k> of F_{2^n}; Weil restriction to n Boolean equations in N = 3k
//! variables (the same resultant formula as `pq_descent_symbolic::semaev_s4_word`).
//! System (m = 4, split presentation): S_3(x1, x2, U) = 0 and S_4(U, x3, x4, xR) = 0, with U a
//! free element of F_{2^n} (n more Boolean variables); 2n equations in N = 4k + n variables,
//! every equation of Boolean degree <= 3.
//!
//! Closure W_D: the smallest F_2-subspace of multilinear polynomials of degree <= D containing
//! the equations and closed under f -> x_i f whenever deg f <= D - 1 (Boolean ring, x_i^2 = x_i).
//! "Solved at D" means W_D equals the degree-<=D part of the vanishing ideal of the solution set
//! over F_2 (found by enumeration): rank W_D = M(N, <= D) - rank(evaluation vectors of solutions).
//! For an inconsistent system that is 1 in W_D.
//!
//! Output: one JSON line per (instance, target, D) on stdout.

use std::collections::HashMap;
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
}

// ---------------------------------------------------------------- GF(2^n)
fn pmod(mut a: u128, f: u128) -> u128 {
    let df = 127 - f.leading_zeros();
    while a != 0 && 127 - a.leading_zeros() >= df {
        a ^= f << (127 - a.leading_zeros() - df);
    }
    a
}
fn pmulmod(a: u128, b: u128, f: u128) -> u128 {
    let mut r = 0u128;
    let mut a = a;
    let mut b = b;
    while b != 0 {
        if b & 1 == 1 {
            r ^= a;
        }
        b >>= 1;
        a = pmod(a << 1, f);
    }
    pmod(r, f)
}
fn pgcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let r = pmod(a, b);
        a = b;
        b = r;
    }
    a
}
/// Ben-Or irreducibility test.
fn irreducible(f: u128, n: u32) -> bool {
    let mut xp = 2u128; // x
    for _ in 1..=n / 2 {
        xp = pmulmod(xp, xp, f); // x^(2^i)
        if pgcd(f, xp ^ 2) != 1 {
            return false;
        }
    }
    true
}
/// Lowest-weight irreducible: trinomial, else pentanomial (lexicographically first).
fn find_irreducible(n: u32) -> u128 {
    let top = 1u128 << n;
    for a in 1..n {
        let f = top | (1 << a) | 1;
        if irreducible(f, n) {
            return f;
        }
    }
    for a in 1..n {
        for b in (a + 1)..n {
            for c in (b + 1)..n {
                let f = top | (1 << a) | (1 << b) | (1 << c) | 1;
                if irreducible(f, n) {
                    return f;
                }
            }
        }
    }
    panic!("no irreducible")
}

#[derive(Clone)]
struct Gf {
    n: u32,
    f: u128,
}
impl Gf {
    fn mul(&self, a: u64, b: u64) -> u64 {
        let mut r = 0u128;
        let mut bb = b;
        let mut sh = 0;
        while bb != 0 {
            if bb & 1 == 1 {
                r ^= (a as u128) << sh;
            }
            bb >>= 1;
            sh += 1;
        }
        pmod(r, self.f) as u64
    }
    fn sqr(&self, a: u64) -> u64 {
        self.mul(a, a)
    }
    fn pow(&self, a: u64, mut e: u128) -> u64 {
        let mut r = 1u64;
        let mut b = a;
        while e != 0 {
            if e & 1 == 1 {
                r = self.mul(r, b);
            }
            b = self.sqr(b);
            e >>= 1;
        }
        r
    }
    fn inv(&self, a: u64) -> u64 {
        self.pow(a, (1u128 << self.n) - 2)
    }
    fn trace(&self, a: u64) -> u64 {
        let mut t = a;
        let mut s = a;
        for _ in 1..self.n {
            s = self.sqr(s);
            t ^= s;
        }
        t & 1
    }
    fn mask(&self) -> u64 {
        (1u64 << self.n) - 1
    }
}

// S3(x1,x2,X) = A X^2 + B X + C, A = (x1+x2)^2, B = x1 x2, C = (x1 x2)^2 + b.
fn s3w(g: &Gf, b: u64, x1: u64, x2: u64, x3: u64) -> u64 {
    let a = g.sqr(x1 ^ x2);
    let bb = g.mul(x1, x2);
    g.mul(a, g.sqr(x3)) ^ g.mul(bb, x3) ^ g.sqr(bb) ^ b
}
fn s4w(g: &Gf, b: u64, x1: u64, x2: u64, x3: u64, x4: u64) -> u64 {
    let f = |p: u64, q: u64| {
        let bb = g.mul(p, q);
        (g.sqr(p ^ q), bb, g.sqr(bb) ^ b)
    };
    let (a1, b1, c1) = f(x1, x2);
    let (a2, b2, c2) = f(x3, x4);
    let t1 = g.sqr(g.mul(a1, c2) ^ g.mul(a2, c1));
    let t2 = g.mul(g.mul(a1, b2) ^ g.mul(a2, b1), g.mul(b1, c2) ^ g.mul(b2, c1));
    t1 ^ t2
}

// ---------------------------------------------------------------- F_{2^n}[v]/(v^2-v)
#[derive(Clone, Default)]
struct FP(HashMap<u64, u64>);
impl FP {
    fn constant(c: u64) -> FP {
        let mut p = FP::default();
        if c != 0 {
            p.0.insert(0, c);
        }
        p
    }
    fn linear(off: u32, basis: &[u64]) -> FP {
        let mut p = FP::default();
        for (j, &e) in basis.iter().enumerate() {
            if e != 0 {
                p.0.insert(1u64 << (off + j as u32), e);
            }
        }
        p
    }
    fn addt(&mut self, m: u64, c: u64) {
        if c == 0 {
            return;
        }
        let e = self.0.entry(m).or_insert(0);
        *e ^= c;
        if *e == 0 {
            self.0.remove(&m);
        }
    }
    fn add(&self, o: &FP) -> FP {
        let mut r = self.clone();
        for (&m, &c) in &o.0 {
            r.addt(m, c);
        }
        r
    }
    fn mul(&self, o: &FP, g: &Gf) -> FP {
        let mut r = FP::default();
        for (&ma, &ca) in &self.0 {
            for (&mb, &cb) in &o.0 {
                r.addt(ma | mb, g.mul(ca, cb));
            }
        }
        r
    }
    fn sqr(&self, g: &Gf) -> FP {
        FP(self.0.iter().map(|(&m, &c)| (m, g.sqr(c))).collect())
    }
    fn eval(&self, pt: u64) -> u64 {
        self.0.iter().filter(|(&m, _)| pt & m == m).fold(0, |a, (_, &c)| a ^ c)
    }
    /// Boolean coordinate polynomials: equation j = monomials whose coefficient has bit j.
    fn split(&self, n: u32) -> Vec<Vec<u64>> {
        let mut out = vec![Vec::new(); n as usize];
        for (&m, &c) in &self.0 {
            for j in 0..n {
                if (c >> j) & 1 == 1 {
                    out[j as usize].push(m);
                }
            }
        }
        out
    }
}
fn s3_sym(g: &Gf, b: u64, x1: &FP, x2: &FP, x3: &FP) -> FP {
    let a = x1.add(x2).sqr(g);
    let bb = x1.mul(x2, g);
    a.mul(&x3.sqr(g), g).add(&bb.mul(x3, g)).add(&bb.sqr(g)).add(&FP::constant(b))
}
fn s4_sym(g: &Gf, b: u64, x1: &FP, x2: &FP, x3: &FP, x4: &FP) -> FP {
    let bc = FP::constant(b);
    let f = |p: &FP, q: &FP| {
        let bb = p.mul(q, g);
        (p.add(q).sqr(g), bb.clone(), bb.sqr(g).add(&bc))
    };
    let (a1, b1, c1) = f(x1, x2);
    let (a2, b2, c2) = f(x3, x4);
    let t1 = a1.mul(&c2, g).add(&a2.mul(&c1, g)).sqr(g);
    let l = a1.mul(&b2, g).add(&a2.mul(&b1, g));
    let r = b1.mul(&c2, g).add(&b2.mul(&c1, g));
    t1.add(&l.mul(&r, g))
}

// ---------------------------------------------------------------- systems
struct System {
    nvars: u32,
    eqs: Vec<Vec<u64>>, // each a list of monomial masks
    sols: Vec<u64>,     // all F_2 solutions (masks), by enumeration
}

fn span(basis: &[u64]) -> Vec<u64> {
    let mut s = vec![0u64; 1 << basis.len()];
    for i in 1..s.len() {
        let j = i.trailing_zeros() as usize;
        s[i] = s[i & (i - 1)] ^ basis[j];
    }
    s
}

fn build_m3(g: &Gf, b: u64, xr: u64, v: &[u64], rng: &mut Rng) -> System {
    let k = v.len() as u32;
    let x1 = FP::linear(0, v);
    let x2 = FP::linear(k, v);
    let x3 = FP::linear(2 * k, v);
    let p = s4_sym(g, b, &x1, &x2, &x3, &FP::constant(xr));
    // check the symbolic expansion against the word formula on random points
    let sp = span(v);
    let km = (1u64 << k) - 1;
    for _ in 0..64 {
        let pt = rng.next() & ((1u64 << (3 * k)) - 1);
        let (a, bb, c) = (sp[(pt & km) as usize], sp[((pt >> k) & km) as usize], sp[((pt >> (2 * k)) & km) as usize]);
        assert_eq!(p.eval(pt), s4w(g, b, a, bb, c, xr), "symbolic S4 mismatch");
    }
    let eqs: Vec<Vec<u64>> = p.split(g.n).into_iter().filter(|e| !e.is_empty()).collect();
    // enumerate solutions
    let mut sols = Vec::new();
    let sz = 1u64 << k;
    for i in 0..sz {
        for j in 0..sz {
            let (a, bb) = (sp[i as usize], sp[j as usize]);
            let (a1, b1, c1) = (g.sqr(a ^ bb), g.mul(a, bb), g.sqr(g.mul(a, bb)) ^ b);
            for l in 0..sz {
                let c = sp[l as usize];
                let (a2, b2, c2) = (g.sqr(c ^ xr), g.mul(c, xr), g.sqr(g.mul(c, xr)) ^ b);
                let t1 = g.sqr(g.mul(a1, c2) ^ g.mul(a2, c1));
                let t2 = g.mul(g.mul(a1, b2) ^ g.mul(a2, b1), g.mul(b1, c2) ^ g.mul(b2, c1));
                if t1 ^ t2 == 0 {
                    sols.push(i | (j << k) | (l << (2 * k)));
                }
            }
        }
    }
    System { nvars: 3 * k, eqs, sols }
}

fn build_m4(g: &Gf, b: u64, xr: u64, v: &[u64], rng: &mut Rng) -> System {
    let k = v.len() as u32;
    let n = g.n;
    let ub: Vec<u64> = (0..n).map(|j| 1u64 << j).collect();
    let x: Vec<FP> = (0..4).map(|i| FP::linear(i * k, v)).collect();
    let u = FP::linear(4 * k, &ub);
    let pa = s3_sym(g, b, &x[0], &x[1], &u);
    let pb = s4_sym(g, b, &u, &x[2], &x[3], &FP::constant(xr));
    let sp = span(v);
    let km = (1u64 << k) - 1;
    let nv = 4 * k + n;
    for _ in 0..64 {
        let pt = rng.next() & ((1u64 << nv) - 1);
        let xs: Vec<u64> = (0..4).map(|i| sp[((pt >> (i * k)) & km) as usize]).collect();
        let uu = (pt >> (4 * k)) & g.mask();
        assert_eq!(pa.eval(pt), s3w(g, b, xs[0], xs[1], uu), "symbolic S3 mismatch");
        assert_eq!(pb.eval(pt), s4w(g, b, uu, xs[2], xs[3], xr), "symbolic S4 mismatch");
    }
    let mut eqs: Vec<Vec<u64>> = pa.split(n);
    eqs.extend(pb.split(n));
    eqs.retain(|e| !e.is_empty());
    // solutions: for each (x1, x2), every U with S3 = 0; then every (x3, x4) with S4(U, x3, x4, xR) = 0
    let sz = 1u64 << k;
    let mut sols = Vec::new();
    let mut roots: Vec<(u64, u64)> = Vec::new(); // (pair index, U)
    for i in 0..sz {
        for j in 0..sz {
            for uu in 0..(1u64 << n) {
                if s3w(g, b, sp[i as usize], sp[j as usize], uu) == 0 {
                    roots.push((i | (j << k), uu));
                }
            }
        }
    }
    for &(pij, uu) in &roots {
        for i in 0..sz {
            for j in 0..sz {
                if s4w(g, b, uu, sp[i as usize], sp[j as usize], xr) == 0 {
                    sols.push(pij | (i << (2 * k)) | (j << (3 * k)) | (uu << (4 * k)));
                }
            }
        }
    }
    System { nvars: nv, eqs, sols }
}


/// Roots U in F_{2^n} of S3(p, q, U) = A U^2 + B U + C (A = (p+q)^2, B = pq, C = B^2 + b).
/// `ast[c]` = some z with z^2 + z = c, or u64::MAX.
fn s3_roots(g: &Gf, b: u64, p: u64, q: u64, ast: &[u64]) -> Vec<u64> {
    let a = g.sqr(p ^ q);
    let bb = g.mul(p, q);
    let c = g.sqr(bb) ^ b;
    if a == 0 {
        if bb == 0 {
            return if c == 0 { (0..(1u64 << g.n)).collect() } else { vec![] };
        }
        return vec![g.mul(c, g.inv(bb))];
    }
    if bb == 0 {
        // U^2 = C / A: the unique square root
        let t = g.mul(c, g.inv(a));
        return vec![g.pow(t, 1u128 << (g.n - 1))];
    }
    // U = (B/A) z, z^2 + z = A C / B^2
    let rhs = g.mul(g.mul(a, c), g.inv(g.sqr(bb)));
    let z = ast[rhs as usize];
    if z == u64::MAX {
        return vec![];
    }
    let s = g.mul(bb, g.inv(a));
    vec![g.mul(s, z), g.mul(s, z ^ 1)]
}

/// Chain presentation of S_{m+1}(x1..xm, xR) = 0 (m = 3 or 4), every equation of Boolean
/// degree <= 3: S3(x1, x2, U1) = 0, S3(U1, x3, U2) = 0, ..., S3(U_{m-2}, x_m, xR) = 0,
/// with U_j free in F_{2^n}. Variables: x-blocks (k each) then U-blocks (n each).
fn build_chain(m: u32, g: &Gf, b: u64, xr: u64, v: &[u64], rng: &mut Rng) -> System {
    let k = v.len() as u32;
    if m == 2 {
        // positive control: the direct S3(x1, x2, xR) descent (Boolean degree <= 2, N = 2k)
        let p = s3_sym(g, b, &FP::linear(0, v), &FP::linear(k, v), &FP::constant(xr));
        let sp = span(v);
        let mut sols = Vec::new();
        for i in 0..(1u64 << k) {
            for j in 0..(1u64 << k) {
                let pt = i | (j << k);
                let w = s3w(g, b, sp[i as usize], sp[j as usize], xr);
                assert_eq!(p.eval(pt), w, "symbolic S3 mismatch");
                if w == 0 {
                    sols.push(pt);
                }
            }
        }
        let mut eqs = p.split(g.n);
        eqs.retain(|e| !e.is_empty());
        return System { nvars: 2 * k, eqs, sols };
    }
    let n = g.n;
    let ub: Vec<u64> = (0..n).map(|j| 1u64 << j).collect();
    let x: Vec<FP> = (0..m).map(|i| FP::linear(i * k, v)).collect();
    let u: Vec<FP> = (0..m - 2).map(|j| FP::linear(m * k + j * n, &ub)).collect();
    let nv = m * k + (m - 2) * n;
    assert!(nv <= 64, "too many variables for a 64-bit mask");
    let mut polys: Vec<FP> = Vec::new();
    polys.push(s3_sym(g, b, &x[0], &x[1], &u[0]));
    for j in 1..(m - 2) as usize {
        polys.push(s3_sym(g, b, &u[j - 1], &x[j + 1], &u[j]));
    }
    polys.push(s3_sym(g, b, &u[(m - 3) as usize], &x[(m - 1) as usize], &FP::constant(xr)));
    let sp = span(v);
    let km = (1u64 << k) - 1;
    let full = if nv == 64 { u64::MAX } else { (1u64 << nv) - 1 };
    for _ in 0..64 {
        let pt = rng.next() & full;
        let xs: Vec<u64> = (0..m).map(|i| sp[((pt >> (i * k)) & km) as usize]).collect();
        let us: Vec<u64> = (0..m - 2).map(|j| (pt >> (m * k + j * n)) & g.mask()).collect();
        let mut want = vec![s3w(g, b, xs[0], xs[1], us[0])];
        for j in 1..(m - 2) as usize {
            want.push(s3w(g, b, us[j - 1], xs[j + 1], us[j]));
        }
        want.push(s3w(g, b, us[(m - 3) as usize], xs[(m - 1) as usize], xr));
        for (pp, w) in polys.iter().zip(want) {
            assert_eq!(pp.eval(pt), w, "symbolic S3 chain mismatch");
        }
    }
    let mut eqs: Vec<Vec<u64>> = Vec::new();
    for pp in &polys {
        eqs.extend(pp.split(n));
    }
    eqs.retain(|e| !e.is_empty());
    // solutions: walk the chain, solving each S3 for the next U
    let mut ast = vec![u64::MAX; 1usize << n];
    for z in 0..(1u64 << n) {
        let c = g.sqr(z) ^ z;
        ast[c as usize] = z;
    }
    let sz = 1u64 << k;
    let mut sols = Vec::new();
    // partial: (mask so far, last U)
    let mut partial: Vec<(u64, u64)> = Vec::new();
    for i in 0..sz {
        for j in 0..sz {
            for r in s3_roots(g, b, sp[i as usize], sp[j as usize], &ast) {
                partial.push((i | (j << k) | (r << (m * k)), r));
            }
        }
    }
    for step in 1..(m - 2) {
        let mut next = Vec::new();
        for &(msk, ul) in &partial {
            for i in 0..sz {
                for r in s3_roots(g, b, ul, sp[i as usize], &ast) {
                    next.push((msk | (i << ((step + 1) * k)) | (r << (m * k + step * n)), r));
                }
            }
        }
        partial = next;
    }
    for &(msk, ul) in &partial {
        for i in 0..sz {
            if s3w(g, b, ul, sp[i as usize], xr) == 0 {
                sols.push(msk | (i << ((m - 1) * k)));
            }
        }
    }
    System { nvars: nv, eqs, sols }
}


/// Negative control: a random system with the chain system's shape (same variable count, same
/// number of equations and the same degree for each equation, every monomial of degree <= that
/// degree present with probability 1/2), with one planted solution. Only the planted solution is
/// known, so `deficit` is read against it; a large deficit means a genuine stall.
fn random_like(sys: &System, rng: &mut Rng) -> System {
    let nv = sys.nvars;
    let full = if nv == 64 { u64::MAX } else { (1u64 << nv) - 1 };
    let planted = rng.next() & full;
    let mut eqs = Vec::new();
    for e in &sys.eqs {
        let d = e.iter().map(|m| m.count_ones()).max().unwrap_or(0);
        let mut ms: Vec<u64> = Vec::new();
        for deg in 1..=d {
            gen_masks(nv, deg, 0, 0, &mut ms);
        }
        let mut eq: Vec<u64> = ms.into_iter().filter(|_| rng.next() & 1 == 1).collect();
        let val = eq.iter().filter(|&&m| planted & m == m).count() & 1;
        if val == 1 {
            eq.push(0);
        }
        eqs.push(eq);
    }
    System { nvars: nv, eqs, sols: vec![planted] }
}

// ---------------------------------------------------------------- closure
fn binom(n: u64, k: u64) -> u64 {
    if k > n {
        return 0;
    }
    let mut r = 1u64;
    for i in 0..k {
        r = r * (n - i) / (i + 1);
    }
    r
}

struct ClosureResult {
    cols: usize,
    rank: usize,
    target: usize,
    solved: bool,
    gen_deg: u32,
    products: u64,
    dim_by_deg: Vec<usize>, // number of pivots of each degree
    secs: f64,
}

fn closure(sys: &System, d: u32) -> Option<ClosureResult> {
    let t0 = Instant::now();
    let nv = sys.nvars;
    let gen_deg = sys.eqs.iter().flat_map(|e| e.iter().map(|m| m.count_ones())).max().unwrap_or(0);
    if gen_deg > d {
        return None;
    }
    // columns: degree descending; within degree, masks ascending
    let mut monos: Vec<u64> = Vec::new();
    for deg in (0..=d).rev() {
        let mut v: Vec<u64> = Vec::new();
        gen_masks(nv, deg, 0, 0, &mut v);
        v.sort_unstable();
        monos.extend(v);
    }
    let cols = monos.len();
    let idx: HashMap<u64, usize> = monos.iter().enumerate().map(|(i, &m)| (m, i)).collect();
    let words = cols.div_ceil(64);
    // target rank = cols - rank of solution evaluation vectors
    let mut ev: Vec<Vec<u64>> = Vec::new();
    for &s in &sys.sols {
        let mut row = vec![0u64; words];
        for (c, &m) in monos.iter().enumerate() {
            if s & m == m {
                row[c / 64] |= 1 << (c % 64);
            }
        }
        ev.push(row);
    }
    let evrank = small_rank(ev, words);
    let target = cols - evrank;

    let mut pivot: Vec<u32> = vec![u32::MAX; cols];
    let mut rows: Vec<Vec<u64>> = Vec::new();
    let mut lead: Vec<usize> = Vec::new();
    let mut queue: Vec<usize> = Vec::new();
    let mut products = 0u64;
    let deg_of = |c: usize| monos[c].count_ones();

    let insert = |mut r: Vec<u64>, rows: &mut Vec<Vec<u64>>, lead: &mut Vec<usize>, pivot: &mut Vec<u32>, queue: &mut Vec<usize>| -> bool {
        let mut w = 0;
        loop {
            while w < words && r[w] == 0 {
                w += 1;
            }
            if w == words {
                return false;
            }
            let c = w * 64 + r[w].trailing_zeros() as usize;
            let p = pivot[c];
            if p == u32::MAX {
                pivot[c] = rows.len() as u32;
                if deg_of(c) < d {
                    queue.push(rows.len());
                }
                lead.push(c);
                rows.push(r);
                return true;
            }
            let pr = &rows[p as usize];
            for x in w..words {
                r[x] ^= pr[x];
            }
        }
    };

    for e in &sys.eqs {
        let mut r = vec![0u64; words];
        for m in e {
            let c = idx[m];
            r[c / 64] ^= 1 << (c % 64);
        }
        insert(r, &mut rows, &mut lead, &mut pivot, &mut queue);
    }
    let mut qi = 0;
    while qi < queue.len() && rows.len() < target {
        let ri = queue[qi];
        qi += 1;
        // monomials of row ri
        let mut ms: Vec<u64> = Vec::new();
        for w in 0..words {
            let mut x = rows[ri][w];
            while x != 0 {
                ms.push(monos[w * 64 + x.trailing_zeros() as usize]);
                x &= x - 1;
            }
        }
        for i in 0..nv {
            let mut r = vec![0u64; words];
            let bit = 1u64 << i;
            for &m in &ms {
                let c = idx[&(m | bit)];
                r[c / 64] ^= 1 << (c % 64);
            }
            products += 1;
            insert(r, &mut rows, &mut lead, &mut pivot, &mut queue);
            if rows.len() >= target {
                break;
            }
        }
    }
    let rank = rows.len();
    let mut dim_by_deg = vec![0usize; d as usize + 1];
    for &c in &lead {
        dim_by_deg[deg_of(c) as usize] += 1;
    }
    Some(ClosureResult {
        cols,
        rank,
        target,
        solved: rank == target,
        gen_deg,
        products,
        dim_by_deg,
        secs: t0.elapsed().as_secs_f64(),
    })
}

fn gen_masks(nv: u32, deg: u32, start: u32, cur: u64, out: &mut Vec<u64>) {
    if deg == 0 {
        out.push(cur);
        return;
    }
    for i in start..nv {
        gen_masks(nv, deg - 1, i + 1, cur | (1 << i), out);
    }
}

fn small_rank(mut m: Vec<Vec<u64>>, words: usize) -> usize {
    let mut rank = 0;
    let rowsn = m.len();
    for c in 0..words * 64 {
        let (w, b) = (c / 64, c % 64);
        if let Some(p) = (rank..rowsn).find(|&r| (m[r][w] >> b) & 1 == 1) {
            m.swap(rank, p);
            for r in 0..rowsn {
                if r != rank && (m[r][w] >> b) & 1 == 1 {
                    let pr = m[rank].clone();
                    for x in 0..words {
                        m[r][x] ^= pr[x];
                    }
                }
            }
            rank += 1;
            if rank == rowsn {
                break;
            }
        }
    }
    rank
}

// ---------------------------------------------------------------- main
fn arg(args: &[String], k: &str, def: &str) -> String {
    args.iter().position(|a| a == k).map(|i| args[i + 1].clone()).unwrap_or(def.to_string())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let m: u32 = arg(&args, "--m", "3").parse().unwrap();
    let n: u32 = arg(&args, "--n", "12").parse().unwrap();
    let k: u32 = arg(&args, "--k", "4").parse().unwrap();
    let seed: u64 = arg(&args, "--seed", "1").parse().unwrap();
    let targets: u32 = arg(&args, "--targets", "4").parse().unwrap();
    let dmax: u32 = arg(&args, "--dmax", "4").parse().unwrap();
    let dmin: u32 = arg(&args, "--dmin", "3").parse().unwrap();
    let basis = arg(&args, "--basis", "random");
    let pres = arg(&args, "--pres", "chain");
    let cell = arg(&args, "--cell", "");
    let maxcols: usize = arg(&args, "--maxcols", "60000").parse().unwrap();

    let f = find_irreducible(n);
    let g = Gf { n, f };
    let mut rng = Rng(seed ^ ((n as u64) << 40) ^ ((k as u64) << 32) ^ ((m as u64) << 56) ^ 0x4B32);
    // random curve y^2 + xy = x^3 + a x^2 + b
    let a = rng.next() & 1;
    let b = loop {
        let b = rng.next() & g.mask();
        if b > 1 {
            break b;
        }
    };
    // subspace
    let v: Vec<u64> = if basis == "standard" {
        (0..k).map(|j| 1u64 << j).collect()
    } else {
        // random k-dim subspace: k independent vectors
        let mut v: Vec<u64> = Vec::new();
        let mut ech: Vec<u64> = Vec::new();
        while v.len() < k as usize {
            let x = rng.next() & g.mask();
            let mut y = x;
            for &e in &ech {
                y = y.min(y ^ e);
            }
            if y != 0 {
                ech.push(y);
                ech.sort_unstable_by(|p, q| q.cmp(p));
                v.push(x);
            }
        }
        v
    };
    for t in 0..targets {
        // target: abscissa of a curve point, Tr(x + a + b/x^2) = 0
        let xr = loop {
            let x = rng.next() & g.mask();
            if x == 0 {
                continue;
            }
            let w = x ^ a ^ g.mul(b, g.sqr(g.inv(x)));
            if g.trace(w) == 0 {
                break x;
            }
        };
        let tb = Instant::now();
        let sys = match pres.as_str() {
            "chain" => build_chain(m, &g, b, xr, &v, &mut rng),
            "random" => {
                let c = build_chain(m, &g, b, xr, &v, &mut rng);
                random_like(&c, &mut rng)
            }
            _ => if m == 3 { build_m3(&g, b, xr, &v, &mut rng) } else { build_m4(&g, b, xr, &v, &mut rng) },
        };
        let build_secs = tb.elapsed().as_secs_f64();
        let nsol = sys.sols.len();
        // sanity: every solution satisfies every equation
        for &s in &sys.sols {
            for e in &sys.eqs {
                let val = e.iter().filter(|&&mm| s & mm == mm).count() & 1;
                assert_eq!(val, 0, "enumerated solution fails an equation");
            }
        }
        let mut first_solved: Option<u32> = None;
        for d in dmin..=dmax {
            let cols: u64 = (0..=d as u64).map(|j| binom(sys.nvars as u64, j)).sum();
            if cols as usize > maxcols {
                println!("{{\"cell\":\"{cell}\",\"m\":{m},\"n\":{n},\"k\":{k},\"N\":{},\"eqs\":{},\"seed\":{seed},\"basis\":\"{basis}\",\"pres\":\"{pres}\",\"target\":{t},\"xr\":{xr},\"b\":{b},\"a\":{a},\"nsol\":{nsol},\"D\":{d},\"skipped\":\"cols {cols} > maxcols\"}}", sys.nvars, sys.eqs.len());
                continue;
            }
            match closure(&sys, d) {
                None => println!("{{\"cell\":\"{cell}\",\"m\":{m},\"n\":{n},\"k\":{k},\"N\":{},\"eqs\":{},\"seed\":{seed},\"basis\":\"{basis}\",\"pres\":\"{pres}\",\"target\":{t},\"nsol\":{nsol},\"D\":{d},\"skipped\":\"generator degree > D\"}}", sys.nvars, sys.eqs.len()),
                Some(r) => {
                    if r.solved && first_solved.is_none() {
                        first_solved = Some(d);
                    }
                    println!(
                        "{{\"cell\":\"{cell}\",\"m\":{m},\"n\":{n},\"k\":{k},\"N\":{},\"eqs\":{},\"seed\":{seed},\"basis\":\"{basis}\",\"pres\":\"{pres}\",\"irr\":\"{:#x}\",\"a\":{a},\"b\":{b},\"target\":{t},\"xr\":{xr},\"nsol\":{nsol},\"gen_deg\":{},\"D\":{d},\"cols\":{},\"rank\":{},\"target_rank\":{},\"deficit\":{},\"solved\":{},\"products\":{},\"dim_by_deg\":{:?},\"build_secs\":{:.3},\"secs\":{:.3}}}",
                        sys.nvars, sys.eqs.len(), f, r.gen_deg, r.cols, r.rank, r.target, r.target - r.rank, r.solved, r.products, r.dim_by_deg, build_secs, r.secs
                    );
                }
            }
            if first_solved.is_some() {
                break; // W_{D+1} contains x * W_D: solved at D implies solved above
            }
        }
    }
}
