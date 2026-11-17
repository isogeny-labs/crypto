//! A novel one-parameter family of genus-3 hyperelliptic covers of a short
//! Weierstrass elliptic curve, and the explicit, same-field plane quartic and
//! checked transfer map that Smith's trigonal construction produces from it.
//!
//! # The cover family
//!
//! For `E: y^2 = x^3 + a x + b` over `F_p` (`p` odd, `3 < p < 2^31`) and a free
//! parameter `gamma != 0`, the degree-2 map
//!
//! ```text
//! phi: H_gamma -> E,   phi(u, v) = ( (u^2 + gamma) / u ,  v / u^2 )
//! ```
//!
//! pulls `E` back to the hyperelliptic curve
//!
//! ```text
//! H_gamma:  v^2 = f(u) = u * [ (u^2+gamma)^3 + a u^2 (u^2+gamma) + b u^3 ].
//! ```
//!
//! Expanding, `f` has ascending coefficients
//! `[0, gamma^3, 0, 3 gamma^2 + a gamma, b, 3 gamma + a, 0, 1]`: a monic degree-7
//! polynomial, so `H_gamma` is a genus-3 hyperelliptic curve with one point at
//! infinity.  At `gamma = 1, a = -3` this is `v^2 = u^7 + b u^4 + u`, the known
//! degree-2 genus-3 cover of the `a = -3` curve (research/p256_genus_dlp,
//! genus-3 row).  `gamma` free gives a *family*, searched here for a member on
//! which Smith's trigonal method applies.
//!
//! # Why the pulled-back points lie on `H_gamma`
//!
//! For `P = (x0, y0)` on `E`, its two preimages solve `u^2 - x0 u + gamma = 0`,
//! and their `H`-ordinate is `w = y0 u^2`.  With `x0 = (u^2+gamma)/u`,
//! `f(u)/u^4 = [(u^2+gamma)^3 + a u^2 (u^2+gamma) + b u^3]/u^3 = x0^3 + a x0 + b
//! = y0^2`, and `w^2/u^4 = y0^2`, so `w^2 = f(u)` is an identity: every pulled-
//! back point is on `H_gamma`.  The fibre over `O_E` is `(0,0) + infty` (`f(0)=0`,
//! so `(0,0)` is a Weierstrass point; `infty` is the odd-degree point at
//! infinity), both unramified.
//!
//! # The pullback homomorphism
//!
//! In the one-infinity model `HJac::point(x,y) = [(x,y) - infty]`, so
//!
//! ```text
//! phi^*([P] - [O_E]) = [(u1,w1) - infty] + [(u2,w2) - infty]
//!                    - [(0,0) - infty] - [infty - infty]
//!                    = pair(P) - point(0,0),
//! ```
//!
//! where `pair(P)` is `point(u1,w1) + point(u2,w2)` when `x0` splits, and the
//! Mumford class `U = u^2 - x0 u + gamma`, `V = y0 x0 u - y0 gamma` (the degree-2
//! reduced divisor of the conjugate pair) otherwise.  Because `phi` has degree 2
//! and the transferred subgroup here has odd prime order `r`, `gcd(2, r) = 1`
//! and `phi^*` is injective on it (`phi_* phi^* = [2]`).
//!
//! # The full chain
//!
//! `E(F_p) --phi^*--> J_{H_gamma}(F_p) --Psi--> Pic^0(C)`, with `Psi` Smith's
//! trigonal transfer ([`smith_canonical::Transfer`]) onto the explicit plane
//! quartic `C = CanonicalMap::quartic` over the same `F_p`.  This connects the
//! elliptic (cover) stage to the non-hyperelliptic index-calculus stage.
//!
//! Scope: this is an exact-correctness construction, not a speed claim.  For a
//! prime-field target, index calculus on a same-field genus-3 `C` is `O~(p)`
//! group operations, above rho's `O~(sqrt r)`; the deliverable is the verified
//! explicit artifact and the checked map, not an improvement over rho.

use crate::cryptanalysis::smith_canonical as sc;
use crate::cryptanalysis::smith_prop24 as sp;

/// Ascending coefficients `[c0..c7]` of `f(u)` for `H_gamma`, monic degree 7.
pub fn cover_f(p: u64, a: u64, b: u64, gamma: u64) -> Vec<u64> {
    let m = |x: u128| (x % p as u128) as u64;
    let g = gamma as u128;
    let a = a as u128;
    let g2 = m(g * g);
    let g3 = m(g * g2 as u128);
    let c1 = g3;
    let c3 = m(3 * g2 as u128 + a * g);
    let c4 = b % p;
    let c5 = m(3 * g + a);
    vec![0, c1, 0, c3, c4, c5, 0, 1]
}

/// A minimal affine elliptic curve `y^2 = x^3 + a x + b` over `F_p`.
#[derive(Clone, Debug)]
pub struct Ecc {
    pub p: u64,
    pub a: u64,
    pub b: u64,
}

/// A point of [`Ecc`]: `None` is the point at infinity.
pub type EPt = Option<(u64, u64)>;

impl Ecc {
    pub fn new(p: u64, a: u64, b: u64) -> Ecc {
        Ecc {
            p,
            a: a % p,
            b: b % p,
        }
    }
    pub fn on(&self, pt: &EPt) -> bool {
        match pt {
            None => true,
            Some((x, y)) => {
                let p = self.p as u128;
                let x = *x as u128 % p;
                let lhs = (*y as u128 % p).pow(2) % p;
                let rhs = (x * x % p * x + self.a as u128 * x + self.b as u128) % p;
                lhs == rhs
            }
        }
    }
    pub fn neg(&self, pt: &EPt) -> EPt {
        pt.map(|(x, y)| (x, (self.p - y % self.p) % self.p))
    }
    pub fn add(&self, pp: &EPt, qq: &EPt) -> EPt {
        let p = self.p;
        let (p1, p2) = match (pp, qq) {
            (None, _) => return *qq,
            (_, None) => return *pp,
            (Some(p1), Some(p2)) => (*p1, *p2),
        };
        let (x1, y1) = p1;
        let (x2, y2) = p2;
        if x1 == x2 && (y1 + y2) % p == 0 {
            return None;
        }
        let m = if x1 == x2 && y1 == y2 {
            // doubling
            let num = (3u128 * (x1 as u128).pow(2) % p as u128 + self.a as u128) % p as u128;
            let den = sp::inv_mod(2 * y1 % p, p) as u128;
            (num * den % p as u128) as u64
        } else {
            let num = (y2 as u128 + p as u128 - y1 as u128) % p as u128;
            let den = sp::inv_mod((x2 + p - x1) % p, p) as u128;
            (num * den % p as u128) as u64
        };
        let m2 = m as u128;
        let x3 = (m2 * m2 % p as u128 + 2 * p as u128 - x1 as u128 - x2 as u128) % p as u128;
        let y3 = (m2 * ((x1 as u128 + p as u128 - x3) % p as u128) % p as u128 + p as u128
            - y1 as u128)
            % p as u128;
        Some((x3 as u64, y3 as u64))
    }
    pub fn mul(&self, pt: &EPt, mut k: u128) -> EPt {
        let mut r = None;
        let mut acc = *pt;
        while k > 0 {
            if k & 1 == 1 {
                r = self.add(&r, &acc);
            }
            acc = self.add(&acc, &acc);
            k >>= 1;
        }
        r
    }
    /// Order of `pt` given the group order `n` (tries divisors of `n`).
    pub fn order(&self, pt: &EPt, n: u128) -> u128 {
        let mut ord = n;
        let mut d = 2u128;
        let mut m = n;
        // factor n
        let mut fs: Vec<u128> = Vec::new();
        while d * d <= m {
            while m.is_multiple_of(d) {
                fs.push(d);
                m /= d;
            }
            d += 1;
        }
        if m > 1 {
            fs.push(m);
        }
        for pf in fs {
            while ord.is_multiple_of(pf) && self.mul(pt, ord / pf).is_none() {
                ord /= pf;
            }
        }
        ord
    }
}

