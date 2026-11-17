//! Semaev summation polynomials over a prime field, as dense polynomials,
//! and the structural checks lead 4 of the prime-field program needs
//! (`research/prime_s5_small_root_leads_20261028/`, idea L4-B).
//!
//! `S₃(x₁, x₂, x₃)` is the classical three-point polynomial of
//! `y² = x³ + ax + b`; higher ones are built by
//!
//! ```text
//! S_{m+1}(x₁, …, x_m; z) = Res_X( S₃(x₁, x₂, X), S_m(X, x₃, …, x_m; z) )
//! ```
//!
//! with `S₃(x₁, x₂; z) = S₃(x₁, x₂, z)`, the same recursion (and argument
//! order) as `l4b_step1.gp` / `l4b_step1_s5.gp`, so values match PARI's
//! `polresultant` exactly.  For a fixed `z ∈ F_p` the result is a dense
//! polynomial in `m` variables of degree `2^{m−1}` in each, recovered by
//! evaluation on the grid `{0, …, 2^{m−1}}^m` and tensor interpolation.
//!
//! The checks:
//! - [`box_vertices_nonzero`]: the `2^m` corner coefficients of the box
//!   `[0, 2^{m−1}]^m` are nonzero, i.e. the Newton polytope is the full box
//!   (premise P2 of L4-B);
//! - [`irreducible_on_random_line`]: factor-degree patterns of restrictions
//!   to random lines (distinct-degree factorisation) leave no possible
//!   factor degree, which proves the polynomial irreducible over `F_p`
//!   (premise P1 over `F_p`).
//!   Absolute irreducibility is **not** decided this way: over `F_{p^k}`
//!   each restriction's factors split further whatever the polynomial does,
//!   so line patterns cannot separate the two.
//!
//! Field elements are `u64` below `p < 2^62`; products go through `u128`.

use rand::{rngs::StdRng, Rng, SeedableRng};

/// `F_p` arithmetic for `p < 2^62`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fp {
    pub p: u64,
}

impl Fp {
    pub fn new(p: u64) -> Self {
        assert!(p > 2 && p < (1 << 62), "Fp needs an odd p below 2^62");
        Self { p }
    }
    #[inline]
    pub fn add(self, a: u64, b: u64) -> u64 {
        let s = a + b;
        if s >= self.p {
            s - self.p
        } else {
            s
        }
    }
    #[inline]
    pub fn sub(self, a: u64, b: u64) -> u64 {
        if a >= b {
            a - b
        } else {
            a + self.p - b
        }
    }
    #[inline]
    pub fn neg(self, a: u64) -> u64 {
        if a == 0 {
            0
        } else {
            self.p - a
        }
    }
    #[inline]
    pub fn mul(self, a: u64, b: u64) -> u64 {
        ((a as u128 * b as u128) % self.p as u128) as u64
    }
    pub fn pow(self, mut a: u64, mut e: u64) -> u64 {
        let mut r = 1 % self.p;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(r, a);
            }
            a = self.mul(a, a);
            e >>= 1;
        }
        r
    }
    /// Inverse of a nonzero element (Fermat).
    pub fn inv(self, a: u64) -> u64 {
        assert!(!a.is_multiple_of(self.p), "inverse of zero");
        self.pow(a, self.p - 2)
    }
    pub fn reduce_i64(self, v: i64) -> u64 {
        v.rem_euclid(self.p as i64) as u64
    }
}

/// The curve `y² = x³ + ax + b` over `F_p`, coefficients reduced.
#[derive(Clone, Copy, Debug)]
pub struct Curve {
    pub f: Fp,
    pub a: u64,
    pub b: u64,
}

impl Curve {
    pub fn new(p: u64, a: u64, b: u64) -> Self {
        let f = Fp::new(p);
        Self {
            f,
            a: a % p,
            b: b % p,
        }
    }

