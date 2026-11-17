//! Degree probe for the digit functions of the canonical torsion section.
//!
//! For a prime-order curve `E: y^2 = x^3 + a x + b` over `F_p` (`r = #E(F_p)`
//! prime, `p != r`), the prime-to-p torsion of `E(Z/p^k)` maps isomorphically
//! onto `E(F_p)`, giving a homomorphic section `s_k`.  The digits `x_j` of
//! `x(s_k(P)) = x + p x_1 + p^2 x_2 + ...` depend on the integer lift of
//! `(a, b)` in `[0, p)`.  The probe asks for the least `d` such that the digit
//! vector on a Kummer base `V = g * mu_{2^t}` lies in the span of the monomials
//! `x^e` with `popcount(e) <= d`.
//!
//! `jet_digit_degree --p P --t T --k K [--a A --b B] [--seed S] [--curves N]
//!                   [--out FILE.jsonl] [--verify] [--tower kummer|dickson]`
//!
//! `--tower dickson` (split case, `2^t | p - 1`) replaces the Kummer base by
//! `V = { w + 1/w : w = u * omega^i }` (`u^2` outside `mu_{2^t}`) with tower
//! variables `y_0 = v`, `y_{j+1} = y_j^2 - 2`; the monomial basis is the set of
//! multilinear monomials `prod_{j in J} y_j` of degree `|J|`.
//!
//! Assumption: the affine lift `(x, y, 1)` is made to lie on the lifted curve
//! mod `p^k` by Hensel-lifting `y` (x is kept as the integer in `[0, p)`), so
//! the torsion projection `[u]P~` is independent of the lift.
use crypto_lib::cryptanalysis::ic_boundary::PrimeCurve;
use std::io::Write;
use std::time::Instant;

// ---------------------------------------------------------------- ring maths
fn mulmod(a: u128, b: u128, m: u128) -> u128 {
    (a % m) * (b % m) % m
}
fn addmod(a: u128, b: u128, m: u128) -> u128 {
    (a + b) % m
}
fn submod(a: u128, b: u128, m: u128) -> u128 {
    (a % m + m - b % m) % m
}
fn powmod(mut a: u128, mut e: u128, m: u128) -> u128 {
    let mut r = 1 % m;
    a %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = mulmod(r, a, m);
        }
        a = mulmod(a, a, m);
        e >>= 1;
    }
    r
}
/// Inverse of `a` modulo `m` (extended Euclid); requires gcd(a, m) = 1.
fn invmod(a: u128, m: u128) -> u128 {
    let (mut r0, mut r1) = (m as i128, (a % m) as i128);
    let (mut t0, mut t1) = (0i128, 1i128);
    while r1 != 0 {
        let q = r0 / r1;
        (r0, r1) = (r1, r0 - q * r1);
        (t0, t1) = (t1, t0 - q * t1);
    }
    assert_eq!(r0, 1, "invmod: gcd(a, m) != 1");
    t0.rem_euclid(m as i128) as u128
}

// ------------------------------------------------------------ complete curve
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Proj {
    x: u128,
    y: u128,
    z: u128,
}
const IDENT: Proj = Proj { x: 0, y: 1, z: 0 };

