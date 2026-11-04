//! pq2_rank: exact lower bound on the solving degree of the m = 2 Weil-descended
//! third summation polynomial system over F_{2^n}.
//!
//! For each instance (ordinary curve y^2 + xy = x^3 + a x^2 + b over F_{2^n}, a
//! random F_2-subspace V of dimension n' = ceil(n/2), a target x_R with a nonempty
//! solution set) the program computes, all exactly over F_2:
//!
//! * the descended system F: n bilinear equations in the 2n' bit variables (a, b)
//!   plus the 2n' field equations;
//! * rho = rank of the square-free quadratic parts of the n descended equations;
//! * Z = the full solution set in F_2^{2n'} (enumerated through x_1 in V), its
//!   affine dimension, and k = N - affdim(Z): the number of independent linear
//!   polynomials in the ideal with distinct leading variables;
//! * low_d = dim( rowspace(M_{<=d}) ∩ P_{<=1} ) for the Macaulay matrices of
//!   degree d = 2, 3 (and 4 for small n);
//! * d_lin = the least d with low_d >= k.
//!
//! By `SolvingDegreeBound.linear_count_of_gb_in_subspace` (Lean, kernel-checked)
//! any Groebner basis for a degree-compatible order that lies in rowspace(M_{<=d})
//! forces low_d >= k, so the Caminata-Gorla solving degree is at least d_lin.
//! The first fall degree of these systems is 2 (Kosters-Yeo).
//!
//! No external crates. Deterministic (xorshift64*, seed printed).

use std::collections::HashMap;
use std::fmt::Write as _;
use std::time::Instant;

// ---------------------------------------------------------------------------
// Carry-less multiplication and GF(2^n)

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "pclmulqdq")]
unsafe fn clmul_intrin(a: u64, b: u64) -> u128 {
    use std::arch::x86_64::*;
    let x = _mm_set_epi64x(0, a as i64);
    let y = _mm_set_epi64x(0, b as i64);
    let r = _mm_clmulepi64_si128(x, y, 0);
    let lo = _mm_cvtsi128_si64(r) as u64;
    let hi = _mm_extract_epi64(r, 1) as u64;
    ((hi as u128) << 64) | lo as u128
}

fn clmul_slow(a: u64, b: u64) -> u128 {
    let mut r: u128 = 0;
    let a = a as u128;
    for i in 0..64 {
        if (b >> i) & 1 == 1 {
            r ^= a << i;
        }
    }
    r
}

#[cfg(target_arch = "x86_64")]
fn clmul(a: u64, b: u64) -> u128 {
    if std::is_x86_feature_detected!("pclmulqdq") {
        unsafe { clmul_intrin(a, b) }
    } else {
        clmul_slow(a, b)
    }
}

#[cfg(not(target_arch = "x86_64"))]
fn clmul(a: u64, b: u64) -> u128 {
    clmul_slow(a, b)
}

/// GF(2^n) with n <= 62, elements as bit-polynomials in u64, modulus f of degree n.
#[derive(Clone)]
struct Gf {
    n: u32,
    f: u64, // includes the x^n term
}

impl Gf {
    fn reduce(&self, mut r: u128) -> u64 {
        let n = self.n;
        let f = self.f as u128;
        let mut d = 127 - r.leading_zeros() as i32;
        while d >= n as i32 {
            r ^= f << (d as u32 - n);
            if r == 0 {
                return 0;
            }
            d = 127 - r.leading_zeros() as i32;
        }
        r as u64
    }
    fn mul(&self, a: u64, b: u64) -> u64 {
        self.reduce(clmul(a, b))
    }
    fn sq(&self, a: u64) -> u64 {
        self.mul(a, a)
    }
    fn pow(&self, mut a: u64, mut e: u128) -> u64 {
        let mut r = 1u64;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(r, a);
            }
            a = self.sq(a);
            e >>= 1;
        }
        r
    }
    fn inv(&self, a: u64) -> u64 {
        assert!(a != 0, "inverse of zero");
        // a^(2^n - 2)
        let e = ((1u128 << self.n) - 2) as u128;
        self.pow(a, e)
    }
    fn sqrt(&self, a: u64) -> u64 {
        // a^(2^(n-1))
        let mut r = a;
        for _ in 0..self.n - 1 {
            r = self.sq(r);
        }
        r
    }
    fn trace(&self, a: u64) -> u64 {
        let mut t = a;
        let mut s = a;
        for _ in 1..self.n {
            s = self.sq(s);
            t ^= s;
        }
        debug_assert!(t == 0 || t == 1);
        t
    }
    fn mask(&self) -> u64 {
        if self.n == 64 {
            u64::MAX
        } else {
            (1u64 << self.n) - 1
        }
    }
}