/// `phi^*([P] - [O_E])` as a class of `J_{H_gamma}(F_p)`, or `None` if `P` is
/// `O_E` or the fibre over `P` is ramified (`x0^2 = 4 gamma`), which this map
/// does not transfer.
pub fn pullback(jac: &sc::HJac, gamma: u64, pt: &EPt) -> Option<sc::Mumford> {
    let p = jac.p;
    let (x0, y0) = (*pt)?;
    let ng = |x: u64| (p - x % p) % p;
    let m = |x: u128| (x % p as u128) as u64;
    // u^2 - x0 u + gamma = 0
    let disc = m((x0 as u128 * x0 as u128 + p as u128 - 4 * gamma as u128 % p as u128) % p as u128);
    let base0 = jac.point(0, 0);
    let pair = match sp::sqrt_mod(disc, p) {
        Some(s) if disc != 0 => {
            let inv2 = sp::inv_mod(2, p);
            let u1 = m((x0 as u128 + s as u128) % p as u128 * inv2 as u128);
            let u2 = m((x0 as u128 + ng(s) as u128) % p as u128 * inv2 as u128);
            let w1 = m(y0 as u128 * u1 as u128 % p as u128 * u1 as u128);
            let w2 = m(y0 as u128 * u2 as u128 % p as u128 * u2 as u128);
            jac.add(&jac.point(u1, w1), &jac.point(u2, w2))
        }
        Some(_) => return None, // ramified: double preimage
        None => {
            // conjugate pair over F_{p^2}: U = u^2 - x0 u + gamma,
            // V = y0 x0 u - y0 gamma  (= y0 u^2 mod U).
            let u = vec![gamma % p, ng(x0), 1];
            let v = vec![
                ng(m(y0 as u128 * gamma as u128)),
                m(y0 as u128 * x0 as u128),
            ];
            sc::Mumford { u, v }
        }
    };
    Some(jac.sub(&pair, &base0))
}

/// The explicit, checked chain `E -> J_{H_gamma} -> Pic^0(C)` for one instance.
pub struct Chain {
    pub p: u64,
    pub a: u64,
    pub b: u64,
    pub gamma: u64,
    pub e_order: u128,
    pub e_prime_subgroup: u128,
    pub f: Vec<u64>,
    pub quartic: [u32; 15],
    pub jac: sc::HJac,
    pub transfer: sc::Transfer,
    pub pts_over_u: Vec<(u64, u64)>,
}

/// Count `#E(F_p)` by brute force over `x` (toy sizes only).
pub fn ecc_order(e: &Ecc) -> u128 {
    let p = e.p;
    let mut n = 1u128; // O
    for x in 0..p {
        let rhs = ((x as u128 * x as u128 % p as u128 * x as u128)
            + e.a as u128 * x as u128
            + e.b as u128)
            % p as u128;
        match sp::chi(rhs as u64, p) {
            0 => n += 1,
            1 => n += 2,
            _ => {}
        }
    }
    n
}

/// `(largest prime factor of n, n / that factor)`.
fn largest_prime_factor(mut n: u128) -> (u128, u128) {
    let n0 = n;
    let mut d = 2u128;
    let mut best = 1u128;
    while d * d <= n {
        if n.is_multiple_of(d) {
            while n.is_multiple_of(d) {
                n /= d;
            }
            best = d;
        }
        d += 1;
    }
    if n > 1 {
        best = n;
    }
    (best, n0 / best)
}

impl Chain {
    /// Search `gamma` in `[1, gamma_max]` for a cover on which Smith's trigonal
    /// method applies, with a smooth quartic and `#C = #H`.  `require_prime_e`
    /// keeps only prime-order `E` (NIST cofactor-1 shape); otherwise the largest
    /// prime factor of `#E` is used as the transferred subgroup.
    pub fn build(
        p: u64,
        a: u64,
        b: u64,
        gamma_max: u64,
        require_prime_e: bool,
    ) -> Result<Chain, String> {
        let e = Ecc::new(p, a, b);
        let n = ecc_order(&e);
        let (r, cof) = largest_prime_factor(n);
        if require_prime_e && cof != 1 {
            return Err(format!("#E = {n} not prime (cofactor {cof})"));
        }
        if r < 5 {
            return Err(format!("#E = {n} has no usable prime subgroup"));
        }
        for gamma in 1..=gamma_max {
            if gamma.is_multiple_of(p) {
                continue;
            }
            let f = cover_f(p, a, b, gamma);
            let jac = match sc::HJac::new(&f, p) {
                Ok(j) => j,
                Err(_) => continue,
            };
            let tr = match sc::find_trigonal(&f, p, 1) {
                Ok(t) => t,
                Err(_) => continue,
            };
            let cm = match sc::CanonicalMap::new(tr) {
                Ok(c) => c,
                Err(_) => continue,
            };
            if !cm.is_smooth() {
                continue;
            }
            let quartic = cm.quartic;
            // #C = #H over F_p
            if sc::count_quartic_points(p, &quartic, 1) != sc::count_h_points(&f, p, 1) {
                continue;
            }
            let transfer = match sc::Transfer::new(cm) {
                Ok(t) => t,
                Err(_) => continue,
            };
            let pts_over_u = transfer.usable_points();
            if pts_over_u.len() < 8 {
                continue;
            }
            return Ok(Chain {
                p,
                a,
                b,
                gamma,
                e_order: n,
                e_prime_subgroup: r,
                f,
                quartic,
                jac,
                transfer,
                pts_over_u,
            });
        }
        Err("no gamma gave a Smith-positive smooth cover".into())
    }

    pub fn ecc(&self) -> Ecc {
        Ecc::new(self.p, self.a, self.b)
    }

    /// `Psi(phi^*([P]-[O]))` in `Pic^0(C)`.
    pub fn transfer_point(
        &self,
        pt: &EPt,
        rng: &mut sp::SplitMix64,
    ) -> Result<crate::cryptanalysis::plane_quartic_pic::PicClass, String> {
        let d =
            pullback(&self.jac, self.gamma, pt).ok_or("pullback undefined (O_E or ramified)")?;
        self.transfer
            .transfer_divisor(&self.jac, &d, &self.pts_over_u, rng)
    }
}

