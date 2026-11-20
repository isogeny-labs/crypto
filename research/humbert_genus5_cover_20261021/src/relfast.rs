//! HUMBERT_REL_FAST=1: a pure-speed path for the hyperplane relation search
//! (`cover::hyperplane_relation`).  It draws the same random numbers, tries the
//! same candidates in the same order and returns the same relations; only the
//! arithmetic changes:
//!
//! * F_p products are reduced by a Barrett step on u64 with lazy reduction
//!   (p < 2^16: the largest accumulator, `Quart::mul`, is at most
//!   4 (p-1)^2 (1 + 3 (p-1)) < 12 p^3 < 2^52; every other sum is at most a few
//!   products < 2^36), never by u128 `%`;
//! * no heap allocation per candidate (fixed arrays for the 4 x 5 kernel, the
//!   norm polynomial, the residual and the root search);
//! * the norm N(G) = prod over the 16 sign vectors of (a - sum eps_i beta_i),
//!   beta_i = b_i w_i, is evaluated at each interpolation node from
//!   B_i = beta_i^2 in K4 by the closed form below (13 K4 products per node)
//!   instead of 8 products of L elements (140 K4 products per node);
//! * the residual n / known is interpolated from N(G) / known at the nodes 0..=4,
//!   with N(G) at node 5 as the divisibility check (6 norm values instead of 9),
//!   unless a chosen x is one of those nodes (then the 9-node path and a division);
//! * x^p mod the residual is computed once (in F_p[x]/(res), a quartic ring like
//!   K4) and gives both the root histogram and the split test;
//! * the roots of a split residual come from equal-degree splitting with the
//!   deterministic shifts delta = 0, 1, 2, ... (and the quadratic formula with a
//!   square-root table) instead of a scan of all x in F_p; they are returned in
//!   increasing order, as the scan returned them.
//!
//! Closed form.  With P(z) = prod_{e0,e2} (z - e0 beta_0 - e2 beta_2)
//!   = z^4 - 2 s z^2 + d,   s = B0 + B2,  d = (B0 - B2)^2,
//! and tau = e1 beta_1 + e3 beta_3, T = tau^2, m = a^2 + T:
//!   P(a - tau) P(a + tau) = H(T) = A^2 - 16 a^2 T (m - s)^2,
//!   A = m^2 + 4 a^2 T - 2 s m + d.
//! T takes the values sigma +- 2 kappa, sigma = B1 + B3, kappa = beta_1 beta_3,
//! kappa^2 = D = B1 B3, so N = H(sigma + 2 kappa) H(sigma - 2 kappa) is the norm
//! h0^2 - D h1^2 of H(sigma + 2X) computed in K4[X]/(X^2 - D).
//! B_i(t) = b_i^2 eta_i (t - lam_i) is linear in the node t, so s, sigma, d and D
//! are polynomials in t whose K4 coefficients are computed once per candidate.
use crate::cover::{identify, FactorBase, Nodes, Relation, Setup, Stats};
use crate::curve::Rng;
use crate::field::E4;
use std::cell::Cell;

pub fn enabled() -> bool {
    std::env::var("HUMBERT_REL_FAST").map(|v| v == "1").unwrap_or(false)
}

/// Barrett reduction modulo p < 2^16 for any u64 input (the reduction itself
/// is exact for any p; the bound is for the lazy sums of the callers).
#[derive(Clone, Copy, Debug)]
pub struct Fp {
    pub p: u64,
    mu: u64,
}
impl Fp {
    pub fn new(p: u64) -> Fp {
        assert!(p >= 3 && p < (1 << 16));
        Fp { p, mu: ((1u128 << 64) / p as u128) as u64 }
    }
    /// a mod p for every a < 2^64 (q is floor(a/p) or one less, so r < 2p).
    #[inline(always)]
    pub fn red(&self, a: u64) -> u64 {
        let q = ((a as u128 * self.mu as u128) >> 64) as u64;
        let r = a - q * self.p;
        if r >= self.p {
            r - self.p
        } else {
            r
        }
    }
    #[inline(always)]
    fn add(&self, a: u64, b: u64) -> u64 {
        let c = a + b;
        if c >= self.p {
            c - self.p
        } else {
            c
        }
    }
    #[inline(always)]
    fn sub(&self, a: u64, b: u64) -> u64 {
        if a >= b {
            a - b
        } else {
            a + self.p - b
        }
    }
    #[inline(always)]
    fn neg(&self, a: u64) -> u64 {
        if a == 0 {
            0
        } else {
            self.p - a
        }
    }
}

/// F_p[t]/(m(t)) for a monic quartic m: K4 itself, or F_p[x]/(res) for a residual.
/// `red[h]` = t^(4+h) reduced.
#[derive(Clone, Copy, Debug)]
struct Quart {
    red: [[u64; 4]; 3],
}
impl Quart {
    /// 16 + 12 F_p products (counted by the caller as 28), 4 reductions.
    #[inline(always)]
    fn mul(&self, f: &Fp, a: &E4, b: &E4) -> E4 {
        let mut c = [0u64; 7];
        for i in 0..4 {
            for j in 0..4 {
                c[i + j] += a[i] * b[j]; // <= 4 products < 2^34 (p < 2^16)
            }
        }
        let mut r = [c[0], c[1], c[2], c[3]];
        for h in 0..3 {
            for i in 0..4 {
                r[i] += c[4 + h] * self.red[h][i]; // < 2^34 + 3 * 2^50 < 2^52
            }
        }
        [f.red(r[0]), f.red(r[1]), f.red(r[2]), f.red(r[3])]
    }
    /// t * a: 4 F_p products.
    #[inline(always)]
    fn mul_t(&self, f: &Fp, a: &E4) -> E4 {
        let a3 = a[3];
        [f.red(a3 * self.red[0][0]), f.red(a[0] + a3 * self.red[0][1]), f.red(a[1] + a3 * self.red[0][2]), f.red(a[2] + a3 * self.red[0][3])]
    }
    /// The ring F_p[x]/(m) for a monic quartic m = m0 + m1 x + m2 x^2 + m3 x^3 + x^4.
    fn for_monic(f: &Fp, m: &[u64; 4]) -> Quart {
        let x4 = [f.neg(m[0]), f.neg(m[1]), f.neg(m[2]), f.neg(m[3])];
        let mut q = Quart { red: [x4, [0; 4], [0; 4]] };
        q.red[1] = q.mul_t(f, &x4);
        q.red[2] = q.mul_t(f, &q.red[1]);
        q
    }
}

