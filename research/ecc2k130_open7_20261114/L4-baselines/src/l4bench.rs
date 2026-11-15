//! L4 lane harness: the "fix m-1 factor-base points, gcd with L_V" decomposition
//! oracle (arm G), counted in field operations, run on the exact targets an
//! `ic descent` JSON recorded, with its solution sets checked against the
//! repository engines' and against brute force.
//!
//! std only; build: rustc --edition 2021 -C opt-level=3 -C target-cpu=native l4bench.rs
//!
//! Subcommands:
//!   selftest  N L M TARGETS SEED            G vs brute force on random targets
//!   gcd       IC_JSON OUT_JSONL             G on every cell/target of an ic descent JSON
//!   summarize ROWS_JSONL... --out DIR       ratios, crossovers, verdict, SVG

use std::collections::BTreeMap;
use std::fmt::Write as _;

// ───────────────────────── field GF(2^n), n <= 63 ─────────────────────────

#[derive(Default, Clone, Debug)]
struct Ctr {
    mul: std::cell::Cell<u64>,
    sqr: std::cell::Cell<u64>,
    add: std::cell::Cell<u64>,
    inv: std::cell::Cell<u64>,
}

#[derive(Clone)]
struct Gf {
    n: u32,
    mask: u64,
    low: u64, // modulus minus x^n
}

#[inline(always)]
fn clmul(a: u64, b: u64) -> u128 {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        use std::arch::x86_64::*;
        let x = _mm_set_epi64x(0, a as i64);
        let y = _mm_set_epi64x(0, b as i64);
        let r = _mm_clmulepi64_si128(x, y, 0);
        let lo = _mm_cvtsi128_si64(r) as u64;
        let hi = _mm_extract_epi64(r, 1) as u64;
        ((hi as u128) << 64) | lo as u128
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let mut r = 0u128;
        for i in 0..64 {
            if (b >> i) & 1 == 1 {
                r ^= (a as u128) << i;
            }
        }
        r
    }
}

fn is_irreducible(f: u128, n: u32) -> bool {
    // Rabin test via x^(2^k) mod f, gcd checks; n <= 63.
    let modf = |mut x: u128| -> u128 {
        for i in (n as i32..128).rev() {
            if (x >> i) & 1 == 1 {
                x ^= f << (i as u32 - n);
            }
        }
        x
    };
    let mulm = |a: u128, b: u128| -> u128 {
        let mut r = 0u128;
        let mut a = a;
        let mut b = b;
        while b != 0 {
            if b & 1 == 1 {
                r ^= a;
            }
            b >>= 1;
            a <<= 1;
            a = modf(a);
        }
        modf(r)
    };
    let gcd = |mut a: u128, mut b: u128| -> u128 {
        let deg = |x: u128| 127 - x.leading_zeros() as i32;
        while b != 0 {
            while a != 0 && deg(a) >= deg(b) {
                a ^= b << (deg(a) - deg(b));
            }
            std::mem::swap(&mut a, &mut b);
        }
        a
    };
    // x^(2^n) == x mod f
    let mut x = 2u128;
    let mut pows = vec![];
    for _ in 0..n {
        x = mulm(x, x);
        pows.push(x);
    }
    if x != 2 {
        return false;
    }
    let mut p = n;
    let mut primes = vec![];
    let mut d = 2;
    while d * d <= p {
        if p % d == 0 {
            primes.push(d);
            while p % d == 0 {
                p /= d;
            }
        }
        d += 1;
    }
    if p > 1 {
        primes.push(p);
    }
    for q in primes {
        let k = n / q;
        let xk = pows[(k - 1) as usize];
        if gcd(f, xk ^ 2) != 1 {
            return false;
        }
    }
    true
}

impl Gf {
    /// Smallest-mask irreducible of degree n (as koblitz_index_calculus::find_irreducible).
    fn new(n: u32) -> Gf {
        let hi = 1u128 << n;
        let mut low = 1u64;
        loop {
            if is_irreducible(hi | low as u128, n) {
                break;
            }
            low += 1;
        }
        Gf {
            n,
            mask: (1u64 << n) - 1,
            low,
        }
    }
    #[inline(always)]
    fn reduce(&self, mut x: u128) -> u64 {
        loop {
            let hi = x >> self.n;
            if hi == 0 {
                return x as u64;
            }
            x = (x & self.mask as u128) ^ clmul(hi as u64, self.low);
        }
    }
    #[inline(always)]
    fn mul(&self, c: &Ctr, a: u64, b: u64) -> u64 {
        c.mul.set(c.mul.get() + (1));
        self.reduce(clmul(a, b))
    }
    #[inline(always)]
    fn sqr(&self, c: &Ctr, a: u64) -> u64 {
        c.sqr.set(c.sqr.get() + (1));
        self.reduce(clmul(a, a))
    }
    #[inline(always)]
    fn add(&self, c: &Ctr, a: u64, b: u64) -> u64 {
        c.add.set(c.add.get() + (1));
        a ^ b
    }
    fn mul_raw(&self, a: u64, b: u64) -> u64 {
        self.reduce(clmul(a, b))
    }
    fn inv(&self, c: &Ctr, a: u64) -> u64 {
        assert!(a != 0);
        c.inv.set(c.inv.get() + (1));
        // a^(2^n - 2), uncounted internally (priced by Itoh-Tsujii in summary)
        let mut r = 1u64;
        let mut base = a;
        let mut e: u64 = (1u64 << self.n) - 2;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul_raw(r, base);
            }
            base = self.mul_raw(base, base);
            e >>= 1;
        }
        r
    }
    fn sqr_k(&self, a: u64, k: u32) -> u64 {
        let mut x = a;
        for _ in 0..k {
            x = self.mul_raw(x, x);
        }
        x
    }
    /// Half-trace (n odd): solution z of z^2 + z = a when Tr(a) = 0.
    fn half_trace(&self, c: &Ctr, a: u64) -> u64 {
        let mut h = a;
        let mut t = a;
        for _ in 0..(self.n - 1) / 2 {
            t = self.sqr(c, t);
            t = self.sqr(c, t);
            h = self.add(c, h, t);
        }
        h
    }
}

// ───────────────────────── subspace polynomial ─────────────────────────

/// a_0..a_l of L_V(t) = sum a_i t^(2^i) for V = span(basis).
fn subspace_poly(gf: &Gf, c: &Ctr, basis: &[u64]) -> Vec<u64> {
    let mut a = vec![1u64];
    for &b in basis {
        let mut lb = 0u64;
        for (j, &aj) in a.iter().enumerate() {
            let bp = gf.sqr_k(b, j as u32);
            c.sqr.set(c.sqr.get() + (j as u64));
            lb = gf.add(c, lb, gf.mul(c, aj, bp));
        }
        let mut next = vec![0u64; a.len() + 1];
        for (j, &aj) in a.iter().enumerate() {
            let s = gf.sqr(c, aj);
            next[j + 1] = gf.add(c, next[j + 1], s);
            let m = gf.mul(c, lb, aj);
            next[j] = gf.add(c, next[j], m);
        }
        a = next;
    }
    a
}

/// Horner coefficients b_j = a_j^(2^-j).
fn horner_coeffs(gf: &Gf, c: &Ctr, a: &[u64]) -> Vec<u64> {
    let n = gf.n;
    a.iter()
        .enumerate()
        .map(|(j, &aj)| {
            let k = (n - (j as u32 % n)) % n;
            c.sqr.set(c.sqr.get() + (k as u64));
            gf.sqr_k(aj, k)
        })
        .collect()
}