/// BSGS for `k` with `[k] base = target` in `Pic^0(C)`, `k in [0, n)`.
pub fn bsgs_quartic(
    c: &crate::cryptanalysis::plane_quartic_pic::PlaneQuartic,
    base: &crate::cryptanalysis::plane_quartic_pic::PicClass,
    target: &crate::cryptanalysis::plane_quartic_pic::PicClass,
    n: u128,
) -> Option<u128> {
    use crate::cryptanalysis::plane_quartic_pic::PicKey;
    use std::collections::HashMap;
    let m = (n as f64).sqrt().ceil() as u128 + 1;
    let mut table: HashMap<PicKey, u128> = HashMap::new();
    let mut cur = c.zero();
    for j in 0..m {
        table.entry(cur.key()).or_insert(j);
        cur = c.add(&cur, base);
    }
    // factor = [m] base ; we search target - i*factor in table
    let factor = c.mul_scalar(base, m);
    let neg_factor = c.neg(&factor);
    let mut gamma = target.clone();
    for i in 0..=m {
        if let Some(&j) = table.get(&gamma.key()) {
            let k = (i * m + j) % n;
            return Some(k);
        }
        gamma = c.add(&gamma, &neg_factor);
    }
    None
}

// ---------------------------------------------------------------------------
// Wide primes (`p < 2^62`, e.g. the 56-bit ladder curve): the stages whose
// cost is polynomial in `log p`. Nothing here enumerates the ~`p` points of a
// curve; `Pic^0(C)` arithmetic (and hence `Psi`) is not available at this size.

pub mod wide {
    use crate::cryptanalysis::plane_quartic_pic::mons_of_degree;
    use crate::cryptanalysis::smith_canonical as sc;
    use crate::cryptanalysis::smith_prop24 as sp;
    use crate::cryptanalysis::smith_trigonal::{Fp2, Trigonal, F2};

    #[inline]
    fn mm(a: u64, b: u64, p: u64) -> u64 {
        ((a as u128 * b as u128) % p as u128) as u64
    }
    #[inline]
    fn ad(a: u64, b: u64, p: u64) -> u64 {
        ((a as u128 + b as u128) % p as u128) as u64
    }
    #[inline]
    fn sb(a: u64, b: u64, p: u64) -> u64 {
        ad(a, p - b % p, p)
    }

    /// Dense ternary form, `c[i][j][k]` the coefficient of `x^i y^j z^k`.
    pub type Tern = [[[u64; 5]; 5]; 5];

    pub fn tern_from_mons(q: &[u64; 15]) -> Tern {
        let mut t = [[[0u64; 5]; 5]; 5];
        for (&(i, j, k), &c) in mons_of_degree(4).iter().zip(q) {
            t[i as usize][j as usize][k as usize] = c;
        }
        t
    }

    fn tmul(a: &Tern, b: &Tern, p: u64) -> Tern {
        let mut r = [[[0u64; 5]; 5]; 5];
        for i in 0..5 {
            for j in 0..5 - i {
                for k in 0..5 - i - j {
                    if a[i][j][k] == 0 {
                        continue;
                    }
                    for u in 0..5 - i - j - k {
                        for v in 0..5 - i - j - k - u {
                            for w in 0..5 - i - j - k - u - v {
                                if b[u][v][w] != 0 {
                                    let c = &mut r[i + u][j + v][k + w];
                                    *c = ad(*c, mm(a[i][j][k], b[u][v][w], p), p);
                                }
                            }
                        }
                    }
                }
            }
        }
        r
    }

    /// `F(A (x, y, z)^T)`: the form in the coordinates `X = A x`.
    pub fn tern_compose(f: &Tern, a: &[[u64; 3]; 3], p: u64) -> Tern {
        let lin = |row: &[u64; 3]| {
            let mut l = [[[0u64; 5]; 5]; 5];
            l[1][0][0] = row[0] % p;
            l[0][1][0] = row[1] % p;
            l[0][0][1] = row[2] % p;
            l
        };
        let (lx, ly, lz) = (lin(&a[0]), lin(&a[1]), lin(&a[2]));
        let mut one = [[[0u64; 5]; 5]; 5];
        one[0][0][0] = 1;
        let powers = |l: &Tern| {
            let mut v = vec![one];
            for e in 1..5 {
                v.push(tmul(&v[e - 1], l, p));
            }
            v
        };
        let (px, py, pz) = (powers(&lx), powers(&ly), powers(&lz));
        let mut out = [[[0u64; 5]; 5]; 5];
        for i in 0..5 {
            for j in 0..5 - i {
                let k = 4 - i - j;
                let c = f[i][j][k];
                if c == 0 {
                    continue;
                }
                let m = tmul(&tmul(&px[i], &py[j], p), &pz[k], p);
                for (u, mu) in m.iter().enumerate() {
                    for (v, mv) in mu.iter().enumerate() {
                        for (w, &x) in mv.iter().enumerate() {
                            if x != 0 {
                                out[u][v][w] = ad(out[u][v][w], mm(c, x, p), p);
                            }
                        }
                    }
                }
            }
        }
        out
    }

    /// `F` at a point of `P^2(F_{p^2})`.
    pub fn quartic_eval_fp2(q: &[u64; 15], h: [Fp2; 3], p: u64) -> Fp2 {
        let k = F2::new(p);
        let mut acc = Fp2::default();
        for (&(i, j, l), &c) in mons_of_degree(4).iter().zip(q) {
            if c == 0 {
                continue;
            }
            let mut m = k.c(c);
            for _ in 0..i {
                m = k.mul(m, h[0]);
            }
            for _ in 0..j {
                m = k.mul(m, h[1]);
            }
            for _ in 0..l {
                m = k.mul(m, h[2]);
            }
            acc = k.add(acc, m);
        }
        acc
    }

    fn det(mut m: Vec<Vec<u64>>, p: u64) -> u64 {
        let n = m.len();
        let mut d = 1u64;
        for c in 0..n {
            let Some(r) = (c..n).find(|&r| m[r][c] != 0) else {
                return 0;
            };
            if r != c {
                m.swap(r, c);
                d = sb(0, d, p);
            }
            d = mm(d, m[c][c], p);
            let iv = sp::inv_mod(m[c][c], p);
            for r in c + 1..n {
                if m[r][c] == 0 {
                    continue;
                }
                let f = mm(m[r][c], iv, p);
                for cc in c..n {
                    let t = mm(f, m[c][cc], p);
                    m[r][cc] = sb(m[r][cc], t, p);
                }
            }
        }
        d
    }

    /// Sylvester determinant of `a` (formal degree `a.len()-1`) and `b`
    /// (formal degree `b.len()-1`), coefficients ascending; leading zeros
    /// are kept, so this is the specialization of the generic resultant.
    fn sylvester(a: &[u64], b: &[u64], p: u64) -> u64 {
        let (m, n) = (a.len() - 1, b.len() - 1);
        let s = m + n;
        let mut mat = vec![vec![0u64; s]; s];
        for r in 0..n {
            for (i, &c) in a.iter().rev().enumerate() {
                mat[r][r + i] = c % p;
            }
        }
        for r in 0..m {
            for (i, &c) in b.iter().rev().enumerate() {
                mat[n + r][r + i] = c % p;
            }
        }
        det(mat, p)
    }

    fn pdeg(a: &[u64]) -> isize {
        a.iter().rposition(|&c| c != 0).map_or(-1, |d| d as isize)
    }