// Polynomials over F_2 as u128 (degree < 128), for the irreducibility test.
fn pdeg(a: u128) -> i32 {
    if a == 0 {
        -1
    } else {
        127 - a.leading_zeros() as i32
    }
}
fn pmod(mut a: u128, m: u128) -> u128 {
    let dm = pdeg(m);
    assert!(dm >= 0);
    while pdeg(a) >= dm {
        a ^= m << (pdeg(a) - dm);
    }
    a
}
fn pgcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let r = pmod(a, b);
        a = b;
        b = r;
    }
    a
}
fn pmulmod(a: u128, b: u128, m: u128) -> u128 {
    // degrees < 64 so clmul fits
    assert!(pdeg(a) < 64 && pdeg(b) < 64);
    pmod(clmul(a as u64, b as u64), m)
}

/// Ben-Or irreducibility test for f of degree n over F_2.
fn is_irreducible(f: u128, n: u32) -> bool {
    let mut x = 2u128; // x
    for _ in 1..=n / 2 {
        x = pmulmod(x, x, f); // x^(2^i)
        if pgcd(x ^ 2, f) != 1 {
            return false;
        }
    }
    true
}

fn find_modulus(n: u32) -> u64 {
    // trinomials first, then pentanomials
    for k in 1..n {
        let f = (1u128 << n) | (1u128 << k) | 1;
        if is_irreducible(f, n) {
            return f as u64;
        }
    }
    for k1 in 1..n {
        for k2 in 1..k1 {
            for k3 in 1..k2 {
                let f = (1u128 << n) | (1u128 << k1) | (1u128 << k2) | (1u128 << k3) | 1;
                if is_irreducible(f, n) {
                    return f as u64;
                }
            }
        }
    }
    panic!("no irreducible polynomial found for n = {n}");
}

// ---------------------------------------------------------------------------
// RNG

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }
    fn elem(&mut self, gf: &Gf) -> u64 {
        self.next() & gf.mask()
    }
}

// ---------------------------------------------------------------------------
// Binary curve y^2 + x y = x^3 + a x^2 + b

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Pt {
    Inf,
    A(u64, u64),
}

struct Curve<'a> {
    gf: &'a Gf,
    a: u64,
    b: u64,
}

impl<'a> Curve<'a> {
    fn on_curve(&self, p: Pt) -> bool {
        match p {
            Pt::Inf => true,
            Pt::A(x, y) => {
                let g = self.gf;
                let lhs = g.sq(y) ^ g.mul(x, y);
                let rhs = g.mul(g.sq(x), x) ^ g.mul(self.a, g.sq(x)) ^ self.b;
                lhs == rhs
            }
        }
    }
    fn neg(&self, p: Pt) -> Pt {
        match p {
            Pt::Inf => Pt::Inf,
            Pt::A(x, y) => Pt::A(x, x ^ y),
        }
    }
    fn add(&self, p: Pt, q: Pt) -> Pt {
        let g = self.gf;
        match (p, q) {
            (Pt::Inf, _) => q,
            (_, Pt::Inf) => p,
            (Pt::A(x1, y1), Pt::A(x2, y2)) => {
                if x1 == x2 {
                    if y1 == y2 {
                        if x1 == 0 {
                            return Pt::Inf;
                        }
                        // doubling
                        let l = x1 ^ g.mul(y1, g.inv(x1));
                        let x3 = g.sq(l) ^ l ^ self.a;
                        let y3 = g.sq(x1) ^ g.mul(l ^ 1, x3);
                        Pt::A(x3, y3)
                    } else {
                        Pt::Inf
                    }
                } else {
                    let l = g.mul(y1 ^ y2, g.inv(x1 ^ x2));
                    let x3 = g.sq(l) ^ l ^ x1 ^ x2 ^ self.a;
                    let y3 = g.mul(l, x1 ^ x3) ^ x3 ^ y1;
                    Pt::A(x3, y3)
                }
            }
        }
    }
    /// Points with abscissa x (0, 1 or 2 of them).
    fn lift(&self, x: u64) -> Vec<Pt> {
        let g = self.gf;
        if x == 0 {
            return vec![Pt::A(0, g.sqrt(self.b))];
        }
        // y = x z, z^2 + z = x + a + b / x^2
        let c = x ^ self.a ^ g.mul(self.b, g.inv(g.sq(x)));
        if g.trace(c) != 0 {
            return vec![];
        }
        let z = solve_artin_schreier(g, c);
        let y = g.mul(x, z);
        vec![Pt::A(x, y), Pt::A(x, y ^ x)]
    }
    /// Semaev S3 for this curve: (x1 x2 + x1 x3 + x2 x3)^2 + x1 x2 x3 + b.
    fn s3(&self, x1: u64, x2: u64, x3: u64) -> u64 {
        let g = self.gf;
        let s = g.mul(x1, x2) ^ g.mul(x1, x3) ^ g.mul(x2, x3);
        g.sq(s) ^ g.mul(g.mul(x1, x2), x3) ^ self.b
    }
}