#[inline(always)]
fn k_add(f: &Fp, a: &E4, b: &E4) -> E4 {
    [f.add(a[0], b[0]), f.add(a[1], b[1]), f.add(a[2], b[2]), f.add(a[3], b[3])]
}
#[inline(always)]
fn k_sub(f: &Fp, a: &E4, b: &E4) -> E4 {
    [f.sub(a[0], b[0]), f.sub(a[1], b[1]), f.sub(a[2], b[2]), f.sub(a[3], b[3])]
}

/// Per-candidate polynomials in the node t (see the module comment).
struct Pre {
    c0: u64,
    c1: u64,
    /// s(t) = t sg - sh
    sg: E4,
    sh: E4,
    /// sigma(t) = t qg - qh
    qg: E4,
    qh: E4,
    /// d(t) = t^2 uu - 2t uv + vv
    uu: E4,
    uv: E4,
    vv: E4,
    /// D(t) = t^2 g13 - t x13 + h13
    g13: E4,
    x13: E4,
    h13: E4,
}

/// Lagrange basis at the nodes 0..N-1 (coefficients low to high): the same
/// computation, in the same order, as `poly::Interp::new(N - 1, p)`, with every
/// modular product added to `fpm` (the products of `poly::mul` it skips for a
/// zero coefficient are not done and not counted; `invm` is `powm(den, p - 2)`,
/// one squaring per bit of p - 2 and one product per set bit).
fn interp_basis_counted<const N: usize>(p: u64, fpm: &mut u64) -> [[u64; N]; N] {
    use crate::field::{invm, mulm, subm};
    assert!((N as u64) <= p, "need p > interpolation degree");
    let mut basis = [[0u64; N]; N];
    for t in 0..N {
        let mut num = vec![1u64];
        let mut den = 1u64;
        for s in 0..N {
            if s == t {
                continue;
            }
            *fpm += 2 * num.iter().filter(|&&c| c != 0).count() as u64;
            num = crate::poly::mul(&num, &[(p - s as u64 % p) % p, 1], p);
            den = mulm(den, subm(t as u64, s as u64, p), p);
            *fpm += 1;
        }
        let e = p - 2;
        *fpm += (64 - e.leading_zeros()) as u64 + e.count_ones() as u64;
        let di = invm(den, p);
        for (i, c) in num.iter().enumerate() {
            basis[t][i] = mulm(*c, di, p);
            *fpm += 1;
        }
    }
    basis
}

/// Precomputed context for the fast hyperplane search (built once per run).
pub struct RelFast {
    pub f: Fp,
    k4: Quart,
    /// tconj[j][i] = (t^j)^{p^i}
    tconj: [[E4; 4]; 4],
    eta: [E4; 4],
    /// eta_i lam_i
    eta_lam: [E4; 4],
    /// Lagrange basis at nodes 0..=8 (coefficients low to high)
    basis: [[u64; 9]; 9],
    /// Lagrange basis at nodes 0..=4
    basis5: [[u64; 5]; 5],
    /// inverses mod p (inv[0] unused)
    inv: Vec<u32>,
    /// a square root of each square mod p, u32::MAX for non-squares
    sqrt: Vec<u32>,
    /// F_p products done by this path (K4 products counted as 28 each)
    pub fp_muls: Cell<u64>,
}

impl RelFast {
    /// Builds the tables.  Every F_p product of the build is counted in
    /// `fp_muls` as it runs: the 4 K4 products eta_i lam_i (28 each, like the
    /// per-candidate K4 products of this path, so they are not also added to
    /// the K4 counter), the Lagrange basis at nodes 0..=4 (see
    /// `interp_basis_counted`), p - 2 products for the inverse table and p for
    /// the square-root table.
    pub fn new(s: &Setup, nodes: &Nodes) -> RelFast {
        let p = s.p;
        let f = Fp::new(p);
        let k = s.k();
        let k4 = Quart { red: k.red };
        let mut fpm: u64 = 0;
        let mut eta_lam = [[0u64; 4]; 4];
        for i in 0..4 {
            eta_lam[i] = k4.mul(&f, &s.etas[i], &s.lams[i]);
        }
        fpm += 4 * 28;
        assert_eq!(nodes.interp8.n, 8);
        let mut basis = [[0u64; 9]; 9];
        for t in 0..9 {
            for (i, &c) in nodes.interp8.basis[t].iter().enumerate() {
                basis[t][i] = c;
            }
        }
        let basis5 = interp_basis_counted::<5>(p, &mut fpm);
        let mut inv = vec![0u32; p as usize];
        inv[1] = 1;
        for i in 2..p {
            // inv[i] = -(p / i) * inv[p mod i]
            inv[i as usize] = f.neg(f.red((p / i) * inv[(p % i) as usize] as u64)) as u32;
            fpm += 1;
        }
        let mut sqrt = vec![u32::MAX; p as usize];
        for y in 0..p {
            let sq = f.red(y * y) as usize;
            fpm += 1;
            if sqrt[sq] == u32::MAX {
                sqrt[sq] = y as u32;
            }
        }
        RelFast { f, k4, tconj: s.tconj, eta: s.etas, eta_lam, basis, basis5, inv, sqrt, fp_muls: Cell::new(fpm) }
    }