    /// `S₃(u, v, w) = (u − v)²w² − 2((u + v)(uv + a) + 2b)w + (uv − a)² − 4b(u + v)`.
    pub fn s3(&self, u: u64, v: u64, w: u64) -> u64 {
        let f = self.f;
        let d = f.sub(u, v);
        let s = f.add(u, v);
        let uv = f.mul(u, v);
        let t2 = f.mul(f.mul(d, d), f.mul(w, w));
        let mid = f.add(f.mul(s, f.add(uv, self.a)), f.mul(2, self.b));
        let t1 = f.mul(f.mul(2, mid), w);
        let c = f.sub(uv, self.a);
        let t0 = f.sub(f.mul(c, c), f.mul(f.mul(4, self.b), s));
        f.add(f.sub(t2, t1), t0)
    }

    /// `S₃(u, v, X)` as coefficients in `X`, lowest first.
    fn s3_in_last(&self, u: u64, v: u64) -> [u64; 3] {
        let f = self.f;
        let d = f.sub(u, v);
        let s = f.add(u, v);
        let uv = f.mul(u, v);
        let mid = f.add(f.mul(s, f.add(uv, self.a)), f.mul(2, self.b));
        let c = f.sub(uv, self.a);
        [
            f.sub(f.mul(c, c), f.mul(f.mul(4, self.b), s)),
            f.neg(f.mul(2, mid)),
            f.mul(d, d),
        ]
    }

    /// `S_{m+1}(xs; z)` at one point, `m = xs.len() ≥ 2`, by the module's
    /// recursion.
    pub fn summation(&self, xs: &[u64], z: u64) -> u64 {
        assert!(xs.len() >= 2, "S_{{m+1}} needs m ≥ 2 summands");
        if xs.len() == 2 {
            return self.s3(xs[0], xs[1], z);
        }
        let f = self.f;
        // S_m(X, x₃, …; z) has degree 2^{m−2} in X: interpolate it.
        let deg = 1usize << (xs.len() - 2);
        let nodes: Vec<u64> = (0..=deg as u64).collect();
        let mut rest = Vec::with_capacity(xs.len() - 1);
        let values: Vec<u64> = nodes
            .iter()
            .map(|&t| {
                rest.clear();
                rest.push(t);
                rest.extend_from_slice(&xs[2..]);
                self.summation(&rest, z)
            })
            .collect();
        let inner = interpolate(f, &nodes, &values);
        let quad = self.s3_in_last(xs[0], xs[1]);
        resultant_formal(f, &quad, 2, &inner, deg)
    }

    /// `S_{m+1}(x₁, …, x_m; z)` as a dense polynomial for fixed `z`.
    pub fn summation_poly(&self, m: usize, z: u64) -> MPoly {
        assert!(
            (2..=5).contains(&m),
            "dense S_{{m+1}} is built for m = 2..5"
        );
        let d = 1usize << (m - 1);
        let side = d + 1;
        let total = side.pow(m as u32);
        let mut grid = vec![0u64; total];
        let mut xs = vec![0u64; m];
        for (idx, slot) in grid.iter_mut().enumerate() {
            let mut r = idx;
            for x in xs.iter_mut() {
                *x = (r % side) as u64;
                r /= side;
            }
            *slot = self.summation(&xs, z);
        }
        MPoly::interpolate_grid(self.f, m, d, grid)
    }
}

/// Lagrange interpolation: the coefficients (lowest first) of the unique
/// polynomial of degree `< nodes.len()` through `(nodes[i], values[i])`.
pub fn interpolate(f: Fp, nodes: &[u64], values: &[u64]) -> Vec<u64> {
    let n = nodes.len();
    let mut out = vec![0u64; n];
    for i in 0..n {
        // basis_i = Π_{j≠i} (X − x_j)/(x_i − x_j)
        let mut basis = vec![1u64];
        let mut denom = 1u64;
        for j in 0..n {
            if j == i {
                continue;
            }
            let mut next = vec![0u64; basis.len() + 1];
            for (k, &c) in basis.iter().enumerate() {
                next[k + 1] = f.add(next[k + 1], c);
                next[k] = f.sub(next[k], f.mul(c, nodes[j]));
            }
            basis = next;
            denom = f.mul(denom, f.sub(nodes[i], nodes[j]));
        }
        let scale = f.mul(values[i], f.inv(denom));
        for (k, &c) in basis.iter().enumerate() {
            out[k] = f.add(out[k], f.mul(c, scale));
        }
    }
    out
}

