//! K2 decisive-cell instrument: blocked (Four-Russians) GF(2) closure; derived from k2_flatness_20261115/k2close.rs (system builders, controls and closure_ref unchanged).
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

fn closure_ref(sys: &System, d: u32) -> Option<ClosureResult> {
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


// ================================================================ blocked GF(2) closure (M4RI-style)
// Same closure W_D and the same "solved at D" criterion as `closure_ref`, computed with a
// reduced row echelon basis (pivot = first set column; columns degree-descending), tile-major
// packed u64 storage, and Method-of-Four-Russians batch reduction. Rows are processed in
// batches of products x_i * (basis row of pivot degree < D); degree-fall rows go back to the queue.
const TW: usize = 64; // tile width in words (512 B)
const BATCH: usize = 2048;

#[derive(Clone, Copy)]
struct SendPtr(*mut u64);
unsafe impl Send for SendPtr {}
unsafe impl Sync for SendPtr {}
#[derive(Clone, Copy)]
struct SendPtrU8(*mut u8);
unsafe impl Send for SendPtrU8 {}
unsafe impl Sync for SendPtrU8 {}

fn vmhwm_kb() -> u64 {
    std::fs::read_to_string("/proc/self/status")
        .ok()
        .and_then(|s| s.lines().find(|l| l.starts_with("VmHWM")).and_then(|l| l.split_whitespace().nth(1).and_then(|v| v.parse().ok())))
        .unwrap_or(0)
}

struct M4Result {
    cols: usize,
    rank: usize,
    target: usize,
    solved: bool,
    censored: bool, // stopped by --maxsecs: rank is a lower bound, not the closure
    early_one: bool, // stopped as soon as the constant 1 became a pivot (rank then = cols by closure)
    gen_deg: u32,
    products: u64,
    batches: u32,
    dim_by_deg: Vec<usize>,
    secs: f64,
    t_prod: f64,
    t_red: f64,
    t_int: f64,
    t_back: f64,
    peak_rss_kb: u64,
    matrix_bytes: u64,
}

#[inline]
fn bit(row: &[u64], c: usize) -> bool {
    (row[c >> 6] >> (c & 63)) & 1 == 1
}

fn closure_m4(sys: &System, d: u32, nth: usize, maxsecs: f64) -> Option<M4Result> {
    let t0 = Instant::now();
    let nv = sys.nvars as usize;
    let gen_deg = sys.eqs.iter().flat_map(|e| e.iter().map(|m| m.count_ones())).max().unwrap_or(0);
    if gen_deg > d {
        return None;
    }
    let mut monos: Vec<u64> = Vec::new();
    for deg in (0..=d).rev() {
        let mut v: Vec<u64> = Vec::new();
        gen_masks(sys.nvars, deg, 0, 0, &mut v);
        v.sort_unstable();
        monos.extend(v);
    }
    let cols = monos.len();
    let idx: HashMap<u64, usize> = monos.iter().enumerate().map(|(i, &m)| (m, i)).collect();
    let words = cols.div_ceil(64);
    let nt = words.div_ceil(TW);
    let wp = nt * TW;
    let degcol: Vec<u8> = monos.iter().map(|m| m.count_ones() as u8).collect();
    // target = cols - rank(evaluation vectors of the solutions)
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
    let target = cols - small_rank(ev, words);
    // multiplication maps: col of monomial m -> col of m | x_i (u32::MAX when degree would exceed d)
    let mulmap: Vec<Vec<u32>> = (0..nv)
        .map(|i| {
            let b = 1u64 << i;
            monos.iter().map(|&m| if m & b != 0 { idx[&m] as u32 } else { idx.get(&(m | b)).map(|&c| c as u32).unwrap_or(u32::MAX) }).collect()
        })
        .collect();
    let mulmap: Vec<Vec<u32>> = mulmap.into_iter().enumerate().map(|(i, mut v)| {
        // identity for monomials already containing x_i (set above); fine as is
        let _ = i;
        v.shrink_to_fit();
        v
    }).collect();

    // basis, tile-major: bas[t][r*TW .. (r+1)*TW]
    let mut bas: Vec<Vec<u64>> = (0..nt).map(|_| Vec::with_capacity(target.min(cols) * TW)).collect();
    let mut pcol: Vec<u32> = Vec::new(); // pivot column of storage row r
    let mut spc: Vec<u32> = Vec::new(); // pivot columns, ascending
    let mut sord: Vec<u32> = Vec::new(); // storage row of each (ascending pivot)
    let mut queue: Vec<u32> = Vec::new();
    let mut qi = 0usize; // next queued row
    let mut vi = 0usize; // next variable for that row
    let mut products = 0u64;
    let mut batches = 0u32;
    let (mut t_prod, mut t_red, mut t_int, mut t_back) = (0f64, 0f64, 0f64, 0f64);
    let mut early_one = false;
    let mut censored = false;
    let one_col = cols - 1;

    let mut x = vec![0u64; BATCH * wp];
    let mut first = true;
    loop {
        // ---- build the batch (row-major)
        let tp = Instant::now();
        let nrows;
        if first {
            first = false;
            let mut r = 0;
            for e in &sys.eqs {
                let row = &mut x[r * wp..(r + 1) * wp];
                for &m in e {
                    let c = idx[&m];
                    row[c >> 6] ^= 1 << (c & 63);
                }
                r += 1;
                assert!(r <= BATCH);
            }
            nrows = r;
        } else {
            let mut plist: Vec<(u32, u32)> = Vec::with_capacity(BATCH);
            while plist.len() < BATCH && qi < queue.len() {
                plist.push((queue[qi], vi as u32));
                vi += 1;
                if vi == nv {
                    vi = 0;
                    qi += 1;
                }
            }
            nrows = plist.len();
            if nrows == 0 {
                break;
            }
            for v in x[..nrows * wp].iter_mut() {
                *v = 0;
            }
            let per = nrows.div_ceil(nth).max(1);
            let basr = &bas;
            let mm = &mulmap;
            std::thread::scope(|s| {
                for (chunk, pl) in x[..nrows * wp].chunks_mut(per * wp).zip(plist.chunks(per)) {
                    s.spawn(move || {
                        for (orow, &(r, v)) in chunk.chunks_mut(wp).zip(pl.iter()) {
                            let map = &mm[v as usize];
                            let r = r as usize;
                            for (t, tile) in basr.iter().enumerate() {
                                let src = &tile[r * TW..(r + 1) * TW];
                                for (w, &word) in src.iter().enumerate() {
                                    let mut z = word;
                                    while z != 0 {
                                        let c = (t * TW + w) * 64 + z.trailing_zeros() as usize;
                                        z &= z - 1;
                                        let c2 = map[c] as usize;
                                        orow[c2 >> 6] ^= 1 << (c2 & 63);
                                    }
                                }
                            }
                        }
                    });
                }
            });
            products += nrows as u64;
        }
        t_prod += tp.elapsed().as_secs_f64();
        batches += 1;

        // ---- reduce the batch against the old basis (Four Russians over groups of 8 pivots)
        let tr = Instant::now();
        let nold = pcol.len();
        if nold > 0 {
            let ng = nold.div_ceil(8);
            let mut pb = vec![0u8; ng * nrows]; // pb[g*nrows + b]
            {
                let pbp = SendPtrU8(pb.as_mut_ptr());
                let per = nrows.div_ceil(nth).max(1);
                let spc_r = &spc;
                let xr = &x;
                std::thread::scope(|s| {
                    for b0 in (0..nrows).step_by(per) {
                        let b1 = (b0 + per).min(nrows);
                        s.spawn(move || {
                            let pbp = pbp;
                            for b in b0..b1 {
                                let row = &xr[b * wp..(b + 1) * wp];
                                for g in 0..ng {
                                    let mut v = 0u8;
                                    let hi = (8 * g + 8).min(spc_r.len());
                                    for (i, &c) in spc_r[8 * g..hi].iter().enumerate() {
                                        v |= (((row[(c >> 6) as usize] >> (c & 63)) & 1) as u8) << i;
                                    }
                                    unsafe { *pbp.0.add(g * nrows + b) = v };
                                }
                            }
                        });
                    }
                });
            }
            let nz: Vec<Vec<u32>> = (0..ng).map(|g| (0..nrows).filter(|&b| pb[g * nrows + b] != 0).map(|b| b as u32).collect()).collect();
            let xp = SendPtr(x.as_mut_ptr());
            let basr = &bas;
            let (spc_r, sord_r, pb_r, nz_r) = (&spc, &sord, &pb, &nz);
            std::thread::scope(|s| {
                for tid in 0..nth {
                    s.spawn(move || {
                        let xp = xp;
                        let mut tab = vec![0u64; 256 * TW];
                        let mut t = tid;
                        while t < nt {
                            let tile = &basr[t];
                            for g in 0..ng {
                                let list = &nz_r[g];
                                if list.is_empty() {
                                    continue;
                                }
                                if (spc_r[8 * g] >> 6) as usize >= (t + 1) * TW {
                                    continue; // group is zero in this tile
                                }
                                let hi = (8 * g + 8).min(spc_r.len());
                                let len = hi - 8 * g;
                                let rows: Vec<&[u64]> = (0..len).map(|i| { let r = sord_r[8 * g + i] as usize; &tile[r * TW..(r + 1) * TW] }).collect();
                                if list.len() < 48 {
                                    for &b in list {
                                        let v = pb_r[g * nrows + b as usize];
                                        let dst = unsafe { std::slice::from_raw_parts_mut(xp.0.add(b as usize * wp + t * TW), TW) };
                                        for i in 0..len {
                                            if (v >> i) & 1 == 1 {
                                                for w in 0..TW {
                                                    dst[w] ^= rows[i][w];
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    for w in 0..TW {
                                        tab[w] = 0;
                                    }
                                    for e in 1..(1usize << len) {
                                        let lb = e.trailing_zeros() as usize;
                                        let prev = e & (e - 1);
                                        let (a, b2) = tab.split_at_mut(e * TW);
                                        let pr = &a[prev * TW..(prev + 1) * TW];
                                        for w in 0..TW {
                                            b2[w] = pr[w] ^ rows[lb][w];
                                        }
                                    }
                                    for &b in list {
                                        let v = pb_r[g * nrows + b as usize] as usize;
                                        let dst = unsafe { std::slice::from_raw_parts_mut(xp.0.add(b as usize * wp + t * TW), TW) };
                                        let src = &tab[v * TW..(v + 1) * TW];
                                        for w in 0..TW {
                                            dst[w] ^= src[w];
                                        }
                                    }
                                }
                            }
                            t += nth;
                        }
                    });
                }
            });
        }
        t_red += tr.elapsed().as_secs_f64();

        // ---- eliminate inside the batch (reduced echelon among the new rows)
        let ti = Instant::now();
        let mut newp: Vec<(u32, u32)> = Vec::new(); // (pivot col, batch row)
        for b in 0..nrows {
            let (a, rest) = x.split_at_mut(b * wp);
            let row = &mut rest[..wp];
            for &(c, p) in &newp {
                if bit(row, c as usize) {
                    let pr = &a[p as usize * wp..(p as usize + 1) * wp];
                    for w in (c as usize >> 6)..wp {
                        row[w] ^= pr[w];
                    }
                }
            }
            let lead = match row.iter().position(|&w| w != 0) {
                None => continue,
                Some(w) => w * 64 + row[w].trailing_zeros() as usize,
            };
            for &(_, q) in &newp {
                let qr = &mut a[q as usize * wp..(q as usize + 1) * wp];
                if bit(qr, lead) {
                    for w in (lead >> 6)..wp {
                        qr[w] ^= row[w];
                    }
                }
            }
            newp.push((lead as u32, b as u32));
        }
        t_int += ti.elapsed().as_secs_f64();
        newp.sort_unstable();
        let rn = newp.len();

        // ---- back-substitute the new pivots out of the old rows
        let tb = Instant::now();
        if rn > 0 && nold > 0 {
            let ngn = rn.div_ceil(8);
            let mut ib = vec![0u8; ngn * nold]; // ib[g*nold + j]
            {
                let ibp = SendPtrU8(ib.as_mut_ptr());
                let per = nold.div_ceil(nth).max(1);
                let basr = &bas;
                let np = &newp;
                std::thread::scope(|s| {
                    for j0 in (0..nold).step_by(per) {
                        let j1 = (j0 + per).min(nold);
                        s.spawn(move || {
                            let ibp = ibp;
                            for j in j0..j1 {
                                for g in 0..ngn {
                                    let mut v = 0u8;
                                    let hi = (8 * g + 8).min(rn);
                                    for (i, &(c, _)) in np[8 * g..hi].iter().enumerate() {
                                        let w = (c >> 6) as usize;
                                        let tile = &basr[w / TW];
                                        v |= (((tile[j * TW + w % TW] >> (c & 63)) & 1) as u8) << i;
                                    }
                                    unsafe { *ibp.0.add(g * nold + j) = v };
                                }
                            }
                        });
                    }
                });
            }
            let nzb: Vec<Vec<u32>> = (0..ngn).map(|g| (0..nold).filter(|&j| ib[g * nold + j] != 0).map(|j| j as u32).collect()).collect();
            let mut parts: Vec<Vec<(usize, &mut Vec<u64>)>> = (0..nth).map(|_| Vec::new()).collect();
            for (t, tile) in bas.iter_mut().enumerate() {
                parts[t % nth].push((t, tile));
            }
            let (xr, np, ib_r, nzb_r) = (&x, &newp, &ib, &nzb);
            std::thread::scope(|s| {
                for part in parts {
                    s.spawn(move || {
                        let mut tab = vec![0u64; 256 * TW];
                        for (t, tile) in part {
                            for g in 0..ngn {
                                let list = &nzb_r[g];
                                if list.is_empty() {
                                    continue;
                                }
                                if (np[8 * g].0 >> 6) as usize >= (t + 1) * TW {
                                    continue;
                                }
                                let hi = (8 * g + 8).min(rn);
                                let len = hi - 8 * g;
                                let rows: Vec<&[u64]> = (0..len).map(|i| { let b = np[8 * g + i].1 as usize; &xr[b * wp + t * TW..b * wp + (t + 1) * TW] }).collect();
                                if list.len() < 48 {
                                    for &j in list {
                                        let v = ib_r[g * nold + j as usize];
                                        let dst = &mut tile[j as usize * TW..(j as usize + 1) * TW];
                                        for i in 0..len {
                                            if (v >> i) & 1 == 1 {
                                                for w in 0..TW {
                                                    dst[w] ^= rows[i][w];
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    for w in 0..TW {
                                        tab[w] = 0;
                                    }
                                    for e in 1..(1usize << len) {
                                        let lb = e.trailing_zeros() as usize;
                                        let prev = e & (e - 1);
                                        let (a, b2) = tab.split_at_mut(e * TW);
                                        let pr = &a[prev * TW..(prev + 1) * TW];
                                        for w in 0..TW {
                                            b2[w] = pr[w] ^ rows[lb][w];
                                        }
                                    }
                                    for &j in list {
                                        let v = ib_r[g * nold + j as usize] as usize;
                                        let dst = &mut tile[j as usize * TW..(j as usize + 1) * TW];
                                        let src = &tab[v * TW..(v + 1) * TW];
                                        for w in 0..TW {
                                            dst[w] ^= src[w];
                                        }
                                    }
                                }
                            }
                        }
                    });
                }
            });
        }
        // append the new rows
        for &(c, b) in &newp {
            let r = pcol.len();
            for (t, tile) in bas.iter_mut().enumerate() {
                tile.extend_from_slice(&x[b as usize * wp + t * TW..b as usize * wp + (t + 1) * TW]);
            }
            pcol.push(c);
            if (degcol[c as usize] as u32) < d {
                queue.push(r as u32);
            }
            if c as usize == one_col {
                early_one = true;
            }
        }
        // merge the sorted pivot lists
        {
            let mut ns: Vec<u32> = Vec::with_capacity(spc.len() + rn);
            let mut no: Vec<u32> = Vec::with_capacity(spc.len() + rn);
            let (mut i, mut j) = (0, 0);
            while i < spc.len() || j < rn {
                if j == rn || (i < spc.len() && spc[i] < newp[j].0) {
                    ns.push(spc[i]);
                    no.push(sord[i]);
                    i += 1;
                } else {
                    ns.push(newp[j].0);
                    no.push((nold + j) as u32);
                    j += 1;
                }
            }
            spc = ns;
            sord = no;
        }
        t_back += tb.elapsed().as_secs_f64();
        if std::env::var("K2_PROGRESS").is_ok() {
            eprintln!("batch {batches} rows {nrows} new {rn} rank {} queue {}/{} t={:.0}s red={:.0} int={:.0} back={:.0} rss={}MB", pcol.len(), qi, queue.len(), t0.elapsed().as_secs_f64(), t_red, t_int, t_back, vmhwm_kb() / 1024);
        }
        if early_one || pcol.len() >= target {
            break;
        }
        if t0.elapsed().as_secs_f64() > maxsecs {
            censored = true;
            break;
        }
    }
    let rank = if early_one { cols } else { pcol.len() };
    let mut dim_by_deg = vec![0usize; d as usize + 1];
    for &c in &pcol {
        dim_by_deg[degcol[c as usize] as usize] += 1;
    }
    Some(M4Result {
        cols,
        rank,
        target,
        solved: rank == target && !censored,
        censored,
        early_one,
        gen_deg,
        products,
        batches,
        dim_by_deg,
        secs: t0.elapsed().as_secs_f64(),
        t_prod,
        t_red,
        t_int,
        t_back,
        peak_rss_kb: vmhwm_kb(),
        matrix_bytes: (cols as u64) * (wp as u64) * 8,
    })
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
    let engine = arg(&args, "--engine", "m4");
    let nth: usize = arg(&args, "--threads", "4").parse().unwrap();
    let only_target: i64 = arg(&args, "--only-target", "-1").parse().unwrap();
    let maxsecs: f64 = arg(&args, "--maxsecs", "1e9").parse().unwrap();
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
        if only_target >= 0 && t as i64 != only_target {
            continue; // the system was built above only to keep the RNG stream identical
        }
        for d in dmin..=dmax {
            let cols: u64 = (0..=d as u64).map(|j| binom(sys.nvars as u64, j)).sum();
            if cols as usize > maxcols {
                println!("{{\"cell\":\"{cell}\",\"m\":{m},\"n\":{n},\"k\":{k},\"N\":{},\"eqs\":{},\"seed\":{seed},\"basis\":\"{basis}\",\"pres\":\"{pres}\",\"target\":{t},\"xr\":{xr},\"b\":{b},\"a\":{a},\"nsol\":{nsol},\"D\":{d},\"skipped\":\"cols {cols} > maxcols\"}}", sys.nvars, sys.eqs.len());
                continue;
            }
            let hdr = format!("\"cell\":\"{cell}\",\"engine\":\"{engine}\",\"m\":{m},\"n\":{n},\"k\":{k},\"N\":{},\"eqs\":{},\"seed\":{seed},\"basis\":\"{basis}\",\"pres\":\"{pres}\",\"irr\":\"{:#x}\",\"a\":{a},\"b\":{b},\"target\":{t},\"xr\":{xr},\"nsol\":{nsol}", sys.nvars, sys.eqs.len(), f);
            if engine == "ref" {
                match closure_ref(&sys, d) {
                    None => println!("{{{hdr},\"D\":{d},\"skipped\":\"generator degree > D\"}}"),
                    Some(r) => {
                        if r.solved && first_solved.is_none() { first_solved = Some(d); }
                        println!("{{{hdr},\"gen_deg\":{},\"D\":{d},\"cols\":{},\"rank\":{},\"target_rank\":{},\"deficit\":{},\"solved\":{},\"products\":{},\"dim_by_deg\":{:?},\"build_secs\":{:.3},\"secs\":{:.3}}}",
                            r.gen_deg, r.cols, r.rank, r.target, r.target - r.rank, r.solved, r.products, r.dim_by_deg, build_secs, r.secs);
                    }
                }
            } else {
                match closure_m4(&sys, d, nth, maxsecs) {
                    None => println!("{{{hdr},\"D\":{d},\"skipped\":\"generator degree > D\"}}"),
                    Some(r) => {
                        if r.solved && first_solved.is_none() { first_solved = Some(d); }
                        println!("{{{hdr},\"gen_deg\":{},\"D\":{d},\"cols\":{},\"rank\":{},\"target_rank\":{},\"deficit\":{},\"solved\":{},\"early_one\":{},\"censored\":{},\"products\":{},\"batches\":{},\"dim_by_deg\":{:?},\"build_secs\":{:.3},\"secs\":{:.3},\"t_prod\":{:.2},\"t_reduce\":{:.2},\"t_inner\":{:.2},\"t_backsub\":{:.2},\"peak_rss_kb\":{},\"matrix_bytes\":{},\"threads\":{}}}",
                            r.gen_deg, r.cols, r.rank, r.target, r.target - r.rank, r.solved, r.early_one, r.censored, r.products, r.batches, r.dim_by_deg, build_secs, r.secs, r.t_prod, r.t_red, r.t_int, r.t_back, r.peak_rss_kb, r.matrix_bytes, nth);
                    }
                }
            }
            if first_solved.is_some() {
                break; // W_{D+1} contains x * W_D: solved at D implies solved above
            }
        }
    }
}