fn eval_lv(gf: &Gf, a: &[u64], x: u64) -> u64 {
    let mut s = 0;
    for (j, &aj) in a.iter().enumerate() {
        s ^= gf.mul_raw(aj, gf.sqr_k(x, j as u32));
    }
    s
}

// ───────────────────────── generic polynomials (counted) ─────────────────────────

fn deg(p: &[u64]) -> i32 {
    for i in (0..p.len()).rev() {
        if p[i] != 0 {
            return i as i32;
        }
    }
    -1
}

/// Pseudo-remainder gcd; returns the last non-zero remainder (not monic).
fn pgcd(gf: &Gf, c: &Ctr, f: &[u64], g: &[u64]) -> Vec<u64> {
    let mut a = f.to_vec();
    let mut b = g.to_vec();
    loop {
        let db = deg(&b);
        if db < 0 {
            return a;
        }
        loop {
            let da = deg(&a);
            if da < db {
                break;
            }
            let la = a[da as usize];
            let lb = b[db as usize];
            let shift = (da - db) as usize;
            for i in 0..=da as usize {
                if a[i] != 0 {
                    a[i] = gf.mul(c, a[i], lb);
                }
            }
            for i in 0..=db as usize {
                if b[i] != 0 {
                    let t = gf.mul(c, b[i], la);
                    a[i + shift] = gf.add(c, a[i + shift], t);
                }
            }
            a[da as usize] = 0;
        }
        std::mem::swap(&mut a, &mut b);
    }
}

fn poly_eval(gf: &Gf, c: &Ctr, p: &[u64], x: u64) -> u64 {
    let d = deg(p);
    if d < 0 {
        return 0;
    }
    let mut r = p[d as usize];
    for i in (0..d as usize).rev() {
        r = gf.mul(c, r, x);
        r = gf.add(c, r, p[i]);
    }
    r
}

/// x^k mod monic f (deg f = df), for k < 2*df, as a vector of length df.
fn sqr_mod_monic(gf: &Gf, c: &Ctr, u: &[u64], f: &[u64], df: usize) -> Vec<u64> {
    let mut w = vec![0u64; 2 * df];
    for i in 0..df {
        if u[i] != 0 {
            w[2 * i] = gf.sqr(c, u[i]);
        }
    }
    for k in (df..2 * df).rev() {
        let t = w[k];
        if t != 0 {
            w[k] = 0;
            for i in 0..df {
                if f[i] != 0 {
                    let m = gf.mul(c, t, f[i]);
                    w[k - df + i] = gf.add(c, w[k - df + i], m);
                }
            }
        }
    }
    w.truncate(df);
    w
}

/// Roots in V of an arbitrary polynomial f (coefficients low->high), counted:
/// generic route (used for degenerate leading coefficients).
fn roots_in_v_generic(
    gf: &Gf,
    c: &Ctr,
    f: &[u64],
    hb: &[u64],
    vlist_len_bits: u32,
    vbasis_shift: u32,
) -> Vec<u64> {
    let d = deg(f);
    let vsize = 1u64 << vlist_len_bits;
    if d < 0 {
        return (0..vsize).map(|v| v << vbasis_shift).collect();
    }
    if d == 0 {
        return vec![];
    }
    let d = d as usize;
    let li = gf.inv(c, f[d]);
    let mut m: Vec<u64> = (0..d).map(|i| if f[i] != 0 { gf.mul(c, f[i], li) } else { 0 }).collect();
    let mut monic = m.clone();
    monic.push(1);
    // L_V mod f by Horner in phi(u) = u^2 mod f; phi is semilinear.
    let l = hb.len() - 1;
    let mut u = vec![0u64; d.max(2)];
    // b_l t mod f
    let mut t_mod = vec![0u64; d.max(2)];
    if d >= 2 {
        t_mod[1] = 1;
    } else {
        t_mod[0] = m[0]; // t = -f0 = f0
    }
    for i in 0..d {
        if t_mod[i] != 0 {
            u[i] = gf.mul(c, hb[l], t_mod[i]);
        }
    }
    for j in (0..l).rev() {
        u = sqr_mod_monic(gf, c, &u[..d], &m, d);
        u.resize(d.max(2), 0);
        for i in 0..d {
            if t_mod[i] != 0 && hb[j] != 0 {
                let t = gf.mul(c, hb[j], t_mod[i]);
                u[i] = gf.add(c, u[i], t);
            }
        }
    }
    u.truncate(d);
    let g = pgcd(gf, c, &monic, &u);
    m.clear();
    let dg = deg(&g);
    if dg <= 0 {
        return vec![];
    }
    // the gcd divides L_V: its roots are in V; find them by evaluation over V.
    let mut out = vec![];
    for v in 0..vsize {
        let x = v << vbasis_shift;
        if poly_eval(gf, c, &g, x) == 0 {
            out.push(x);
        }
    }
    out
}

// ───────────────────────── arm G, m = 2 ─────────────────────────

/// All ordered (x1, x2) in V^2 with S3(x1, x2, xr) = 0, V = span{e_k << shift}.
/// Returns solutions as (x1, x2) pairs and the per-target counter.
fn g_m2(gf: &Gf, xr: u64, b: u64, l: u32, shift: u32, hb: &[u64]) -> (Vec<(u64, u64)>, Ctr) {
    let c = Ctr::default();
    let mut sols = vec![];
    let basis: Vec<u64> = (0..l).map(|k| 1u64 << (k + shift)).collect();
    let xr2 = gf.sqr(&c, xr);
    // deltas for the affine-in-x1 quantities x1^2, x1*xr, (x1*xr)^2
    let mut d_s1 = vec![];
    let mut d_b = vec![];
    let mut d_c = vec![];
    for &e in &basis {
        d_s1.push(gf.sqr(&c, e));
        let t = gf.mul(&c, e, xr);
        d_b.push(t);
        d_c.push(gf.sqr(&c, t));
    }
    let size = 1u64 << l;
    let chunk = 1024usize;
    let mut x1 = 0u64;
    let (mut s1, mut bb, mut cc) = (0u64, 0u64, b); // C = B^2 + b
    let mut buf: Vec<(u64, u64, u64, u64)> = Vec::with_capacity(chunk); // x1, A, B, C
    let lidx = hb.len() - 1;
    let mut flush = |buf: &mut Vec<(u64, u64, u64, u64)>, c: &Ctr, sols: &mut Vec<(u64, u64)>| {
        // batch inversion of the non-zero A's
        let idx: Vec<usize> = (0..buf.len()).filter(|&i| buf[i].1 != 0).collect();
        let k = idx.len();
        let mut pref = vec![0u64; k];
        let mut invs = vec![0u64; k];
        if k > 0 {
            pref[0] = buf[idx[0]].1;
            for i in 1..k {
                pref[i] = gf.mul(c, pref[i - 1], buf[idx[i]].1);
            }
            let mut acc = gf.inv(c, pref[k - 1]);
            for i in (1..k).rev() {
                invs[i] = gf.mul(c, acc, pref[i - 1]);
                acc = gf.mul(c, acc, buf[idx[i]].1);
            }
            invs[0] = acc;
        }
        let mut ii = 0;
        for i in 0..buf.len() {
            let (x1, a, bq, cq) = buf[i];
            if a == 0 {
                // f = B X + C (x1 = xr)
                if bq != 0 {
                    let r = gf.mul(c, cq, gf.inv(c, bq));
                    c.add.set(c.add.get() + (1)); // membership test
                    if in_v(r, l, shift) {
                        sols.push((x1, r));
                    }
                }
                continue;
            }
            let ai = invs[ii];
            ii += 1;
            let cf = gf.mul(c, bq, ai);
            let df = gf.mul(c, cq, ai);
            // Horner: u = b_l t ; for j = l-1..0: u = phi(u) + b_j t
            let (mut u0, mut u1) = (0u64, hb[lidx]);
            for j in (0..lidx).rev() {
                let s0 = gf.sqr(c, u0);
                let s1q = gf.sqr(c, u1);
                let m0 = gf.mul(c, s1q, df);
                u0 = gf.add(c, s0, m0);
                u1 = gf.mul(c, s1q, cf);
                u1 = gf.add(c, u1, hb[j]);
            }
            if u1 == 0 {
                if u0 == 0 {
                    // f | L_V: both roots in V. X = cf*z, z^2+z = df/cf^2
                    let ci = gf.inv(c, cf);
                    let ci2 = gf.sqr(c, ci);
                    let rhs = gf.mul(c, df, ci2);
                    let z = gf.half_trace(c, rhs);
                    let r1 = gf.mul(c, cf, z);
                    let r2 = gf.add(c, r1, cf);
                    for r in [r1, r2] {
                        assert!(in_v(r, l, shift), "f | L_V but root outside V");
                        sols.push((x1, r));
                    }
                }
                // else gcd = 1
            } else {
                // candidate root t = u0/u1: f(t) u1^2 = u0^2 + cf u0 u1 + df u1^2
                let a0 = gf.sqr(c, u0);
                let a1 = gf.mul(c, cf, gf.mul(c, u0, u1));
                let a2 = gf.mul(c, df, gf.sqr(c, u1));
                let v = gf.add(c, gf.add(c, a0, a1), a2);
                if v == 0 {
                    let t = gf.mul(c, u0, gf.inv(c, u1));
                    c.add.set(c.add.get() + (1));
                    assert!(in_v(t, l, shift), "gcd root outside V");
                    sols.push((x1, t));
                }
            }
        }
        buf.clear();
    };
    for i in 0..size {
        if i > 0 {
            let k = i.trailing_zeros() as usize;
            x1 ^= basis[k];
            s1 = gf.add(&c, s1, d_s1[k]);
            bb = gf.add(&c, bb, d_b[k]);
            cc = gf.add(&c, cc, d_c[k]);
        }
        let a = gf.add(&c, s1, xr2);
        buf.push((x1, a, bb, cc));
        if buf.len() == chunk {
            flush(&mut buf, &c, &mut sols);
        }
    }
    if !buf.is_empty() {
        flush(&mut buf, &c, &mut sols);
    }
    sols.sort();
    (sols, c)
}

