//! Stack-array polynomial kernel, precomputed `F_p` inverse table,
//! partition-direct `m = 9` line factorization, single-flush bucket sieve,
//! and Stickelberger-screened stack quartic extraction for extension-field
//! graph index calculus.
//!
//! Every `F_p` multiplication and addition runs through `gaudry_cubic::{mm, am, sm}`
//! (which fire `jv_bitops::hook_mul` and `jv_bitops::hook_add`), the one-time
//! inverse table build runs through `residual_walk::inv_mod` (`hook_inv`), and
//! every table lookup and word scan charges `jv_bitops::words`.

use rand::rngs::StdRng;

use crate::cryptanalysis::gaudry_cubic::{am, mm, sm};
use crate::cryptanalysis::jv_bitops as work;
use crate::cryptanalysis::jv_cover::{BaseEl, Ctx, Fld, Fq, E2};
use crate::cryptanalysis::jv_sieve::two_large_prime::{
    principal_relation as remainder, quartic_discriminant as discriminant, TwoLargePrimeSieve,
};
use crate::cryptanalysis::jv_sieve::{
    self as sieve, fdeg, FpPoly, FpRing, HitBuckets, Line, SharedB, SieveCost, SieveSetup,
};
use crate::cryptanalysis::residual_walk::inv_mod;

/// Build `inv_fp[a] = a^{-1} mod p` for all `a in 1..p` (`inv_fp[0] = 0`) using
/// a single Montgomery batch inversion (`1` inversion + `3(p - 2)` multiplications).
pub fn build_inv_table(p: u64) -> Vec<u64> {
    let pu = p as usize;
    let mut out = vec![0u64; pu];
    if pu <= 1 {
        return out;
    }
    if pu == 2 {
        out[1] = 1;
        return out;
    }
    let mut prefix = vec![0u64; pu];
    let mut acc = 1u64;
    prefix[1] = 1;
    for x in 1..(pu - 1) {
        acc = mm(acc, x as u64, p);
        prefix[x + 1] = acc;
    }
    acc = mm(acc, (pu - 1) as u64, p);
    let mut inv = inv_mod(acc, p);
    for x in (1..pu).rev() {
        out[x] = mm(inv, prefix[x], p);
        if x > 1 {
            inv = mm(inv, x as u64, p);
        }
    }
    work::words((2 * pu) as u64);
    out
}

#[inline]
pub fn table_inv(inv_fp: &[u64], a: u64) -> u64 {
    debug_assert!(a != 0 && (a as usize) < inv_fp.len());
    work::words(1);
    inv_fp[a as usize]
}

/// Fixed-capacity stack polynomial over `F_p` (`deg <= 17`, zero heap allocations).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StackPoly {
    pub len: usize,
    pub c: [u64; 18],
}

impl StackPoly {
    pub const ZERO: Self = Self {
        len: 0,
        c: [0u64; 18],
    };

    #[inline]
    pub fn from_slice(s: &[u64]) -> Self {
        let mut out = Self::ZERO;
        let n = s.len().min(18);
        out.c[..n].copy_from_slice(&s[..n]);
        out.len = n;
        out.trim();
        out
    }

    #[inline]
    pub fn to_vec(self) -> FpPoly {
        self.c[..self.len].to_vec()
    }

    #[inline]
    pub fn as_slice(&self) -> &[u64] {
        &self.c[..self.len]
    }

    #[inline]
    pub fn trim(&mut self) {
        while self.len > 0 && self.c[self.len - 1] == 0 {
            self.len -= 1;
        }
    }

    #[inline]
    pub fn deg(&self) -> isize {
        self.len as isize - 1
    }

    #[inline]
    pub fn is_one(&self) -> bool {
        self.len == 1 && self.c[0] == 1
    }

    #[inline]
    pub fn is_x(&self) -> bool {
        self.len == 2 && self.c[0] == 0 && self.c[1] == 1
    }

    #[inline]
    pub fn monic(&self, p: u64, inv_fp: &[u64]) -> Self {
        if self.len == 0 {
            return Self::ZERO;
        }
        let lc = self.c[self.len - 1];
        if lc == 1 {
            return *self;
        }
        let inv_lc = table_inv(inv_fp, lc);
        let mut out = *self;
        for i in 0..(self.len - 1) {
            out.c[i] = mm(out.c[i], inv_lc, p);
        }
        out.c[self.len - 1] = 1;
        out
    }

    #[inline]
    pub fn scale(&self, k: u64, p: u64) -> Self {
        if self.len == 0 || k == 0 {
            return Self::ZERO;
        }
        if k == 1 {
            return *self;
        }
        let mut out = Self::ZERO;
        out.len = self.len;
        for i in 0..self.len {
            out.c[i] = mm(self.c[i], k, p);
        }
        out.trim();
        out
    }

    #[inline]
    pub fn add(&self, rhs: &Self, p: u64) -> Self {
        let mut out = Self::ZERO;
        let n = self.len.max(rhs.len);
        out.len = n;
        for i in 0..n {
            let a = if i < self.len { self.c[i] } else { 0 };
            let b = if i < rhs.len { rhs.c[i] } else { 0 };
            out.c[i] = am(a, b, p);
        }
        out.trim();
        out
    }

    #[inline]
    pub fn sub(&self, rhs: &Self, p: u64) -> Self {
        let mut out = Self::ZERO;
        let n = self.len.max(rhs.len);
        out.len = n;
        for i in 0..n {
            let a = if i < self.len { self.c[i] } else { 0 };
            let b = if i < rhs.len { rhs.c[i] } else { 0 };
            out.c[i] = sm(a, b, p);
        }
        out.trim();
        out
    }

    #[inline]
    pub fn sub_x(&self, p: u64) -> Self {
        let mut out = *self;
        if out.len < 2 {
            out.len = 2;
        }
        out.c[1] = sm(out.c[1], 1, p);
        out.trim();
        out
    }

    #[inline]
    pub fn mul(&self, rhs: &Self, p: u64) -> Self {
        if self.len == 0 || rhs.len == 0 {
            return Self::ZERO;
        }
        let mut out = Self::ZERO;
        out.len = self.len + rhs.len - 1;
        debug_assert!(out.len <= 18);
        for i in 0..self.len {
            let ai = self.c[i];
            if ai == 0 {
                continue;
            }
            for j in 0..rhs.len {
                let prod = mm(ai, rhs.c[j], p);
                out.c[i + j] = am(out.c[i + j], prod, p);
            }
        }
        out.trim();
        out
    }

    #[inline]
    pub fn sqr(&self, p: u64) -> Self {
        if self.len == 0 {
            return Self::ZERO;
        }
        let mut out = Self::ZERO;
        out.len = 2 * self.len - 1;
        debug_assert!(out.len <= 18);
        for i in 0..self.len {
            let ai = self.c[i];
            if ai == 0 {
                continue;
            }
            let diag = mm(ai, ai, p);
            out.c[2 * i] = am(out.c[2 * i], diag, p);
            let two_ai = am(ai, ai, p);
            for j in (i + 1)..self.len {
                let cross = mm(two_ai, self.c[j], p);
                out.c[i + j] = am(out.c[i + j], cross, p);
            }
        }
        out.trim();
        out
    }

    #[inline]
    pub fn mul_linear_root(&self, root: u64, p: u64) -> Self {
        // Multiply by `(x - root)`.
        if self.len == 0 {
            return Self::ZERO;
        }
        let neg_r = if root == 0 { 0 } else { sm(p, root, p) };
        let mut out = Self::ZERO;
        out.len = self.len + 1;
        out.c[0] = mm(self.c[0], neg_r, p);
        for i in 1..self.len {
            out.c[i] = am(self.c[i - 1], mm(self.c[i], neg_r, p), p);
        }
        out.c[self.len] = self.c[self.len - 1];
        out.trim();
        out
    }

    #[inline]
    pub fn synthetic_div(&mut self, root: u64, p: u64) -> bool {
        if self.len <= 1 {
            return false;
        }
        let d = self.len - 1;
        let mut carry = self.c[d];
        self.c[d] = 0;
        for k in (0..d).rev() {
            let cur = self.c[k];
            self.c[k] = carry;
            carry = am(cur, mm(carry, root, p), p);
        }
        if carry != 0 {
            return false;
        }
        self.len = d;
        self.trim();
        true
    }

    #[inline]
    pub fn eval(&self, x: u64, p: u64) -> u64 {
        if self.len == 0 {
            return 0;
        }
        let mut acc = self.c[self.len - 1];
        for i in (0..(self.len - 1)).rev() {
            acc = am(mm(acc, x, p), self.c[i], p);
        }
        acc
    }

    #[inline]
    pub fn rem_monic(&self, m: &Self, p: u64) -> Self {
        debug_assert!(m.len >= 1 && m.c[m.len - 1] == 1);
        if self.len < m.len {
            return *self;
        }
        if m.len == 1 {
            return Self::ZERO;
        }
        let mut r = *self;
        let dm = m.len - 1;
        while r.len >= m.len {
            let shift = r.len - m.len;
            let coef = r.c[r.len - 1];
            r.c[r.len - 1] = 0;
            if coef != 0 {
                for i in 0..dm {
                    let mc = m.c[i];
                    if mc != 0 {
                        r.c[shift + i] = sm(r.c[shift + i], mm(coef, mc, p), p);
                    }
                }
            }
            r.len -= 1;
            r.trim();
        }
        r
    }