/// Sylvester resultant of `a` (formal degree `da`) and `b` (formal degree
/// `db`), coefficients lowest first, in PARI's `polresultant(a, b)` order.
/// Formal degrees are kept even when leading coefficients vanish, so the
/// value is the specialisation of the generic resultant.
pub fn resultant_formal(f: Fp, a: &[u64], da: usize, b: &[u64], db: usize) -> u64 {
    let n = da + db;
    if n == 0 {
        return 1;
    }
    let coef = |p: &[u64], k: usize| p.get(k).copied().unwrap_or(0);
    let mut mat = vec![vec![0u64; n]; n];
    for (r, row) in mat.iter_mut().enumerate().take(db) {
        for k in 0..=da {
            row[r + k] = coef(a, da - k);
        }
    }
    for r in 0..da {
        for k in 0..=db {
            mat[db + r][r + k] = coef(b, db - k);
        }
    }
    determinant(f, &mut mat)
}

fn determinant(f: Fp, m: &mut [Vec<u64>]) -> u64 {
    let n = m.len();
    let mut det = 1u64;
    for c in 0..n {
        let Some(piv) = (c..n).find(|&r| m[r][c] != 0) else {
            return 0;
        };
        if piv != c {
            m.swap(piv, c);
            det = f.neg(det);
        }
        det = f.mul(det, m[c][c]);
        let inv = f.inv(m[c][c]);
        for r in c + 1..n {
            if m[r][c] == 0 {
                continue;
            }
            let factor = f.mul(m[r][c], inv);
            for k in c..n {
                let t = f.mul(factor, m[c][k]);
                m[r][k] = f.sub(m[r][k], t);
            }
        }
    }
    det
}

/// A dense polynomial in `vars` variables of degree at most `deg` in each;
/// `coeffs[Σ e_i·(deg+1)^i]` is the coefficient of `Π x_i^{e_i}`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MPoly {
    pub f: Fp,
    pub vars: usize,
    pub deg: usize,
    pub coeffs: Vec<u64>,
}

impl MPoly {
    /// From values on the grid `{0, …, deg}^vars` (same indexing as
    /// `coeffs`), interpolating one variable at a time.
    pub fn interpolate_grid(f: Fp, vars: usize, deg: usize, mut grid: Vec<u64>) -> Self {
        let side = deg + 1;
        let nodes: Vec<u64> = (0..side as u64).collect();
        let mut stride = 1usize;
        for _ in 0..vars {
            let block = stride * side;
            let mut line = vec![0u64; side];
            for base in (0..grid.len()).step_by(block) {
                for off in 0..stride {
                    for (k, v) in line.iter_mut().enumerate() {
                        *v = grid[base + off + k * stride];
                    }
                    let c = interpolate(f, &nodes, &line);
                    for (k, &v) in c.iter().enumerate() {
                        grid[base + off + k * stride] = v;
                    }
                }
            }
            stride = block;
        }
        Self {
            f,
            vars,
            deg,
            coeffs: grid,
        }
    }

    pub fn coeff(&self, exps: &[usize]) -> u64 {
        let side = self.deg + 1;
        let idx = exps.iter().rev().fold(0usize, |acc, &e| acc * side + e);
        self.coeffs[idx]
    }

    pub fn eval(&self, xs: &[u64]) -> u64 {
        let f = self.f;
        let side = self.deg + 1;
        let mut acc = self.coeffs.clone();
        let mut len = acc.len();
        for &x in xs.iter().rev() {
            len /= side;
            let mut next = vec![0u64; len];
            for (i, slot) in next.iter_mut().enumerate() {
                let mut h = 0u64;
                for k in (0..side).rev() {
                    h = f.add(f.mul(h, x), acc[k * len + i]);
                }
                *slot = h;
            }
            acc = next;
        }
        acc[0]
    }

    /// Exponent vectors with a nonzero coefficient.
    pub fn support(&self) -> Vec<Vec<usize>> {
        let side = self.deg + 1;
        self.coeffs
            .iter()
            .enumerate()
            .filter(|(_, &c)| c != 0)
            .map(|(idx, _)| {
                let mut r = idx;
                (0..self.vars)
                    .map(|_| {
                        let e = r % side;
                        r /= side;
                        e
                    })
                    .collect()
            })
            .collect()
    }