#[inline(always)]
fn in_v(x: u64, l: u32, shift: u32) -> bool {
    (x & ((1u64 << shift) - 1)) == 0 && (x >> shift) < (1u64 << l)
}

// ───────────────────────── arm G, m = 3 ─────────────────────────

fn g_m3(
    gf: &Gf,
    xr: u64,
    b: u64,
    l: u32,
    shift: u32,
    hb: &[u64],
) -> (Vec<(u64, u64, u64)>, Ctr) {
    let c = Ctr::default();
    let mut sols = vec![];
    let basis: Vec<u64> = (0..l).map(|k| 1u64 << (k + shift)).collect();
    let size = 1u64 << l;
    let xr2 = gf.sqr(&c, xr);
    let d_sq: Vec<u64> = basis.iter().map(|&e| gf.sqr(&c, e)).collect();
    let lidx = hb.len() - 1;
    let mut x1 = 0u64;
    let mut x1sq = 0u64;
    for i1 in 0..size {
        if i1 > 0 {
            let k = i1.trailing_zeros() as usize;
            x1 ^= basis[k];
            x1sq = gf.add(&c, x1sq, d_sq[k]);
        }
        // row deltas: q = x1*x2, r = q^2 + b, linear in x2
        let d_q: Vec<u64> = basis.iter().map(|&e| gf.mul(&c, e, x1)).collect();
        let d_r: Vec<u64> = d_q.iter().map(|&t| gf.sqr(&c, t)).collect();
        // pass 1: quartic coefficients for the row
        let mut row: Vec<(u64, [u64; 5])> = Vec::with_capacity(size as usize);
        let (mut x2, mut x2sq, mut q, mut r) = (0u64, 0u64, 0u64, b);
        for i2 in 0..size {
            if i2 > 0 {
                let k = i2.trailing_zeros() as usize;
                x2 ^= basis[k];
                x2sq = gf.add(&c, x2sq, d_sq[k]);
                q = gf.add(&c, q, d_q[k]);
                r = gf.add(&c, r, d_r[k]);
            }
            let p = gf.add(&c, x1sq, x2sq);
            let a2 = gf.add(&c, gf.mul(&c, p, xr2), r);
            let a0 = if b == 1 {
                gf.add(&c, p, gf.mul(&c, r, xr2))
            } else {
                let t = gf.mul(&c, p, b);
                gf.add(&c, t, gf.mul(&c, r, xr2))
            };
            let p1 = gf.mul(&c, p, xr);
            let p0 = gf.mul(&c, q, xr2);
            let q1 = gf.mul(&c, r, xr);
            let q0 = if b == 1 { q } else { gf.mul(&c, q, b) };
            // P = q X^2 + p1 X + p0 ; Q = p0 X^2 + q1 X + q0
            let qp0 = gf.mul(&c, q, p0);
            let qq0 = gf.mul(&c, p0, q0);
            let h4 = gf.add(&c, gf.sqr(&c, a2), qp0);
            let h3 = gf.add(&c, gf.mul(&c, q, q1), gf.mul(&c, p1, p0));
            let t2 = gf.mul(&c, q, q0);
            let h2 = gf.add(
                &c,
                gf.add(&c, t2, gf.mul(&c, p1, q1)),
                gf.sqr(&c, p0),
            );
            let h1 = gf.add(&c, gf.mul(&c, p1, q0), gf.mul(&c, p0, q1));
            let h0 = gf.add(&c, gf.sqr(&c, a0), qq0);
            row.push((x2, [h0, h1, h2, h3, h4]));
        }
        // batch inversion of the non-zero leading coefficients
        let idx: Vec<usize> = (0..row.len()).filter(|&i| row[i].1[4] != 0).collect();
        let k = idx.len();
        let mut invs = vec![0u64; k];
        if k > 0 {
            let mut pref = vec![0u64; k];
            pref[0] = row[idx[0]].1[4];
            for i in 1..k {
                pref[i] = gf.mul(&c, pref[i - 1], row[idx[i]].1[4]);
            }
            let mut acc = gf.inv(&c, pref[k - 1]);
            for i in (1..k).rev() {
                invs[i] = gf.mul(&c, acc, pref[i - 1]);
                acc = gf.mul(&c, acc, row[idx[i]].1[4]);
            }
            invs[0] = acc;
        }
        let mut ii = 0;
        for (x2, h) in row.iter() {
            let roots: Vec<u64> = if h[4] == 0 {
                roots_in_v_generic(gf, &c, &h[..4], hb, l, shift)
            } else {
                let li = invs[ii];
                ii += 1;
                let f: [u64; 4] = [
                    gf.mul(&c, h[0], li),
                    gf.mul(&c, h[1], li),
                    gf.mul(&c, h[2], li),
                    gf.mul(&c, h[3], li),
                ];
                // t^4 = f0 + f1 t + f2 t^2 + f3 t^3 ; t^6 table
                let [f0, f1, f2, f3] = f;
                let kq = gf.add(&c, f2, gf.sqr(&c, f3));
                let r6 = [
                    gf.mul(&c, kq, f0),
                    gf.add(&c, gf.mul(&c, kq, f1), gf.mul(&c, f3, f0)),
                    {
                        let t = gf.add(&c, gf.mul(&c, kq, f2), gf.mul(&c, f3, f1));
                        gf.add(&c, t, f0)
                    },
                    {
                        let t = gf.add(&c, gf.mul(&c, kq, f3), gf.mul(&c, f3, f2));
                        gf.add(&c, t, f1)
                    },
                ];
                let mut u = [0u64, hb[lidx], 0, 0];
                for j in (0..lidx).rev() {
                    let s0 = gf.sqr(&c, u[0]);
                    let s1 = gf.sqr(&c, u[1]);
                    let s2 = gf.sqr(&c, u[2]);
                    let s3 = gf.sqr(&c, u[3]);
                    let mut nu = [s0, 0, s1, 0];
                    for i in 0..4 {
                        let a = gf.mul(&c, s2, f[i]);
                        let bq = gf.mul(&c, s3, r6[i]);
                        nu[i] = gf.add(&c, nu[i], gf.add(&c, a, bq));
                    }
                    nu[1] = gf.add(&c, nu[1], hb[j]);
                    u = nu;
                }
                let monic = [f0, f1, f2, f3, 1];
                let g = pgcd(gf, &c, &monic, &u);
                if deg(&g) <= 0 {
                    vec![]
                } else {
                    let mut out = vec![];
                    for v in 0..size {
                        let x = v << shift;
                        if poly_eval(gf, &c, &g, x) == 0 {
                            out.push(x);
                        }
                    }
                    out
                }
            };
            for x3 in roots {
                sols.push((x1, *x2, x3));
            }
        }
    }
    sols.sort();
    (sols, c)
}