/// Solve z^2 + z = c (requires Tr(c) = 0). Generic method: the F_2-linear map
/// L(z) = z^2 + z has kernel {0, 1}; we solve by Gaussian elimination on the
/// matrix of L (n x n over F_2). Slow path, used only to build the solver table.
fn solve_artin_schreier(g: &Gf, c: u64) -> u64 {
    // Build columns L(e_i) and row-reduce [L^T | I] once per call (cheap for n <= 62).
    let n = g.n as usize;
    // rows: for each basis vector e_i, the image L(e_i) with a tag e_i
    let mut rows: Vec<(u64, u64)> = (0..n).map(|i| (g.sq(1u64 << i) ^ (1u64 << i), 1u64 << i)).collect();
    // Eliminate to express c as a combination of images.
    let mut target = c;
    let mut combo = 0u64;
    let mut used = vec![false; n];
    for bit in (0..n).rev() {
        let m = 1u64 << bit;
        // find a row with this bit set
        let mut piv = None;
        for (ri, r) in rows.iter().enumerate() {
            if !used[ri] && r.0 & m != 0 {
                piv = Some(ri);
                break;
            }
        }
        if let Some(pi) = piv {
            used[pi] = true;
            let (pv, pt) = rows[pi];
            for (ri, r) in rows.iter_mut().enumerate() {
                if ri != pi && r.0 & m != 0 {
                    r.0 ^= pv;
                    r.1 ^= pt;
                }
            }
            if target & m != 0 {
                target ^= pv;
                combo ^= pt;
            }
        }
    }
    assert!(target == 0, "Artin-Schreier equation has no solution (trace nonzero)");
    debug_assert!(g.sq(combo) ^ combo == c);
    combo
}

/// Fast Artin-Schreier solver: precomputed linear pseudo-inverse.
struct AsSolver {
    n: usize,
    /// rows[i] = mask m_i such that z = sum_i parity(m_i & c) e_i for c in image(L)
    rows: Vec<u64>,
}

impl AsSolver {
    fn new(g: &Gf) -> Self {
        let n = g.n as usize;
        // We want, for c in image(L), z with L(z) = c. Compute z's i-th coordinate as
        // a linear functional of c: z_i = parity(rows[i] & c). Build by solving for
        // each basis vector of the image... Simpler: solve L(z) = c for c = each e_j
        // with Tr(e_j) = 0, and for e_j with Tr = 1 use e_j + e_{j0} where Tr(e_{j0}) = 1.
        let j0 = (0..n).find(|&j| g.trace(1u64 << j) == 1).expect("some basis vector has trace 1");
        let mut zcols: Vec<u64> = vec![0; n]; // zcols[j] = z with L(z) = e_j (+ e_{j0} if needed)
        for j in 0..n {
            let c = if g.trace(1u64 << j) == 0 { 1u64 << j } else { (1u64 << j) ^ (1u64 << j0) };
            zcols[j] = solve_artin_schreier(g, c);
        }
        // For c in image: c = sum_j c_j e_j, and sum_j c_j (e_j + [tr(e_j)=1] e_{j0}) = c + (#odd) e_{j0}
        // where #odd = sum_j c_j tr(e_j) = Tr(c) = 0 for c in the image. So z = sum_j c_j zcols[j].
        let mut rows = vec![0u64; n];
        for i in 0..n {
            let mut m = 0u64;
            for j in 0..n {
                if (zcols[j] >> i) & 1 == 1 {
                    m |= 1u64 << j;
                }
            }
            rows[i] = m;
        }
        AsSolver { n, rows }
    }
    #[inline]
    fn solve(&self, c: u64) -> u64 {
        let mut z = 0u64;
        for i in 0..self.n {
            if (self.rows[i] & c).count_ones() & 1 == 1 {
                z |= 1u64 << i;
            }
        }
        z
    }
}