    #[inline]
    pub fn divrem_gen(&self, b: &Self, p: u64, inv_fp: &[u64]) -> (Self, Self) {
        debug_assert!(b.len >= 1);
        if self.len < b.len {
            return (Self::ZERO, *self);
        }
        let b_lc = b.c[b.len - 1];
        let inv_lc = if b_lc == 1 {
            1
        } else {
            table_inv(inv_fp, b_lc)
        };
        let mut q = Self::ZERO;
        q.len = self.len - b.len + 1;
        let mut r = *self;
        let db = b.len - 1;
        while r.len >= b.len {
            let shift = r.len - b.len;
            let top = r.c[r.len - 1];
            let coef = if inv_lc == 1 { top } else { mm(top, inv_lc, p) };
            q.c[shift] = coef;
            r.c[r.len - 1] = 0;
            if coef != 0 {
                for i in 0..db {
                    let bc = b.c[i];
                    if bc != 0 {
                        r.c[shift + i] = sm(r.c[shift + i], mm(coef, bc, p), p);
                    }
                }
            }
            r.len -= 1;
            r.trim();
        }
        q.trim();
        (q, r)
    }

    #[inline]
    pub fn gcd_monic(&self, rhs: &Self, p: u64, inv_fp: &[u64]) -> Self {
        let mut x = *self;
        let mut y = *rhs;
        while y.len > 0 {
            let (_, r) = x.divrem_gen(&y, p, inv_fp);
            x = y;
            y = r;
        }
        x.monic(p, inv_fp)
    }

    #[inline]
    pub fn deriv(&self, p: u64) -> Self {
        if self.len <= 1 {
            return Self::ZERO;
        }
        let mut out = Self::ZERO;
        out.len = self.len - 1;
        for i in 1..self.len {
            out.c[i - 1] = mm(self.c[i], (i as u64) % p, p);
        }
        out.trim();
        out
    }

    /// `x^e mod m` for monic `m` (`deg m >= 1`).
    #[inline]
    pub fn powmod_x(e: u128, m: &Self, p: u64) -> Self {
        if m.len <= 1 {
            return Self::ZERO;
        }
        if e == 0 {
            let mut one = Self::ZERO;
            one.len = 1;
            one.c[0] = 1;
            return one;
        }
        let top = 127 - e.leading_zeros();
        let mut acc = Self::ZERO;
        acc.len = 2;
        acc.c[1] = 1;
        acc = acc.rem_monic(m, p);
        for bit in (0..top).rev() {
            acc = acc.sqr(p).rem_monic(m, p);
            if ((e >> bit) & 1) != 0 {
                // Multiply `acc` by `x` and reduce mod monic `m` (`1` step, `deg m` muls!).
                let mut shifted = Self::ZERO;
                if acc.len > 0 {
                    shifted.len = acc.len + 1;
                    shifted.c[1..=acc.len].copy_from_slice(&acc.c[..acc.len]);
                    if shifted.len == m.len {
                        let coef = shifted.c[m.len - 1];
                        shifted.c[m.len - 1] = 0;
                        shifted.len = m.len - 1;
                        if coef != 0 {
                            for i in 0..(m.len - 1) {
                                let mc = m.c[i];
                                if mc != 0 {
                                    shifted.c[i] = sm(shifted.c[i], mm(coef, mc, p), p);
                                }
                            }
                        }
                        shifted.trim();
                    }
                }
                acc = shifted;
            }
        }
        acc
    }

    /// Horner modular composition `f(arg) mod m` for monic `m`.
    #[inline]
    pub fn compose_mod(&self, arg: &Self, m: &Self, p: u64) -> Self {
        if self.len == 0 || m.len <= 1 {
            return Self::ZERO;
        }
        let mut acc = Self::ZERO;
        acc.len = 1;
        acc.c[0] = self.c[self.len - 1];
        for i in (0..(self.len - 1)).rev() {
            acc = acc.mul(arg, p).rem_monic(m, p);
            let ci = self.c[i];
            if ci != 0 {
                if acc.len == 0 {
                    acc.len = 1;
                    acc.c[0] = ci;
                } else {
                    acc.c[0] = am(acc.c[0], ci, p);
                    acc.trim();
                }
            }
        }
        acc.rem_monic(m, p)
    }
}

fn push_pair_as_line(
    d1: &StackPoly,
    d2: &StackPoly,
    lead_g: u64,
    half: u64,
    w_mod_p: u64,
    p: u64,
    b: &[E2],
    re: &FpPoly,
    nb: &FpPoly,
    out: &mut Vec<Line>,
) {
    let v1 = d1.to_vec();
    let v2 = d2.to_vec();
    let (a0, cofac) = if (v1.len(), &v1) <= (v2.len(), &v2) {
        (v1, d2)
    } else {
        (v2, d1)
    };
    let scale_a1 = mm(lead_g, half, p);
    let a1_sp = cofac.scale(scale_a1, p);
    let a0_sp = StackPoly::from_slice(&a0);
    let p1 = a0_sp.sqr(p).to_vec();
    let p2 = a1_sp.sqr(p).scale(w_mod_p, p).to_vec();
    out.push(Line {
        a0,
        a1: a1_sp.to_vec(),
        b: b.to_vec(),
        p1,
        p2,
        p3: re.clone(),
        p4: nb.clone(),
    });
}

fn divisors_deg4_from_atoms(atoms: &[StackPoly], p: u64, out_divs: &mut Vec<StackPoly>) {
    fn rec(
        atoms: &[StackPoly],
        idx: usize,
        rem_deg: usize,
        acc: StackPoly,
        p: u64,
        out: &mut Vec<StackPoly>,
    ) {
        if rem_deg == 0 {
            out.push(acc);
            return;
        }
        if idx >= atoms.len() {
            return;
        }
        let d = atoms[idx].deg() as usize;
        if d <= rem_deg {
            // Take `atoms[idx]` (matching `divisors_of_degree` recursion order: j=0 first, then j=1).
            rec(atoms, idx + 1, rem_deg, acc, p, out);
            let prod = acc.mul(&atoms[idx], p);
            rec(atoms, idx + 1, rem_deg - d, prod, p, out);
        } else {
            rec(atoms, idx + 1, rem_deg, acc, p, out);
        }
    }
    let mut one = StackPoly::ZERO;
    one.len = 1;
    one.c[0] = 1;
    rec(atoms, 0, 4, one, p, out_divs);
}