// ───────────────────────── brute force references ─────────────────────────

fn s3(gf: &Gf, x1: u64, x2: u64, x3: u64, b: u64) -> u64 {
    let m = |a, b| gf.mul_raw(a, b);
    let s = m(x1, x2) ^ m(x1, x3) ^ m(x2, x3);
    m(s, s) ^ m(m(x1, x2), x3) ^ b
}

fn s4(gf: &Gf, x1: u64, x2: u64, x3: u64, xr: u64, b: u64) -> u64 {
    // Res_U(S3(x1,x2,U), S3(U,x3,xr)) evaluated numerically
    let m = |a, b| gf.mul_raw(a, b);
    let a = m(x1 ^ x2, x1 ^ x2);
    let bq = m(x1, x2);
    let cq = m(bq, bq) ^ b;
    let d = m(x3 ^ xr, x3 ^ xr);
    let e = m(x3, xr);
    let g = m(e, e) ^ b;
    let t1 = m(a, g) ^ m(cq, d);
    m(t1, t1) ^ m(m(a, e) ^ m(bq, d), m(bq, g) ^ m(cq, e))
}

fn brute_m2(gf: &Gf, xr: u64, b: u64, l: u32) -> Vec<(u64, u64)> {
    let mut v = vec![];
    for x1 in 0..(1u64 << l) {
        for x2 in 0..(1u64 << l) {
            if s3(gf, x1, x2, xr, b) == 0 {
                v.push((x1, x2));
            }
        }
    }
    v
}

fn brute_m3(gf: &Gf, xr: u64, b: u64, l: u32) -> Vec<(u64, u64, u64)> {
    let mut v = vec![];
    let s = 1u64 << l;
    for x1 in 0..s {
        for x2 in 0..s {
            for x3 in 0..s {
                if s4(gf, x1, x2, x3, xr, b) == 0 {
                    v.push((x1, x2, x3));
                }
            }
        }
    }
    v
}

// ───────────────────────── pricing ─────────────────────────

fn inv_cost(n: u32) -> (u64, u64) {
    // Itoh-Tsujii: (n-1) sqr, bitlen(n-1)-1 + popcount(n-1)-1 mul
    let e = (n - 1) as u64;
    let bitlen = 64 - e.leading_zeros() as u64;
    (bitlen - 1 + e.count_ones() as u64 - 1, e)
}

/// word-op price of a counter at price model p in {"P1","P4","P2n"}
fn price(c: &Ctr, n: u32, p: &str) -> f64 {
    let (im, is) = inv_cost(n);
    let mul = (c.mul.get() + c.inv.get() * im) as f64;
    let sqr = (c.sqr.get() + c.inv.get() * is) as f64;
    let add = c.add.get() as f64;
    match p {
        "P1" => mul + sqr + add,
        "P4" => 4.0 * mul + 3.0 * sqr + add,
        "P2n" => 2.0 * n as f64 * mul + n as f64 * sqr + add,
        _ => unreachable!(),
    }
}

// ───────────────────────── tiny JSON ─────────────────────────

#[derive(Debug, Clone)]
enum J {
    Null,
    B(bool),
    N(f64, String),
    S(String),
    A(Vec<J>),
    O(Vec<(String, J)>),
}
impl J {
    fn get(&self, k: &str) -> Option<&J> {
        if let J::O(v) = self {
            v.iter().find(|(a, _)| a == k).map(|(_, b)| b)
        } else {
            None
        }
    }
    fn arr(&self) -> &[J] {
        if let J::A(v) = self {
            v
        } else {
            &[]
        }
    }
    fn u64(&self) -> u64 {
        match self {
            J::N(_, s) => s.parse::<u64>().unwrap_or_else(|_| s.parse::<f64>().unwrap() as u64),
            J::S(s) => s.parse().unwrap(),
            _ => panic!("not a number: {:?}", self),
        }
    }
    fn u128(&self) -> u128 {
        match self {
            J::N(_, s) => s.parse::<u128>().unwrap(),
            J::S(s) => s.parse().unwrap(),
            _ => panic!("not a number"),
        }
    }
    fn str(&self) -> &str {
        if let J::S(s) = self {
            s
        } else {
            ""
        }
    }
}

struct P<'a> {
    s: &'a [u8],
    i: usize,
}
impl<'a> P<'a> {
    fn ws(&mut self) {
        while self.i < self.s.len() && (self.s[self.i] as char).is_whitespace() {
            self.i += 1;
        }
    }
    fn val(&mut self) -> J {
        self.ws();
        match self.s[self.i] {
            b'{' => {
                self.i += 1;
                let mut v = vec![];
                loop {
                    self.ws();
                    if self.s[self.i] == b'}' {
                        self.i += 1;
                        break;
                    }
                    let k = if let J::S(k) = self.val() { k } else { panic!() };
                    self.ws();
                    assert_eq!(self.s[self.i], b':');
                    self.i += 1;
                    let x = self.val();
                    v.push((k, x));
                    self.ws();
                    if self.s[self.i] == b',' {
                        self.i += 1;
                    }
                }
                J::O(v)
            }
            b'[' => {
                self.i += 1;
                let mut v = vec![];
                loop {
                    self.ws();
                    if self.s[self.i] == b']' {
                        self.i += 1;
                        break;
                    }
                    v.push(self.val());
                    self.ws();
                    if self.s[self.i] == b',' {
                        self.i += 1;
                    }
                }
                J::A(v)
            }
            b'"' => {
                self.i += 1;
                let mut out = String::new();
                while self.s[self.i] != b'"' {
                    if self.s[self.i] == b'\\' {
                        self.i += 1;
                        let ch = self.s[self.i];
                        match ch {
                            b'n' => out.push('\n'),
                            b't' => out.push('\t'),
                            b'u' => {
                                let h = std::str::from_utf8(&self.s[self.i + 1..self.i + 5]).unwrap();
                                let cp = u32::from_str_radix(h, 16).unwrap();
                                out.push(char::from_u32(cp).unwrap_or('?'));
                                self.i += 4;
                            }
                            c => out.push(c as char),
                        }
                        self.i += 1;
                    } else {
                        // copy a UTF-8 run
                        let st = self.i;
                        while self.s[self.i] != b'"' && self.s[self.i] != b'\\' {
                            self.i += 1;
                        }
                        out.push_str(std::str::from_utf8(&self.s[st..self.i]).unwrap());
                    }
                }
                self.i += 1;
                J::S(out)
            }
            b't' => {
                self.i += 4;
                J::B(true)
            }
            b'f' => {
                self.i += 5;
                J::B(false)
            }
            b'n' => {
                self.i += 4;
                J::Null
            }
            _ => {
                let st = self.i;
                while self.i < self.s.len() && b"+-0123456789.eE".contains(&self.s[self.i]) {
                    self.i += 1;
                }
                let t = std::str::from_utf8(&self.s[st..self.i]).unwrap().to_string();
                J::N(t.parse().unwrap_or(0.0), t)
            }
        }
    }
}