    fn pgcd(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
        let mut a = a.to_vec();
        let mut b = b.to_vec();
        while pdeg(&b) >= 0 {
            let db = pdeg(&b) as usize;
            let ib = sp::inv_mod(b[db], p);
            while pdeg(&a) >= db as isize {
                let da = pdeg(&a) as usize;
                let f = mm(a[da], ib, p);
                for i in 0..=db {
                    a[da - db + i] = sb(a[da - db + i], mm(f, b[i], p), p);
                }
            }
            std::mem::swap(&mut a, &mut b);
        }
        a.truncate((pdeg(&a) + 1).max(0) as usize);
        a
    }

    /// Coefficients (ascending, length `n`) of the polynomial of degree `< n`
    /// through `(x_i, y_i)`, by Newton's divided differences.
    fn interpolate(xs: &[u64], ys: &[u64], p: u64) -> Vec<u64> {
        let n = xs.len();
        let mut c = ys.to_vec();
        for j in 1..n {
            for i in (j..n).rev() {
                let num = sb(c[i], c[i - 1], p);
                let den = sb(xs[i], xs[i - j], p);
                c[i] = mm(num, sp::inv_mod(den, p), p);
            }
        }
        let mut poly = vec![0u64; n];
        for i in (0..n).rev() {
            // poly = poly * (x - xs[i]) + c[i]
            let mut next = vec![0u64; n];
            for k in 0..n {
                if poly[k] == 0 {
                    continue;
                }
                if k + 1 < n {
                    next[k + 1] = ad(next[k + 1], poly[k], p);
                }
                next[k] = sb(next[k], mm(poly[k], xs[i], p), p);
            }
            next[0] = ad(next[0], c[i], p);
            poly = next;
        }
        poly
    }

    /// One affine chart's certificate. `g[i][j]` is the coefficient of
    /// `u^i v^j` of the dehomogenized form; `v` is eliminated. Returns
    /// `(deg R_a, deg R_b, deg gcd)`, or an error if a resultant vanishes
    /// identically or exceeds the Bezout bound 12.
    fn chart_certificate(g: &[[u64; 5]; 5], p: u64) -> Result<(isize, isize, isize), String> {
        // d/du and d/dv
        let mut gu = [[0u64; 5]; 5];
        let mut gv = [[0u64; 5]; 5];
        for i in 0..5 {
            for j in 0..5 {
                if i > 0 {
                    gu[i - 1][j] = ad(gu[i - 1][j], mm(i as u64, g[i][j], p), p);
                }
                if j > 0 {
                    gv[i][j - 1] = ad(gv[i][j - 1], mm(j as u64, g[i][j], p), p);
                }
            }
        }
        let at = |h: &[[u64; 5]; 5], u0: u64, dv: usize| -> Vec<u64> {
            (0..=dv)
                .map(|j| {
                    let mut s = 0u64;
                    let mut up = 1u64;
                    for row in h.iter() {
                        s = ad(s, mm(row[j], up, p), p);
                        up = mm(up, u0, p);
                    }
                    s
                })
                .collect()
        };
        const NPTS: u64 = 30;
        let xs: Vec<u64> = (1..=NPTS).collect();
        let ra: Vec<u64> = xs
            .iter()
            .map(|&u0| sylvester(&at(g, u0, 4), &at(&gu, u0, 3), p))
            .collect();
        let rb: Vec<u64> = xs
            .iter()
            .map(|&u0| sylvester(&at(g, u0, 4), &at(&gv, u0, 3), p))
            .collect();
        let pa = interpolate(&xs, &ra, p);
        let pb = interpolate(&xs, &rb, p);
        let (da, db) = (pdeg(&pa), pdeg(&pb));
        if da < 0 || db < 0 {
            return Err("a resultant vanishes identically (F has a repeated component)".into());
        }
        if da > 12 || db > 12 {
            return Err(format!(
                "resultant degree {da}/{db} exceeds the Bezout bound 12"
            ));
        }
        Ok((da, db, pdeg(&pgcd(&pa, &pb, p))))
    }

    /// Smoothness certificate for the plane quartic `q` (`mons_of_degree(4)`
    /// order) over `F_p`, `p > 3`. `Ok(true)`: certified smooth (charts
    /// `z = 1`, `y = 1` and the point `(1:0:0)`); `Ok(false)`: a chart's gcd is
    /// nontrivial (singular, or not certified); `Err`: degenerate input.
    pub fn smooth_certificate(q: &[u64; 15], p: u64, seed: u64) -> Result<(bool, String), String> {
        let f = tern_from_mons(q);
        let mut rng = sp::SplitMix64(seed);
        for _attempt in 0..64 {
            let a: [[u64; 3]; 3] = std::array::from_fn(|_| std::array::from_fn(|_| rng.below(p)));
            let am: Vec<Vec<u64>> = a.iter().map(|r| r.to_vec()).collect();
            if det(am, p) == 0 {
                continue;
            }
            let g = tern_compose(&f, &a, p);
            // y^4 and z^4 coefficients nonzero: the eliminated variable has
            // full degree in each chart.
            if g[0][4][0] == 0 || g[0][0][4] == 0 {
                continue;
            }
            // Chart z = 1, variables (u, v) = (x, y), eliminate y.
            let mut c1 = [[0u64; 5]; 5];
            // Chart y = 1, variables (u, v) = (x, z), eliminate z.
            let mut c2 = [[0u64; 5]; 5];
            for i in 0..5 {
                for j in 0..5 - i {
                    let k = 4 - i - j;
                    c1[i][j] = g[i][j][k];
                    c2[i][k] = g[i][j][k];
                }
            }
            let (a1, b1, g1) = chart_certificate(&c1, p)?;
            let (a2, b2, g2) = chart_certificate(&c2, p)?;
            // (1:0:0): F = x^4 coefficient; dF/dx = 4 x^3 coefficient,
            // dF/dy = coefficient of x^3 y, dF/dz = coefficient of x^3 z.
            let pt_ok = g[4][0][0] != 0 || g[3][1][0] != 0 || g[3][0][1] != 0;
            let msg = format!(
                "chart z=1: deg R_a {a1}, deg R_b {b1}, deg gcd {g1}; chart y=1: deg R_a {a2}, deg R_b {b2}, deg gcd {g2}; (1:0:0) nonsingular: {pt_ok}"
            );
            return Ok((g1 == 0 && g2 == 0 && pt_ok, msg));
        }
        Err("no admissible coordinate change in 64 tries".into())
    }

    /// One explicit instance at a wide prime.
    pub struct WideChain {
        pub tr: Trigonal,
        pub cm: sc::CanonicalMap,
    }

    /// Build Smith's trigonal map and the canonical map of a Smith-positive
    /// `f`, without the point enumeration of `CanonicalMap::new`.
    pub fn trigonal(f: &[u64], p: u64, seed: u64) -> Result<WideChain, String> {
        let tr = sc::find_trigonal(f, p, seed)?;
        let tmat = sc::psi_matrix(&tr);
        let cm = sc::CanonicalMap {
            tr: tr.clone(),
            tmat,
            quartic: [0; 15],
            fit_points: 0,
        };
        Ok(WideChain { tr, cm })
    }