#[derive(Clone, Copy)]
struct Ring {
    m: u128,
    a: u128,
    b3: u128,
}
impl Ring {
    fn new(m: u128, a: u64, b: u64) -> Self {
        Ring {
            m,
            a: a as u128 % m,
            b3: 3 * b as u128 % m,
        }
    }
    /// Renes-Costello-Batina 2016, Algorithm 1 (ADD, arbitrary a).
    fn add(&self, p: Proj, q: Proj) -> Proj {
        let m = self.m;
        let (a, b3) = (self.a, self.b3);
        let mul = |x, y| mulmod(x, y, m);
        let ad = |x, y| addmod(x, y, m);
        let sb = |x, y| submod(x, y, m);
        let (x1, y1, z1, x2, y2, z2) = (p.x, p.y, p.z, q.x, q.y, q.z);
        let mut t0 = mul(x1, x2);
        let mut t1 = mul(y1, y2);
        let mut t2 = mul(z1, z2);
        let mut t3 = ad(x1, y1);
        let mut t4 = ad(x2, y2);
        t3 = mul(t3, t4);
        t4 = ad(t0, t1);
        t3 = sb(t3, t4);
        t4 = ad(x1, z1);
        let mut t5 = ad(x2, z2);
        t4 = mul(t4, t5);
        t5 = ad(t0, t2);
        t4 = sb(t4, t5);
        t5 = ad(y1, z1);
        let mut x3 = ad(y2, z2);
        t5 = mul(t5, x3);
        x3 = ad(t1, t2);
        t5 = sb(t5, x3);
        let mut z3 = mul(a, t4);
        x3 = mul(b3, t2);
        z3 = ad(x3, z3);
        x3 = sb(t1, z3);
        z3 = ad(t1, z3);
        let mut y3 = mul(x3, z3);
        t1 = ad(t0, t0);
        t1 = ad(t1, t0);
        t2 = mul(a, t2);
        t4 = mul(b3, t4);
        t1 = ad(t1, t2);
        t2 = sb(t0, t2);
        t2 = mul(a, t2);
        t4 = ad(t4, t2);
        t0 = mul(t1, t4);
        y3 = ad(y3, t0);
        t0 = mul(t5, t4);
        x3 = mul(t3, x3);
        x3 = sb(x3, t0);
        t0 = mul(t3, t1);
        z3 = mul(t5, z3);
        z3 = ad(z3, t0);
        Proj {
            x: x3,
            y: y3,
            z: z3,
        }
    }
    /// Renes-Costello-Batina 2016, Algorithm 3 (DBL, arbitrary a).
    fn dbl(&self, p: Proj) -> Proj {
        let m = self.m;
        let (a, b3) = (self.a, self.b3);
        let mul = |x, y| mulmod(x, y, m);
        let ad = |x, y| addmod(x, y, m);
        let sb = |x, y| submod(x, y, m);
        let (x, y, z) = (p.x, p.y, p.z);
        let mut t0 = mul(x, x);
        let t1 = mul(y, y);
        let mut t2 = mul(z, z);
        let mut t3 = mul(x, y);
        t3 = ad(t3, t3);
        let mut z3 = mul(x, z);
        z3 = ad(z3, z3);
        let mut x3 = mul(a, z3);
        let mut y3 = mul(b3, t2);
        y3 = ad(x3, y3);
        x3 = sb(t1, y3);
        y3 = ad(t1, y3);
        y3 = mul(x3, y3);
        x3 = mul(t3, x3);
        z3 = mul(b3, z3);
        t2 = mul(a, t2);
        t3 = sb(t0, t2);
        t3 = mul(a, t3);
        t3 = ad(t3, z3);
        z3 = ad(t0, t0);
        t0 = ad(z3, t0);
        t0 = ad(t0, t2);
        t0 = mul(t0, t3);
        y3 = ad(y3, t0);
        t2 = mul(y, z);
        t2 = ad(t2, t2);
        t0 = mul(t2, t3);
        x3 = sb(x3, t0);
        z3 = mul(t2, t1);
        z3 = ad(z3, z3);
        z3 = ad(z3, z3);
        Proj {
            x: x3,
            y: y3,
            z: z3,
        }
    }
    fn mul_scalar(&self, p: Proj, k: u128) -> Proj {
        let mut acc = IDENT;
        for i in (0..128).rev() {
            acc = self.dbl(acc);
            if (k >> i) & 1 == 1 {
                acc = self.add(acc, p);
            }
        }
        acc
    }
    /// Normalise to affine `(x, y)`; `Z` must be a unit (Z mod p != 0 is the
    /// caller's check, since the ring does not know p).
    fn affine(&self, p: Proj) -> (u128, u128) {
        let zi = invmod(p.z, self.m);
        (mulmod(p.x, zi, self.m), mulmod(p.y, zi, self.m))
    }
}

// ------------------------------------------------------------------ rng, F_p
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    fn below(self_: &mut Self, n: u64) -> u64 {
        self_.next() % n
    }
}

fn is_square(a: u64, p: u64) -> bool {
    a.is_multiple_of(p) || powmod(a as u128, ((p - 1) / 2) as u128, p as u128) == 1
}
/// Tonelli-Shanks square root mod odd prime p (a must be a residue).
fn sqrt_mod(a: u64, p: u64) -> u64 {
    let (a, pp) = (a as u128 % p as u128, p as u128);
    if a == 0 {
        return 0;
    }
    let mut q = pp - 1;
    let mut s = 0;
    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }
    let mut z = 2u128;
    while powmod(z, (pp - 1) / 2, pp) != pp - 1 {
        z += 1;
    }
    let (mut m, mut c) = (s, powmod(z, q, pp));
    let mut t = powmod(a, q, pp);
    let mut r = powmod(a, q.div_ceil(2), pp);
    while t != 1 {
        let mut i = 0;
        let mut tt = t;
        while tt != 1 {
            tt = mulmod(tt, tt, pp);
            i += 1;
        }
        let mut b = c;
        for _ in 0..(m - i - 1) {
            b = mulmod(b, b, pp);
        }
        m = i;
        c = mulmod(b, b, pp);
        t = mulmod(t, c, pp);
        r = mulmod(r, b, pp);
    }
    r as u64
}