fn parse(s: &str) -> J {
    // `ic` may print log lines before the JSON; start at the first '{'.
    let st = s.find('{').expect("no JSON object");
    P { s: s[st..].as_bytes(), i: 0 }.val()
}

// ───────────────────────── rng ─────────────────────────

struct Sm(u64);
impl Sm {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
}

// ───────────────────────── commands ─────────────────────────

fn prep(gf: &Gf, l: u32, shift: u32) -> (Vec<u64>, Vec<u64>, Ctr) {
    let c = Ctr::default();
    let basis: Vec<u64> = (0..l).map(|k| 1u64 << (k + shift)).collect();
    let a = subspace_poly(gf, &c, &basis);
    // sanity: L_V vanishes on V, not on a vector outside
    for v in 0..(1u64 << l.min(10)) {
        assert_eq!(eval_lv(gf, &a, v << shift), 0);
    }
    let hb = horner_coeffs(gf, &c, &a);
    (a, hb, c)
}

fn selftest(args: &[String]) {
    let n: u32 = args[0].parse().unwrap();
    let l: u32 = args[1].parse().unwrap();
    let m: u32 = args[2].parse().unwrap();
    let t: usize = args[3].parse().unwrap();
    let seed: u64 = args[4].parse().unwrap();
    let gf = Gf::new(n);
    let (_a, hb, _) = prep(&gf, l, 0);
    let mut rng = Sm(seed);
    let mut sat = 0;
    for i in 0..t {
        let xr = rng.next() & gf.mask;
        if m == 2 {
            let (s, c) = g_m2(&gf, xr, 1, l, 0, &hb);
            let bf = brute_m2(&gf, xr, 1, l);
            assert_eq!(s, bf, "m2 mismatch target {i} xr={xr}");
            sat += (!s.is_empty()) as usize;
            println!("t{i} xr={xr} sols={} mul={} sqr={} add={} inv={}", s.len(), c.mul.get(), c.sqr.get(), c.add.get(), c.inv.get());
        } else {
            let (s, c) = g_m3(&gf, xr, 1, l, 0, &hb);
            let bf = brute_m3(&gf, xr, 1, l);
            assert_eq!(s, bf, "m3 mismatch target {i} xr={xr}");
            sat += (!s.is_empty()) as usize;
            println!("t{i} xr={xr} sols={} mul={} sqr={} add={} inv={}", s.len(), c.mul.get(), c.sqr.get(), c.add.get(), c.inv.get());
        }
    }
    println!("SELFTEST OK n={n} l={l} m={m}: {t} targets, {sat} satisfiable, all equal to brute force (modulus low=0x{:x})", gf.low);
}

fn mask_m2(s: &(u64, u64), l: u32) -> u128 {
    s.0 as u128 | ((s.1 as u128) << l)
}
fn mask_m3(s: &(u64, u64, u64), l: u32) -> u128 {
    s.0 as u128 | ((s.1 as u128) << l) | ((s.2 as u128) << (2 * l))
}