/// Partition-direct line extractor for `m = 9` (`deg B = 1`), preserving the
/// exact `Line` set and exact `StdRng` synchronization of [`sieve::lines_for_b_hint`].
pub fn lines_for_b_partition_m9(
    ring: &FpRing,
    fq: &Fq,
    setup: &SieveSetup,
    b: &[E2],
    inv_fp: &[u64],
    rng: &mut StdRng,
    roots_hint: Option<&[u64]>,
) -> Vec<Line> {
    let p = setup.p;
    if setup.m != 9 || b.len() != 2 || b[1] != E2::ONE || roots_hint.is_none() {
        return sieve::lines_for_b_hint(ring, fq, setup, b, rng, true, roots_hint);
    }
    let roots = roots_hint.unwrap();

    // Fast `B(x)^2 * h(x)` for `B(x) = c + x` (`c = b0 + t*b1`), `B^2 = c^2 + 2c x + x^2`.
    let c = b[0];
    let c_sq = fq.sq(&c);
    let two_c = fq.add(&c, &c);
    let h = &setup.h;
    let h_len = h.len();
    let mut b2h = [E2::ZERO; 11];
    let out_len = h_len + 2;
    for i in 0..h_len {
        let hi = h[i];
        if hi == E2::ZERO {
            continue;
        }
        // `hi * c^2` at `i`, `hi * 2c` at `i + 1`, `hi * 1` at `i + 2`.
        let p0 = if hi == E2::ONE {
            c_sq
        } else {
            fq.mul(&hi, &c_sq)
        };
        let p1 = if hi == E2::ONE {
            two_c
        } else {
            fq.mul(&hi, &two_c)
        };
        b2h[i] = fq.add(&b2h[i], &p0);
        b2h[i + 1] = fq.add(&b2h[i + 1], &p1);
        b2h[i + 2] = fq.add(&b2h[i + 2], &hi);
    }

    let mut g_vec: FpPoly = b2h[..out_len].iter().map(|z| z.0[1]).collect();
    while g_vec.last() == Some(&0) {
        g_vec.pop();
    }
    let mut re_vec: FpPoly = b2h[..out_len].iter().map(|z| z.0[0]).collect();
    while re_vec.last() == Some(&0) {
        re_vec.pop();
    }
    if g_vec.is_empty() {
        return Vec::new();
    }
    let dg = fdeg(&g_vec);
    if dg < 4 {
        return Vec::new();
    }
    if dg != 8 {
        // `dg` in `{4, 5, 6, 7}` or `9` (occurs on `O(1/p)` planes when `b1 == 0` or `Im` cancels):
        // delegate directly so edge-degree semantics and `rng` stay 100% identical.
        return sieve::lines_for_b_hint(ring, fq, setup, b, rng, true, Some(roots));
    }

    let lead_g = g_vec[8];
    let g_monic = StackPoly::from_slice(&g_vec).monic(p, inv_fp);

    // Divide out the hinted ascending linear roots and reject/fallback if any root is repeated.
    let mut rest = g_monic;
    for &r in roots {
        if !rest.synthetic_div(r, p) || rest.eval(r, p) == 0 {
            return sieve::lines_for_b_hint(ring, fq, setup, b, rng, true, Some(roots));
        }
    }
    // Check squarefreeness of root-free cofactor `rest`: `gcd(rest, rest') == 1`.
    if rest.len > 2 {
        let d_rest = rest.deriv(p);
        if !rest.gcd_monic(&d_rest, p, inv_fp).is_one() {
            return sieve::lines_for_b_hint(ring, fq, setup, b, rng, true, Some(roots));
        }
    }

    let dr = rest.deg() as usize;
    let half = table_inv(inv_fp, 2 % p);
    let w_mod_p = setup.w % p;
    // `N(B) = (x + b0)^2 - w * b1^2 = (b0^2 - w*b1^2) + 2*b0*x + x^2`.
    let b0 = c.0[0];
    let b1 = c.0[1];
    let nb0 = sm(mm(b0, b0, p), mm(w_mod_p, mm(b1, b1, p), p), p);
    let nb1 = am(b0, b0, p);
    let nb_vec = vec![nb0, nb1, 1u64];

    match dr {
        8 => {
            // `t == 0` (`36.8%` of planes): `rest` has degree 8 and no linear roots.
            let xp = StackPoly::powmod_x(p as u128, &rest, p);
            let xp2 = xp.compose_mod(&xp, &rest, p);
            let g2 = rest.gcd_monic(&xp2.sub_x(p), p, inv_fp);
            match g2.deg() {
                2 => Vec::new(), // `(2, 6)` or `(2, 3, 3)` -> zero degree-4 divisors!
                4 => {
                    // `(2, 2, 4)` -> unique complementary pair `{g2, rest / g2}`, zero CZ!
                    let (q4, _) = rest.divrem_gen(&g2, p, inv_fp);
                    let mut out = Vec::with_capacity(1);
                    push_pair_as_line(
                        &g2, &q4, lead_g, half, w_mod_p, p, b, &re_vec, &nb_vec, &mut out,
                    );
                    out
                }
                0 => {
                    // `(8)`, `(3, 5)`, or `(4, 4)`: degree-4 divisor exists iff `xp4 == x` (`(4, 4)`).
                    let xp4 = xp2.compose_mod(&xp2, &rest, p);
                    if !xp4.is_x() {
                        Vec::new()
                    } else {
                        sieve::lines_for_b_hint(ring, fq, setup, b, rng, true, Some(roots))
                    }
                }
                _ => sieve::lines_for_b_hint(ring, fq, setup, b, rng, true, Some(roots)),
            }
        }
        7 => {
            // `t == 1` (`36.8%` of planes): `rest` has degree 7 and no linear roots.
            let xp = StackPoly::powmod_x(p as u128, &rest, p);
            let xp2 = xp.compose_mod(&xp, &rest, p);
            let g2 = rest.gcd_monic(&xp2.sub_x(p), p, inv_fp);
            match g2.deg() {
                2 => Vec::new(), // `(2, 5)` -> zero degree-4 divisors!
                4 => {
                    // `(2, 2, 3)` -> unique pair `{g2, (x - r0) * c3}`, zero CZ!
                    let (c3, _) = rest.divrem_gen(&g2, p, inv_fp);
                    let d2 = c3.mul_linear_root(roots[0], p);
                    let mut out = Vec::with_capacity(1);
                    push_pair_as_line(
                        &g2, &d2, lead_g, half, w_mod_p, p, b, &re_vec, &nb_vec, &mut out,
                    );
                    out
                }
                0 => {
                    // `(7)` or `(3, 4)`.
                    let xp3 = xp2.compose_mod(&xp, &rest, p);
                    let g3 = rest.gcd_monic(&xp3.sub_x(p), p, inv_fp);
                    if g3.deg() == 3 {
                        let (q4, _) = rest.divrem_gen(&g3, p, inv_fp);
                        let d1 = g3.mul_linear_root(roots[0], p);
                        let mut out = Vec::with_capacity(1);
                        push_pair_as_line(
                            &d1, &q4, lead_g, half, w_mod_p, p, b, &re_vec, &nb_vec, &mut out,
                        );
                        out
                    } else {
                        Vec::new()
                    }
                }
                _ => sieve::lines_for_b_hint(ring, fq, setup, b, rng, true, Some(roots)),
            }
        }
        6 => {
            // `t == 2` (`18.4%` of planes): `(6)`, `(2, 4)`, `(3, 3)`, `(2, 2, 2)`.
            let xp = StackPoly::powmod_x(p as u128, &rest, p);
            let xp2 = xp.compose_mod(&xp, &rest, p);
            let g2 = rest.gcd_monic(&xp2.sub_x(p), p, inv_fp);
            match g2.deg() {
                2 => {
                    let (q4, _) = rest.divrem_gen(&g2, p, inv_fp);
                    let mut atoms = Vec::with_capacity(4);
                    for &r in roots {
                        atoms.push(StackPoly::from_slice(&[if r == 0 { 0 } else { p - r }, 1]));
                    }
                    atoms.push(g2);
                    atoms.push(q4);
                    emit_lines_from_atoms(
                        &atoms, &g_monic, lead_g, half, w_mod_p, p, inv_fp, b, &re_vec, &nb_vec,
                    )
                }
                0 => {
                    let xp3 = xp2.compose_mod(&xp, &rest, p);
                    if !xp3.is_x() {
                        Vec::new() // `(6)` irreducible!
                    } else {
                        sieve::lines_for_b_hint(ring, fq, setup, b, rng, true, Some(roots))
                    }
                }
                _ => sieve::lines_for_b_hint(ring, fq, setup, b, rng, true, Some(roots)),
            }
        }
        5 => {
            // `t == 3` (`6.1%` of planes): `(5)` or `(2, 3)`.
            let xp = StackPoly::powmod_x(p as u128, &rest, p);
            let xp2 = xp.compose_mod(&xp, &rest, p);
            let g2 = rest.gcd_monic(&xp2.sub_x(p), p, inv_fp);
            if g2.deg() == 0 {
                Vec::new() // `(5)` irreducible!
            } else if g2.deg() == 2 {
                let (c3, _) = rest.divrem_gen(&g2, p, inv_fp);
                let mut atoms = Vec::with_capacity(5);
                for &r in roots {
                    atoms.push(StackPoly::from_slice(&[if r == 0 { 0 } else { p - r }, 1]));
                }
                atoms.push(g2);
                atoms.push(c3);
                emit_lines_from_atoms(
                    &atoms, &g_monic, lead_g, half, w_mod_p, p, inv_fp, b, &re_vec, &nb_vec,
                )
            } else {
                sieve::lines_for_b_hint(ring, fq, setup, b, rng, true, Some(roots))
            }
        }
        4 => {
            // `t == 4`: `(4)` irreducible or `(2, 2)`.
            let xp = StackPoly::powmod_x(p as u128, &rest, p);
            let xp2 = xp.compose_mod(&xp, &rest, p);
            if !xp2.is_x() {
                let mut atoms = Vec::with_capacity(5);
                for &r in roots {
                    atoms.push(StackPoly::from_slice(&[if r == 0 { 0 } else { p - r }, 1]));
                }
                atoms.push(rest);
                emit_lines_from_atoms(
                    &atoms, &g_monic, lead_g, half, w_mod_p, p, inv_fp, b, &re_vec, &nb_vec,
                )
            } else {
                sieve::lines_for_b_hint(ring, fq, setup, b, rng, true, Some(roots))
            }
        }
        0 | 2 | 3 => {
            // `dr in {0, 2, 3}` with `rest` root-free => `rest` is already `1` or irreducible!
            let mut atoms = Vec::with_capacity(roots.len() + 1);
            for &r in roots {
                atoms.push(StackPoly::from_slice(&[if r == 0 { 0 } else { p - r }, 1]));
            }
            if dr >= 2 {
                atoms.push(rest);
            }
            emit_lines_from_atoms(
                &atoms, &g_monic, lead_g, half, w_mod_p, p, inv_fp, b, &re_vec, &nb_vec,
            )
        }
        _ => sieve::lines_for_b_hint(ring, fq, setup, b, rng, true, Some(roots)),
    }
}