// ------------------------------------------------------------------- section
#[derive(Clone, Copy)]
struct Curve {
    p: u64,
    a: u64,
    b: u64,
    r: u64,
    /// Lift offsets: the lifted curve has coefficients `a + la·p`, `b + lb·p`
    /// (read mod p^k). `(0, 0)` is the trivial lift of the integers in [0, p).
    la: u64,
    lb: u64,
}
impl Curve {
    fn lifted(&self) -> (u64, u64) {
        (self.a + self.la * self.p, self.b + self.lb * self.p)
    }
}
fn rhs(c: &Curve, x: u64) -> u64 {
    let (p, x) = (c.p as u128, x as u128);
    ((x * x % p * x + c.a as u128 * x + c.b as u128) % p) as u64
}
fn pk(p: u64, k: u32) -> u128 {
    (p as u128).pow(k)
}

/// Hensel-lift y (a root of y^2 = f(x) mod p) to a root mod p^k.
fn hensel_y(c: &Curve, k: u32, x: u64, y: u64) -> u128 {
    let m = pk(c.p, k);
    let xx = x as u128;
    let (la, lb) = c.lifted();
    let f = addmod(
        addmod(powmod(xx, 3, m), mulmod(la as u128 % m, xx, m), m),
        lb as u128 % m,
        m,
    );
    let mut yy = y as u128;
    for _ in 0..8 {
        let num = submod(mulmod(yy, yy, m), f, m);
        let den = invmod(mulmod(2, yy, m), m);
        yy = submod(yy, mulmod(num, den, m), m);
    }
    assert_eq!(mulmod(yy, yy, m), f, "hensel failed");
    assert_eq!(yy % c.p as u128, y as u128);
    yy
}

/// u with u = 0 mod p^{k-1} and u = 1 mod r: u = p^{k-1} * (p^{-(k-1)} mod r).
/// (The task text writes r * (r^{-1} mod p^{k-1}), which has the two residues
/// swapped; this is the scalar that has the stated properties.)
fn section_scalar(c: &Curve, k: u32) -> u128 {
    let q = pk(c.p, k - 1);
    q * invmod(q % c.r as u128, c.r as u128)
}

/// `s_k(P)` for `P = (x, y)` in E(F_p), projectively mod p^k.
fn section(c: &Curve, k: u32, x: u64, y: u64) -> Proj {
    let (la, lb) = c.lifted();
    let ring = Ring::new(pk(c.p, k), la, lb);
    let pt = Proj {
        x: x as u128,
        y: hensel_y(c, k, x, y),
        z: 1,
    };
    ring.mul_scalar(pt, section_scalar(c, k))
}

#[derive(Default, Clone, Debug)]
struct Verify {
    points: usize,
    reduces: usize,
    on_curve: usize,
    order_r: usize,
    even: usize,
    hom_pairs: usize,
}

/// Self-checks for one point; returns the normalised x-coordinate in [0, p^k).
fn check_point(c: &Curve, k: u32, x: u64, y: u64, full: bool, v: &mut Verify) -> u128 {
    let m = pk(c.p, k);
    let (la, lb) = c.lifted();
    let ring = Ring::new(m, la, lb);
    let s = section(c, k, x, y);
    assert_ne!(s.z % c.p as u128, 0, "Z not a unit");
    let (xt, yt) = ring.affine(s);
    assert_eq!(xt % c.p as u128, x as u128, "x does not reduce");
    assert_eq!(yt % c.p as u128, y as u128, "y does not reduce");
    v.reduces += 1;
    let lhs = mulmod(yt, yt, m);
    let r = addmod(
        addmod(powmod(xt, 3, m), mulmod(ring.a, xt, m), m),
        lb as u128 % m,
        m,
    );
    assert_eq!(lhs, r, "section point not on lifted curve");
    v.on_curve += 1;
    v.points += 1;
    if full {
        let o = ring.mul_scalar(s, c.r as u128);
        assert!(o.x.is_multiple_of(m) && o.z.is_multiple_of(m), "[r]s != O");
        v.order_r += 1;
    }
    xt
}