fn gcd_cmd(args: &[String]) {
    let ic = std::fs::read_to_string(&args[0]).unwrap();
    let out = &args[1];
    let j = parse(&ic);
    let cells = j.get("cells").or(j.get("measured")).map(|x| x.arr().to_vec()).unwrap_or_default();
    let mut lines = String::new();
    for cell in &cells {
        let family = cell.get("family").unwrap().str().to_string();
        let n = cell.get("n").unwrap().u64() as u32;
        let l = cell.get("n_prime").unwrap().u64() as u32;
        let m = cell.get("summands").unwrap().u64() as u32;
        let reference = cell.get("reference_engine").map(|x| x.str().to_string()).unwrap_or_default();
        let gf = Gf::new(n);
        let (_a, hb, pre) = prep(&gf, l, 0);
        // negative control: subspace span{z..z^l}
        let (_aw, hbw, _) = prep(&gf, l, 1);
        let mut neg_differs = 0usize;
        let mut sat_targets = 0usize;
        for (ti, t) in cell.get("per_target").unwrap().arr().iter().enumerate() {
            let xr = t.get("x_r").unwrap().u64();
            let runs = t.get("runs").unwrap();
            // engines
            let mut eng: BTreeMap<String, (String, Vec<u128>, Vec<u64>)> = BTreeMap::new();
            let mut bit: BTreeMap<String, Vec<[u64; 6]>> = BTreeMap::new();
            if let J::O(kv) = runs {
                for (name, rs) in kv {
                    let rs = rs.arr();
                    let verdict = rs[0].get("verdict").unwrap().str().to_string();
                    let sols: Vec<u128> = rs[0].get("solutions").unwrap().arr().iter().map(|x| x.u128()).collect();
                    let ops: Vec<u64> = rs.iter().map(|r| r.get("ops").unwrap().u64()).collect();
                    // determinism across repetitions
                    for r in rs.iter() {
                        let s2: Vec<u128> = r.get("solutions").unwrap().arr().iter().map(|x| x.u128()).collect();
                        if r.get("verdict").unwrap().str() == verdict && s2 != sols {
                            eprintln!("WARN: {name} solutions differ across repeats");
                        }
                    }
                    let bv: Vec<[u64; 6]> = rs.iter().filter_map(|r| { let e = r.get("extra")?; let g = |k: &str| e.get(k).map(|x| x.u64()).unwrap_or(0); if g("bitop_complete") == 1 { Some([g("bitop_word_ops"), g("bitop_hash_probes"), g("bitop_field_adds"), g("bitop_field_muls"), g("bitop_field_sqrs"), g("bitop_field_invs")]) } else { None } }).collect();
                    bit.insert(name.clone(), bv);
                    eng.insert(name.clone(), (verdict, sols, ops));
                }
            }
            let t0 = std::time::Instant::now();
            let (gsols, c, cneg, brute_ok): (Vec<u128>, Ctr, Vec<u128>, Option<bool>) = if m == 2 {
                let (s, c) = g_m2(&gf, xr, 1, l, 0, &hb);
                let (s_aa, c_aa) = g_m2(&gf, xr, 1, l, 0, &hb);
                assert!(s == s_aa && c.mul.get() == c_aa.mul.get() && c.sqr.get() == c_aa.sqr.get() && c.add.get() == c_aa.add.get() && c.inv.get() == c_aa.inv.get(), "A/A failed");
                let (sn, _) = g_m2(&gf, xr, 1, l, 1, &hbw);
                let bo = if 2 * l <= 21 { Some(brute_m2(&gf, xr, 1, l) == s) } else { None };
                let mut v: Vec<u128> = s.iter().map(|x| mask_m2(x, l)).collect();
                v.sort();
                let mut vn: Vec<u128> = sn.iter().map(|x| (x.0 as u128) | ((x.1 as u128) << 64)).collect();
                vn.sort();
                (v, c, vn, bo)
            } else {
                let (s, c) = g_m3(&gf, xr, 1, l, 0, &hb);
                let (s_aa, c_aa) = g_m3(&gf, xr, 1, l, 0, &hb);
                assert!(s == s_aa && c.mul.get() == c_aa.mul.get() && c.sqr.get() == c_aa.sqr.get() && c.add.get() == c_aa.add.get() && c.inv.get() == c_aa.inv.get(), "A/A failed");
                let (sn, _) = g_m3(&gf, xr, 1, l, 1, &hbw);
                let bo = if 3 * l <= 21 { Some(brute_m3(&gf, xr, 1, l) == s) } else { None };
                let mut v: Vec<u128> = s.iter().map(|x| mask_m3(x, l)).collect();
                v.sort();
                let mut vn: Vec<u128> = sn.iter().map(|x| (x.0 as u128) ^ ((x.1 as u128) << 42) ^ ((x.2 as u128) << 84)).collect();
                vn.sort();
                (v, c, vn, bo)
            };
            let wall_ms = t0.elapsed().as_secs_f64() * 1e3 / 2.0;
            if !gsols.is_empty() {
                sat_targets += 1;
            }
            // negative control: the wrong-subspace oracle's raw answer differs from the
            // reference set (decoded to raw pairs for comparison)
            let refset: Option<&Vec<u128>> = eng.get(&reference).map(|e| &e.1);
            let decode_raw = |mask: u128| -> u128 {
                let lm = (1u128 << l) - 1;
                if m == 2 {
                    (mask & lm) | (((mask >> l) & lm) << 64)
                } else {
                    (mask & lm) ^ (((mask >> l) & lm) << 42) ^ (((mask >> (2 * l)) & lm) << 84)
                }
            };
            if let Some(rs) = refset {
                let mut raw: Vec<u128> = rs.iter().map(|&x| decode_raw(x)).collect();
                raw.sort();
                if raw != cneg {
                    neg_differs += 1;
                }
            }
            let mut agree = BTreeMap::new();
            for (name, (verdict, sols, _)) in &eng {
                let decided = verdict == "solved" || verdict == "unsatisfiable";
                agree.insert(name.clone(), if decided { Some(*sols == gsols) } else { None });
            }
            let mut line = String::new();
            write!(
                line,
                "{{\"family\":\"{family}\",\"n\":{n},\"l\":{l},\"m\":{m},\"target\":{ti},\"x_r\":{xr},\"modulus_low\":{},\"reference\":\"{reference}\",\"g\":{{\"mul\":{},\"sqr\":{},\"add\":{},\"inv\":{},\"solutions\":{},\"wall_ms\":{:.3},\"P1\":{:.1},\"P4\":{:.1},\"P2n\":{:.1}}},\"precomp\":{{\"mul\":{},\"sqr\":{},\"add\":{}}},\"brute_agrees\":{},\"engines\":{{",
                gf.low,
                c.mul.get(), c.sqr.get(), c.add.get(), c.inv.get(), gsols.len(), wall_ms,
                price(&c, n, "P1"), price(&c, n, "P4"), price(&c, n, "P2n"),
                pre.mul.get(), pre.sqr.get(), pre.add.get(),
                match brute_ok { Some(b) => b.to_string(), None => "null".into() }
            ).unwrap();
            let mut first = true;
            for (name, (verdict, sols, ops)) in &eng {
                if !first {
                    line.push(',');
                }
                first = false;
                let ag = match agree[name] { Some(b) => b.to_string(), None => "null".into() };
                write!(
                    line,
                    "\"{name}\":{{\"verdict\":\"{verdict}\",\"solutions\":{},\"ops\":[{}],\"bitop\":[{}],\"agrees_with_g\":{ag}}}",
                    sols.len(),
                    ops.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                    bit[name].iter().map(|b| format!("[{},{},{},{},{},{}]", b[0], b[1], b[2], b[3], b[4], b[5])).collect::<Vec<_>>().join(",")
                )
                .unwrap();
            }
            line.push_str("}}\n");
            lines.push_str(&line);
            eprintln!("{family} n={n} l={l} m={m} t{ti}: G sols={} mul={} | {}", gsols.len(), c.mul.get(),
                eng.iter().map(|(k, v)| format!("{k}:{}:{}:{}", v.0, v.1.len(), v.2.first().copied().unwrap_or(0))).collect::<Vec<_>>().join(" "));
        }
        lines.push_str(&format!(
            "{{\"cell_summary\":true,\"family\":\"{family}\",\"n\":{n},\"l\":{l},\"m\":{m},\"satisfiable_targets\":{sat_targets},\"negative_control_targets_differing\":{neg_differs}}}\n"
        ));
    }
    std::fs::write(out, lines).unwrap();
}

// ───────────────────────── summarize ─────────────────────────