    /// The restriction `t ↦ P(u + v·t)` as a univariate polynomial (lowest
    /// first), by evaluation at `vars·deg + 1` points and interpolation.
    pub fn restrict_to_line(&self, u: &[u64], v: &[u64]) -> Vec<u64> {
        let f = self.f;
        let n = self.vars * self.deg + 1;
        let nodes: Vec<u64> = (0..n as u64).collect();
        let mut pt = vec![0u64; self.vars];
        let values: Vec<u64> = nodes
            .iter()
            .map(|&t| {
                for i in 0..self.vars {
                    pt[i] = f.add(u[i], f.mul(v[i], t));
                }
                self.eval(&pt)
            })
            .collect();
        let mut c = interpolate(f, &nodes, &values);
        while c.len() > 1 && *c.last().unwrap() == 0 {
            c.pop();
        }
        c
    }
}

/// `true` when the `2^vars` corner coefficients of the box `[0, deg]^vars`
/// are all nonzero (premise P2 of L4-B: the Newton polytope is the box).
pub fn box_vertices_nonzero(poly: &MPoly) -> bool {
    (0..1usize << poly.vars).all(|mask| {
        let e: Vec<usize> = (0..poly.vars)
            .map(|i| if mask >> i & 1 == 1 { poly.deg } else { 0 })
            .collect();
        poly.coeff(&e) != 0
    })
}

// ---- univariate polynomials over F_p, lowest coefficient first ----

pub fn trim(mut a: Vec<u64>) -> Vec<u64> {
    while a.len() > 1 && *a.last().unwrap() == 0 {
        a.pop();
    }
    a
}

pub fn poly_rem(f: Fp, a: &[u64], m: &[u64]) -> Vec<u64> {
    let m = trim(m.to_vec());
    let dm = m.len() - 1;
    let lead_inv = f.inv(m[dm]);
    let mut r = trim(a.to_vec());
    while r.len() > dm && !(r.len() == 1 && r[0] == 0) {
        let dr = r.len() - 1;
        let q = f.mul(r[dr], lead_inv);
        let shift = dr - dm;
        for (k, &c) in m.iter().enumerate() {
            r[shift + k] = f.sub(r[shift + k], f.mul(q, c));
        }
        r = trim(r);
        if dr == 0 {
            break;
        }
    }
    r
}

pub fn poly_mulmod(f: Fp, a: &[u64], b: &[u64], m: &[u64]) -> Vec<u64> {
    let mut prod = vec![0u64; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        if x == 0 {
            continue;
        }
        for (j, &y) in b.iter().enumerate() {
            prod[i + j] = f.add(prod[i + j], f.mul(x, y));
        }
    }
    poly_rem(f, &prod, m)
}

/// `base^e mod m`.
pub fn poly_powmod(f: Fp, base: &[u64], mut e: u64, m: &[u64]) -> Vec<u64> {
    let mut r = vec![1u64];
    let mut b = poly_rem(f, base, m);
    while e > 0 {
        if e & 1 == 1 {
            r = poly_mulmod(f, &r, &b, m);
        }
        b = poly_mulmod(f, &b, &b, m);
        e >>= 1;
    }
    r
}

pub fn poly_gcd(f: Fp, a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut x = trim(a.to_vec());
    let mut y = trim(b.to_vec());
    while !(y.len() == 1 && y[0] == 0) {
        let r = poly_rem(f, &x, &y);
        x = y;
        y = r;
    }
    x
}

fn is_one_up_to_unit(a: &[u64]) -> bool {
    a.len() == 1 && a[0] != 0
}

fn prime_factors(mut n: usize) -> Vec<usize> {
    let mut out = Vec::new();
    let mut q = 2;
    while q * q <= n {
        if n.is_multiple_of(q) {
            out.push(q);
            while n.is_multiple_of(q) {
                n /= q;
            }
        }
        q += 1;
    }
    if n > 1 {
        out.push(n);
    }
    out
}