#[allow(clippy::too_many_arguments)]
fn emit_lines_from_atoms(
    atoms: &[StackPoly],
    g_monic: &StackPoly,
    lead_g: u64,
    half: u64,
    w_mod_p: u64,
    p: u64,
    inv_fp: &[u64],
    b: &[E2],
    re_vec: &FpPoly,
    nb_vec: &FpPoly,
) -> Vec<Line> {
    let mut divs = Vec::new();
    divisors_deg4_from_atoms(atoms, p, &mut divs);
    let mut out = Vec::new();
    let scale_a1 = mm(lead_g, half, p);
    for d in &divs {
        let (cofac, _) = g_monic.divrem_gen(d, p, inv_fp);
        let a0_vec = d.to_vec();
        let cofac_vec = cofac.to_vec();
        if (a0_vec.len(), &a0_vec) > (cofac_vec.len(), &cofac_vec) {
            continue;
        }
        let a1_sp = cofac.scale(scale_a1, p);
        let p1 = d.sqr(p).to_vec();
        let p2 = a1_sp.sqr(p).scale(w_mod_p, p).to_vec();
        out.push(Line {
            a0: a0_vec,
            a1: a1_sp.to_vec(),
            b: b.to_vec(),
            p1,
            p2,
            p3: re_vec.clone(),
            p4: nb_vec.clone(),
        });
    }
    out
}

/// Stack-allocated forward-difference table at `x = 0`: `Delta^k P(0)` for `deg <= 9`.
#[inline]
fn stack_diff_table(poly: &[u64], p: u64, adds: &mut u64) -> ([u64; 10], usize) {
    let d = poly.len().max(1) - 1;
    let mut v = [0u64; 10];
    for (x, slot) in v.iter_mut().enumerate().take(d + 1) {
        if !poly.is_empty() {
            let mut acc = poly[poly.len() - 1];
            for k in (0..(poly.len() - 1)).rev() {
                acc = am(mm(acc, x as u64, p), poly[k], p);
            }
            *slot = acc;
        }
    }
    let mut table = [0u64; 10];
    for (k, t_slot) in table.iter_mut().enumerate().take(d + 1) {
        *t_slot = v[0];
        for i in 0..(d - k) {
            v[i] = sm(v[i + 1], v[i], p);
            *adds += 1;
        }
    }
    (table, d)
}

/// Reusable scratch buffers for single-flush stack sieve line execution (`0` per-line allocations).
pub struct FastSieveScratch {
    pending: Vec<(u64, u64, u64, u16)>,
    prefix: Vec<u64>,
}

impl FastSieveScratch {
    pub fn new(p: u64) -> Self {
        let cap = p as usize;
        Self {
            pending: Vec::with_capacity(cap),
            prefix: Vec::with_capacity(cap),
        }
    }
}

/// Single-flush, zero-inversion-call (`inv_fp` table), stack-difference-table variant of
/// [`sieve::sieve_line_buckets`] producing the exact same `HitBuckets` and hit list.
pub fn sieve_line_buckets_fast(
    setup: &SieveSetup,
    line: &Line,
    inv_fp: &[u64],
    buckets: &mut HitBuckets,
    scratch: &mut FastSieveScratch,
    shared: Option<&SharedB>,
    minimum: u8,
) -> (Vec<u64>, SieveCost) {
    let p = setup.p;
    let m = minimum;
    let mut cost = SieveCost::default();
    buckets.advance_line_id();
    let id = buckets.current_id();

    let (mut t2, d2) = stack_diff_table(&line.p2, p, &mut cost.adds);
    let (mut t3, d3, mut t4, d4) = if shared.is_some() {
        ([0u64; 10], 0usize, [0u64; 10], 0usize)
    } else {
        let (a3, deg3) = stack_diff_table(&line.p3, p, &mut cost.adds);
        let (a4, deg4) = stack_diff_table(&line.p4, p, &mut cost.adds);
        (a3, deg3, a4, deg4)
    };

    let mut hits: Vec<u64> = Vec::new();
    scratch.pending.clear();

    for x in 0..p {
        cost.steps += 1;
        if setup.in_base[x as usize] {
            cost.base_steps += 1;
            let a = t2[0];
            let (re, n4) = match shared {
                Some(sh) => {
                    cost.shared_reads += 2;
                    sh.at(x as usize)
                }
                None => (t3[0], t4[0]),
            };
            let r = mm(n4, setup.rho[x as usize], p);
            if a == 0 {
                if re != 0 {
                    let c = StackPoly::from_slice(&line.a0).eval(x, p);
                    let s = mm(mm(c, c, p), table_inv(inv_fp, re), p);
                    buckets.record_hit(id, s, x as u16, m, &mut hits, &mut cost);
                }
            } else {
                scratch.pending.push((re, r, am(a, a, p), x as u16));
            }
        }
        for k in 0..d2 {
            t2[k] = am(t2[k], t2[k + 1], p);
        }
        cost.adds += d2 as u64;
        if shared.is_none() {
            for k in 0..d3 {
                t3[k] = am(t3[k], t3[k + 1], p);
            }
            for k in 0..d4 {
                t4[k] = am(t4[k], t4[k + 1], p);
            }
            cost.adds += (d3 + d4) as u64;
        }
    }

    // Single Montgomery batch inversion across all base steps of the line, inverted via `inv_fp`!
    let n = scratch.pending.len();
    if n > 0 {
        scratch.prefix.clear();
        let mut acc = 1u64;
        for &(_, _, a2, _) in &scratch.pending {
            scratch.prefix.push(acc);
            acc = mm(acc, a2, p);
        }
        let mut inv = table_inv(inv_fp, acc);
        for i in (0..n).rev() {
            let (re, r, a2, x_u16) = scratch.pending[i];
            let inv_a2 = mm(inv, scratch.prefix[i], p);
            if i > 0 {
                inv = mm(inv, a2, p);
            }
            let s1 = mm(am(re, r, p), inv_a2, p);
            buckets.record_hit(id, s1, x_u16, m, &mut hits, &mut cost);
            if r != 0 {
                let s2 = mm(sm(re, r, p), inv_a2, p);
                buckets.record_hit(id, s2, x_u16, m, &mut hits, &mut cost);
            }
        }
        scratch.pending.clear();
    }

    work::words(cost.steps + 4 * cost.lookups + 2 * cost.shared_reads);
    (hits, cost)
}

/// Precompute `inv_2w_sq[y] = (2 * w * y^2 mod p)^{-1} mod p` (`0` at `y = 0`) once per solve.
/// Eliminates the entire `3 * |FB|`-mul Montgomery batch inversion loop on every sieve line!
pub fn build_inv_2w_sq_table(p: u64, w: u64, inv_fp: &[u64]) -> Vec<u64> {
    let pu = p as usize;
    let mut tbl = vec![0u64; pu];
    let w_mod = w % p;
    let two_w = am(w_mod, w_mod, p);
    for (y, slot) in tbl.iter_mut().enumerate().skip(1) {
        let yy = mm(y as u64, y as u64, p);
        let denom = mm(two_w, yy, p);
        *slot = table_inv(inv_fp, denom);
        work::words(1);
    }
    tbl
}

/// Pre-evaluated plane arrays `(Re(B^2 h)(x), N(B)(x) * rho(x))` shared across all lines of one `B`.
/// Pre-multiplying `N(B)(x) * rho(x)` once per multi-line plane avoids repeating `|FB|` muls per line.
pub struct SharedBFast {
    re: Vec<u64>,
    r_base: Vec<u64>,
}

impl SharedBFast {
    pub fn build(setup: &SieveSetup, line: &Line) -> Self {
        let p = setup.p;
        let pu = p as usize;
        let mut adds = 0u64;
        let mut re = Vec::with_capacity(pu);
        let (mut t3, d3) = stack_diff_table(&line.p3, p, &mut adds);
        for _ in 0..p {
            re.push(t3[0]);
            for k in 0..d3 {
                t3[k] = am(t3[k], t3[k + 1], p);
            }
        }
        let mut r_base = vec![0u64; pu];
        let (mut t4, d4) = stack_diff_table(&line.p4, p, &mut adds);
        for x in 0..pu {
            if setup.in_base[x] {
                r_base[x] = mm(t4[0], setup.rho[x], p);
            }
            for k in 0..d4 {
                t4[k] = am(t4[k], t4[k + 1], p);
            }
        }
        work::words(5 * p);
        Self { re, r_base }
    }
}