    #[inline(always)]
    fn kmul(&self, a: &E4, b: &E4) -> E4 {
        self.k4.mul(&self.f, a, b)
    }

    /// Per-candidate part of the norm evaluation: 18 K4 products.
    fn norm_pre(&self, c: &[u64; 12], fpm: &mut u64) -> Pre {
        let f = &self.f;
        // b_i = sum_j c_{2+j} (t^j)^{p^i}
        let mut b = [[0u64; 4]; 4];
        for i in 0..4 {
            let mut acc = [0u64; 4];
            for j in 0..4 {
                let cj = c[2 + j];
                for l in 0..4 {
                    acc[l] += cj * self.tconj[j][i][l];
                }
            }
            b[i] = [f.red(acc[0]), f.red(acc[1]), f.red(acc[2]), f.red(acc[3])];
        }
        *fpm += 64;
        // B_i(t) = t g_i - h_i
        let mut g = [[0u64; 4]; 4];
        let mut h = [[0u64; 4]; 4];
        for i in 0..4 {
            let bsq = self.kmul(&b[i], &b[i]);
            g[i] = self.kmul(&bsq, &self.eta[i]);
            h[i] = self.kmul(&bsq, &self.eta_lam[i]);
        }
        // d(t) = (t u - v)^2 = t^2 uu - 2t uv + vv,  u = g0 - g2, v = h0 - h2
        let u = k_sub(f, &g[0], &g[2]);
        let v = k_sub(f, &h[0], &h[2]);
        let uu = self.kmul(&u, &u);
        let uv = self.kmul(&u, &v);
        let vv = self.kmul(&v, &v);
        // s(t) = t sg - sh,  sigma(t) = t qg - qh
        let sg = k_add(f, &g[0], &g[2]);
        let sh = k_add(f, &h[0], &h[2]);
        let qg = k_add(f, &g[1], &g[3]);
        let qh = k_add(f, &h[1], &h[3]);
        // D(t) = (t g1 - h1)(t g3 - h3) = t^2 g13 - t x13 + h13
        let g13 = self.kmul(&g[1], &g[3]);
        let h13 = self.kmul(&h[1], &h[3]);
        let x13 = k_sub(f, &k_sub(f, &self.kmul(&k_add(f, &g[1], &h[1]), &k_add(f, &g[3], &h[3])), &g13), &h13);
        *fpm += 28 * 18;
        Pre { c0: c[0], c1: c[1], sg, sh, qg, qh, uu, uv, vv, g13, x13, h13 }
    }

    /// N(G) at the node x = t (an F_p value): 13 K4 products.
    fn norm_at(&self, pr: &Pre, t: u64, fpm: &mut u64) -> u64 {
        let f = &self.f;
        let p = f.p;
        let tt = t % p;
        let t2 = f.red(tt * tt);
        let mt = f.neg(tt);
        let m2t = f.neg(f.red(2 * tt));
        let a = f.red(pr.c0 + pr.c1 * tt);
        let a2 = f.red(a * a);
        *fpm += 4;
        let mut s = [0u64; 4];
        let mut sig = [0u64; 4];
        let mut d = [0u64; 4];
        let mut dd = [0u64; 4];
        for l in 0..4 {
            s[l] = f.red(tt * pr.sg[l] + (p - 1) * pr.sh[l]);
            sig[l] = f.red(tt * pr.qg[l] + (p - 1) * pr.qh[l]);
            d[l] = f.red(t2 * pr.uu[l] + m2t * pr.uv[l] + pr.vv[l]);
            dd[l] = f.red(t2 * pr.g13[l] + mt * pr.x13[l] + pr.h13[l]);
        }
        *fpm += 4 * 8;
        // m = a^2 + T, T = sigma + 2X
        let mut m0 = sig;
        m0[0] = f.add(m0[0], a2);
        let m0sq = self.kmul(&m0, &m0);
        let sm0 = self.kmul(&s, &m0);
        let c4a2 = f.red(4 * a2);
        let c8a2 = f.red(8 * a2);
        let k16 = f.neg(f.red(16 * a2));
        let k32 = f.red(2 * k16);
        *fpm += 4;
        // A = m^2 + 4 a^2 T - 2 s m + d
        let mut a0 = [0u64; 4];
        let mut a1 = [0u64; 4];
        for l in 0..4 {
            a0[l] = f.red(m0sq[l] + 4 * dd[l] + c4a2 * sig[l] + (p - 2) * sm0[l] + d[l]);
            a1[l] = f.red(4 * m0[l] + (p - 4) * s[l]);
        }
        a1[0] = f.add(a1[0], c8a2);
        *fpm += 4 * 5;
        // w = m - s = (w0, 2), w^2 = (w0^2 + 4D, 4 w0)
        let w0 = k_sub(f, &m0, &s);
        let wsq = self.kmul(&w0, &w0);
        let mut q0 = [0u64; 4];
        let mut q1 = [0u64; 4];
        for l in 0..4 {
            q0[l] = f.red(wsq[l] + 4 * dd[l]);
            q1[l] = f.red(4 * w0[l]);
        }
        *fpm += 4 * 2;
        // T w^2 = (sigma q0 + 2 D q1, sigma q1 + 2 q0)
        let sq0 = self.kmul(&sig, &q0);
        let dq1 = self.kmul(&dd, &q1);
        let sq1 = self.kmul(&sig, &q1);
        // A^2 = (A0^2 + D A1^2, 2 A0 A1)
        let a0sq = self.kmul(&a0, &a0);
        let a1sq = self.kmul(&a1, &a1);
        let da1sq = self.kmul(&dd, &a1sq);
        let a0a1 = self.kmul(&a0, &a1);
        // H = A^2 - 16 a^2 T w^2
        let mut h0 = [0u64; 4];
        let mut h1 = [0u64; 4];
        for l in 0..4 {
            h0[l] = f.red(a0sq[l] + da1sq[l] + k16 * sq0[l] + k32 * dq1[l]);
            h1[l] = f.red(2 * a0a1[l] + k16 * sq1[l] + k32 * q0[l]);
        }
        *fpm += 4 * 5;
        // N = H0^2 - D H1^2
        let h0sq = self.kmul(&h0, &h0);
        let h1sq = self.kmul(&h1, &h1);
        let dh1sq = self.kmul(&dd, &h1sq);
        let n = k_sub(f, &h0sq, &dh1sq);
        assert!(n[1] == 0 && n[2] == 0 && n[3] == 0, "norm value not in F_p: Frobenius structure broken");
        *fpm += 28 * 13;
        n[0]
    }