/// Homomorphism check s(P)+s(Q) == s(P+Q) for random pairs from `pts`.
fn check_hom(c: &Curve, k: u32, pts: &[(u64, u64)], pairs: usize, rng: &mut Rng, v: &mut Verify) {
    let m = pk(c.p, k);
    let (la, lb) = c.lifted();
    let ring = Ring::new(m, la, lb);
    let ring1 = Ring::new(c.p as u128, c.a, c.b);
    let mut done = 0;
    while done < pairs {
        let pick = |rng: &mut Rng| {
            let (x, y) = pts[Rng::below(rng, pts.len() as u64) as usize];
            if rng.next() & 1 == 1 {
                (x, (c.p - y) % c.p)
            } else {
                (x, y)
            }
        };
        let (p1, q1) = (pick(rng), pick(rng));
        let sp = section(c, k, p1.0, p1.1);
        let sq = section(c, k, q1.0, q1.1);
        let sum = ring1.add(
            Proj {
                x: p1.0 as u128,
                y: p1.1 as u128,
                z: 1,
            },
            Proj {
                x: q1.0 as u128,
                y: q1.1 as u128,
                z: 1,
            },
        );
        if sum.z == 0 {
            continue; // P + Q = O: no affine point to lift
        }
        let (sx, sy) = ring1.affine(sum);
        let ss = section(c, k, sx as u64, sy as u64);
        let lhs = ring.add(sp, sq);
        assert_eq!(mulmod(lhs.x, ss.z, m), mulmod(ss.x, lhs.z, m), "hom X");
        assert_eq!(mulmod(lhs.y, ss.z, m), mulmod(ss.y, lhs.z, m), "hom Y");
        done += 1;
        v.hom_pairs += 1;
    }
}

// ------------------------------------------------------------- degree tests
struct Echelon {
    n: usize,
    p: u64,
    basis: Vec<(usize, Vec<u64>)>,
}
impl Echelon {
    fn reduce(&self, v: &mut [u64]) {
        let p = self.p;
        for (piv, b) in &self.basis {
            let c = v[*piv];
            if c != 0 {
                let c = p - c;
                for i in 0..self.n {
                    v[i] = (v[i] + c * b[i]) % p;
                }
            }
        }
    }
    fn in_span(&self, f: &[u64]) -> bool {
        let mut v = f.to_vec();
        self.reduce(&mut v);
        v.iter().all(|&e| e == 0)
    }
    /// Insert a column; returns true if rank grew.
    fn insert(&mut self, col: &[u64]) -> bool {
        let mut v = col.to_vec();
        self.reduce(&mut v);
        if let Some(piv) = v.iter().position(|&e| e != 0) {
            let inv = invmod(v[piv] as u128, self.p as u128) as u64;
            for e in v.iter_mut() {
                *e = *e * inv % self.p;
            }
            self.basis.push((piv, v));
            true
        } else {
            false
        }
    }
}

/// Kummer monomial table: `rows[i][e] = vs[i]^e`, `e` in `0..2^t`.
fn kummer_rows(vs: &[u64], t: u32, p: u64) -> Vec<Vec<u64>> {
    let ne = 1usize << t;
    vs.iter()
        .map(|&v| {
            let mut row = Vec::with_capacity(ne);
            let mut cur = 1u64;
            for _ in 0..ne {
                row.push(cur);
                cur = cur * v % p;
            }
            row
        })
        .collect()
}

/// Dickson monomial table: `rows[i][J] = prod_{j in J} towers[i][j]`, with the
/// bit set `J` read as the integer `e = sum_{j in J} 2^j`.
fn dickson_rows(towers: &[Vec<u64>], t: u32, p: u64) -> Vec<Vec<u64>> {
    let ne = 1usize << t;
    towers
        .iter()
        .map(|y| {
            let mut row = vec![1u64; ne];
            for e in 1..ne {
                let low = e.trailing_zeros() as usize;
                row[e] = row[e & (e - 1)] * y[low] % p;
            }
            row
        })
        .collect()
}

/// Least popcount-degree of each function on the abscissae `vs`, and d_full
/// (Kummer columns `v^e`).
fn degrees(vs: &[u64], t: u32, p: u64, funcs: &[Vec<u64>]) -> (Vec<usize>, usize) {
    degrees_rows(&kummer_rows(vs, t, p), t, p, funcs)
}