/// Zero-batch-mul (`inv_2w_sq[A_1'(x)]` lookup) + degree-4 `A_1'(x)` forward-difference sieve.
/// Halves per-line difference-table `FieldAdd`s (`deg 4` vs `deg 8`) and eliminates `3 * |FB|`
/// Montgomery batch-inversion `FieldMul`s per line while producing the exact same hit sets.
pub fn sieve_line_buckets_zero_mul(
    setup: &SieveSetup,
    line: &Line,
    inv_fp: &[u64],
    inv_2w_sq: &[u64],
    buckets: &mut HitBuckets,
    shared: Option<&SharedBFast>,
    minimum: u8,
) -> (Vec<u64>, SieveCost) {
    let p = setup.p;
    let m = minimum;
    let mut cost = SieveCost::default();
    buckets.advance_line_id();
    let id = buckets.current_id();

    let (mut t_a1, d_a1) = stack_diff_table(&line.a1, p, &mut cost.adds);
    let (mut t3, d3, mut t4, d4) = if shared.is_some() {
        ([0u64; 10], 0usize, [0u64; 10], 0usize)
    } else {
        let (a3, deg3) = stack_diff_table(&line.p3, p, &mut cost.adds);
        let (a4, deg4) = stack_diff_table(&line.p4, p, &mut cost.adds);
        (a3, deg3, a4, deg4)
    };

    let mut hits: Vec<u64> = Vec::new();
    let a0_sp = StackPoly::from_slice(&line.a0);

    for x in 0..p {
        let xu = x as usize;
        cost.steps += 1;
        if setup.in_base[xu] {
            cost.base_steps += 1;
            let y = t_a1[0];
            let (re, r) = match shared {
                Some(sh) => {
                    cost.shared_reads += 2;
                    (sh.re[xu], sh.r_base[xu])
                }
                None => (t3[0], mm(t4[0], setup.rho[xu], p)),
            };
            if y == 0 {
                if re != 0 {
                    let c = a0_sp.eval(x, p);
                    let s = mm(mm(c, c, p), table_inv(inv_fp, re), p);
                    buckets.record_hit(id, s, x as u16, m, &mut hits, &mut cost);
                }
            } else {
                work::words(1);
                let inv_a2 = inv_2w_sq[y as usize];
                let s1 = mm(am(re, r, p), inv_a2, p);
                buckets.record_hit(id, s1, x as u16, m, &mut hits, &mut cost);
                if r != 0 {
                    let s2 = mm(sm(re, r, p), inv_a2, p);
                    buckets.record_hit(id, s2, x as u16, m, &mut hits, &mut cost);
                }
            }
        }
        for k in 0..d_a1 {
            t_a1[k] = am(t_a1[k], t_a1[k + 1], p);
        }
        cost.adds += d_a1 as u64;
        if shared.is_none() {
            for k in 0..d3 {
                t3[k] = am(t3[k], t3[k + 1], p);
            }
            for k in 0..d4 {
                t4[k] = am(t4[k], t4[k + 1], p);
            }
            cost.adds += (d3 + d4) as u64;
        }
    }

    work::words(cost.steps + 4 * cost.lookups + 2 * cost.shared_reads);
    (hits, cost)
}

#[inline]
fn pow_small_fp(mut base: u64, mut exp: usize, p: u64) -> u64 {
    let mut acc = 1u64;
    while exp > 0 {
        if (exp & 1) != 0 {
            acc = mm(acc, base, p);
        }
        exp >>= 1;
        if exp > 0 {
            base = mm(base, base, p);
        }
    }
    acc
}

/// Simultaneous squarefreeness test + exact discriminant quadratic-character check
/// (`None` if `gcd(f, f') != 1`; `Some(true)` if `disc(f)` is a nonzero square in `F_p`;
/// `Some(false)` if `disc(f)` is a non-square in `F_p`) for monic `f` of degree `>= 2`.
#[inline]
pub fn sqfree_and_disc_square(
    f: &StackPoly,
    p: u64,
    inv_fp: &[u64],
    sqrt_table: &[u64],
) -> Option<bool> {
    let d = f.deg();
    if d < 2 {
        return Some(true);
    }
    let du = d as usize;
    let mut a = *f;
    let mut b = f.deriv(p);
    let mut res = 1u64;
    let mut neg_flips = (du * (du - 1)) / 2;

    while b.len > 0 {
        let m = a.deg() as usize;
        let n = b.deg() as usize;
        let b_lead = b.c[n];
        res = mm(res, pow_small_fp(b_lead, m, p), p);
        if n == 0 {
            if (neg_flips & 1) != 0 && res != 0 {
                res = sm(p, res, p);
            }
            if res == 0 {
                return None;
            }
            work::words(1);
            return Some(sqrt_table[res as usize] != u64::MAX);
        }
        if ((m * n) & 1) != 0 {
            neg_flips += 1;
        }
        let b_monic = b.monic(p, inv_fp);
        let r = a.rem_monic(&b_monic, p);
        if r.len == 0 {
            return None;
        }
        a = b_monic;
        b = r;
    }
    None
}

/// Precomputed Frobenius basis `cols[j] = x^{j p} mod m` (`j = 0..deg(m)-1`),
/// constructed using `.sqr(p)` on all even columns `2, 4, 6` and applying the linear
/// Frobenius endomorphism `v(x) |-> v(x^p) mod m` in `<= d(d-1)` `FieldMul`s (`<= 56` vs `735`!).
struct FrobBasis {
    d: usize,
    cols: [StackPoly; 8],
}

impl FrobBasis {
    #[inline]
    fn build(xp: &StackPoly, m: &StackPoly, p: u64) -> Self {
        let d = m.deg() as usize;
        debug_assert!((2..=8).contains(&d));
        let mut cols = [StackPoly::ZERO; 8];
        cols[0].len = 1;
        cols[0].c[0] = 1;
        cols[1] = *xp;
        for j in 2..d {
            cols[j] = if (j & 1) == 0 {
                cols[j / 2].sqr(p).rem_monic(m, p)
            } else {
                cols[j - 1].mul(xp, p).rem_monic(m, p)
            };
        }
        Self { d, cols }
    }

    #[inline]
    fn apply(&self, v: &StackPoly, p: u64) -> StackPoly {
        if v.len == 0 {
            return StackPoly::ZERO;
        }
        let mut out = StackPoly::ZERO;
        out.len = self.d;
        out.c[0] = v.c[0];
        let lim = v.len.min(self.d);
        for j in 1..lim {
            let vj = v.c[j];
            if vj == 0 {
                continue;
            }
            let col = &self.cols[j];
            for i in 0..col.len {
                let ci = col.c[i];
                if ci != 0 {
                    out.c[i] = am(out.c[i], mm(vj, ci, p), p);
                }
            }
        }
        out.trim();
        out
    }
}