    /// Coefficients (low to high, length 9) of the norm polynomial of
    /// G = c0 + c1 x + sum_j c_{2+j} g_{2+j}: equal to `cover::norm_poly(.., true)`
    /// (zero-padded).  Returns (coefficients, K4 products, F_p products).
    pub fn norm8(&self, c: &[u64; 12]) -> ([u64; 9], u64, u64) {
        let f = &self.f;
        let mut fpm = 0u64;
        let pr = self.norm_pre(c, &mut fpm);
        let mut vals = [0u64; 9];
        for t in 0..9u64 {
            vals[t as usize] = self.norm_at(&pr, t, &mut fpm);
        }
        let mut out = [0u64; 9];
        for i in 0..9 {
            let mut acc = 0u64;
            for t in 0..9 {
                acc += vals[t] * self.basis[t][i]; // 9 products < 2^36
            }
            out[i] = f.red(acc);
        }
        fpm += 81;
        (out, 18 + 9 * 13, fpm)
    }

    /// The residual n / known from 6 nodes instead of 9, when no chosen x is a node
    /// (x = 0..=5): res(t) = N(t) / known(t) at t = 0..=4 interpolated (degree <= 4),
    /// and N(5) = res(5) known(5) checked in place of the divisibility test.
    /// None when a chosen x is a node (the caller takes the 9-node path).
    /// Returns (res, deg n == 8, K4 products, F_p products).
    pub fn residual6(&self, c: &[u64; 12], known: &[u64; 5], xs: &[u64; 4]) -> Option<([u64; 5], bool, u64, u64)> {
        if xs.iter().any(|&x| x <= 5) {
            return None;
        }
        let f = &self.f;
        let mut fpm = 0u64;
        let pr = self.norm_pre(c, &mut fpm);
        let ev = |a: &[u64; 5], t: u64| -> u64 {
            let mut acc = 0u64;
            for &cc in a.iter().rev() {
                acc = f.red(acc * t + cc);
            }
            acc
        };
        let mut rv = [0u64; 5];
        for t in 0..5u64 {
            let nt = self.norm_at(&pr, t, &mut fpm);
            rv[t as usize] = f.red(nt * self.inv(ev(known, t)));
        }
        fpm += 5 * 6;
        let mut res = [0u64; 5];
        for i in 0..5 {
            let mut acc = 0u64;
            for t in 0..5 {
                acc += rv[t] * self.basis5[t][i];
            }
            res[i] = f.red(acc);
        }
        fpm += 25;
        let full = res[4] != 0;
        if full {
            let n5 = self.norm_at(&pr, 5, &mut fpm);
            assert_eq!(f.red(ev(&res, 5) * ev(known, 5)), n5, "hyperplane does not vanish on its defining points");
            fpm += 11;
            return Some((res, true, 18 + 6 * 13, fpm));
        }
        Some((res, false, 18 + 5 * 13, fpm))
    }

    #[inline(always)]
    fn inv(&self, a: u64) -> u64 {
        debug_assert!(a != 0 && a < self.f.p);
        self.inv[a as usize] as u64
    }

    /// x^e mod the monic quartic of ring q (square and multiply by x).
    fn pow_x(&self, q: &Quart, e: u64, fpm: &mut u64) -> E4 {
        let f = &self.f;
        let mut r: E4 = [1, 0, 0, 0];
        let bits = 64 - e.leading_zeros();
        for i in (0..bits).rev() {
            r = q.mul(f, &r, &r);
            *fpm += 28;
            if (e >> i) & 1 == 1 {
                r = q.mul_t(f, &r);
                *fpm += 4;
            }
        }
        r
    }

    /// Degree of gcd(a, b) for polynomials given as coefficient arrays with degrees
    /// (-1 = zero polynomial); deg b >= 0.
    fn gcd_deg(&self, mut a: [u64; 5], mut da: isize, mut b: [u64; 5], mut db: isize, fpm: &mut u64) -> isize {
        let f = &self.f;
        while db >= 0 {
            // a <- a mod b
            let li = self.inv(b[db as usize]);
            while da >= db {
                let c = f.red(a[da as usize] * li);
                let sh = (da - db) as usize;
                for j in 0..=db as usize {
                    a[sh + j] = f.sub(a[sh + j], f.red(c * b[j]));
                }
                *fpm += 2 + db as u64;
                debug_assert_eq!(a[da as usize], 0);
                da -= 1;
                while da >= 0 && a[da as usize] == 0 {
                    da -= 1;
                }
            }
            std::mem::swap(&mut a, &mut b);
            std::mem::swap(&mut da, &mut db);
        }
        da
    }