    /// Images under `psi` of the points of `X` over random affine points of
    /// `H` (sampled, not enumerated): `n` distinct normalized points.
    pub fn sample_images(
        w: &WideChain,
        n: usize,
        rng: &mut sp::SplitMix64,
    ) -> Result<(Vec<[Fp2; 3]>, u64), String> {
        let p = w.tr.p;
        let k = F2::new(p);
        let mut out: Vec<[Fp2; 3]> = Vec::new();
        let mut draws = 0u64;
        while out.len() < n {
            draws += 1;
            if draws > 200_000 {
                return Err(format!("only {} images after {draws} draws", out.len()));
            }
            let x = rng.below(p);
            let fx = {
                let mut acc = 0u64;
                for &c in w.tr.f.iter().rev() {
                    acc = ad(mm(acc, x, p), c, p);
                }
                acc
            };
            let Some(y) = sp::sqrt_mod(fx, p) else {
                continue;
            };
            let Ok(ip) = w.tr.image_points(x, y) else {
                continue;
            };
            for xp in &ip.points {
                if let Ok(h) = w.cm.psi_xpoint(xp) {
                    if let Some(hn) = sc::normalize_fp2(&k, h) {
                        if !out.contains(&hn) {
                            out.push(hn);
                        }
                    }
                }
            }
        }
        out.truncate(n);
        Ok((out, draws))
    }

    // -----------------------------------------------------------------------
    // `Psi = psi_* . phi` into `Pic^0(C)` at wide primes, with the 64-bit
    // `plane_quartic_pic64`. Ported from `smith_canonical::Transfer`; the
    // differences: no point enumeration anywhere (base points come from sampled
    // rational image points, the pool of points of `H` is sampled), and a split
    // point is accepted when its `e_class` is defined, not when it belongs to a
    // pre-enumerated list.

    use crate::cryptanalysis::plane_quartic_pic64 as pq;
    use std::cell::RefCell;
    use std::collections::HashMap;

    fn nullspace_fp(rows: &[Vec<u64>], n: usize, p: u64) -> Vec<Vec<u64>> {
        let mut m: Vec<Vec<u64>> = rows
            .iter()
            .map(|r| r.iter().map(|&x| x % p).collect())
            .collect();
        let mut piv: Vec<usize> = Vec::new();
        let mut r = 0;
        for c in 0..n {
            let Some(k) = (r..m.len()).find(|&k| m[k][c] != 0) else {
                continue;
            };
            m.swap(r, k);
            let iv = sp::inv_mod(m[r][c], p);
            for x in m[r].iter_mut() {
                *x = mm(*x, iv, p);
            }
            for k in 0..m.len() {
                if k != r && m[k][c] != 0 {
                    let f = m[k][c];
                    for cc in 0..n {
                        let t = mm(f, m[r][cc], p);
                        m[k][cc] = sb(m[k][cc], t, p);
                    }
                }
            }
            piv.push(c);
            r += 1;
            if r == m.len() {
                break;
            }
        }
        let free: Vec<usize> = (0..n).filter(|c| !piv.contains(c)).collect();
        free.iter()
            .map(|&fc| {
                let mut v = vec![0u64; n];
                v[fc] = 1;
                for (i, &pc) in piv.iter().enumerate() {
                    v[pc] = sb(0, m[i][fc], p);
                }
                v
            })
            .collect()
    }

    fn mon_vals(k: &F2, mons: &[(u8, u8, u8)], h: &[Fp2; 3]) -> Vec<Fp2> {
        mons.iter()
            .map(|&(a, b, c)| {
                let mut v = k.c(1);
                for _ in 0..a {
                    v = k.mul(v, h[0]);
                }
                for _ in 0..b {
                    v = k.mul(v, h[1]);
                }
                for _ in 0..c {
                    v = k.mul(v, h[2]);
                }
                v
            })
            .collect()
    }

    fn det3(k: &F2, a: &[Fp2; 3], b: &[Fp2; 3], c: &[Fp2; 3]) -> Fp2 {
        let m = |x: Fp2, y: Fp2, z: Fp2, w: Fp2| k.sub(k.mul(x, y), k.mul(z, w));
        k.add(
            k.sub(
                k.mul(a[0], m(b[1], c[2], b[2], c[1])),
                k.mul(a[1], m(b[0], c[2], b[2], c[0])),
            ),
            k.mul(a[2], m(b[0], c[1], b[1], c[0])),
        )
    }

    fn rational(h: &[Fp2; 3]) -> bool {
        h.iter().all(|z| z.b == 0)
    }

    /// The affine points of a reduced class `d` of `J_H` when `u` splits
    /// into distinct linear factors over `F_p`: `(x, v(x))` for the roots `x`
    /// of `u`. Polylog in `p` (factoring, not `smith_canonical::proots`, which
    /// scans `0..p`). `None` if `u` has a repeated or nonlinear factor.
    pub fn split64(d: &sc::Mumford, p: u64) -> Option<Vec<(u64, u64)>> {
        let du = pdeg(&d.u);
        if du <= 0 {
            return Some(Vec::new());
        }
        let seed =
            d.u.iter()
                .fold(0x9E37_79B9_7F4A_7C15u64, |h, &c| h.rotate_left(13) ^ c);
        let mut rng = sp::SplitMix64(seed);
        let fs = sp::factor_squarefree(&d.u, p, &mut rng)?;
        if fs.len() as isize != du || fs.iter().any(|f| pdeg(f) != 1) {
            return None;
        }
        let mut out: Vec<(u64, u64)> = fs
            .iter()
            .map(|f| {
                // monic linear factor x + c: root -c
                let x = (p - f[0] % p) % p;
                let mut y = 0u64;
                for &c in d.v.iter().rev() {
                    y = ad(mm(y, x, p), c, p);
                }
                (x, y)
            })
            .collect();
        out.sort();
        Some(out)
    }