// ---------------------------------------------------------------------------
// F_2 subspace with membership test and coordinates

struct Subspace {
    basis: Vec<u64>,      // v_1..v_{n'}
    red: Vec<(u64, u64)>, // reduced vectors with their coordinate combos
}

impl Subspace {
    fn new(basis: Vec<u64>) -> Option<Self> {
        let mut red: Vec<(u64, u64)> = Vec::new();
        for (i, &v) in basis.iter().enumerate() {
            let mut x = v;
            let mut combo = 1u64 << i;
            for &(rv, rc) in &red {
                let top = 1u64 << (63 - rv.leading_zeros());
                if x & top != 0 {
                    x ^= rv;
                    combo ^= rc;
                }
            }
            if x == 0 {
                return None; // dependent
            }
            // keep red sorted by leading bit descending so reduction works
            red.push((x, combo));
            red.sort_by(|a, b| b.0.leading_zeros().cmp(&a.0.leading_zeros()));
            // after sort, earlier entries have higher leading bits; re-reduce fully
            // (full reduced echelon not needed for membership+coords; descending order suffices)
        }
        Some(Subspace { basis, red })
    }
    /// Coordinates of x in the basis if x in V.
    #[inline]
    fn coords(&self, x: u64) -> Option<u64> {
        let mut x = x;
        let mut combo = 0u64;
        for &(rv, rc) in &self.red {
            let top = 1u64 << (63 - rv.leading_zeros());
            if x & top != 0 {
                x ^= rv;
                combo ^= rc;
            }
        }
        if x == 0 {
            Some(combo)
        } else {
            None
        }
    }
    fn element(&self, g: &Gf, coords: u64) -> u64 {
        let _ = g;
        let mut x = 0u64;
        for (i, &v) in self.basis.iter().enumerate() {
            if (coords >> i) & 1 == 1 {
                x ^= v;
            }
        }
        x
    }
}

// ---------------------------------------------------------------------------
// F_2 bit matrices

struct BitMat {
    rows: Vec<Vec<u64>>,
    words: usize,
}

impl BitMat {
    fn new(words: usize) -> Self {
        BitMat { rows: Vec::new(), words }
    }
    fn push_row(&mut self, r: Vec<u64>) {
        debug_assert!(r.len() == self.words);
        self.rows.push(r);
    }
    /// Row-reduce choosing pivots in increasing column order. Returns the pivot
    /// columns in order found.
    fn echelon(&mut self) -> Vec<usize> {
        let mut pivots = Vec::new();
        let nrows = self.rows.len();
        let mut r0 = 0usize;
        for col in 0..self.words * 64 {
            if r0 >= nrows {
                break;
            }
            let w = col / 64;
            let m = 1u64 << (col % 64);
            let mut piv = None;
            for r in r0..nrows {
                if self.rows[r][w] & m != 0 {
                    piv = Some(r);
                    break;
                }
            }
            let Some(pr) = piv else { continue };
            self.rows.swap(r0, pr);
            let (head, tail) = self.rows.split_at_mut(r0 + 1);
            let prow = &head[r0];
            for r in tail.iter_mut() {
                if r[w] & m != 0 {
                    for k in w..self.words {
                        r[k] ^= prow[k];
                    }
                }
            }
            pivots.push(col);
            r0 += 1;
        }
        pivots
    }
}

fn rank_of_u64_rows(rows: &[u64]) -> usize {
    let mut basis: Vec<u64> = Vec::new();
    for &r in rows {
        let mut x = r;
        for &b in &basis {
            let top = 1u64 << (63 - b.leading_zeros());
            if x & top != 0 {
                x ^= b;
            }
        }
        if x != 0 {
            basis.push(x);
            basis.sort_by(|a, b| b.leading_zeros().cmp(&a.leading_zeros()));
        }
    }
    basis.len()
}

// ---------------------------------------------------------------------------
// Monomials in N variables up to degree d: canonical encoding as sorted variable
// indices in a u64 (8 bits each, 0xFF = unused), highest-degree first ordering.