    /// (a * b) mod monic m of degree d <= 4 (a, b reduced).
    fn mulmod_small(&self, a: &[u64; 5], b: &[u64; 5], m: &[u64; 5], d: usize, fpm: &mut u64) -> [u64; 5] {
        let f = &self.f;
        let mut c = [0u64; 9];
        for i in 0..d {
            for j in 0..d {
                c[i + j] += a[i] * b[j];
            }
        }
        *fpm += (d * d) as u64;
        for x in c.iter_mut() {
            *x = f.red(*x);
        }
        for k in (d..2 * d - 1).rev() {
            let ck = c[k];
            if ck == 0 {
                continue;
            }
            c[k] = 0;
            for i in 0..d {
                c[k - d + i] = f.sub(c[k - d + i], f.red(ck * m[i]));
            }
            *fpm += d as u64;
        }
        let mut r = [0u64; 5];
        r[..d].copy_from_slice(&c[..d]);
        r
    }

    /// Monic gcd of a (degree < d, given as array) and monic m (degree d).
    fn gcd_monic(&self, a: &[u64; 5], m: &[u64; 5], d: usize, fpm: &mut u64) -> ([u64; 5], usize) {
        let f = &self.f;
        let deg = |x: &[u64; 5], n: isize| -> isize {
            let mut k = n;
            while k >= 0 && x[k as usize] == 0 {
                k -= 1;
            }
            k
        };
        let mut x = *m;
        let mut dx = d as isize;
        let mut y = *a;
        let mut dy = deg(&y, d as isize - 1);
        while dy >= 0 {
            let li = self.inv(y[dy as usize]);
            while dx >= dy {
                let c = f.red(x[dx as usize] * li);
                let sh = (dx - dy) as usize;
                for j in 0..=dy as usize {
                    x[sh + j] = f.sub(x[sh + j], f.red(c * y[j]));
                }
                *fpm += 2 + dy as u64;
                dx = deg(&x, dx - 1);
            }
            std::mem::swap(&mut x, &mut y);
            std::mem::swap(&mut dx, &mut dy);
        }
        let dx = dx as usize;
        let li = self.inv(x[dx]);
        for c in x.iter_mut().take(dx + 1) {
            *c = f.red(*c * li);
        }
        *fpm += dx as u64 + 1;
        (x, dx)
    }

    /// Appends the roots of a monic m of degree d that is a product of distinct
    /// linear factors over F_p (equal-degree splitting, deterministic shifts).
    fn split_roots(&self, m: &[u64; 5], d: usize, out: &mut Vec<u64>, fpm: &mut u64) {
        let f = &self.f;
        let p = f.p;
        match d {
            0 => {}
            1 => out.push(f.neg(m[0])),
            2 => {
                let disc = f.sub(f.red(m[1] * m[1]), f.red(4 * m[0]));
                let sq = self.sqrt[disc as usize];
                assert!(sq != u32::MAX, "split quadratic with a non-square discriminant");
                let i2 = self.inv(2);
                let nb = f.neg(m[1]);
                out.push(f.red(f.add(nb, sq as u64) * i2));
                out.push(f.red(f.sub(nb, sq as u64) * i2));
                *fpm += 4;
            }
            _ => {
                let e = (p - 1) / 2;
                for delta in 0..p {
                    // h = (x + delta)^e mod m
                    let base = {
                        let mut b = [0u64; 5];
                        b[0] = delta;
                        b[1] = 1;
                        b
                    };
                    let mut h = [0u64; 5];
                    h[0] = 1;
                    let bits = 64 - e.leading_zeros();
                    for i in (0..bits).rev() {
                        h = self.mulmod_small(&h, &h, m, d, fpm);
                        if (e >> i) & 1 == 1 {
                            h = self.mulmod_small(&h, &base, m, d, fpm);
                        }
                    }
                    h[0] = f.sub(h[0], 1);
                    if h.iter().all(|&x| x == 0) {
                        continue;
                    }
                    let (g, dg) = self.gcd_monic(&h, m, d, fpm);
                    if dg == 0 || dg == d {
                        continue;
                    }
                    // q = m / g (exact)
                    let mut r = *m;
                    let dq = d - dg;
                    let mut q = [0u64; 5];
                    for i in (0..=dq).rev() {
                        let c = r[i + dg];
                        q[i] = c;
                        if c != 0 {
                            for j in 0..=dg {
                                r[i + j] = f.sub(r[i + j], f.red(c * g[j]));
                            }
                            *fpm += dg as u64 + 1;
                        }
                    }
                    debug_assert!(r[..dg].iter().all(|&x| x == 0));
                    self.split_roots(&g, dg, out, fpm);
                    self.split_roots(&q, dq, out, fpm);
                    return;
                }
                panic!("equal-degree splitting found no separating shift");
            }
        }
    }

    /// Sorted roots of a monic quartic that splits into distinct linear factors.
    pub fn roots_split_quartic(&self, m: &[u64; 4], fpm: &mut u64) -> Vec<u64> {
        let mm = [m[0], m[1], m[2], m[3], 1];
        let mut out = Vec::with_capacity(4);
        self.split_roots(&mm, 4, &mut out, fpm);
        out.sort_unstable();
        out
    }
}