fn summarize(args: &[String]) {
    let mut files = vec![];
    let mut outdir = String::from(".");
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--out" {
            outdir = args[i + 1].clone();
            i += 2;
        } else {
            files.push(args[i].clone());
            i += 1;
        }
    }
    // (n,l,m) -> accumulators
    #[derive(Default)]
    struct Acc {
        t: usize,
        g: [Vec<f64>; 3],
        i_ops: Vec<f64>,
        ic: [Vec<f64>; 3],
        fc: Vec<f64>,
        f_ops: Vec<f64>,
        i_budget: usize,
        gate_fail: usize,
        brute_fail: usize,
        sat: usize,
        neg: usize,
        sols: usize,
        gmul: Vec<f64>,
    }
    let mut cells: BTreeMap<(u32, u32, u32), Acc> = BTreeMap::new();
    let mut ops_nondet = 0usize;
    for f in &files {
        for line in std::fs::read_to_string(f).unwrap().lines() {
            if line.trim().is_empty() {
                continue;
            }
            let j = parse(line);
            let key = (j.get("n").unwrap().u64() as u32, j.get("l").unwrap().u64() as u32, j.get("m").unwrap().u64() as u32);
            let a = cells.entry(key).or_default();
            if j.get("cell_summary").is_some() {
                a.sat += j.get("satisfiable_targets").unwrap().u64() as usize;
                a.neg += j.get("negative_control_targets_differing").unwrap().u64() as usize;
                continue;
            }
            a.t += 1;
            let g = j.get("g").unwrap();
            for (k, p) in ["P1", "P4", "P2n"].iter().enumerate() {
                if let Some(J::N(v, _)) = g.get(p) {
                    a.g[k].push(*v);
                }
            }
            a.gmul.push(g.get("mul").unwrap().u64() as f64);
            a.sols += g.get("solutions").unwrap().u64() as usize;
            if let Some(J::B(false)) = j.get("brute_agrees") {
                a.brute_fail += 1;
            }
            if let Some(J::O(engs)) = j.get("engines") {
                for (name, e) in engs {
                    let ops: Vec<u64> = e.get("ops").unwrap().arr().iter().map(|x| x.u64()).collect();
                    let verdict = e.get("verdict").unwrap().str();
                    if ops.windows(2).any(|w| w[0] != w[1]) && verdict != "budget" {
                        ops_nondet += 1;
                    }
                    if let Some(J::B(false)) = e.get("agrees_with_g") {
                        a.gate_fail += 1;
                    }
                    let decided = verdict == "solved" || verdict == "unsatisfiable";
                    let bits: Vec<Vec<u64>> = e.get("bitop").map(|b| b.arr().iter().map(|r| r.arr().iter().map(|x| x.u64()).collect()).collect()).unwrap_or_default();
                    let nn = key.0;
                    let pc = |b: &Vec<u64>, p: &str| -> f64 { let (im, is) = inv_cost(nn); let mul = (b[3] + b[5] * im) as f64; let sqr = (b[4] + b[5] * is) as f64; let base = (b[0] + b[1] + b[2]) as f64; match p { "P1" => base + mul + sqr, "P4" => base + 4.0 * mul + 3.0 * sqr, _ => base + 2.0 * nn as f64 * mul + nn as f64 * sqr } };
                    if name == "inherited-f4" && decided && !bits.is_empty() {
                        for (k, p) in ["P1", "P4", "P2n"].iter().enumerate() {
                            a.ic[k].push(bits.iter().map(|b| pc(b, p)).fold(f64::INFINITY, f64::min));
                        }
                    }
                    if name == "fes-f2" && decided && !bits.is_empty() {
                        a.fc.push(bits.iter().map(|b| pc(b, "P1")).fold(f64::INFINITY, f64::min));
                    }
                    if name == "inherited-f4" {
                        if decided {
                            a.i_ops.push(ops[0] as f64);
                        } else if verdict == "budget" {
                            a.i_budget += 1;
                        }
                    }
                    if name == "fes-f2" && decided {
                        a.f_ops.push(ops[0] as f64);
                    }
                }
            }
        }
    }
    let gm = |v: &[f64]| -> f64 {
        if v.is_empty() {
            f64::NAN
        } else {
            (v.iter().map(|x| x.ln()).sum::<f64>() / v.len() as f64).exp()
        }
    };
    let mn = |v: &[f64]| v.iter().cloned().fold(f64::INFINITY, f64::min);
    let mx = |v: &[f64]| v.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let mut md = String::new();
    let mut js = String::from("{\"cells\":[");
    md.push_str("| n | l | m | targets | sat | G mul (geo-mean) | G P1 | G P4 | G P2n | I word XORs (geo-mean; range) | I budget hits | F word XORs | I/G P1 | I/G P4 | I/G P2n | F/G P1 | F/G P4 | F/G P2n | S cited (P4) | G/S P4 | gate fails | brute fails | neg-ctrl differs |\n");
    md.push_str("|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|\n");
    let mut md2 = String::from("\n### Complete counts (bitop/v1: word ops + hash probes + field ops priced at the band; min over repeats)\n\n| n | l | m | I complete (P4) | F complete | I/G P1 | I/G P4 | I/G P2n | F/G P1 | F/G P4 | F/G P2n |\n|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|\n");
    let mut js2 = String::new();
    let mut first = true;
    let mut gate_total = 0;
    let mut neg_bad = 0;
    let mut s_beaten = false;
    let mut s_never_beaten = true;
    for (&(n, l, m), a) in &cells {
        let g = [gm(&a.g[0]), gm(&a.g[1]), gm(&a.g[2])];
        let iops = gm(&a.i_ops);
        let fops = gm(&a.f_ops);
        // cited S (pdp_root_finder): subtract2 (m=2): 5|F|-3 mul; rf-mitm (m=3): 5|F|-3 mul + 2|F| sqr + 1 inv
        let fsz = (1u64 << l) as f64;
        let (s_mul, s_sqr) = if m == 2 { (5.0 * fsz - 3.0, 0.0) } else { (5.0 * fsz - 3.0, 2.0 * fsz) };
        let (im, is) = inv_cost(n);
        let s_p = |p: &str| -> f64 {
            let mul = s_mul + if m == 3 { im as f64 } else { 0.0 };
            let sqr = s_sqr + if m == 3 { is as f64 } else { 0.0 };
            match p {
                "P1" => mul + sqr,
                "P4" => 4.0 * mul + 3.0 * sqr,
                _ => 2.0 * n as f64 * mul + n as f64 * sqr,
            }
        };
        let gs = [g[0] / s_p("P1"), g[1] / s_p("P4"), g[2] / s_p("P2n")];
        let icg = [gm(&a.ic[0]), gm(&a.ic[1]), gm(&a.ic[2])];
        let fcg = gm(&a.fc);
        let rr = |x: f64, y: f64| if x.is_nan() || y.is_nan() { "—".to_string() } else { format!("{:.3}", x / y) };
        writeln!(md2, "| {n} | {l} | {m} | {} | {} | {} | {} | {} | {} | {} | {} |", if icg[1].is_nan() { "—".into() } else { format!("{:.3e}", icg[1]) }, if fcg.is_nan() { "—".into() } else { format!("{:.3e}", fcg) }, rr(icg[0], g[0]), rr(icg[1], g[1]), rr(icg[2], g[2]), rr(fcg, g[0]), rr(fcg, g[1]), rr(fcg, g[2])).unwrap();
        if !js2.is_empty() { js2.push(','); }
        let ff = |x: f64| if x.is_nan() { "null".to_string() } else { format!("{x:.6e}") };
        write!(js2, "{{\"n\":{n},\"l\":{l},\"m\":{m},\"i_complete_P1\":{},\"i_complete_P4\":{},\"i_complete_P2n\":{},\"f_complete\":{}}}", ff(icg[0]), ff(icg[1]), ff(icg[2]), ff(fcg)).unwrap();
        if gs.iter().all(|&x| x < 1.0) {
            s_beaten = true;
        }
        if gs.iter().any(|&x| x < 1.0) {
            s_never_beaten = false;
        }
        gate_total += a.gate_fail + a.brute_fail;
        if a.sat > 0 && a.neg == 0 {
            neg_bad += 1;
        }
        let r = |x: f64, y: f64| if x.is_nan() || y.is_nan() { "—".to_string() } else { format!("{:.3}", x / y) };
        writeln!(
            md,
            "| {n} | {l} | {m} | {} | {} | {:.0} | {:.3e} | {:.3e} | {:.3e} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {:.3e} | {:.1} | {} | {} | {} |",
            a.t, a.sat, gm(&a.gmul), g[0], g[1], g[2],
            if a.i_ops.is_empty() { "—".into() } else { format!("{:.3e} [{:.3e}, {:.3e}]", iops, mn(&a.i_ops), mx(&a.i_ops)) },
            a.i_budget,
            if fops.is_nan() { "—".into() } else { format!("{:.3e}", fops) },
            r(iops, g[0]), r(iops, g[1]), r(iops, g[2]), r(fops, g[0]), r(fops, g[1]), r(fops, g[2]),
            s_p("P4"), gs[1], a.gate_fail, a.brute_fail, a.neg
        ).unwrap();
        if !first {
            js.push(',');
        }
        first = false;
        let f = |x: f64| if x.is_nan() { "null".to_string() } else { format!("{x:.6e}") };
        write!(js, "{{\"n\":{n},\"l\":{l},\"m\":{m},\"targets\":{},\"satisfiable\":{},\"g_P1\":{},\"g_P4\":{},\"g_P2n\":{},\"g_mul_geomean\":{},\"i_ops_geomean\":{},\"i_ops_min\":{},\"i_ops_max\":{},\"i_budget_hits\":{},\"f_ops_geomean\":{},\"s_cited_P1\":{},\"s_cited_P4\":{},\"s_cited_P2n\":{},\"gate_fails\":{},\"brute_fails\":{},\"negative_control_differs\":{}}}",
            a.t, a.sat, f(g[0]), f(g[1]), f(g[2]), f(gm(&a.gmul)), f(iops), f(if a.i_ops.is_empty() {f64::NAN} else {mn(&a.i_ops)}), f(if a.i_ops.is_empty() {f64::NAN} else {mx(&a.i_ops)}), a.i_budget, f(fops), f(s_p("P1")), f(s_p("P4")), f(s_p("P2n")), a.gate_fail, a.brute_fail, a.neg).unwrap();
    }
    let verdict = if gate_total > 0 || neg_bad > 0 || ops_nondet > 0 {
        "INCONCLUSIVE"
    } else if s_beaten {
        "IMPROVES"
    } else if s_never_beaten {
        "NEGATIVE"
    } else {
        "NO-CHANGE"
    };
    write!(js, "],\"complete_counts\":[{js2}").unwrap();
    write!(js, "],\"gate_failures\":{gate_total},\"cells_negative_control_failed\":{neg_bad},\"engine_ops_nondeterministic\":{ops_nondet},\"verdict\":\"{verdict}\"}}").unwrap();
    md.push_str(&md2);
    md.push_str(&format!("\nVERDICT: {verdict} (gate failures {gate_total}; cells whose negative control could not fail {neg_bad}; nondeterministic engine ops {ops_nondet})\n"));
    std::fs::create_dir_all(&outdir).unwrap();
    std::fs::write(format!("{outdir}/summary.md"), &md).unwrap();
    std::fs::write(format!("{outdir}/summary.json"), &js).unwrap();
    // SVG: log2 cost vs l for m=2 (n regime + sweep) and m=3
    let mut svg = String::new();
    let (w, h, ml, mb, mt, mr) = (860.0, 520.0, 70.0, 60.0, 40.0, 220.0);
    let pts: Vec<(u32, u32, u32, f64, f64, f64, f64, f64)> = cells
        .iter()
        .map(|(&(n, l, m), a)| (n, l, m, gm(&a.g[0]).log2(), gm(&a.g[1]).log2(), gm(&a.g[2]).log2(), gm(&a.i_ops).log2(), gm(&a.f_ops).log2()))
        .collect();
    let ymax = pts.iter().flat_map(|p| [p.3, p.4, p.5, p.6, p.7]).filter(|x| x.is_finite()).fold(0.0, f64::max).ceil() + 1.0;
    let ymin = pts.iter().flat_map(|p| [p.3, p.4, p.5, p.6, p.7]).filter(|x| x.is_finite()).fold(99.0, f64::min).floor() - 1.0;
    let xs: Vec<f64> = pts.iter().map(|p| (p.2 * p.1) as f64).collect();
    let (xmin, xmax) = (xs.iter().cloned().fold(99.0, f64::min) - 1.0, xs.iter().cloned().fold(0.0, f64::max) + 1.0);
    let sx = |x: f64| ml + (x - xmin) / (xmax - xmin) * (w - ml - mr);
    let sy = |y: f64| h - mb - (y - ymin) / (ymax - ymin) * (h - mb - mt);
    write!(svg, "<svg xmlns='http://www.w3.org/2000/svg' width='{w}' height='{h}' font-family='sans-serif' font-size='12'><rect width='100%' height='100%' fill='white'/>").unwrap();
    write!(svg, "<text x='{}' y='22' font-size='14'>L4 (a): per-target decomposition cost, log2 word ops, vs unknowns N = m*l (K family)</text>", ml).unwrap();
    let mut y = ymin.ceil();
    while y <= ymax {
        write!(svg, "<line x1='{ml}' x2='{}' y1='{:.1}' y2='{:.1}' stroke='#eee'/><text x='{}' y='{:.1}' text-anchor='end'>{y}</text>", w - mr, sy(y), sy(y), ml - 6.0, sy(y) + 4.0).unwrap();
        y += 4.0;
    }
    let mut x = xmin.ceil();
    while x <= xmax {
        write!(svg, "<text x='{:.1}' y='{}' text-anchor='middle'>{x}</text>", sx(x), h - mb + 18.0).unwrap();
        x += 4.0;
    }
    write!(svg, "<text x='{}' y='{}' text-anchor='middle'>N = m*l Boolean unknowns of the descended system</text>", (ml + w - mr) / 2.0, h - 15.0).unwrap();
    let series: [(&str, &str, usize, u32); 8] = [
        ("G m=2 at P4 (gcd with L_V)", "#1f77b4", 4, 2),
        ("G m=2 band P1..P2n", "#aec7e8", 9, 2),
        ("I m=2 inherited-f4 (partial count)", "#d62728", 6, 2),
        ("F m=2 fes-f2", "#2ca02c", 7, 2),
        ("G m=3 at P4", "#9467bd", 4, 3),
        ("G m=3 band P1..P2n", "#c5b0d5", 9, 3),
        ("I m=3 inherited-f4 (partial count)", "#ff7f0e", 6, 3),
        ("", "", 0, 0),
    ];
    let mut ly = mt + 10.0;
    for (name, col, idx, mm) in series.iter() {
        if name.is_empty() {
            continue;
        }
        let mut sel: Vec<&(u32, u32, u32, f64, f64, f64, f64, f64)> = pts.iter().filter(|p| p.2 == *mm).collect();
        sel.sort_by_key(|p| (p.1, p.0));
        for p in &sel {
            let xx = sx((p.2 * p.1) as f64);
            if *idx == 9 {
                if p.3.is_finite() && p.5.is_finite() {
                    write!(svg, "<line x1='{xx:.1}' x2='{xx:.1}' y1='{:.1}' y2='{:.1}' stroke='{col}' stroke-width='6' opacity='0.7'/>", sy(p.3), sy(p.5)).unwrap();
                }
                continue;
            }
            let v = match idx { 4 => p.4, 6 => p.6, _ => p.7 };
            if v.is_finite() {
                let shape = if p.0 == 29 && *mm == 2 { "rect" } else { "circle" };
                if shape == "rect" {
                    write!(svg, "<rect x='{:.1}' y='{:.1}' width='8' height='8' fill='none' stroke='{col}' stroke-width='2'/>", xx - 4.0, sy(v) - 4.0).unwrap();
                } else {
                    write!(svg, "<circle cx='{xx:.1}' cy='{:.1}' r='4' fill='{col}'/>", sy(v)).unwrap();
                }
            }
        }
        write!(svg, "<rect x='{}' y='{:.1}' width='12' height='12' fill='{col}'/><text x='{}' y='{:.1}'>{name}</text>", w - mr + 10.0, ly, w - mr + 28.0, ly + 10.0).unwrap();
        ly += 20.0;
    }
    write!(svg, "<text x='{}' y='{:.1}'>circles: regime cells n'=ceil(n/m)</text><text x='{}' y='{:.1}'>squares: l-sweep at n = 29</text></svg>", w - mr + 10.0, ly + 14.0, w - mr + 10.0, ly + 30.0).unwrap();
    std::fs::write(format!("{outdir}/cost_vs_unknowns.svg"), svg).unwrap();
    print!("{md}");
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    match a.get(1).map(|s| s.as_str()) {
        Some("selftest") => selftest(&a[2..]),
        Some("gcd") => gcd_cmd(&a[2..]),
        Some("summarize") => summarize(&a[2..]),
        _ => eprintln!("usage: l4bench selftest N L M TARGETS SEED | gcd IC_JSON OUT | summarize ROWS... --out DIR"),
    }
}