fn mono_encode(vars: &[u8]) -> u64 {
    let mut v = vars.to_vec();
    v.sort_unstable();
    let mut code: u64 = u64::MAX;
    for (i, &x) in v.iter().enumerate() {
        code &= !(0xFFu64 << (8 * i));
        code |= (x as u64) << (8 * i);
    }
    code
}

fn mono_vars(code: u64) -> Vec<u8> {
    let mut v = Vec::new();
    for i in 0..8 {
        let x = ((code >> (8 * i)) & 0xFF) as u8;
        if x != 0xFF {
            v.push(x);
        }
    }
    v
}

fn mono_mul(a: u64, b: u64) -> u64 {
    let mut v = mono_vars(a);
    v.extend(mono_vars(b));
    mono_encode(&v)
}

/// All monomials in nvars variables of degree exactly d (multisets).
fn monomials_of_degree(nvars: usize, d: usize) -> Vec<u64> {
    let mut out = Vec::new();
    let mut cur: Vec<u8> = Vec::new();
    fn rec(nvars: usize, d: usize, start: usize, cur: &mut Vec<u8>, out: &mut Vec<u64>) {
        if cur.len() == d {
            out.push(mono_encode(cur));
            return;
        }
        for v in start..nvars {
            cur.push(v as u8);
            rec(nvars, d, v, cur, out);
            cur.pop();
        }
    }
    rec(nvars, d, 0, &mut cur, &mut out);
    out
}

/// A polynomial over F_2 as a set of monomial codes (xor-set).
type Poly = Vec<u64>;

fn poly_mul_mono(p: &Poly, m: u64) -> Poly {
    p.iter().map(|&t| mono_mul(t, m)).collect()
}

// ---------------------------------------------------------------------------
// Instance

struct Instance {
    n: u32,
    nprime: usize,
    big_n: usize,
    a: u64,
    b: u64,
    basis: Vec<u64>,
    x_r: u64,
    /// descended equations: n polynomials in variables a_0..a_{n'-1} (indices 0..n'-1)
    /// and b_0..b_{n'-1} (indices n'..2n'-1)
    eqs: Vec<Poly>,
    /// bilinear coefficient matrix for rho: row l, column i*n'+j
    quad_rows: Vec<Vec<u64>>, // each row as bit vector over n'^2 columns (words)
}

fn build_instance(g: &Gf, cv: &Curve, sub: &Subspace, x_r: u64) -> Instance {
    let n = g.n as usize;
    let np = sub.basis.len();
    let big_n = 2 * np;
    // coefficients
    // c_ij = (v_i v_j)^2 + v_i v_j x_R ; la_i = (v_i x_R)^2 ; lb_j = (v_j x_R)^2 ; const b
    let mut eqs: Vec<Poly> = vec![Vec::new(); n];
    let words = (np * np + 63) / 64;
    let mut quad_rows: Vec<Vec<u64>> = vec![vec![0u64; words]; n];
    for i in 0..np {
        for j in 0..np {
            let vv = g.mul(sub.basis[i], sub.basis[j]);
            let c = g.sq(vv) ^ g.mul(vv, x_r);
            let mono = mono_encode(&[i as u8, (np + j) as u8]);
            for l in 0..n {
                if (c >> l) & 1 == 1 {
                    eqs[l].push(mono);
                    let col = i * np + j;
                    quad_rows[l][col / 64] |= 1u64 << (col % 64);
                }
            }
        }
    }
    for i in 0..np {
        let la = g.sq(g.mul(sub.basis[i], x_r));
        let ma = mono_encode(&[i as u8]);
        let mb = mono_encode(&[(np + i) as u8]);
        for l in 0..n {
            if (la >> l) & 1 == 1 {
                eqs[l].push(ma);
                eqs[l].push(mb);
            }
        }
    }
    let one = mono_encode(&[]);
    for l in 0..n {
        if (cv.b >> l) & 1 == 1 {
            eqs[l].push(one);
        }
    }
    Instance { n: g.n, nprime: np, big_n, a: cv.a, b: cv.b, basis: sub.basis.clone(), x_r, eqs, quad_rows }
}

/// Evaluate a descended polynomial at a bit assignment (a | b << n').
fn eval_poly(p: &Poly, assign: u64) -> u64 {
    let mut acc = 0u64;
    'outer: for &m in p {
        for v in mono_vars(m) {
            if (assign >> v) & 1 == 0 {
                continue 'outer;
            }
        }
        acc ^= 1;
    }
    acc
}