/// Rabin's test: a polynomial of degree `n ≥ 1` over `F_p` is irreducible
/// iff `x^{p^n} ≡ x (mod g)` and `gcd(x^{p^{n/q}} − x, g) = 1` for every
/// prime `q | n`.
pub fn is_irreducible(f: Fp, g: &[u64]) -> bool {
    let g = trim(g.to_vec());
    let n = g.len() - 1;
    if n == 0 {
        return false;
    }
    if n == 1 {
        return true;
    }
    let x = vec![0u64, 1];
    // frob[k] = x^{p^k} mod g, built by repeated p-th powers.
    let mut frob = vec![poly_rem(f, &x, &g)];
    for k in 1..=n {
        let next = poly_powmod(f, &frob[k - 1], f.p, &g);
        frob.push(next);
    }
    let minus_x = |h: &[u64]| {
        let mut h = h.to_vec();
        if h.len() < 2 {
            h.resize(2, 0);
        }
        h[1] = f.sub(h[1], 1);
        trim(h)
    };
    if !(minus_x(&frob[n]).len() == 1 && minus_x(&frob[n])[0] == 0) {
        return false;
    }
    prime_factors(n)
        .into_iter()
        .all(|q| is_one_up_to_unit(&poly_gcd(f, &minus_x(&frob[n / q]), &g)))
}

pub fn poly_div_exact(f: Fp, a: &[u64], m: &[u64]) -> Vec<u64> {
    let m = trim(m.to_vec());
    let dm = m.len() - 1;
    let lead_inv = f.inv(m[dm]);
    let mut r = trim(a.to_vec());
    if r.len() <= dm {
        return vec![0];
    }
    let mut q = vec![0u64; r.len() - dm];
    for dr in (dm..r.len()).rev() {
        let c = f.mul(r[dr], lead_inv);
        q[dr - dm] = c;
        for (k, &x) in m.iter().enumerate() {
            r[dr - dm + k] = f.sub(r[dr - dm + k], f.mul(c, x));
        }
    }
    trim(q)
}

pub fn derivative(f: Fp, a: &[u64]) -> Vec<u64> {
    if a.len() < 2 {
        return vec![0];
    }
    trim(
        a.iter()
            .enumerate()
            .skip(1)
            .map(|(i, &c)| f.mul(i as u64 % f.p, c))
            .collect(),
    )
}

/// Degrees of the irreducible factors of a squarefree `g` over `F_p`
/// (distinct-degree factorisation); `None` if `g` is not squarefree.
pub fn factor_degrees(f: Fp, g: &[u64]) -> Option<Vec<usize>> {
    let mut g = trim(g.to_vec());
    if g.len() < 2 {
        return Some(Vec::new());
    }
    if !is_one_up_to_unit(&poly_gcd(f, &g, &derivative(f, &g))) {
        return None;
    }
    let mut out = Vec::new();
    let mut h = poly_rem(f, &[0, 1], &g);
    let mut d = 1;
    while 2 * d < g.len() {
        h = poly_powmod(f, &h, f.p, &g);
        let mut hx = h.clone();
        if hx.len() < 2 {
            hx.resize(2, 0);
        }
        hx[1] = f.sub(hx[1], 1);
        let gd = poly_gcd(f, &trim(hx), &g);
        let k = gd.len() - 1;
        if k > 0 {
            out.extend(std::iter::repeat_n(d, k / d));
            g = poly_div_exact(f, &g, &gd);
            h = poly_rem(f, &h, &g);
        }
        d += 1;
    }
    if g.len() > 1 {
        out.push(g.len() - 1);
    }
    Some(out)
}