/// Stickelberger-screened + linear-Frobenius-matrix `m = 9` line extractor, bit-for-bit identical
/// in emitted `Line`s and `StdRng` state to [`lines_for_b_partition_m9`] and [`sieve::lines_for_b_hint`].
pub fn lines_for_b_frob_stickelberger_m9(
    ring: &FpRing,
    fq: &Fq,
    setup: &SieveSetup,
    b: &[E2],
    inv_fp: &[u64],
    rng: &mut StdRng,
    roots_hint: Option<&[u64]>,
) -> Vec<Line> {
    let p = setup.p;
    if setup.m != 9 || b.len() != 2 || b[1] != E2::ONE || roots_hint.is_none() {
        return sieve::lines_for_b_hint(ring, fq, setup, b, rng, true, roots_hint);
    }
    let roots = roots_hint.unwrap();

    let c = b[0];
    let c_sq = fq.sq(&c);
    let two_c = fq.add(&c, &c);
    let h = &setup.h;
    let h_len = h.len();
    let mut b2h = [E2::ZERO; 11];
    let out_len = h_len + 2;
    for i in 0..h_len {
        let hi = h[i];
        if hi == E2::ZERO {
            continue;
        }
        let p0 = if hi == E2::ONE {
            c_sq
        } else {
            fq.mul(&hi, &c_sq)
        };
        let p1 = if hi == E2::ONE {
            two_c
        } else {
            fq.mul(&hi, &two_c)
        };
        b2h[i] = fq.add(&b2h[i], &p0);
        b2h[i + 1] = fq.add(&b2h[i + 1], &p1);
        b2h[i + 2] = fq.add(&b2h[i + 2], &hi);
    }

    let mut g_vec: FpPoly = b2h[..out_len].iter().map(|z| z.0[1]).collect();
    while g_vec.last() == Some(&0) {
        g_vec.pop();
    }
    let mut re_vec: FpPoly = b2h[..out_len].iter().map(|z| z.0[0]).collect();
    while re_vec.last() == Some(&0) {
        re_vec.pop();
    }
    if g_vec.is_empty() {
        return Vec::new();
    }
    let dg = fdeg(&g_vec);
    if dg < 4 {
        return Vec::new();
    }
    if dg != 8 {
        return sieve::lines_for_b_hint(ring, fq, setup, b, rng, true, Some(roots));
    }

    let lead_g = g_vec[8];
    let g_monic = StackPoly::from_slice(&g_vec).monic(p, inv_fp);

    let mut rest = g_monic;
    for &r in roots {
        if !rest.synthetic_div(r, p) || rest.eval(r, p) == 0 {
            return sieve::lines_for_b_hint(ring, fq, setup, b, rng, true, Some(roots));
        }
    }

    // Combined squarefreeness + Stickelberger discriminant character (`0` extra GCD passes!).
    let disc_sq = if rest.len > 2 {
        match sqfree_and_disc_square(&rest, p, inv_fp, &setup.sqrt) {
            Some(sq) => sq,
            None => return sieve::lines_for_b_hint(ring, fq, setup, b, rng, true, Some(roots)),
        }
    } else {
        true
    };

    let dr = rest.deg() as usize;
    let half = table_inv(inv_fp, 2 % p);
    let w_mod_p = setup.w % p;
    let b0 = c.0[0];
    let b1 = c.0[1];
    let nb0 = sm(mm(b0, b0, p), mm(w_mod_p, mm(b1, b1, p), p), p);
    let nb1 = am(b0, b0, p);
    let nb_vec = vec![nb0, nb1, 1u64];

    match dr {
        8 => {
            // Stickelberger (`d = 8`): `disc_sq == false` <=> `r` is ODD (`(8), (2,2,4), (2,3,3)`),
            // so `(4, 4)` (`r = 2` EVEN) is IMPOSSIBLE when `!disc_sq`!
            let xp = StackPoly::powmod_x(p as u128, &rest, p);
            let frob = FrobBasis::build(&xp, &rest, p);
            let xp2 = frob.apply(&xp, p);
            let g2 = rest.gcd_monic(&xp2.sub_x(p), p, inv_fp);
            match g2.deg() {
                2 => Vec::new(),
                4 => {
                    let (q4, _) = rest.divrem_gen(&g2, p, inv_fp);
                    let mut out = Vec::with_capacity(1);
                    push_pair_as_line(
                        &g2, &q4, lead_g, half, w_mod_p, p, b, &re_vec, &nb_vec, &mut out,
                    );
                    out
                }
                0 => {
                    if !disc_sq {
                        // `r` odd and no quadratics => MUST be `(8)` irreducible! Skip `xp4`!
                        Vec::new()
                    } else {
                        // `(3, 5)` or `(4, 4)`: compute `xp4 = F(F(xp2))` in `2 * 56 = 112` muls!
                        let xp3 = frob.apply(&xp2, p);
                        let xp4 = frob.apply(&xp3, p);
                        if !xp4.is_x() {
                            Vec::new()
                        } else {
                            sieve::lines_for_b_hint(ring, fq, setup, b, rng, true, Some(roots))
                        }
                    }
                }
                _ => sieve::lines_for_b_hint(ring, fq, setup, b, rng, true, Some(roots)),
            }
        }
        7 => {
            // Stickelberger (`d = 7`): `disc_sq == true` <=> `r` is ODD (`(7)` or `(2, 2, 3)`);
            // `disc_sq == false` <=> `r` is EVEN (`(2, 5)` or `(3, 4)`).
            let xp = StackPoly::powmod_x(p as u128, &rest, p);
            let frob = FrobBasis::build(&xp, &rest, p);
            let xp2 = frob.apply(&xp, p);
            let g2 = rest.gcd_monic(&xp2.sub_x(p), p, inv_fp);
            match g2.deg() {
                2 => Vec::new(),
                4 => {
                    let (c3, _) = rest.divrem_gen(&g2, p, inv_fp);
                    let d2 = c3.mul_linear_root(roots[0], p);
                    let mut out = Vec::with_capacity(1);
                    push_pair_as_line(
                        &g2, &d2, lead_g, half, w_mod_p, p, b, &re_vec, &nb_vec, &mut out,
                    );
                    out
                }
                0 => {
                    if disc_sq {
                        // `r` odd and no quadratics => MUST be `(7)` irreducible! Skip `xp3`!
                        Vec::new()
                    } else {
                        // `r` even and no quadratics => MUST be `(3, 4)`!
                        let xp3 = frob.apply(&xp2, p);
                        let g3 = rest.gcd_monic(&xp3.sub_x(p), p, inv_fp);
                        if g3.deg() == 3 {
                            let (q4, _) = rest.divrem_gen(&g3, p, inv_fp);
                            let d1 = g3.mul_linear_root(roots[0], p);
                            let mut out = Vec::with_capacity(1);
                            push_pair_as_line(
                                &d1, &q4, lead_g, half, w_mod_p, p, b, &re_vec, &nb_vec, &mut out,
                            );
                            out
                        } else {
                            Vec::new()
                        }
                    }
                }
                _ => sieve::lines_for_b_hint(ring, fq, setup, b, rng, true, Some(roots)),
            }
        }
        6 => {
            // Stickelberger (`d = 6`): `disc_sq == false` <=> `r` is ODD (`(6)` or `(2, 2, 2)`);
            // `disc_sq == true` <=> `r` is EVEN (`(2, 4)` or `(3, 3)`).
            let xp = StackPoly::powmod_x(p as u128, &rest, p);
            let frob = FrobBasis::build(&xp, &rest, p);
            let xp2 = frob.apply(&xp, p);
            let g2 = rest.gcd_monic(&xp2.sub_x(p), p, inv_fp);
            match g2.deg() {
                2 => {
                    let (q4, _) = rest.divrem_gen(&g2, p, inv_fp);
                    let mut atoms = Vec::with_capacity(4);
                    for &r in roots {
                        atoms.push(StackPoly::from_slice(&[if r == 0 { 0 } else { p - r }, 1]));
                    }
                    atoms.push(g2);
                    atoms.push(q4);
                    emit_lines_from_atoms(
                        &atoms, &g_monic, lead_g, half, w_mod_p, p, inv_fp, b, &re_vec, &nb_vec,
                    )
                }
                0 => {
                    if !disc_sq {
                        // `r` odd and no quadratics => MUST be `(6)` irreducible! Skip `xp3`!
                        Vec::new()
                    } else {
                        // `r` even and no quadratics => MUST be `(3, 3)`!
                        sieve::lines_for_b_hint(ring, fq, setup, b, rng, true, Some(roots))
                    }
                }
                _ => sieve::lines_for_b_hint(ring, fq, setup, b, rng, true, Some(roots)),
            }
        }
        5 => {
            // Stickelberger (`d = 5`): `disc_sq == true` <=> `r` is ODD => `(5)` irreducible (`0` powmod_x!!);
            // `disc_sq == false` <=> `r` is EVEN => `(2, 3)`!
            if disc_sq {
                Vec::new()
            } else {
                let xp = StackPoly::powmod_x(p as u128, &rest, p);
                let frob = FrobBasis::build(&xp, &rest, p);
                let xp2 = frob.apply(&xp, p);
                let g2 = rest.gcd_monic(&xp2.sub_x(p), p, inv_fp);
                if g2.deg() == 2 {
                    let (c3, _) = rest.divrem_gen(&g2, p, inv_fp);
                    let mut atoms = Vec::with_capacity(5);
                    for &r in roots {
                        atoms.push(StackPoly::from_slice(&[if r == 0 { 0 } else { p - r }, 1]));
                    }
                    atoms.push(g2);
                    atoms.push(c3);
                    emit_lines_from_atoms(
                        &atoms, &g_monic, lead_g, half, w_mod_p, p, inv_fp, b, &re_vec, &nb_vec,
                    )
                } else {
                    sieve::lines_for_b_hint(ring, fq, setup, b, rng, true, Some(roots))
                }
            }
        }
        4 => {
            // Stickelberger (`d = 4`): `disc_sq == false` <=> `r` is ODD => `(4)` irreducible (`0` powmod_x!!);
            // `disc_sq == true` <=> `r` is EVEN => `(2, 2)`!
            if !disc_sq {
                let mut atoms = Vec::with_capacity(5);
                for &r in roots {
                    atoms.push(StackPoly::from_slice(&[if r == 0 { 0 } else { p - r }, 1]));
                }
                atoms.push(rest);
                emit_lines_from_atoms(
                    &atoms, &g_monic, lead_g, half, w_mod_p, p, inv_fp, b, &re_vec, &nb_vec,
                )
            } else {
                sieve::lines_for_b_hint(ring, fq, setup, b, rng, true, Some(roots))
            }
        }
        0 | 2 | 3 => {
            let mut atoms = Vec::with_capacity(roots.len() + 1);
            for &r in roots {
                atoms.push(StackPoly::from_slice(&[if r == 0 { 0 } else { p - r }, 1]));
            }
            if dr >= 2 {
                atoms.push(rest);
            }
            emit_lines_from_atoms(
                &atoms, &g_monic, lead_g, half, w_mod_p, p, inv_fp, b, &re_vec, &nb_vec,
            )
        }
        _ => sieve::lines_for_b_hint(ring, fq, setup, b, rng, true, Some(roots)),
    }
}

#[inline]
fn is_irreducible_quad_stack(u: &[u64; 3], p: u64, sqrt_table: &[u64]) -> bool {
    if u[2] != 1 {
        return false;
    }
    let b2 = mm(u[1], u[1], p);
    let four_c = mm(4 % p, u[0], p);
    let disc = sm(b2, four_c, p);
    work::words(2);
    disc != 0 && sqrt_table[disc as usize] == u64::MAX
}