/// `cover::hyperplane_relation` with the fast arithmetic: same RNG draws, same
/// statistics, same relation (the counter of K4 products gets the K4 products
/// this path does).
pub fn hyperplane_relation(s: &Setup, fb: &FactorBase, rf: &RelFast, rng: &mut Rng, st: &mut Stats) -> Option<Relation> {
    let f = &rf.f;
    let p = f.p;
    let mut fpm = 0u64;
    st.candidates += 1;
    let mut chosen = [0usize; 4];
    let mut nch = 0;
    while nch < 4 {
        let j = fb.pick[rng.below(fb.pick.len() as u64) as usize];
        if chosen[..nch].iter().any(|&c| c / 2 == j) {
            continue;
        }
        chosen[nch] = 2 * j + (rng.next() & 1) as usize;
        nch += 1;
    }
    let hyp_cols = [0usize, 2, 3, 4, 5];
    // ---- 4 x 5 kernel: the same Gauss-Jordan elimination as cover::kernel_fp ----
    let mut m = [[0u64; 5]; 4];
    for (r, &i) in chosen.iter().enumerate() {
        for (t, &col) in hyp_cols.iter().enumerate() {
            m[r][t] = fb.rows[i][col];
        }
    }
    let mut piv = [0usize; 5];
    let mut npiv = 0;
    let mut r = 0;
    for col in 0..5 {
        let Some(pr) = (r..4).find(|&i| m[i][col] != 0) else { continue };
        m.swap(r, pr);
        let inv = rf.inv(m[r][col]);
        for j in 0..5 {
            m[r][j] = f.red(m[r][j] * inv);
        }
        fpm += 5;
        for i in 0..4 {
            if i != r && m[i][col] != 0 {
                let fc = m[i][col];
                for j in 0..5 {
                    m[i][j] = f.sub(m[i][j], f.red(fc * m[r][j]));
                }
                fpm += 5;
            }
        }
        piv[npiv] = col;
        npiv += 1;
        r += 1;
        if r == 4 {
            break;
        }
    }
    if npiv != 4 {
        rf.fp_muls.set(rf.fp_muls.get() + fpm);
        st.degenerate += 1;
        return None;
    }
    let free = (0..5).find(|c| !piv[..npiv].contains(c)).unwrap();
    let mut ker = [0u64; 5];
    ker[free] = 1;
    for ri in 0..npiv {
        ker[piv[ri]] = f.neg(m[ri][free]);
    }
    let mut c = [0u64; 12];
    for (t, &col) in hyp_cols.iter().enumerate() {
        c[col] = ker[t];
    }
    if c[0] == 0 {
        rf.fp_muls.set(rf.fp_muls.get() + fpm);
        st.symmetric += 1;
        let mut pts = Vec::with_capacity(8);
        for &i in &chosen {
            pts.push(i & !1);
            pts.push(i | 1);
        }
        return Some(Relation { pts, inf: -1, sym: true });
    }
    // known = prod (x - x_i), monic quartic
    let mut known = [0u64; 5];
    known[0] = 1;
    let mut dk = 0;
    let mut xs = [0u64; 4];
    for (t, &i) in chosen.iter().enumerate() {
        xs[t] = fb.xs[i / 2] % p;
        let nx = f.neg(xs[t]);
        // known *= (x + nx)
        for j in (0..=dk + 1).rev() {
            let hi = if j > 0 { known[j - 1] } else { 0 };
            let lo = if j <= dk { f.red(known[j] * nx) } else { 0 };
            known[j] = f.add(hi, lo);
        }
        fpm += dk as u64 + 1;
        dk += 1;
    }
    // ---- residual = N(G) / known, N(G) the degree-8 norm polynomial ----
    let k = s.k();
    let res = match rf.residual6(&c, &known, &xs) {
        Some((res, full, k4m, nf)) => {
            k.muls.set(k.muls.get() + k4m);
            fpm += nf;
            if !full {
                // deg n < 8 exactly when deg res < 4
                rf.fp_muls.set(rf.fp_muls.get() + fpm);
                st.degenerate += 1;
                return None;
            }
            res
        }
        None => {
            let (n, k4m, nf) = rf.norm8(&c);
            k.muls.set(k.muls.get() + k4m);
            fpm += nf;
            if n[8] == 0 {
                rf.fp_muls.set(rf.fp_muls.get() + fpm);
                st.degenerate += 1;
                return None;
            }
            // res = n / known (exact), remainder must vanish
            let mut rem = n;
            let mut res = [0u64; 5];
            for i in (0..5).rev() {
                let cq = rem[i + 4];
                res[i] = cq;
                if cq == 0 {
                    continue;
                }
                for j in 0..=4 {
                    rem[i + j] = f.sub(rem[i + j], f.red(cq * known[j]));
                }
                fpm += 5;
            }
            assert!(rem[..4].iter().all(|&x| x == 0), "hyperplane does not vanish on its defining points");
            res
        }
    };
    // monic residual (same roots)
    let li = rf.inv(res[4]);
    let mon = [f.red(res[0] * li), f.red(res[1] * li), f.red(res[2] * li), f.red(res[3] * li)];
    fpm += 4;
    // x^p mod res: number of distinct F_p roots, and the split test (x^p == x)
    let q = Quart::for_monic(f, &mon);
    fpm += 8;
    let mut xp = rf.pow_x(&q, p, &mut fpm);
    xp[1] = f.sub(xp[1], 1);
    let nroots = if xp == [0; 4] {
        4
    } else {
        let mut g = [0u64; 5];
        g[..4].copy_from_slice(&xp);
        let mut dg = 3isize;
        while dg >= 0 && g[dg as usize] == 0 {
            dg -= 1;
        }
        let mm = [mon[0], mon[1], mon[2], mon[3], 1];
        rf.gcd_deg(mm, 4, g, dg, &mut fpm).max(0) as usize
    };
    st.root_hist[nroots.min(4)] += 1;
    if nroots != 4 {
        rf.fp_muls.set(rf.fp_muls.get() + fpm);
        return None;
    }
    // a chosen x that is also a root of the residual: degenerate
    let ev = |x: u64| -> u64 {
        let mut acc = 0u64;
        for &cc in res.iter().rev() {
            acc = f.red(acc * x + cc);
        }
        acc
    };
    fpm += 4 * 5;
    if chosen.iter().any(|&i| ev(fb.xs[i / 2] % p) == 0) {
        rf.fp_muls.set(rf.fp_muls.get() + fpm);
        st.degenerate += 1;
        return None;
    }
    let roots = rf.roots_split_quartic(&mon, &mut fpm);
    let mut pts = chosen.to_vec();
    for x in roots {
        pts.push(identify(fb, &c, x, p).expect("simple F_p root of the residual must be a rational point"));
        fpm += 24;
    }
    rf.fp_muls.set(rf.fp_muls.get() + fpm);
    st.found += 1;
    Some(Relation { pts, inf: -1, sym: false })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cover::{hyperplane_relation as slow_relation, norm_poly, FactorBase, Nodes, Setup, Stats};
    use crate::poly;

    /// The table build counts every F_p product it does: the counted Lagrange
    /// basis equals `poly::Interp::new` and its count equals a separate count
    /// made with test-local counting copies of `poly::mul` and `field::powm`,
    /// and `RelFast::new` reports 4 * 28 + basis + (p - 2) + p.
    #[test]
    fn rel_fast_table_build_counts_every_product() {
        fn cmul(a: &[u64], b: &[u64], p: u64, n: &mut u64) -> Vec<u64> {
            let mut c = vec![0u64; a.len() + b.len() - 1];
            for (i, &ai) in a.iter().enumerate() {
                if ai == 0 {
                    continue;
                }
                for (j, &bj) in b.iter().enumerate() {
                    c[i + j] = (c[i + j] + ai * bj % p) % p;
                    *n += 1;
                }
            }
            while c.last() == Some(&0) {
                c.pop();
            }
            c
        }
        fn cpow(mut a: u64, mut e: u64, p: u64, n: &mut u64) -> u64 {
            let mut r = 1;
            while e > 0 {
                if e & 1 == 1 {
                    r = r * a % p;
                    *n += 1;
                }
                a = a * a % p;
                *n += 1;
                e >>= 1;
            }
            r
        }
        for p in [7u64, 17, 101, 1009, 16001, 24001, 32003] {
            let mut want = 0u64;
            let mut basis = Vec::new();
            for t in 0..5u64 {
                let (mut num, mut den) = (vec![1u64], 1u64);
                for s in 0..5u64 {
                    if s != t {
                        num = cmul(&num, &[(p - s) % p, 1], p, &mut want);
                        den = den * ((t + p - s) % p) % p;
                        want += 1;
                    }
                }
                let di = cpow(den, p - 2, p, &mut want);
                basis.push(num.iter().map(|&c| { want += 1; c * di % p }).collect::<Vec<_>>());
            }
            let mut got = 0u64;
            let b5 = interp_basis_counted::<5>(p, &mut got);
            let reference = poly::Interp::new(4, p);
            for t in 0..5 {
                assert_eq!(&b5[t][..reference.basis[t].len()], &reference.basis[t][..], "p={p}");
                assert_eq!(&b5[t][..basis[t].len()], &basis[t][..], "p={p}");
            }
            assert_eq!(got, want, "p={p}");
        }
        for (p, tw, seed) in [(101u64, 1i32, 7u64), (1009, -1, 2)] {
            let mut rng = Rng(seed);
            let s = Setup::new(p, tw, &mut rng);
            let nodes = Nodes::new(&s);
            let k4_before = s.k().muls.get();
            let rf = RelFast::new(&s, &nodes);
            let mut b = 0u64;
            interp_basis_counted::<5>(p, &mut b);
            assert_eq!(rf.fp_muls.get(), 4 * 28 + b + (p - 2) + p, "p={p}");
            assert_eq!(s.k().muls.get(), k4_before, "table K4 products are counted once, in fp_muls");
        }
    }

    /// p < 2^16: the Barrett step is exact on the extreme inputs, and the
    /// lazy K4 product (the largest accumulator of this path, 4 (p-1)^2 (1 + 3 (p-1))
    /// with every input and every reduction entry p - 1) equals the u128 product.
    #[test]
    fn rel_fast_lazy_bounds_hold_near_2_16() {
        let mut rng = Rng(0x16);
        for p in [32771u64, 49999, 65519, 65521] {
            let f = Fp::new(p);
            let pm = p - 1;
            let worst = 4 * pm * pm * (1 + 3 * pm);
            assert!(worst < 1u64 << 52, "p={p}");
            for a in [0u64, 1, pm, p, 2 * p - 1, pm * pm, 4 * pm * pm, 9 * pm * pm, worst, worst + 1, u64::MAX, u64::MAX - 1, u64::MAX / p * p, u64::MAX / p * p - 1] {
                assert_eq!(f.red(a), a % p, "p={p} a={a}");
            }
            for _ in 0..20000 {
                let a = rng.next();
                assert_eq!(f.red(a), a % p);
                let b = rng.next() >> 12;
                assert_eq!(f.red(b), b % p);
            }
            // all-(p-1) reduction table and inputs: exact against u128 arithmetic
            let reference = |q: &Quart, a: &E4, b: &E4| -> E4 {
                let mut c = [0u128; 7];
                for i in 0..4 {
                    for j in 0..4 {
                        c[i + j] += a[i] as u128 * b[j] as u128;
                    }
                }
                let mut r = [c[0], c[1], c[2], c[3]];
                for h in 0..3 {
                    for i in 0..4 {
                        r[i] += c[4 + h] * q.red[h][i] as u128;
                    }
                }
                [(r[0] % p as u128) as u64, (r[1] % p as u128) as u64, (r[2] % p as u128) as u64, (r[3] % p as u128) as u64]
            };
            let qmax = Quart { red: [[pm; 4]; 3] };
            let full = [pm; 4];
            assert_eq!(qmax.mul(&f, &full, &full), reference(&qmax, &full, &full), "p={p}");
            for _ in 0..2000 {
                let q = Quart::for_monic(&f, &[rng.below(p), rng.below(p), rng.below(p), rng.below(p)]);
                let a = [rng.below(p), rng.below(p), rng.below(p), rng.below(p)];
                let b = [rng.below(p), pm, rng.below(p), pm];
                assert_eq!(q.mul(&f, &a, &b), reference(&q, &a, &b));
                assert_eq!(qmax.mul(&f, &a, &b), reference(&qmax, &a, &b));
            }
        }
    }

    /// The fast relation search equals the reference path on an instance with
    /// p = 65521, the largest prime below 2^16 (norm polynomials, split roots,
    /// whole candidates with the same RNG stream and statistics).
    #[test]
    fn rel_fast_matches_reference_at_p_65521() {
        let (p, tw, seed) = (65521u64, 1i32, 1u64);
        let mut rng = Rng(seed);
        let s = Setup::new_with(p, tw, &mut rng, true);
        let fb = FactorBase::build(&s);
        let nodes = Nodes::new(&s);
        let rf = RelFast::new(&s, &nodes);
        for _ in 0..100 {
            let mut c = [0u64; 12];
            for col in [0usize, 1, 2, 3, 4, 5] {
                c[col] = if rng.next() & 3 == 0 { p - 1 } else { rng.below(p) };
            }
            let slow = norm_poly(&s, &nodes, &c, true);
            let (fast, _, _) = rf.norm8(&c);
            let mut pad = [0u64; 9];
            pad[..slow.len()].copy_from_slice(&slow);
            assert_eq!(pad, fast, "c={c:?}");
        }
        let mut fpm = 0;
        for k in 0..100u64 {
            let rs: Vec<u64> = if k == 0 { vec![p - 4, p - 3, p - 2, p - 1] } else {
                let mut rs: Vec<u64> = Vec::new();
                while rs.len() < 4 {
                    let x = rng.below(p);
                    if !rs.contains(&x) {
                        rs.push(x);
                    }
                }
                rs.sort();
                rs
            };
            let mut m = vec![1u64];
            for &x in &rs {
                m = poly::mul(&m, &[(p - x) % p, 1], p);
            }
            assert_eq!(rf.roots_split_quartic(&[m[0], m[1], m[2], m[3]], &mut fpm), rs);
        }
        let (mut r1, mut r2) = (Rng(seed ^ 77), Rng(seed ^ 77));
        let (mut st1, mut st2) = (Stats::default(), Stats::default());
        let mut found = 0;
        for _ in 0..400 {
            let a = slow_relation(&s, &fb, &nodes, &mut r1, &mut st1);
            let b = hyperplane_relation(&s, &fb, &rf, &mut r2, &mut st2);
            assert_eq!(r1.0, r2.0);
            match (a, b) {
                (None, None) => {}
                (Some(a), Some(b)) => {
                    assert_eq!((a.pts, a.inf, a.sym), (b.pts, b.inf, b.sym));
                    found += 1;
                }
                (a, b) => panic!("reference {a:?} vs fast {b:?}"),
            }
        }
        assert!(found > 0);
        assert_eq!(
            (st1.candidates, st1.degenerate, st1.found, st1.symmetric, st1.root_hist),
            (st2.candidates, st2.degenerate, st2.found, st2.symmetric, st2.root_hist)
        );
    }

    #[test]
    #[should_panic]
    fn fp_rejects_p_at_or_above_2_16() {
        Fp::new(65537);
    }

    #[test]
    fn rel_fast_norm_roots_and_relations_match_the_reference_path() {
        for (p, tw, seed) in [(17u64, 1i32, 3u64), (17, -1, 4), (101, 1, 7), (103, -1, 8)] {
            let mut rng = Rng(seed);
            let s = Setup::new(p, tw, &mut rng);
            let fb = FactorBase::build(&s);
            let nodes = Nodes::new(&s);
            let rf = RelFast::new(&s, &nodes);
            // norm polynomial at random coefficient vectors
            for _ in 0..300 {
                let mut c = [0u64; 12];
                for col in [0usize, 1, 2, 3, 4, 5] {
                    c[col] = rng.below(p);
                }
                let slow = norm_poly(&s, &nodes, &c, true);
                let (fast, _, _) = rf.norm8(&c);
                let mut pad = [0u64; 9];
                pad[..slow.len()].copy_from_slice(&slow);
                assert_eq!(pad, fast, "p={p} c={c:?}");
            }
            // roots of random split quartics
            let mut fpm = 0;
            for _ in 0..200 {
                let mut rs: Vec<u64> = Vec::new();
                while rs.len() < 4 {
                    let x = rng.below(p);
                    if !rs.contains(&x) {
                        rs.push(x);
                    }
                }
                let mut m = vec![1u64];
                for &x in &rs {
                    m = poly::mul(&m, &[(p - x) % p, 1], p);
                }
                rs.sort();
                assert_eq!(rf.roots_split_quartic(&[m[0], m[1], m[2], m[3]], &mut fpm), rs);
                assert_eq!(poly::roots(&m, p), rs);
            }
            // whole candidates: same RNG stream, same statistics, same relations
            let (mut r1, mut r2) = (Rng(seed ^ 77), Rng(seed ^ 77));
            let (mut st1, mut st2) = (Stats::default(), Stats::default());
            let mut found = 0;
            for _ in 0..3000 {
                let a = slow_relation(&s, &fb, &nodes, &mut r1, &mut st1);
                let b = hyperplane_relation(&s, &fb, &rf, &mut r2, &mut st2);
                assert_eq!(r1.0, r2.0);
                match (a, b) {
                    (None, None) => {}
                    (Some(a), Some(b)) => {
                        assert_eq!((a.pts, a.inf, a.sym), (b.pts, b.inf, b.sym));
                        found += 1;
                    }
                    (a, b) => panic!("p={p}: reference {a:?} vs fast {b:?}"),
                }
            }
            assert!(found > 0);
            assert_eq!(
                (st1.candidates, st1.degenerate, st1.found, st1.symmetric, st1.root_hist),
                (st2.candidates, st2.degenerate, st2.found, st2.symmetric, st2.root_hist)
            );
        }
    }
}