/// Least degree of each function in the span of the columns `rows[.][e]`
/// added in order of `popcount(e)`, and d_full.
fn degrees_rows(pw: &[Vec<u64>], t: u32, p: u64, funcs: &[Vec<u64>]) -> (Vec<usize>, usize) {
    let n = pw.len();
    let ne = 1usize << t;
    let mut ech = Echelon {
        n,
        p,
        basis: Vec::new(),
    };
    let mut res: Vec<Option<usize>> = vec![None; funcs.len()];
    let mut d_full = None;
    for d in 0..=t as usize {
        if ech.basis.len() < n {
            for e in 0..ne {
                if e.count_ones() as usize == d {
                    let col: Vec<u64> = (0..n).map(|i| pw[i][e]).collect();
                    ech.insert(&col);
                    if ech.basis.len() == n {
                        break;
                    }
                }
            }
        }
        if ech.basis.len() == n && d_full.is_none() {
            d_full = Some(d);
        }
        for (i, f) in funcs.iter().enumerate() {
            if res[i].is_none() && ech.in_span(f) {
                res[i] = Some(d);
            }
        }
        if d_full.is_some() && res.iter().all(|r| r.is_some()) {
            break;
        }
    }
    (
        res.into_iter()
            .map(|r| r.expect("not in span at d = t"))
            .collect(),
        d_full.unwrap(),
    )
}

// --------------------------------------------------------------------- probe
struct Row {
    json: String,
    summary: String,
}

/// A primitive `2^t`-th root of unity in `F_p`.
fn omega_2t(p: u64, t: u32) -> u64 {
    let mut h = 2u64;
    while powmod(h as u128, ((p - 1) / 2) as u128, p as u128) != (p - 1) as u128 {
        h += 1;
    }
    powmod(h as u128, ((p - 1) >> t) as u128, p as u128) as u64
}

/// Dickson base: for each `w = u * omega^i` the tower `y_0 = w + 1/w`,
/// `y_{j+1} = y_j^2 - 2` (`j = 0..t-2`).  Returns the `2^t` towers and the
/// common top constant `c = u^{2^t} + u^{-2^t}`.
fn dickson_base(p: u64, t: u32, u: u64) -> (Vec<Vec<u64>>, u64) {
    let pp = p as u128;
    let omega = omega_2t(p, t);
    let c_expected = {
        let ut = powmod(u as u128, 1u128 << t, pp);
        ((ut + invmod(ut, pp)) % pp) as u64
    };
    let mut towers = Vec::with_capacity(1usize << t);
    let mut w = u;
    for _ in 0..(1u64 << t) {
        let v = ((w as u128 + invmod(w as u128, pp)) % pp) as u64;
        let mut y = vec![v];
        for j in 1..t as usize {
            let prev = y[j - 1] as u128;
            y.push(((prev * prev + pp - 2) % pp) as u64);
        }
        let top = y[t as usize - 1] as u128;
        assert_eq!(((top * top + pp - 2) % pp) as u64, c_expected, "top != c");
        towers.push(y);
        w = (w as u128 * omega as u128 % pp) as u64;
    }
    let mut vals: Vec<u64> = towers.iter().map(|y| y[0]).collect();
    vals.sort_unstable();
    vals.dedup();
    assert_eq!(vals.len(), 1usize << t, "Dickson base |V| != 2^t");
    (towers, c_expected)
}

/// `u` in `F_p^*` with `u^{2^{t+1}} != 1` (so `u^2` is outside `mu_{2^t}`).
fn dickson_u(p: u64, t: u32, rng: &mut Rng) -> u64 {
    loop {
        let u = 1 + Rng::below(rng, p - 1);
        if powmod(u as u128, 1u128 << (t + 1), p as u128) != 1 {
            return u;
        }
    }
}

/// Abscissae among the base values `vals` (with their indices).
fn select_abscissae(c: &Curve, vals: &[u64]) -> Vec<(usize, u64, u64)> {
    let mut out = Vec::new();
    for (i, &v) in vals.iter().enumerate() {
        let f = rhs(c, v);
        if f != 0 && is_square(f, c.p) {
            out.push((i, v, sqrt_mod(f, c.p)));
        }
    }
    out
}

fn abscissae(c: &Curve, t: u32, g: u64) -> Vec<(u64, u64)> {
    let p = c.p;
    let omega = omega_2t(p, t);
    let mut out = Vec::new();
    let mut zeta = 1u64;
    for _ in 0..(1u64 << t) {
        let v = (g as u128 * zeta as u128 % p as u128) as u64;
        let f = rhs(c, v);
        if f != 0 && is_square(f, p) {
            out.push((v, sqrt_mod(f, p)));
        }
        zeta = (zeta as u128 * omega as u128 % p as u128) as u64;
    }
    out
}