/// Irreducibility over `F_p` from factor-degree patterns on seeded random
/// lines.  A factor of `poly` of total degree `a` restricts to a factor of
/// degree `a` on every line where the restriction keeps full total degree
/// `vars·deg`, so `a` must be a subset sum of each such restriction's
/// factor degrees.  Returns the number of lines used once the only
/// surviving sums are `0` and `vars·deg` (which proves `poly` irreducible
/// over `F_p`), or `None` if `lines` lines do not settle it.
pub fn irreducible_on_random_line(poly: &MPoly, lines: usize, rng: &mut StdRng) -> Option<usize> {
    let f = poly.f;
    let full = poly.vars * poly.deg;
    let mut alive = vec![true; full + 1];
    for line in 1..=lines {
        let u: Vec<u64> = (0..poly.vars).map(|_| rng.gen_range(0..f.p)).collect();
        let v: Vec<u64> = (0..poly.vars).map(|_| rng.gen_range(0..f.p)).collect();
        let r = poly.restrict_to_line(&u, &v);
        if r.len() != full + 1 {
            continue;
        }
        let Some(degs) = factor_degrees(f, &r) else {
            continue;
        };
        let mut sums = vec![false; full + 1];
        sums[0] = true;
        for d in degs {
            for s in (d..=full).rev() {
                sums[s] |= sums[s - d];
            }
        }
        for (a, s) in alive.iter_mut().zip(&sums) {
            *a &= *s;
        }
        if alive[1..full].iter().all(|&a| !a) {
            return Some(line);
        }
    }
    None
}

/// One `z` of the L4-B step-1 check.
#[derive(Clone, Debug, serde::Serialize)]
pub struct StructureRow {
    pub z: u64,
    pub box_vertices_nonzero: bool,
    pub support_size: usize,
    pub irreducible_line: Option<usize>,
}