/// Enumerate Z: all (a, b) with S3(x1(a), x2(b), x_R) = 0.
fn enumerate_z(g: &Gf, cv: &Curve, sub: &Subspace, x_r: u64, solver: &AsSolver) -> Vec<u64> {
    let np = sub.basis.len();
    let mut z = Vec::new();
    for ca in 0..(1u64 << np) {
        let x1 = sub.element(g, ca);
        // A X^2 + B X + C = 0 with A = (x1 + x3)^2, B = x1 x3, C = (x1 x3)^2 + b
        let aa = g.sq(x1 ^ x_r);
        let bb = g.mul(x1, x_r);
        let cc = g.sq(bb) ^ cv.b;
        let mut roots: Vec<u64> = Vec::new();
        if aa == 0 {
            if bb != 0 {
                roots.push(g.mul(cc, g.inv(bb)));
            } else if cc == 0 {
                // identically zero: every X is a root (cannot happen: C = b != 0)
                unreachable!("degenerate S3 row");
            }
        } else if bb == 0 {
            roots.push(g.sqrt(g.mul(cc, g.inv(aa))));
        } else {
            let binv = g.inv(bb);
            let d = g.mul(g.mul(aa, cc), g.sq(binv));
            if g.trace(d) == 0 {
                let y = solver.solve(d);
                let scale = g.mul(bb, g.inv(aa));
                roots.push(g.mul(scale, y));
                roots.push(g.mul(scale, y ^ 1));
            }
        }
        for x2 in roots {
            debug_assert!(cv.s3(x1, x2, x_r) == 0);
            if let Some(cb) = sub.coords(x2) {
                z.push(ca | (cb << np));
            }
        }
    }
    z.sort_unstable();
    z.dedup();
    z
}

fn affine_dim(z: &[u64]) -> usize {
    if z.is_empty() {
        return 0;
    }
    let z0 = z[0];
    let diffs: Vec<u64> = z.iter().map(|&v| v ^ z0).collect();
    rank_of_u64_rows(&diffs)
}

/// dim( rowspace(M_{<=d}) ∩ P_{<=1} ) for the generators (descended eqs + field eqs).
fn low_dim(inst: &Instance, d: usize) -> (usize, usize, usize) {
    let nv = inst.big_n;
    // column ordering: degree d, d-1, ..., 2 first (block H), then degree 1, 0 (block L)
    let mut cols: Vec<u64> = Vec::new();
    for deg in (2..=d).rev() {
        cols.extend(monomials_of_degree(nv, deg));
    }
    let h_count = cols.len();
    cols.extend(monomials_of_degree(nv, 1));
    cols.extend(monomials_of_degree(nv, 0));
    let index: HashMap<u64, usize> = cols.iter().enumerate().map(|(i, &c)| (c, i)).collect();
    let words = (cols.len() + 63) / 64;
    let mut mat = BitMat::new(words);
    // generators
    let mut gens: Vec<Poly> = inst.eqs.clone();
    for v in 0..nv {
        gens.push(vec![mono_encode(&[v as u8, v as u8]), mono_encode(&[v as u8])]);
    }
    for gpoly in &gens {
        if gpoly.is_empty() {
            continue;
        }
        let gdeg = gpoly.iter().map(|&m| mono_vars(m).len()).max().unwrap_or(0);
        for mdeg in 0..=(d.saturating_sub(gdeg)) {
            for m in monomials_of_degree(nv, mdeg) {
                let p = poly_mul_mono(gpoly, m);
                let mut row = vec![0u64; words];
                for t in p {
                    let ci = index[&t];
                    row[ci / 64] ^= 1u64 << (ci % 64);
                }
                mat.push_row(row);
            }
        }
    }
    let nrows = mat.rows.len();
    let pivots = mat.echelon();
    let rank = pivots.len();
    let rank_h = pivots.iter().filter(|&&c| c < h_count).count();
    (rank - rank_h, rank, nrows)
}

fn hex(x: u64) -> String {
    format!("0x{x:x}")
}