/// Deterministic zero-RNG Frobenius-norm resolvent splitter on stack `StackPoly` (`0` `FieldInv`s).
pub fn split_quartic_from_xp_resolvent_stack(
    q: &StackPoly,
    xp: &StackPoly,
    p: u64,
    inv_fp: &[u64],
    sqrt_table: &[u64],
) -> Result<[[u64; 3]; 2], &'static str> {
    let inv2 = p.div_ceil(2);
    for alpha in [0u64, 1u64] {
        let mut xp_a = *xp;
        if alpha != 0 {
            if xp_a.len == 0 {
                xp_a.len = 1;
                xp_a.c[0] = alpha;
            } else {
                xp_a.c[0] = am(xp_a.c[0], alpha, p);
                xp_a.trim();
            }
        }
        let lin = StackPoly::from_slice(&[alpha, 1]);
        let n_poly = lin.mul(&xp_a, p).rem_monic(q, p);
        if n_poly.len < 3 {
            continue;
        }
        let d = n_poly.len - 1;
        let s_poly = n_poly.sqr(p).rem_monic(q, p);
        let sd = if d < s_poly.len { s_poly.c[d] } else { 0 };
        let t_tr = mm(sd, table_inv(inv_fp, n_poly.c[d]), p);
        let n0 = if n_poly.len > 0 { n_poly.c[0] } else { 0 };
        let s0 = if s_poly.len > 0 { s_poly.c[0] } else { 0 };
        let d_nm = sm(mm(t_tr, n0, p), s0, p);
        let disc = sm(mm(t_tr, t_tr, p), mm(4 % p, d_nm, p), p);
        work::words(2);
        let r = sqrt_table[disc as usize];
        if disc == 0 || r == u64::MAX {
            return Err("resolvent discriminant not nonzero square");
        }
        let lam1 = mm(am(t_tr, r, p), inv2, p);
        let mut shifted_n = n_poly;
        if shifted_n.len == 0 {
            shifted_n.len = 1;
            shifted_n.c[0] = sm(0, lam1, p);
        } else {
            shifted_n.c[0] = sm(shifted_n.c[0], lam1, p);
            shifted_n.trim();
        }
        let u1 = q.gcd_monic(&shifted_n, p, inv_fp);
        if u1.len != 3 {
            continue;
        }
        let (u2, rem) = q.divrem_gen(&u1, p, inv_fp);
        if rem.len != 0 || u2.len != 3 {
            return Err("resolvent exact division failed");
        }
        let a1 = [u1.c[0], u1.c[1], 1];
        let a2 = [u2.c[0], u2.c[1], 1];
        let mut out = if a1 <= a2 { [a1, a2] } else { [a2, a1] };
        if out[0] == out[1]
            || !is_irreducible_quad_stack(&out[0], p, sqrt_table)
            || !is_irreducible_quad_stack(&out[1], p, sqrt_table)
        {
            return Err("resolvent factor irreducibility failed");
        }
        out.sort();
        return Ok(out);
    }
    Err("deterministic resolvent failed on alpha in {0, 1}")
}

/// Stickelberger-accelerated stack quartic `(2, 2)` splitter:
/// When `disc_screen_passed == true`, `Delta(q)` is already certified to be a nonzero
/// square in `F_p^*`, so `q` is squarefree with an even number of irreducible factors
/// (`(2, 2)`, `(1, 3)`, or `(1, 1, 1, 1)`). Thus `gcd(q, x^p - x) == 1` immediately
/// rules out `(1, 3)` and `(1, 1, 1, 1)` (`70%` of square-discriminant quartics) and
/// certifies `(2, 2)` without computing `x^{p^2} mod q`!
pub fn split_quartic_stickelberger_stack(
    tail_slice: &[u64],
    p: u64,
    inv_fp: &[u64],
    sqrt_table: &[u64],
    disc_screen_passed: bool,
) -> Result<[[u64; 3]; 2], &'static str> {
    if tail_slice.len() != 5 || tail_slice[4] == 0 {
        return Err("canonical quartic required");
    }
    let q = StackPoly::from_slice(tail_slice).monic(p, inv_fp);
    let xp = StackPoly::powmod_x(p as u128, &q, p);
    // Test linear-root-freeness FIRST (`70%` of square-discriminant quartics fail right here!).
    if !q.gcd_monic(&xp.sub_x(p), p, inv_fp).is_one() {
        return Err("quartic has F_p linear root");
    }
    if !disc_screen_passed {
        let xp2 = xp.compose_mod(&xp, &q, p);
        if !xp2.is_x() {
            return Err("quartic not a product of quadratics");
        }
    }
    split_quartic_from_xp_resolvent_stack(&q, &xp, p, inv_fp, sqrt_table)
}

#[inline]
fn rem_e2_mod_quad_stack(f: &Fq, poly: &[E2], u_fp: &[u64; 3]) -> [E2; 2] {
    if poly.is_empty() {
        return [E2::ZERO, E2::ZERO];
    }
    if poly.len() == 1 {
        return [poly[0], E2::ZERO];
    }
    let mut r0 = poly[poly.len() - 2];
    let mut r1 = poly[poly.len() - 1];
    let u0 = f.from_fp(u_fp[0]);
    let u1 = f.from_fp(u_fp[1]);
    for i in (0..(poly.len() - 2)).rev() {
        // Shift `(r0 + r1 x) * x + poly[i]` mod `(u0 + u1 x + x^2)`.
        let top = r1;
        let new_r1 = f.sub(&r0, &f.mul(&top, &u1));
        let new_r0 = f.sub(&poly[i], &f.mul(&top, &u0));
        r1 = new_r1;
        r0 = new_r0;
    }
    [r0, r1]
}

#[inline]
fn inv_linear_e2_mod_quad_stack(
    f: &Fq,
    c0: E2,
    c1: E2,
    u_fp: &[u64; 3],
    inv_fp: &[u64],
) -> Result<[E2; 2], &'static str> {
    // Invert `C(x) = c0 + c1 x` modulo `U(x) = u0 + u1 x + x^2` over `F_{p^2}`:
    // `(c0 + c1 x)(c0 - c1 u1 - c1 x) = c0^2 - c0 c1 u1 + c1^2 u0 (mod U)`.
    let p = f.p;
    let u0 = f.from_fp(u_fp[0]);
    let u1 = f.from_fp(u_fp[1]);
    let c1_u1 = f.mul(&c1, &u1);
    let num0 = f.sub(&c0, &c1_u1);
    let num1 = f.neg(&c1);
    let det = f.add(
        &f.sub(&f.sq(&c0), &f.mul(&c0, &c1_u1)),
        &f.mul(&f.sq(&c1), &u0),
    );
    if det == E2::ZERO {
        return Err("non-invertible linear denominator mod U");
    }
    // Invert `det = a + t b` in `F_{p^2}` via `inv_fp[a^2 - w b^2]` (`0` `FieldInv`s!).
    let a = det.0[0];
    let b = det.0[1];
    let norm = sm(mm(a, a, p), mm(f.w % p, mm(b, b, p), p), p);
    if norm == 0 {
        return Err("singular F_{p^2} norm");
    }
    let inv_n = table_inv(inv_fp, norm);
    let det_inv = E2([
        mm(a, inv_n, p),
        mm(if b == 0 { 0 } else { sm(p, b, p) }, inv_n, p),
    ]);
    Ok([f.mul(&num0, &det_inv), f.mul(&num1, &det_inv)])
}

#[inline]
fn mul_linear_e2_mod_quad_stack(f: &Fq, a: &[E2; 2], b: &[E2; 2], u_fp: &[u64; 3]) -> [E2; 2] {
    let c0 = f.mul(&a[0], &b[0]);
    let c1 = f.add(&f.mul(&a[0], &b[1]), &f.mul(&a[1], &b[0]));
    let c2 = f.mul(&a[1], &b[1]);
    rem_e2_mod_quad_stack(f, &[c0, c1, c2], u_fp)
}

#[inline]
fn eval_e2_horner_fp(f: &Fq, poly: &[E2], x: u64) -> E2 {
    if poly.is_empty() {
        return E2::ZERO;
    }
    let xe = f.from_fp(x);
    let mut acc = poly[poly.len() - 1];
    for i in (0..(poly.len() - 1)).rev() {
        acc = f.add(&f.mul(&acc, &xe), &poly[i]);
    }
    acc
}

#[inline]
fn canonical_mumford_stack(f: &Fq, u_fp: &[u64; 3], v: &[E2; 2]) -> ([u64; 6], i8) {
    let na = f.neg(&v[0]);
    let nb = f.neg(&v[1]);
    let key = [u_fp[0], u_fp[1], v[0].0[0], v[0].0[1], v[1].0[0], v[1].0[1]];
    let neg = [u_fp[0], u_fp[1], na.0[0], na.0[1], nb.0[0], nb.0[1]];
    work::words(18);
    if neg < key {
        (neg, -1)
    } else {
        (key, 1)
    }
}

/// Stack-array, Stickelberger-screened, zero-`FieldInv` (`inv_fp` table), deferred-ordinate
/// `2-LP` source extractor producing bit-for-bit identical `ReferencedSource` certificates!
#[allow(clippy::too_many_arguments)]
pub fn extract_2lp_source_fast(
    ctx: &Ctx,
    config: &TwoLargePrimeSieve,
    base: &[BaseEl],
    base_by_x: &[u32],
    inv_fp: &[u64],
    line: &Line,
    s: u64,
    raw_xs: &[u16],
    screen_ctx: Option<&discriminant::Context<'_>>,
    screen_calls: &mut u64,
    screen_refusals: &mut u64,
) -> Result<
    crate::cryptanalysis::jv_sieve::two_large_prime::extension_anchored_dlp::ReferencedSource,
    &'static str,