/// L4-B step 1 at arity `m` (`S_{m+1}`): `zs` seeded random `z`, each
/// checked for the full box Newton polytope and for irreducibility over
/// `F_p` on up to `lines` random lines.
pub fn structure_check(
    curve: &Curve,
    m: usize,
    zs: usize,
    lines: usize,
    seed: u64,
) -> Vec<StructureRow> {
    let mut rng = StdRng::seed_from_u64(seed);
    (0..zs)
        .map(|_| {
            let z = rng.gen_range(0..curve.f.p);
            let poly = curve.summation_poly(m, z);
            StructureRow {
                z,
                box_vertices_nonzero: box_vertices_nonzero(&poly),
                support_size: poly.coeffs.iter().filter(|&&c| c != 0).count(),
                irreducible_line: irreducible_on_random_line(&poly, lines, &mut rng),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The frontier campaign's 40-bit curve, `icv1-fp40-tm924137-0ecaaeb9`.
    fn c40() -> Curve {
        let p = 1_099_511_627_563u64;
        Curve::new(p, p - 3, 56)
    }

    /// Values printed by PARI/GP (`polresultant`, the recursion of
    /// `l4b_step1*.gp`) at z = 123456789.
    #[test]
    fn values_match_pari() {
        let c = c40();
        let p = c.f.p;
        let z = 123_456_789u64;
        let cases: [([u64; 4], u64, u64, u64); 3] = [
            ([2, 3, 5, 7], 1099511625279, 491535065431, 657063666607),
            (
                [1000003, 77, 99991, 123],
                350099246783,
                670974088405,
                313156019929,
            ),
            ([p - 1, 5, p - 9, 31337], 3464, 448157148734, 636692400027),
        ];
        for (x, s3, s4, s5) in cases {
            assert_eq!(c.s3(x[0], x[1], x[2]), s3);
            assert_eq!(c.summation(&x[..3], z), s4);
            assert_eq!(c.summation(&x[..4], z), s5);
        }
    }

    /// Affine addition on the curve, for the vanishing checks.
    fn add(c: &Curve, p1: (u64, u64), p2: (u64, u64)) -> (u64, u64) {
        let f = c.f;
        let lam = if p1 == p2 {
            let num = f.add(f.mul(3, f.mul(p1.0, p1.0)), c.a);
            f.mul(num, f.inv(f.mul(2, p1.1)))
        } else {
            f.mul(f.sub(p2.1, p1.1), f.inv(f.sub(p2.0, p1.0)))
        };
        let x3 = f.sub(f.sub(f.mul(lam, lam), p1.0), p2.0);
        let y3 = f.sub(f.mul(lam, f.sub(p1.0, x3)), p1.1);
        (x3, y3)
    }

    fn sqrt(f: Fp, a: u64) -> Option<u64> {
        // p ≡ 3 mod 4 for every curve used here.
        assert_eq!(f.p % 4, 3);
        let r = f.pow(a, (f.p + 1) / 4);
        (f.mul(r, r) == a).then_some(r)
    }

    fn points(c: &Curve, n: usize, seed: u64) -> Vec<(u64, u64)> {
        let f = c.f;
        let mut rng = StdRng::seed_from_u64(seed);
        let mut out = Vec::new();
        while out.len() < n {
            let x = rng.gen_range(0..f.p);
            let rhs = f.add(f.add(f.mul(f.mul(x, x), x), f.mul(c.a, x)), c.b);
            if let Some(y) = sqrt(f, rhs) {
                out.push((x, y));
            }
        }
        out
    }

    #[test]
    fn summation_polynomials_vanish_on_real_sums() {
        let c = c40();
        let pts = points(&c, 4, 7);
        let s12 = add(&c, pts[0], pts[1]);
        let s123 = add(&c, s12, pts[2]);
        let s1234 = add(&c, s123, pts[3]);
        assert_eq!(c.s3(pts[0].0, pts[1].0, s12.0), 0);
        assert_eq!(c.summation(&[pts[0].0, pts[1].0, pts[2].0], s123.0), 0);
        assert_eq!(
            c.summation(&[pts[0].0, pts[1].0, pts[2].0, pts[3].0], s1234.0),
            0
        );
        // A wrong target does not vanish.
        assert_ne!(c.summation(&[pts[0].0, pts[1].0, pts[2].0], s12.0), 0);
    }

    #[test]
    fn dense_polynomial_agrees_with_pointwise_evaluation() {
        let c = c40();
        let mut rng = StdRng::seed_from_u64(3);
        for m in [2usize, 3, 4] {
            let z = rng.gen_range(0..c.f.p);
            let poly = c.summation_poly(m, z);
            assert_eq!(poly.deg, 1 << (m - 1));
            for _ in 0..5 {
                let xs: Vec<u64> = (0..m).map(|_| rng.gen_range(0..c.f.p)).collect();
                assert_eq!(poly.eval(&xs), c.summation(&xs, z), "m = {m}");
            }
        }
    }

    #[test]
    fn rabin_test_separates_irreducible_from_reducible() {
        let f = Fp::new(1_099_511_627_563);
        // x² + 1 is irreducible since p ≡ 3 mod 4; (x − 1)(x − 2) is not.
        assert!(is_irreducible(f, &[1, 0, 1]));
        assert!(!is_irreducible(f, &[2, f.p - 3, 1]));
        // (x² + 1)² is reducible and not squarefree.
        assert!(!is_irreducible(f, &[1, 0, 2, 0, 1]));
        // (x² + 1)(x² + x + 3): reducible, no roots in F_p if x²+x+3 has none.
        let a = [1u64, 0, 1];
        let b = [3u64, 1, 1];
        let mut prod = vec![0u64; 5];
        for (i, &x) in a.iter().enumerate() {
            for (j, &y) in b.iter().enumerate() {
                prod[i + j] = f.add(prod[i + j], f.mul(x, y));
            }
        }
        assert!(!is_irreducible(f, &prod));
    }

    #[test]
    fn factor_degrees_of_a_product() {
        let f = Fp::new(1_099_511_627_563);
        // (x − 1)(x − 2)(x² + 1)
        let lin = [2u64, f.p - 3, 1];
        let q = [1u64, 0, 1];
        let mut prod = vec![0u64; 5];
        for (i, &x) in lin.iter().enumerate() {
            for (j, &y) in q.iter().enumerate() {
                prod[i + j] = f.add(prod[i + j], f.mul(x, y));
            }
        }
        let mut d = factor_degrees(f, &prod).unwrap();
        d.sort();
        assert_eq!(d, vec![1, 1, 2]);
        assert_eq!(factor_degrees(f, &[1, 0, 2, 0, 1]), None);
    }

    /// The GP runs' reading on the 40-bit curve, natively: full box and
    /// irreducible over F_p for S₄ and S₅.
    #[test]
    fn s4_and_s5_have_full_box_and_are_irreducible_over_fp() {
        let c = c40();
        for m in [3usize, 4] {
            for row in structure_check(&c, m, 3, 60, 20261028 + m as u64) {
                assert!(row.box_vertices_nonzero, "m = {m}, z = {}", row.z);
                assert!(row.irreducible_line.is_some(), "m = {m}, z = {}", row.z);
            }
        }
    }
}