    /// The transfer `J_H -> Pic^0(C)` at a wide prime.
    pub struct Transfer64<'a> {
        pub w: &'a WideChain,
        pub quartic: [u64; 15],
        pub c: pq::PlaneQuartic,
        bases: Vec<pq::Pt>,
        std2: Vec<(u8, u8, u8)>,
        cache: RefCell<HashMap<(u64, u64), Result<pq::PicClass, String>>>,
    }

    impl<'a> Transfer64<'a> {
        /// `rational_images`: rational points of `C` (normalized), e.g. from
        /// [`sample_images`]; the first is the base point `P0`, up to seven
        /// more serve as auxiliary bases.
        pub fn new(
            w: &'a WideChain,
            quartic: [u64; 15],
            rational_images: &[[Fp2; 3]],
        ) -> Result<Transfer64<'a>, String> {
            let p = w.tr.p;
            let mut bases: Vec<pq::Pt> = Vec::new();
            for h in rational_images.iter().filter(|h| rational(h)) {
                let pt = [h[0].a, h[1].a, h[2].a];
                if !bases.contains(&pt) {
                    bases.push(pt);
                }
                if bases.len() == 8 {
                    break;
                }
            }
            let p0 = *bases.first().ok_or("no rational point of C supplied")?;
            let c = pq::PlaneQuartic::new(p, quartic, p0).map_err(|e| format!("C: {e:?}"))?;
            let bases: Vec<pq::Pt> = bases.iter().map(|&b| c.normalize_point(b)).collect();
            let std2 = c.std_monomials(2);
            Ok(Transfer64 {
                w,
                quartic,
                c,
                bases,
                std2,
                cache: RefCell::new(HashMap::new()),
            })
        }

        /// `[Q1 + Q2 + 2 P0 - K]` for two distinct rational points or a
        /// conjugate pair of `C`.
        pub fn pair_class(&self, q1: [Fp2; 3], q2: [Fp2; 3]) -> Result<pq::PicClass, String> {
            let p = self.w.tr.p;
            let k = F2::new(p);
            let n1 = sc::normalize_fp2(&k, q1).ok_or("zero point")?;
            let n2 = sc::normalize_fp2(&k, q2).ok_or("zero point")?;
            if !k.is_zero(quartic_eval_fp2(&self.quartic, n1, p))
                || !k.is_zero(quartic_eval_fp2(&self.quartic, n2, p))
            {
                return Err("point not on C".into());
            }
            let rat = rational(&n1) && rational(&n2);
            if !rat && n2 != n1.map(|z| k.conj(z)) {
                return Err("the pair is neither rational nor conjugate".into());
            }
            if n1 == n2 {
                return Err("double point: not supported".into());
            }
            let v1 = mon_vals(&k, &self.std2, &n1);
            let v2 = mon_vals(&k, &self.std2, &n2);
            for (bi, base) in self.bases.iter().enumerate() {
                let bf = [k.c(base[0]), k.c(base[1]), k.c(base[2])];
                if k.is_zero(det3(&k, &n1, &n2, &bf)) {
                    continue;
                }
                let mut rows: Vec<Vec<u64>> = if rat {
                    vec![
                        v1.iter().map(|z| z.a).collect(),
                        v2.iter().map(|z| z.a).collect(),
                    ]
                } else {
                    vec![
                        v1.iter().map(|z| z.a).collect(),
                        v1.iter().map(|z| z.b).collect(),
                    ]
                };
                rows.push(mon_vals(&k, &self.std2, &bf).iter().map(|z| z.a).collect());
                let ns = nullspace_fp(&rows, 6, p);
                if ns.len() != 3 {
                    continue;
                }
                let cls = self.c.class_from_conics(ns);
                return Ok(if bi == 0 {
                    cls
                } else {
                    self.c.sub(&cls, &self.c.point_class(*base))
                });
            }
            Err("no usable auxiliary base point".into())
        }

        /// `[E(P) + 2 P0 - K]` for an affine point `P = (x, y)` of `H` over `U`.
        pub fn e_class(&self, x: u64, y: u64) -> Result<pq::PicClass, String> {
            if let Some(r) = self.cache.borrow().get(&(x, y)) {
                return r.clone();
            }
            let r = (|| {
                let ip = self.w.tr.image_points(x, y)?;
                let h1 = self.w.cm.psi_xpoint(&ip.points[0])?;
                let h2 = self.w.cm.psi_xpoint(&ip.points[1])?;
                self.pair_class(h1, h2)
            })();
            self.cache.borrow_mut().insert((x, y), r.clone());
            r
        }

        pub fn point_sum(
            &self,
            plus: &[(u64, u64)],
            minus: &[(u64, u64)],
        ) -> Result<pq::PicClass, String> {
            if plus.len() != minus.len() {
                return Err("point_sum needs a divisor of degree 0".into());
            }
            let mut acc = self.c.zero();
            for &(x, y) in plus {
                acc = self.c.add(&acc, &self.e_class(x, y)?);
            }
            for &(x, y) in minus {
                acc = self.c.sub(&acc, &self.e_class(x, y)?);
            }
            Ok(acc)
        }

        /// A pool of `n` sampled affine points of `H` on which `e_class` is
        /// defined (no enumeration).
        pub fn sample_pool(&self, n: usize, rng: &mut sp::SplitMix64) -> Vec<(u64, u64)> {
            let p = self.w.tr.p;
            let mut out = Vec::new();
            let mut tries = 0;
            while out.len() < n && tries < 100_000 {
                tries += 1;
                let x = rng.below(p);
                let mut fx = 0u64;
                for &c in self.w.tr.f.iter().rev() {
                    fx = ad(mm(fx, x, p), c, p);
                }
                let Some(y) = sp::sqrt_mod(fx, p) else {
                    continue;
                };
                if y != 0 && self.e_class(x, y).is_ok() && !out.contains(&(x, y)) {
                    out.push((x, y));
                }
            }
            out
        }

        /// `Psi(D)` for a class `D` of `J_H(F_p)` (`deg F = 7`): write
        /// `D + S1 = R1 + R2 + R3 - 3 inf` and `S2 = R'1 + R'2 + R'3 - 3 inf`
        /// with `S1, S2` random sums of pool points and every `R` an affine
        /// point on which `e_class` is defined, so
        /// `D = sum R - sum R' + S2 - S1`.
        pub fn transfer_divisor(
            &self,
            jac: &sc::HJac,
            d: &sc::Mumford,
            pool: &[(u64, u64)],
            rng: &mut sp::SplitMix64,
        ) -> Result<pq::PicClass, String> {
            if pool.len() < 8 {
                return Err("pool too small".into());
            }
            let p = self.w.tr.p;
            let ok = |r: &[(u64, u64)]| {
                r.len() == 3 && r.iter().all(|&(x, y)| self.e_class(x, y).is_ok())
            };
            let rand_sum = |rng: &mut sp::SplitMix64| {
                let mut plus = Vec::new();
                let mut minus = Vec::new();
                let mut m = jac.zero();
                for _ in 0..2 {
                    let a = pool[rng.below(pool.len() as u64) as usize];
                    let b = pool[rng.below(pool.len() as u64) as usize];
                    m = jac.add(&m, &jac.point(a.0, a.1));
                    m = jac.add(&m, &jac.point(b.0, (p - b.1) % p));
                    plus.push(a);
                    minus.push(b);
                }
                (m, plus, minus)
            };
            let find = |base: &sc::Mumford, rng: &mut sp::SplitMix64| {
                for _ in 0..20_000 {
                    let (m, plus, minus) = rand_sum(rng);
                    let s = jac.add(base, &m);
                    if let Some(r) = split64(&s, p) {
                        if ok(&r) {
                            return Some((r, plus, minus));
                        }
                    }
                }
                None
            };
            let (r1, p1, m1) = find(d, rng).ok_or("no split representative of D + S1")?;
            let (r2, p2, m2) = find(&jac.zero(), rng).ok_or("no split representative of S2")?;
            let mut plus = r1;
            plus.extend(p2);
            plus.extend(m1);
            let mut minus = r2;
            minus.extend(m2);
            minus.extend(p1);
            self.point_sum(&plus, &minus)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- The cover family -------------------------------------------------

    #[test]
    fn cover_matches_known_p256_form_at_gamma1_am3() {
        // gamma = 1, a = -3 (= p-3) must give f = u^7 + b u^4 + u.
        let p = 10007;
        let b = 17;
        let f = cover_f(p, p - 3, b, 1);
        assert_eq!(f, vec![0, 1, 0, 0, b, 0, 0, 1]);
    }

    #[test]
    fn cover_is_a_genus3_cover_pullback_points_lie_on_h() {
        // For a random E point, both pulled-back points satisfy w^2 = f(u).
        let p = 10007u64;
        let (a, b) = (p - 3, 17);
        let gamma = 5u64;
        let f = cover_f(p, a, b, gamma);
        assert_eq!(super::sp::deg(&f), 7, "genus-3 model");
        let e = Ecc::new(p, a, b);
        let peval = |x: u64| -> u64 {
            let mut acc = 0u128;
            for &c in f.iter().rev() {
                acc = (acc * x as u128 + c as u128) % p as u128;
            }
            acc as u64
        };
        let mut checked = 0;
        for x0 in 0..200u64 {
            let rhs = ((x0 as u128).pow(3) + a as u128 * x0 as u128 + b as u128) % p as u128;
            if let Some(y0) = super::sp::sqrt_mod(rhs as u64, p) {
                assert!(e.on(&Some((x0, y0))));
                let disc = (x0 as u128 * x0 as u128 + p as u128 - 4 * gamma as u128 % p as u128)
                    % p as u128;
                if let Some(s) = super::sp::sqrt_mod(disc as u64, p) {
                    if disc == 0 {
                        continue;
                    }
                    let inv2 = super::sp::inv_mod(2, p);
                    for u in [
                        (x0 as u128 + s as u128) % p as u128 * inv2 as u128 % p as u128,
                        (x0 as u128 + (p - s) as u128) % p as u128 * inv2 as u128 % p as u128,
                    ] {
                        let u = u as u64;
                        let w = (y0 as u128 * u as u128 % p as u128 * u as u128 % p as u128) as u64;
                        assert_eq!(
                            (w as u128 * w as u128 % p as u128) as u64,
                            peval(u),
                            "pulled-back point on H"
                        );
                        checked += 1;
                    }
                }
            }
        }
        assert!(checked > 20, "exercised the split fibre");
    }

    // --- Build the full chain on a NIST-shaped toy target -----------------

    fn find_prime_e(p: u64) -> Option<(u64, u128)> {
        // a = -3, search b for prime #E (cofactor 1).
        let a = p - 3;
        for b in 1..p {
            let e = Ecc::new(p, a, b);
            // nonsingular: 4a^3 + 27b^2 != 0
            let disc = (4 * (a as u128).pow(3) % p as u128 + 27 * (b as u128).pow(2)) % p as u128;
            if disc == 0 {
                continue;
            }
            let n = ecc_order(&e);
            let (_, cof) = largest_prime_factor(n);
            if cof == 1 {
                return Some((b, n));
            }
        }
        None
    }

    #[test]
    fn full_chain_transfers_a_planted_dlp() {
        // A NIST-shaped toy: a = -3, prime-order E over a small prime.
        let p = 1009u64;
        let (b, n) = find_prime_e(p).expect("prime-order a=-3 curve");
        let a = p - 3;
        let chain = match Chain::build(p, a, b, 400, true) {
            Ok(c) => c,
            // Not every (p,b) has a Smith-positive gamma in range; try the
            // next few primes to keep the test deterministic but robust.
            Err(_) => {
                let mut done = None;
                for pp in [1013u64, 1019, 1021, 1031, 1033, 1039, 1049, 1051, 1061] {
                    if let Some((bb, _)) = find_prime_e(pp) {
                        if let Ok(c) = Chain::build(pp, pp - 3, bb, 400, true) {
                            done = Some(c);
                            break;
                        }
                    }
                }
                done.expect("a Smith-positive prime-order instance in the toy range")
            }
        };
        let e = chain.ecc();
        let r = chain.e_prime_subgroup;
        assert_eq!(chain.e_order, r, "E has prime order (NIST cofactor 1)");

        // A generator P of E (order r), and a planted target Q = [k] P.
        let mut gp = None;
        for x in 0..p {
            let rhs = ((x as u128).pow(3) + a as u128 * x as u128 + chain.b as u128) % p as u128;
            if let Some(y) = super::sp::sqrt_mod(rhs as u64, p) {
                let pt = Some((x, y));
                if e.order(&pt, r) == r {
                    gp = Some(pt);
                    break;
                }
            }
        }
        let gp = gp.expect("generator");
        let k: u128 = 123 % r;
        let q = e.mul(&gp, k);

        let mut rng = super::sp::SplitMix64(0x5eed_1234_abcd_0001);

        // pullback is a homomorphism injective on <P>: order preserved.
        let pg = pullback(&chain.jac, chain.gamma, &gp).expect("pullback P");
        let pq = pullback(&chain.jac, chain.gamma, &q).expect("pullback Q");
        assert_eq!(chain.jac.order(&pg, n), r, "phi*(P) keeps order r");
        // phi*([k]P) = [k] phi*(P)
        let kpg = chain.jac.mul(&pg, k);
        assert!(
            chain.jac.is_zero(&chain.jac.sub(&kpg, &pq)),
            "phi* homomorphism on the planted pair"
        );

        // Transfer both to Pic^0(C) and recover k there.
        let psi_g = chain.transfer_point(&gp, &mut rng).expect("Psi phi* P");
        let psi_q = chain.transfer_point(&q, &mut rng).expect("Psi phi* Q");
        let c = &chain.transfer.c;
        // order on C divides r and is nonzero (kernel S has exponent 2, coprime to r).
        assert!(!c.is_zero(&psi_g), "image generator nonzero on C");
        let kk = bsgs_quartic(c, &psi_g, &psi_q, r).expect("log on C");
        assert_eq!(kk % r, k % r, "BSGS on C recovers the planted log");
    }

    // --- Wide primes: controls for the smoothness certificate ------------

    /// `mons_of_degree(4)` index of `x^i y^j z^k`.
    fn mon_index(i: u8, j: u8, k: u8) -> usize {
        crate::cryptanalysis::plane_quartic_pic::mons_of_degree(4)
            .iter()
            .position(|&m| m == (i, j, k))
            .unwrap()
    }

    /// The 56-bit ladder prime (largest p = 3 mod 4 below 2^56).
    const P56: u64 = 72_057_594_037_927_931;

    #[test]
    fn wide_certificate_fermat_quartic_is_smooth() {
        // x^4 + y^4 + z^4 is smooth for p > 2.
        for p in [1009u64, P56] {
            let mut q = [0u64; 15];
            q[mon_index(4, 0, 0)] = 1;
            q[mon_index(0, 4, 0)] = 1;
            q[mon_index(0, 0, 4)] = 1;
            let (ok, msg) = wide::smooth_certificate(&q, p, 7).unwrap();
            assert!(ok, "Fermat quartic must certify smooth at p={p}: {msg}");
        }
    }

    #[test]
    fn wide_certificate_rejects_a_nodal_quartic() {
        // F = x^2 z^2 - y^2 z^2 + x^3 z + y^4: a node at (0:0:1).
        for p in [1009u64, P56] {
            let mut q = [0u64; 15];
            q[mon_index(2, 0, 2)] = 1;
            q[mon_index(0, 2, 2)] = p - 1;
            q[mon_index(3, 0, 1)] = 1;
            q[mon_index(0, 4, 0)] = 1;
            // A degenerate input (Err) is also a non-certification.
            if let Ok((ok, msg)) = wide::smooth_certificate(&q, p, 7) {
                assert!(!ok, "nodal quartic must not certify at p={p}: {msg}");
            }
        }
    }

    #[test]
    fn wide_compose_preserves_values() {
        // F(A x) evaluated at x equals F evaluated at A x.
        let p = P56;
        let mut rng = super::sp::SplitMix64(11);
        let q: [u64; 15] = std::array::from_fn(|_| rng.below(p));
        let a: [[u64; 3]; 3] = std::array::from_fn(|_| std::array::from_fn(|_| rng.below(p)));
        let g = wide::tern_compose(&wide::tern_from_mons(&q), &a, p);
        let mons = crate::cryptanalysis::plane_quartic_pic::mons_of_degree(4);
        let gq: [u64; 15] = std::array::from_fn(|i| {
            let (x, y, z) = mons[i];
            g[x as usize][y as usize][z as usize]
        });
        let k = crate::cryptanalysis::smith_trigonal::F2::new(p);
        for _ in 0..20 {
            let x: [u64; 3] = std::array::from_fn(|_| rng.below(p));
            let ax: [u64; 3] = std::array::from_fn(|r| {
                (0..3).fold(0u128, |s, c| {
                    (s + a[r][c] as u128 * x[c] as u128) % p as u128
                }) as u64
            });
            let lhs = wide::quartic_eval_fp2(&gq, x.map(|v| k.c(v)), p);
            let rhs = wide::quartic_eval_fp2(&q, ax.map(|v| k.c(v)), p);
            assert_eq!(lhs, rhs);
        }
    }

    /// The 64-bit transfer (sampled pool, on-the-fly usability) gives the
    /// same class of `Pic^0(C)` as the u32 original, on the same trigonal map,
    /// canonical map and base point, at a toy prime.
    #[test]
    fn transfer64_matches_u32_transfer_at_toy_prime() {
        let p = 1009u64;
        let (b, _) = find_prime_e(p).expect("prime-order a=-3 curve");
        let a = p - 3;
        let chain = Chain::build(p, a, b, 400, true).expect("toy chain");
        let e = chain.ecc();
        let r = chain.e_prime_subgroup;
        let gp = (0..p)
            .find_map(|x| {
                let rhs = ((x as u128).pow(3) + a as u128 * x as u128 + b as u128) % p as u128;
                super::sp::sqrt_mod(rhs as u64, p).and_then(|y| {
                    let pt = Some((x, y));
                    (e.order(&pt, r) == r).then_some(pt)
                })
            })
            .unwrap();
        let d = pullback(&chain.jac, chain.gamma, &gp).unwrap();
        let mut rng = super::sp::SplitMix64(41);
        let orig = chain
            .transfer
            .transfer_divisor(&chain.jac, &d, &chain.pts_over_u, &mut rng)
            .unwrap();

        let w = wide::WideChain {
            tr: chain.transfer.cm.tr.clone(),
            cm: chain.transfer.cm.clone(),
        };
        let p0 = chain.transfer.c.base_point();
        let k = crate::cryptanalysis::smith_trigonal::F2::new(p);
        let p0h = [k.c(p0[0] as u64), k.c(p0[1] as u64), k.c(p0[2] as u64)];
        let quartic: [u64; 15] = chain.quartic.map(|x| x as u64);
        let t64 = wide::Transfer64::new(&w, quartic, &[p0h]).unwrap();
        let mut rng2 = super::sp::SplitMix64(43);
        let pool = t64.sample_pool(40, &mut rng2);
        assert!(pool.len() >= 8);
        let w64 = t64
            .transfer_divisor(&chain.jac, &d, &pool, &mut rng2)
            .unwrap();
        assert_eq!(
            orig.key().0.map(|x| x as u64),
            w64.key().0,
            "same class of Pic^0(C)"
        );
        // and the 64-bit class is a homomorphic image: [k] Psi(D) = Psi([k]D)
        let kk = 77u128;
        let dk = chain.jac.mul(&d, kk);
        let w64k = t64
            .transfer_divisor(&chain.jac, &dk, &pool, &mut rng2)
            .unwrap();
        assert!(t64.c.eq(&t64.c.mul_scalar(&w64, kk), &w64k));
    }

    /// `wide::split64` (factoring) agrees with `HJac::split` (scan of
    /// `F_p`) on random reduced classes at a toy prime.
    #[test]
    fn split64_matches_hjac_split_at_toy_prime() {
        let p = 1009u64;
        let (b, _) = find_prime_e(p).expect("curve");
        let chain = Chain::build(p, p - 3, b, 400, true).expect("toy chain");
        let jac = &chain.jac;
        let pts = &chain.pts_over_u;
        let mut rng = super::sp::SplitMix64(17);
        let (mut agree, mut split) = (0, 0);
        for _ in 0..400 {
            let mut d = jac.zero();
            for _ in 0..3 {
                let a = pts[rng.below(pts.len() as u64) as usize];
                d = jac.add(&d, &jac.point(a.0, a.1));
            }
            let mut ref_split = jac.split(&d);
            if let Some(r) = ref_split.as_mut() {
                r.sort();
                // HJac::split does not reject repeated roots; split64 does.
                let mut dd = r.clone();
                dd.dedup_by_key(|q| q.0);
                if dd.len() != r.len() {
                    continue;
                }
                split += 1;
            }
            assert_eq!(wide::split64(&d, p), ref_split, "class {:?}", d.u);
            agree += 1;
        }
        assert!(agree > 300 && split > 50, "agree {agree}, split {split}");
    }

    #[test]
    fn negative_control_wrong_scalar_fails() {
        let p = 1009u64;
        let (b, n) = match find_prime_e(p) {
            Some(x) => x,
            None => return,
        };
        let a = p - 3;
        let chain = match Chain::build(p, a, b, 400, true).ok().or_else(|| {
            [1013u64, 1019, 1021, 1031, 1033, 1039, 1049, 1051, 1061]
                .into_iter()
                .find_map(|pp| {
                    find_prime_e(pp)
                        .and_then(|(bb, _)| Chain::build(pp, pp - 3, bb, 400, true).ok())
                })
        }) {
            Some(c) => c,
            None => return,
        };
        let _ = n;
        let e = chain.ecc();
        let r = chain.e_prime_subgroup;
        let gp = (0..chain.p)
            .find_map(|x| {
                let rhs = ((x as u128).pow(3) + a as u128 * x as u128 + chain.b as u128)
                    % chain.p as u128;
                super::sp::sqrt_mod(rhs as u64, chain.p).and_then(|y| {
                    let pt = Some((x, y));
                    (e.order(&pt, r) == r).then_some(pt)
                })
            })
            .unwrap();
        let mut rng = super::sp::SplitMix64(0xA1);
        let psi_g = chain.transfer_point(&gp, &mut rng).unwrap();
        let q = e.mul(&gp, 7 % r);
        let psi_q = chain.transfer_point(&q, &mut rng).unwrap();
        let c = &chain.transfer.c;
        let kk = bsgs_quartic(c, &psi_g, &psi_q, r).unwrap();
        assert_ne!(kk % r, 11 % r, "a wrong scalar is not accepted");
        assert_eq!(kk % r, 7 % r);
    }
}