#[derive(Clone, Copy)]
enum Base {
    Kummer { g: u64 },
    Dickson { u: u64 },
}

fn probe(c: &Curve, t: u32, k: u32, base: Base, full: bool, rng: &mut Rng) -> Row {
    let t0 = Instant::now();
    let p = c.p;
    // pts: abscissae with y; towers: Dickson tower values on them.
    let (pts, towers, dick_c) = match base {
        Base::Kummer { g } => (abscissae(c, t, g), Vec::new(), 0),
        Base::Dickson { u } => {
            let (all, cc) = dickson_base(p, t, u);
            let vals: Vec<u64> = all.iter().map(|y| y[0]).collect();
            let sel = select_abscissae(c, &vals);
            let pts: Vec<(u64, u64)> = sel.iter().map(|&(_, v, y)| (v, y)).collect();
            let tw: Vec<Vec<u64>> = sel.iter().map(|&(i, _, _)| all[i].clone()).collect();
            (pts, tw, cc)
        }
    };
    let n = pts.len();
    let mut ver = Verify::default();
    let mut digits: Vec<Vec<u64>> = vec![Vec::with_capacity(n); k as usize - 1];
    for (i, &(x, y)) in pts.iter().enumerate() {
        let xt = check_point(c, k, x, y, full, &mut ver);
        for j in 1..k {
            digits[j as usize - 1].push(((xt / pk(p, j)) % p as u128) as u64);
        }
        if full && i < 10 {
            let xn = check_point(c, k, x, (p - y) % p, false, &mut Verify::default());
            assert_eq!(xn, xt, "x_j not even");
            ver.even += 1;
        }
    }
    if full {
        check_hom(c, k, &pts, 20, rng, &mut ver);
    }
    let vs: Vec<u64> = pts.iter().map(|q| q.0).collect();
    let rows = match base {
        Base::Kummer { .. } => Vec::new(),
        Base::Dickson { .. } => dickson_rows(&towers, t, p),
    };
    let (mut names, mut funcs): (Vec<String>, Vec<Vec<u64>>) = match base {
        Base::Kummer { .. } => (
            vec!["x".into(), "x3".into(), "xtop".into()],
            vec![
                vs.clone(),
                vs.iter()
                    .map(|&v| powmod(v as u128, 3, p as u128) as u64)
                    .collect(),
                vs.iter()
                    .map(|&v| powmod(v as u128, (1u128 << t) - 1, p as u128) as u64)
                    .collect(),
            ],
        ),
        Base::Dickson { .. } => (
            vec!["y0".into(), "y0y1".into(), "ytop".into()],
            vec![
                rows.iter().map(|r| r[1]).collect(),
                rows.iter().map(|r| r[3]).collect(),
                rows.iter().map(|r| r[(1usize << t) - 1]).collect(),
            ],
        ),
    };
    for j in 1..k {
        names.push(match j {
            1 => "x1".into(),
            2 => "x2".into(),
            3 => "x3digit".into(),
            _ => format!("x{}digit", j),
        });
        funcs.push(digits[j as usize - 1].clone());
    }
    let first_random = funcs.len();
    for _ in 0..3 {
        funcs.push((0..n).map(|_| Rng::below(rng, p)).collect());
    }
    let (degs, d_full) = match base {
        Base::Kummer { .. } => degrees(&vs, t, p, &funcs),
        Base::Dickson { .. } => degrees_rows(&rows, t, p, &funcs),
    };
    let mut deg_json = String::new();
    for (i, nm) in names.iter().enumerate() {
        deg_json += &format!("\"{}\":{},", nm, degs[i]);
    }
    let rnd: Vec<String> = degs[first_random..].iter().map(|d| d.to_string()).collect();
    deg_json += &format!("\"random\":[{}]", rnd.join(","));
    let secs = t0.elapsed().as_secs_f64();
    let (tower_name, g_val, tower_json) = match base {
        Base::Kummer { g } => ("kummer", g, String::new()),
        Base::Dickson { u } => ("dickson", 0, format!(",\"u\":{},\"c\":{}", u, dick_c)),
    };
    let json = format!(
        "{{\"p\":{},\"tower\":\"{}\"{},\"t\":{},\"k\":{},\"a\":{},\"b\":{},\"lift_a\":{},\"lift_b\":{},\"r\":{},\"g\":{},\"n_abscissae\":{},\"size_V\":{},\
\"lift_note\":\"lifted curve a+lift_a*p, b+lift_b*p\",\"deg\":{{{}}},\"d_full\":{},\
\"verify\":{{\"points\":{},\"reduces\":{},\"on_curve\":{},\"order_r\":{},\"even\":{},\"hom_pairs\":{}}},\
\"wall_seconds\":{:.3}}}",
        p, tower_name, tower_json, t, k, c.a, c.b, c.la, c.lb, c.r, g_val, n, 1u64 << t, deg_json, d_full,
        ver.points, ver.reduces, ver.on_curve, ver.order_r, ver.even, ver.hom_pairs, secs
    );
    let summary =
        format!(
        "p={} tower={}{} t={} k={} a={} b={} r={} g={} n={} 2^t={} deg={{{}}} d_full={} verify={:?} {:.2}s",
        p, tower_name, tower_json, t, k, c.a, c.b, c.r, g_val, n, 1u64 << t, deg_json, d_full, ver, secs
    );
    Row { json, summary }
}