> {
    use crate::cryptanalysis::jv_sieve::two_large_prime::extension_anchored_dlp::ReferencedSource;

    let cnt = raw_xs.len();
    if !matches!(cnt, 5 | 7 | 9) {
        return Err("hit count outside {5, 7, 9}");
    }
    let f = &ctx.f.f;
    let p = f.p;
    if s == 0 || s >= p {
        return Err("zero/out-of-range sieve parameter");
    }

    let mut xs_sorted = [0u16; 9];
    xs_sorted[..cnt].copy_from_slice(raw_xs);
    xs_sorted[..cnt].sort_unstable();
    work::words((cnt * 3) as u64);

    let mut hit_indices = [0usize; 9];
    for k in 0..cnt {
        work::words(1);
        let idx = *base_by_x.get(xs_sorted[k] as usize).unwrap_or(&u32::MAX);
        if idx == u32::MAX {
            return Err("hit x not in factor base");
        }
        hit_indices[k] = idx as usize;
    }

    // Build `norm(x) = P_1(x) + s^2 P_2(x) - s P_3(x)` directly in stack `[u64; 10]`.
    let s2 = mm(s, s, p);
    let max_len = line.p1.len().max(line.p2.len()).max(line.p3.len());
    if max_len != 10 {
        return Err("degree nine required");
    }
    let mut norm_arr = [0u64; 10];
    for (i, slot) in norm_arr.iter_mut().enumerate() {
        let v1 = line.p1.get(i).copied().unwrap_or(0);
        let v2 = line.p2.get(i).copied().unwrap_or(0);
        let v3 = line.p3.get(i).copied().unwrap_or(0);
        let t12 = if v2 == 0 {
            v1
        } else {
            am(v1, mm(s2, v2, p), p)
        };
        *slot = if v3 == 0 {
            t12
        } else {
            sm(t12, mm(s, v3, p), p)
        };
    }
    if norm_arr[9] == 0 {
        return Err("degree nine required");
    }

    let mut tail = StackPoly {
        len: 10,
        c: {
            let mut c = [0u64; 18];
            c[..10].copy_from_slice(&norm_arr);
            c
        },
    };
    for &idx in &hit_indices[..cnt] {
        let bx = base[idx].x;
        if !tail.synthetic_div(bx, p) {
            return Err("root division");
        }
    }

    let setup = config.setup();
    let mut factors_buf = [[0u64; 3]; 2];
    let n_factors: usize = match tail.deg() {
        0 => 0,
        2 => {
            if tail.c[2] == 0 {
                return Err("degenerate quadratic");
            }
            let q2 = tail.monic(p, inv_fp);
            let arr = [q2.c[0], q2.c[1], 1];
            if !is_irreducible_quad_stack(&arr, p, &setup.sqrt) {
                return Err("reducible quadratic");
            }
            factors_buf[0] = arr;
            1
        }
        4 => {
            if tail.c[4] == 0 {
                return Err("degenerate quartic");
            }
            let disc_passed = if let Some(sc) = screen_ctx {
                *screen_calls += 1;
                let assessment = sc.screen(&tail.c[..5])?;
                if assessment.rejects() {
                    *screen_refusals += 1;
                    return Err("quartic discriminant necessary condition");
                }
                true
            } else {
                false
            };
            let pair = split_quartic_stickelberger_stack(
                &tail.c[..5],
                p,
                inv_fp,
                &setup.sqrt,
                disc_passed,
            )?;
            factors_buf = pair;
            2
        }
        _ => return Err("unexpected tail degree"),
    };

    // Only accepted tails (~14% of hits!) evaluate `B(bx)`, build `(A, C)`, and compute ordinates!
    let sq = if setup.sqrt[s as usize] != u64::MAX {
        E2([setup.sqrt[s as usize], 0])
    } else {
        let r = setup.sqrt[mm(s, setup.winv, p) as usize];
        if r == u64::MAX {
            return Err("square root table");
        }
        E2([0, r])
    };
    let a_len = line.a0.len().max(line.a1.len());
    let mut a_poly = Vec::with_capacity(a_len);
    for i in 0..a_len {
        a_poly.push(E2([
            line.a0.get(i).copied().unwrap_or(0),
            mm(s, line.a1.get(i).copied().unwrap_or(0), p),
        ]));
    }
    let a_trim = remainder::trim(a_poly);
    let c_poly: Vec<E2> = line.b.iter().map(|bz| f.mul(&sq, bz)).collect();
    let c_trim = remainder::trim(c_poly);
    if c_trim.len() != 2 || c_trim[1] == E2::ZERO {
        return Err("non-linear denominator");
    }

    let mut terms = Vec::with_capacity(cnt);
    for &idx in &hit_indices[..cnt] {
        let b_pt = &base[idx];
        let den = eval_e2_horner_fp(f, &c_trim, b_pt.x);
        if den == E2::ZERO {
            return Err("zero denominator");
        }
        let ax = eval_e2_horner_fp(f, &a_trim, b_pt.x);
        let dy = f.mul(&den, &b_pt.y);
        let sign = if f.add(&ax, &dy) == E2::ZERO {
            1
        } else if f.sub(&ax, &dy) == E2::ZERO {
            -1
        } else {
            return Err("ordinate mismatch");
        };
        terms.push((idx, sign));
    }

    let mut remainders = Vec::with_capacity(n_factors);
    for factor in &factors_buf[..n_factors] {
        let inv_c = inv_linear_e2_mod_quad_stack(f, c_trim[0], c_trim[1], factor, inv_fp)?;
        let a_mod = rem_e2_mod_quad_stack(f, &a_trim, factor);
        let prod = mul_linear_e2_mod_quad_stack(f, &a_mod, &inv_c, factor);
        let v_neg = [f.neg(&prod[0]), f.neg(&prod[1])];
        remainders.push(canonical_mumford_stack(f, factor, &v_neg));
    }
    if remainders.windows(2).any(|w| w[0].0 >= w[1].0) {
        return Err("non-distinct canonical Mumford classes");
    }

    let src = ReferencedSource {
        source_id: 0,
        terms,
        remainders,
        a: remainder::encode(&a_trim),
        c: remainder::encode(&c_trim),
        norm: norm_arr.to_vec(),
    };

    if src.remainders.is_empty() {
        let jac = ctx.jac();
        let mut acc = jac.identity();
        for &(idx, sign) in &src.terms {
            let d = if sign == 1 {
                base[idx].d.clone()
            } else {
                jac.neg(&base[idx].d)
            };
            acc = jac.add(&acc, &d);
        }
        if !jac.is_identity(&acc) {
            return Err("full Jacobian check");
        }
    }

    Ok(src)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn partition_m9_matches_lines_for_b_hint_bit_for_bit() {
        for &(p, seed) in &[(307u64, 101u64), (503, 211)] {
            let spec = crate::cryptanalysis::jv_cover::generate_spec(p, seed);
            let ctx = Ctx::new(&spec);
            let mut base_rng = StdRng::seed_from_u64(seed ^ 0xBACE_0001);
            let base = crate::cryptanalysis::jv_cover::factor_base(&ctx, &mut base_rng);
            let setup = sieve::sieve_setup(&ctx.f.f, &ctx.cov.hx, &base, 9);
            let nu = sieve::nu_lines(&setup);
            let ring = FpRing::new(p);
            let inv_fp = build_inv_table(p);
            let mut slot = Vec::new();
            let mut rng_ref = StdRng::seed_from_u64(seed ^ 0xB10C);
            let mut rng_fast = StdRng::seed_from_u64(seed ^ 0xB10C);
            let scramble = sieve::coprime_scramble(
                sieve::b_space(&setup),
                &mut StdRng::seed_from_u64(seed ^ 0x5C4A),
            );
            let batch: Vec<_> = (0..512u128)
                .map(|k| sieve::b_from_index(&setup, k, scramble))
                .collect();
            let ab: Vec<_> = batch.iter().map(|b| (b[0].0[0], b[0].0[1])).collect();
            let hints = sieve::roots_by_incidence(p, &nu, &ab, &mut slot).0;

            for (i, b) in batch.iter().enumerate() {
                let mut l_ref = sieve::lines_for_b_hint(
                    &ring,
                    &ctx.f.f,
                    &setup,
                    b,
                    &mut rng_ref,
                    true,
                    hints[i].as_deref(),
                );
                let mut l_fast = lines_for_b_partition_m9(
                    &ring,
                    &ctx.f.f,
                    &setup,
                    b,
                    &inv_fp,
                    &mut rng_fast,
                    hints[i].as_deref(),
                );
                l_ref.sort_by(|x, y| x.a0.cmp(&y.a0).then_with(|| x.a1.cmp(&y.a1)));
                l_fast.sort_by(|x, y| x.a0.cmp(&y.a0).then_with(|| x.a1.cmp(&y.a1)));
                assert_eq!(
                    l_ref.len(),
                    l_fast.len(),
                    "line count mismatch at p={p}, b_idx={i}"
                );
                for (r, f_line) in l_ref.iter().zip(l_fast.iter()) {
                    assert_eq!(r.a0, f_line.a0);
                    assert_eq!(r.a1, f_line.a1);
                    assert_eq!(r.p1, f_line.p1);
                    assert_eq!(r.p2, f_line.p2);
                    assert_eq!(r.p3, f_line.p3);
                    assert_eq!(r.p4, f_line.p4);
                }
            }
        }
    }
}