fn main() {
    let t_all = Instant::now();
    let seed: u64 = 0x5eed_2026_1104;
    println!("pq2_rank census; seed {seed:#x}; pclmulqdq: {}", std::is_x86_feature_detected!("pclmulqdq"));
    let mut rng = Rng(seed);

    let ns: Vec<u32> = (8..=48).collect::<Vec<_>>().into_iter().chain([50, 52, 54, 56]).collect();
    let instances_per_n = 3usize;
    let d4_max_n = 20u32;

    let mut json = String::new();
    json.push_str("{\n  \"schema\": \"pq2_rank.census/v1\",\n");
    let _ = writeln!(json, "  \"seed\": \"{seed:#x}\",");
    json.push_str("  \"rows\": [\n");
    let mut csv = String::from("n,nprime,N,instance,a,b,x_R,F_size,F_over_V,Z_size,affdim,k,rho,low2,low3,low4,d_lin,ms_z,ms_low3,ms_low4\n");
    let mut first = true;

    for &n in &ns {
        let f = find_modulus(n);
        let g = Gf { n, f };
        // self-tests for the field
        {
            let x = rng.elem(&g);
            let y = rng.elem(&g);
            let z = rng.elem(&g);
            assert_eq!(g.mul(g.mul(x, y), z), g.mul(x, g.mul(y, z)));
            if x != 0 {
                assert_eq!(g.mul(x, g.inv(x)), 1);
            }
            assert_eq!(g.sq(g.sqrt(x)), x);
            assert_eq!(g.pow(x, 1u128 << n), x);
        }
        let solver = AsSolver::new(&g);
        {
            let c = rng.elem(&g);
            if g.trace(c) == 0 {
                let z = solver.solve(c);
                assert_eq!(g.sq(z) ^ z, c);
            }
        }
        let np = ((n + 1) / 2) as usize;
        let mut done = 0usize;
        let mut tries = 0usize;
        while done < instances_per_n && tries < 200 {
            tries += 1;
            // random ordinary curve (b != 0)
            let a = rng.elem(&g);
            let b = loop {
                let v = rng.elem(&g);
                if v != 0 {
                    break v;
                }
            };
            let cv = Curve { gf: &g, a, b };
            // curve self-tests: S3 vanishes on x(P), x(Q), x(P+Q); group law associativity
            {
                let mut pts: Vec<Pt> = Vec::new();
                while pts.len() < 3 {
                    let x = rng.elem(&g);
                    let l = cv.lift(x);
                    for p in l {
                        assert!(cv.on_curve(p));
                        pts.push(p);
                    }
                }
                let (p, q, r) = (pts[0], pts[1], pts[2]);
                assert_eq!(cv.add(cv.add(p, q), r), cv.add(p, cv.add(q, r)));
                if let (Pt::A(x1, _), Pt::A(x2, _), Pt::A(x3, _)) = (p, q, cv.add(p, q)) {
                    assert_eq!(cv.s3(x1, x2, x3), 0, "S3 does not vanish on a true sum");
                }
                assert!(cv.on_curve(cv.neg(p)));
                assert_eq!(cv.add(p, cv.neg(p)), Pt::Inf);
            }
            // random subspace
            let basis: Vec<u64> = (0..np).map(|_| rng.elem(&g)).collect();
            let Some(sub) = Subspace::new(basis.clone()) else { continue };
            // random target with a nonempty solution set: x_R from a random point R
            let x_r = loop {
                let x = rng.elem(&g);
                if !cv.lift(x).is_empty() {
                    break x;
                }
            };
            let t0 = Instant::now();
            let z = enumerate_z(&g, &cv, &sub, x_r, &solver);
            let ms_z = t0.elapsed().as_millis();
            if z.is_empty() {
                continue; // inconsistent system: 1 in the ideal; not an instance for the criterion
            }
            let inst = build_instance(&g, &cv, &sub, x_r);
            // consistency: every z in Z satisfies all descended equations, and a random
            // non-solution assignment fails at least one (bitwise equality of descents)
            for &zz in &z {
                for e in &inst.eqs {
                    assert_eq!(eval_poly(e, zz), 0, "descended equation nonzero on a solution");
                }
            }
            {
                let mut checked = 0;
                while checked < 20 {
                    let asg = rng.next() & ((1u64 << inst.big_n) - 1);
                    let x1 = sub.element(&g, asg & ((1u64 << np) - 1));
                    let x2 = sub.element(&g, asg >> np);
                    let direct = cv.s3(x1, x2, x_r);
                    let mut desc = 0u64;
                    for (l, e) in inst.eqs.iter().enumerate() {
                        desc |= eval_poly(e, asg) << l;
                    }
                    assert_eq!(direct, desc, "Weil descent disagrees with direct S3 evaluation");
                    checked += 1;
                }
            }
            let adim = affine_dim(&z);
            let k = inst.big_n - adim;
            let rho = {
                let words = inst.quad_rows[0].len();
                let mut m = BitMat::new(words);
                for r in &inst.quad_rows {
                    m.push_row(r.clone());
                }
                m.echelon().len()
            };
            // |F'| for n <= 40
            let f_size: Option<usize> = if n <= 40 {
                let mut cnt = 0usize;
                for c in 0..(1u64 << np) {
                    cnt += cv.lift(sub.element(&g, c)).len();
                }
                Some(cnt)
            } else {
                None
            };
            let (low2, _, _) = low_dim(&inst, 2);
            assert!(low2 + rho <= n as usize, "degree-2 bound violated: low2 + rho > n");
            let t3 = Instant::now();
            let (low3, _, _) = low_dim(&inst, 3);
            let ms3 = t3.elapsed().as_millis();
            let (low4, ms4) = if n <= d4_max_n {
                let t4 = Instant::now();
                let (l4, _, _) = low_dim(&inst, 4);
                (Some(l4), t4.elapsed().as_millis())
            } else {
                (None, 0)
            };
            assert!(low2 <= low3);
            if let Some(l4) = low4 {
                assert!(low3 <= l4);
            }
            let d_lin: String = if low2 >= k {
                "2".into()
            } else if low3 >= k {
                "3".into()
            } else if let Some(l4) = low4 {
                if l4 >= k { "4".into() } else { ">4".into() }
            } else {
                ">3".into()
            };
            println!(
                "n={n:2} n'={np:2} N={:2} inst={done} |Z|={:2} affdim={adim} k={k:2} rho={rho:2} low2={low2:2} low3={low3:2} low4={} d_lin={d_lin} ({ms_z} ms Z, {ms3} ms deg3, {ms4} ms deg4)",
                inst.big_n,
                z.len(),
                low4.map(|v| v.to_string()).unwrap_or("-".into())
            );
            if !first {
                json.push_str(",\n");
            }
            first = false;
            let _ = write!(
                json,
                "    {{\"n\": {n}, \"nprime\": {np}, \"N\": {}, \"instance\": {done}, \"a\": \"{}\", \"b\": \"{}\", \"modulus\": \"{}\", \"basis\": [{}], \"x_R\": \"{}\", \"F_size\": {}, \"Z_size\": {}, \"affdim\": {adim}, \"k\": {k}, \"rho\": {rho}, \"low2\": {low2}, \"low3\": {low3}, \"low4\": {}, \"d_lin\": \"{d_lin}\", \"ms_z\": {ms_z}, \"ms_low3\": {ms3}, \"ms_low4\": {ms4}}}",
                inst.big_n,
                hex(a),
                hex(b),
                hex(f),
                basis.iter().map(|&v| format!("\"{}\"", hex(v))).collect::<Vec<_>>().join(", "),
                hex(x_r),
                f_size.map(|v| v.to_string()).unwrap_or("null".into()),
                z.len(),
                low4.map(|v| v.to_string()).unwrap_or("null".into()),
            );
            let _ = writeln!(
                csv,
                "{n},{np},{},{done},{},{},{},{},{},{},{adim},{k},{rho},{low2},{low3},{},{d_lin},{ms_z},{ms3},{ms4}",
                inst.big_n,
                hex(a),
                hex(b),
                hex(x_r),
                f_size.map(|v| v.to_string()).unwrap_or_default(),
                f_size.map(|v| format!("{:.4}", v as f64 / (1u64 << np) as f64)).unwrap_or_default(),
                z.len(),
                low4.map(|v| v.to_string()).unwrap_or_default(),
            );
            done += 1;
            let _ = &inst.a;
            let _ = &inst.basis;
            let _ = inst.x_r;
            let _ = inst.nprime;
            let _ = inst.n;
        }
    }
    json.push_str("\n  ]\n}\n");
    std::fs::create_dir_all("results").unwrap();
    std::fs::write("results/pq2_rank.json", json).unwrap();
    std::fs::write("results/pq2_rank.csv", csv).unwrap();
    println!("done in {:.1} s; wrote results/pq2_rank.json and results/pq2_rank.csv", t_all.elapsed().as_secs_f64());
}