/// Random (a, b) with prime #E(F_p), non-singular, p != #E.
fn find_curve(p: u64, rng: &mut Rng) -> Curve {
    loop {
        let a = Rng::below(rng, p);
        let b = Rng::below(rng, p);
        let disc = (4 * (a as u128).pow(3) + 27 * (b as u128) * (b as u128)) % p as u128;
        if disc == 0 {
            continue;
        }
        if let Some(r) = (PrimeCurve { p, a, b }).prime_order() {
            if r != p && r > 2 {
                return Curve {
                    p,
                    a,
                    b,
                    r,
                    la: 0,
                    lb: 0,
                };
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let get = |name: &str| -> Option<String> {
        args.iter()
            .position(|a| a == name)
            .map(|i| args[i + 1].clone())
    };
    let p: u64 = get("--p").map_or(40961, |s| s.parse().unwrap());
    let t: u32 = get("--t").map_or(6, |s| s.parse().unwrap());
    let k: u32 = get("--k").map_or(4, |s| s.parse().unwrap());
    let seed: u64 = get("--seed").map_or(1, |s| s.parse().unwrap());
    let lift_a: u64 = get("--lift-a").map_or(0, |s| s.parse().unwrap());
    let lift_b: u64 = get("--lift-b").map_or(0, |s| s.parse().unwrap());
    let ncurves: usize = get("--curves").map_or(1, |s| s.parse().unwrap());
    let verify = args.iter().any(|a| a == "--verify");
    let dickson = match get("--tower").as_deref() {
        None | Some("kummer") => false,
        Some("dickson") => true,
        Some(other) => panic!("unknown --tower {other} (kummer|dickson)"),
    };
    assert!(!dickson || t >= 2, "--tower dickson needs t >= 2");
    assert!(
        p < 1 << 16 && (2..=4).contains(&k),
        "need p < 2^16 and 2 <= k <= 4"
    );
    assert_eq!((p - 1) % (1u64 << t), 0, "2^t must divide p - 1");
    let mut rng = Rng(seed);
    let mut out = get("--out").map(|f| std::fs::File::create(f).unwrap());
    let fixed = match (get("--a"), get("--b")) {
        (Some(a), Some(b)) => {
            let (a, b): (u64, u64) = (a.parse().unwrap(), b.parse().unwrap());
            let r = (PrimeCurve { p, a, b })
                .prime_order()
                .expect("curve order is not prime");
            Some(Curve {
                p,
                a,
                b,
                r,
                la: 0,
                lb: 0,
            })
        }
        _ => None,
    };
    for _ in 0..ncurves {
        let mut c = fixed.unwrap_or_else(|| find_curve(p, &mut rng));
        c.la = lift_a;
        c.lb = lift_b;
        let base = if dickson {
            Base::Dickson {
                u: dickson_u(p, t, &mut rng),
            }
        } else {
            Base::Kummer {
                g: 1 + Rng::below(&mut rng, p - 1),
            }
        };
        let row = probe(&c, t, k, base, verify, &mut rng);
        println!("{}", row.summary);
        if let Some(f) = out.as_mut() {
            writeln!(f, "{}", row.json).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn some_points(c: &Curve, n: usize, rng: &mut Rng) -> Vec<(u64, u64)> {
        let mut v = Vec::new();
        while v.len() < n {
            let x = Rng::below(rng, c.p);
            let f = rhs(c, x);
            if f != 0 && is_square(f, c.p) {
                v.push((x, sqrt_mod(f, c.p)));
            }
        }
        v
    }

    #[test]
    fn invmod_powmod_basics() {
        assert_eq!(powmod(3, 4, 1000), 81);
        assert_eq!(powmod(2, 40960, 40961), 1);
        let m = 40961u128.pow(4);
        for a in [1u128, 2, 12345, 40960, m - 1] {
            assert_eq!(mulmod(a, invmod(a, m), m), 1);
        }
        assert_eq!(mulmod(7, invmod(7, 40961), 40961), 1);
    }

    #[test]
    fn section_reduces_and_has_order_r() {
        let mut rng = Rng(7);
        let c = find_curve(40961, &mut rng);
        let pts = some_points(&c, 20, &mut rng);
        for k in 2..=4 {
            let mut v = Verify::default();
            for &(x, y) in &pts {
                check_point(&c, k, x, y, true, &mut v);
            }
            assert_eq!((v.reduces, v.order_r), (20, 20));
        }
    }

    #[test]
    fn section_is_homomorphic() {
        let mut rng = Rng(11);
        let c = find_curve(40961, &mut rng);
        let pts = some_points(&c, 20, &mut rng);
        let mut v = Verify::default();
        check_hom(&c, 3, &pts, 10, &mut rng, &mut v);
        assert_eq!(v.hom_pairs, 10);
    }

    #[test]
    fn degree_sanity() {
        let mut rng = Rng(5);
        let c = find_curve(40961, &mut rng);
        let t = 5;
        let pts = abscissae(&c, t, 1 + Rng::below(&mut rng, c.p - 1));
        let vs: Vec<u64> = pts.iter().map(|q| q.0).collect();
        let p = c.p;
        let f1 = vs.clone();
        let f3: Vec<u64> = vs
            .iter()
            .map(|&v| powmod(v as u128, 3, p as u128) as u64)
            .collect();
        let (d, d_full) = degrees(&vs, t, p, &[f1, f3]);
        assert_eq!(d, vec![1, 2]);
        assert!(d_full <= t as usize);
    }

    #[test]
    fn dickson_base_and_rank() {
        let mut rng = Rng(3);
        let c = find_curve(40961, &mut rng);
        let (p, t) = (c.p, 6u32);
        let u = dickson_u(p, t, &mut rng);
        let (all, cc) = dickson_base(p, t, u);
        assert_eq!(all.len(), 64);
        let mut vals: Vec<u64> = all.iter().map(|y| y[0]).collect();
        vals.sort_unstable();
        vals.dedup();
        assert_eq!(vals.len(), 64);
        for y in &all {
            let top = y[t as usize - 1] as u128;
            assert_eq!(((top * top + p as u128 - 2) % p as u128) as u64, cc);
        }
        let vs: Vec<u64> = all.iter().map(|y| y[0]).collect();
        let sel = select_abscissae(&c, &vs);
        let tw: Vec<Vec<u64>> = sel.iter().map(|&(i, _, _)| all[i].clone()).collect();
        let rows = dickson_rows(&tw, t, p);
        let n = tw.len();
        assert!(n > 0);
        let mut ech = Echelon {
            n,
            p,
            basis: Vec::new(),
        };
        for e in 0..(1usize << t) {
            let col: Vec<u64> = rows.iter().map(|r| r[e]).collect();
            ech.insert(&col);
        }
        assert_eq!(
            ech.basis.len(),
            n,
            "multilinear columns at d = t not full rank"
        );
    }

    #[test]
    fn dickson_degree_sanity() {
        let mut rng = Rng(5);
        let c = find_curve(40961, &mut rng);
        let (p, t) = (c.p, 5u32);
        let u = dickson_u(p, t, &mut rng);
        let (all, _) = dickson_base(p, t, u);
        let vs: Vec<u64> = all.iter().map(|y| y[0]).collect();
        let sel = select_abscissae(&c, &vs);
        let tw: Vec<Vec<u64>> = sel.iter().map(|&(i, _, _)| all[i].clone()).collect();
        let rows = dickson_rows(&tw, t, p);
        let y0: Vec<u64> = rows.iter().map(|r| r[1]).collect();
        let y01: Vec<u64> = rows.iter().map(|r| r[3]).collect();
        let (d, d_full) = degrees_rows(&rows, t, p, &[y0, y01]);
        assert_eq!(d, vec![1, 2]);
        assert!(d_full <= t as usize);
    }
}
