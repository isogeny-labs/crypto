//! `crypto_lib::cryptanalysis::prime_swu_sym`: prime-field (p < 2^32) toy kernel for m = 3 point decomposition.
//!
//! * L1: simplified SWU map (RFC 9380 s6.6.2) as a deterministic factor-base generator, one
//!   atom per x-orbit {P, -P}. Every input u with Z^2 u^4 + Z u^2 != 0 yields a curve point;
//!   the map is not surjective (it reaches about 3/8 of all x-orbits when run over all of F_p).
//! * L2: Semaev's S_4 evaluated in the elementary symmetric functions (e1, e2, e3) of the three
//!   factor-base abscissae, via a parabola resultant (see `eval_s4_symmetric`), so each
//!   unordered triple is evaluated once.
//!
//! Arithmetic is u64 with u128 products, so any p < 2^32 (including 2^31 - 1) is overflow-free.
//! `decompose_target` is an exhaustive scan: C(B+2, 3) ~ B^3/6 S_4 evaluations per target. It is a
//! correctness kernel with deterministic op counts, not a sub-rho algorithm.
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct OpTrace {
    pub field_add: u64,
    pub field_mul: u64,
    pub field_inv: u64,
    pub field_exp: u64,
    pub s4_evals: u64,
    pub ec_verifies: u64,
}

impl OpTrace {
    #[inline]
    pub fn counted_ir_proxy(&self) -> u64 {
        self.field_add + 4 * self.field_mul + 40 * self.field_inv + 48 * self.field_exp
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PrimeField {
    pub p: u64,
    pub non_square_z: u64,
}

impl PrimeField {
    pub fn new(p: u64) -> Self {
        assert!(
            p > 3 && p < (1u64 << 32),
            "Prime31/u32 rung expects 3 < p < 2^32"
        );
        let mut z = 2u64;
        let f = Self { p, non_square_z: 2 };
        while z < p {
            if f.legendre(z) == -1 {
                return Self { p, non_square_z: z };
            }
            z += 1;
        }
        panic!("no quadratic non-residue found mod {}", p);
    }
    #[inline]
    pub fn add(&self, a: u64, b: u64, tr: &mut OpTrace) -> u64 {
        tr.field_add += 1;
        let s = a + b;
        if s >= self.p {
            s - self.p
        } else {
            s
        }
    }
    #[inline]
    pub fn sub(&self, a: u64, b: u64, tr: &mut OpTrace) -> u64 {
        tr.field_add += 1;
        if a >= b {
            a - b
        } else {
            self.p + a - b
        }
    }
    #[inline]
    pub fn neg(&self, a: u64) -> u64 {
        if a == 0 {
            0
        } else {
            self.p - a
        }
    }
    #[inline]
    pub fn mul(&self, a: u64, b: u64, tr: &mut OpTrace) -> u64 {
        tr.field_mul += 1;
        ((a as u128 * b as u128) % (self.p as u128)) as u64
    }
    #[inline]
    pub fn sqr(&self, a: u64, tr: &mut OpTrace) -> u64 {
        self.mul(a, a, tr)
    }
    pub fn pow(&self, mut base: u64, mut exp: u64, tr: &mut OpTrace) -> u64 {
        tr.field_exp += 1;
        let mut acc = 1u64;
        base %= self.p;
        while exp > 0 {
            if exp & 1 == 1 {
                acc = ((acc as u128 * base as u128) % (self.p as u128)) as u64;
            }
            base = ((base as u128 * base as u128) % (self.p as u128)) as u64;
            exp >>= 1;
        }
        acc
    }
    pub fn inv(&self, a: u64, tr: &mut OpTrace) -> u64 {
        assert!(a != 0, "division by zero in F_p");
        tr.field_inv += 1;
        let mut t: i128 = 0;
        let mut new_t: i128 = 1;
        let mut r: i128 = self.p as i128;
        let mut new_r: i128 = (a % self.p) as i128;
        while new_r != 0 {
            let q = r / new_r;
            let tmp_t = t - q * new_t;
            t = new_t;
            new_t = tmp_t;
            let tmp_r = r - q * new_r;
            r = new_r;
            new_r = tmp_r;
        }
        debug_assert_eq!(r, 1, "modulus p must be prime");
        if t < 0 {
            (t + self.p as i128) as u64
        } else {
            t as u64
        }
    }
    pub fn legendre(&self, a: u64) -> i8 {
        if a.is_multiple_of(self.p) {
            return 0;
        }
        let mut dummy = OpTrace::default();
        let r = self.pow(a, (self.p - 1) / 2, &mut dummy);
        if r == 1 {
            1
        } else {
            -1
        }
    }
    pub fn sqrt(&self, n: u64, tr: &mut OpTrace) -> Option<u64> {
        let n = n % self.p;
        if n == 0 {
            return Some(0);
        }
        if self.pow(n, (self.p - 1) / 2, tr) != 1 {
            return None;
        }
        let root = if self.p & 3 == 3 {
            self.pow(n, (self.p + 1) / 4, tr)
        } else {
            let mut q = self.p - 1;
            let mut s = 0u32;
            while q & 1 == 0 {
                q >>= 1;
                s += 1;
            }
            let mut c = self.pow(self.non_square_z, q, tr);
            let mut x = self.pow(n, q.div_ceil(2), tr);
            let mut t = self.pow(n, q, tr);
            let mut m = s;
            while t != 1 {
                let mut i = 1u32;
                let mut tt = self.sqr(t, tr);
                while tt != 1 {
                    tt = self.sqr(tt, tr);
                    i += 1;
                }
                let b = self.pow(c, 1u64 << (m - i - 1), tr);
                x = self.mul(x, b, tr);
                c = self.sqr(b, tr);
                t = self.mul(t, c, tr);
                m = i;
            }
            x
        };
        let neg_root = self.p - root;
        Some(root.min(neg_root))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Point {
    Infinity,
    Affine { x: u64, y: u64 },
}

#[derive(Clone, Copy, Debug)]
pub struct ShortWeierstrass {
    pub f: PrimeField,
    pub a: u64,
    pub b: u64,
    /// Precomputed 2a, 4b, 8b (curve constants; not charged per S_4 evaluation).
    two_a: u64,
    four_b: u64,
    eight_b: u64,
}

impl ShortWeierstrass {
    pub fn new(p: u64, a: u64, b: u64) -> Self {
        let f = PrimeField::new(p);
        let (a, b) = (a % p, b % p);
        let mut t = OpTrace::default();
        let disc = f.add(
            f.mul(4, f.mul(f.sqr(a, &mut t), a, &mut t), &mut t),
            f.mul(27, f.sqr(b, &mut t), &mut t),
            &mut t,
        );
        assert!(disc != 0, "singular curve: 4a^3 + 27b^2 = 0 mod p");
        let two_a = (2 * a) % p;
        let four_b = (4 * b) % p;
        let eight_b = (8 * b) % p;
        Self {
            f,
            a,
            b,
            two_a,
            four_b,
            eight_b,
        }
    }
    #[inline]
    pub fn rhs(&self, x: u64, tr: &mut OpTrace) -> u64 {
        let x2 = self.f.sqr(x, tr);
        let x3 = self.f.mul(x2, x, tr);
        let ax = self.f.mul(self.a, x, tr);
        let s = self.f.add(x3, ax, tr);
        self.f.add(s, self.b, tr)
    }
    pub fn is_on_curve(&self, pt: Point) -> bool {
        match pt {
            Point::Infinity => true,
            Point::Affine { x, y } => {
                let mut dummy = OpTrace::default();
                self.f.sqr(y, &mut dummy) == self.rhs(x, &mut dummy)
            }
        }
    }
    #[inline]
    pub fn negate(&self, pt: Point) -> Point {
        match pt {
            Point::Infinity => Point::Infinity,
            Point::Affine { x, y } => Point::Affine {
                x,
                y: self.f.neg(y),
            },
        }
    }
    pub fn add(&self, p1: Point, p2: Point, tr: &mut OpTrace) -> Point {
        match (p1, p2) {
            (Point::Infinity, q) => q,
            (p, Point::Infinity) => p,
            (Point::Affine { x: x1, y: y1 }, Point::Affine { x: x2, y: y2 }) => {
                if x1 == x2 {
                    if (y1 + y2) % self.f.p == 0 {
                        return Point::Infinity;
                    }
                    if y1 == 0 {
                        return Point::Infinity;
                    }
                    let x1_sq = self.f.sqr(x1, tr);
                    let three_x1_sq = self.f.mul(3, x1_sq, tr);
                    let num = self.f.add(three_x1_sq, self.a, tr);
                    let den = self.f.add(y1, y1, tr);
                    let lam = self.f.mul(num, self.f.inv(den, tr), tr);
                    let lam2 = self.f.sqr(lam, tr);
                    let two_x1 = self.f.add(x1, x1, tr);
                    let x3 = self.f.sub(lam2, two_x1, tr);
                    let dx = self.f.sub(x1, x3, tr);
                    let ldx = self.f.mul(lam, dx, tr);
                    let y3 = self.f.sub(ldx, y1, tr);
                    Point::Affine { x: x3, y: y3 }
                } else {
                    let num = self.f.sub(y2, y1, tr);
                    let den = self.f.sub(x2, x1, tr);
                    let lam = self.f.mul(num, self.f.inv(den, tr), tr);
                    let lam2 = self.f.sqr(lam, tr);
                    let s12 = self.f.add(x1, x2, tr);
                    let x3 = self.f.sub(lam2, s12, tr);
                    let dx = self.f.sub(x1, x3, tr);
                    let ldx = self.f.mul(lam, dx, tr);
                    let y3 = self.f.sub(ldx, y1, tr);
                    Point::Affine { x: x3, y: y3 }
                }
            }
        }
    }
    /// Simplified SWU (RFC 9380 s6.6.2), a != 0, b != 0, any non-square Z. Returns None only in
    /// the exceptional case Z^2 u^4 + Z u^2 = 0. y is canonicalised to min(y, p - y).
    pub fn swu_orbit_point(&self, u: u64, tr: &mut OpTrace) -> Option<Point> {
        assert!(
            self.a != 0 && self.b != 0,
            "Simplified SWU requires a != 0, b != 0"
        );
        let z = self.f.non_square_z;
        let u2 = self.f.sqr(u % self.f.p, tr);
        let tv1 = self.f.mul(z, u2, tr);
        let tv2 = self.f.sqr(tv1, tr);
        let denom = self.f.add(tv2, tv1, tr);
        if denom == 0 {
            return None;
        }
        let inv_d = self.f.inv(denom, tr);
        let one_plus = self.f.add(1, inv_d, tr);
        let neg_b_inv_a = self.f.mul(self.f.neg(self.b), self.f.inv(self.a, tr), tr);
        let x1 = self.f.mul(neg_b_inv_a, one_plus, tr);
        let gx1 = self.rhs(x1, tr);
        if let Some(y1) = self.f.sqrt(gx1, tr) {
            return Some(Point::Affine { x: x1, y: y1 });
        }
        let x2 = self.f.mul(tv1, x1, tr);
        let gx2 = self.rhs(x2, tr);
        let y2 = self
            .f
            .sqrt(gx2, tr)
            .expect("SWU invariant: g(x1) or g(x2) MUST be square");
        Some(Point::Affine { x: x2, y: y2 })
    }
    /// First `target_len` distinct x-orbits hit by u = 1, 2, ... (fewer if F_p is exhausted).
    pub fn build_swu_orbit_factor_base(&self, target_len: usize, tr: &mut OpTrace) -> Vec<Point> {
        let mut fb = Vec::with_capacity(target_len);
        let mut seen_x = HashSet::with_capacity(target_len * 2);
        let mut u = 1u64;
        while fb.len() < target_len && u < self.f.p {
            if let Some(pt @ Point::Affine { x, .. }) = self.swu_orbit_point(u, tr) {
                if seen_x.insert(x) {
                    fb.push(pt);
                }
            }
            u += 1;
        }
        fb
    }
    /// A nonzero constant multiple of Semaev's S_4(x1, x2, x3, xr), written in
    /// e1 = x1+x2+x3, e2 = x1x2+x2x3+x3x1, e3 = x1x2x3.
    ///
    /// Derivation. For four affine points with P1+P2+P3+P4 = O and no vertical pair, the divisor
    /// P1+..+P4 - 4O is that of y - (alpha x^2 + beta x + gamma) with alpha != 0, so
    /// (alpha x^2 + beta x + gamma)^2 - (x^3 + a x + b) = alpha^2 prod (x - x_i).
    /// With s1..s4 the elementary symmetric functions of (x1, x2, x3, xr) and u = beta/alpha,
    /// v = gamma/alpha, w = 1/alpha^2, comparing coefficients gives w = 2u + s1,
    /// 2v = s2 - u^2 and
    ///   A(u) = u^3 + (2a - s2) u + (a s1 - s3) = 0,
    ///   B(u) = u^4 - 2 s2 u^2 - 8b u + (s2^2 - 4b s1 - 4 s4) = 0.
    /// We return Res_u(A, B) = Res_u(A, B - uA), expanded division-free. Checked numerically
    /// (tests below) to equal c * Res_u(S_3(x1,x2,u), S_3(x3,xr,u)) for one constant c, both against
    /// [`crate::cryptanalysis::semaev_higher::s4_in_last_proper`] and an inline Sylvester determinant, at
    /// p = 10007, 10009, 2^31 - 1 and 2147483629; this is evidence, not a symbolic proof.
    /// Cost: 25 field multiplications, 21 additions/subtractions per call (negations uncounted).
    #[inline]
    pub fn eval_s4_symmetric(&self, e1: u64, e2: u64, e3: u64, xr: u64, tr: &mut OpTrace) -> u64 {
        tr.s4_evals += 1;
        let f = &self.f;
        let s1 = f.add(e1, xr, tr);
        let e1_xr = f.mul(e1, xr, tr);
        let s2 = f.add(e2, e1_xr, tr);
        let e2_xr = f.mul(e2, xr, tr);
        let s3 = f.add(e3, e2_xr, tr);
        let s4 = f.mul(e3, xr, tr);
        // A(u) = u^3 + p1 u + p0
        let p1 = f.sub(self.two_a, s2, tr);
        let a_s1 = f.mul(self.a, s1, tr);
        let p0 = f.sub(a_s1, s3, tr);
        // B(u) = u^4 + q2 u^2 + q1 u + q0
        let two_s2 = f.add(s2, s2, tr);
        let q2 = f.neg(two_s2);
        let q1 = f.neg(self.eight_b);
        let s2_sq = f.sqr(s2, tr);
        let four_b_s1 = f.mul(self.four_b, s1, tr);
        let two_s4 = f.add(s4, s4, tr);
        let four_s4 = f.add(two_s4, two_s4, tr);
        let q0_tmp = f.sub(s2_sq, four_b_s1, tr);
        let q0 = f.sub(q0_tmp, four_s4, tr);
        // R(u) = B(u) - u A(u) = r2 u^2 + r1 u + r0;  Res(A, R) = r2^3 A(u_1) A(u_2) over roots of R.
        let r2 = f.sub(q2, p1, tr);
        let r1 = f.sub(q1, p0, tr);
        let r0 = q0;
        let r0_sq = f.sqr(r0, tr);
        let t1 = f.mul(r0_sq, r0, tr);
        let r1_sq = f.sqr(r1, tr);
        let r0_r2 = f.mul(r0, r2, tr);
        let two_r0_r2 = f.add(r0_r2, r0_r2, tr);
        let u12_sq = f.sub(r1_sq, two_r0_r2, tr);
        let p1_r0 = f.mul(p1, r0, tr);
        let t2 = f.mul(p1_r0, u12_sq, tr);
        let r1_cu = f.mul(r1_sq, r1, tr);
        let three_r0_r2 = f.add(two_r0_r2, r0_r2, tr);
        let three_r0_r1_r2 = f.mul(three_r0_r2, r1, tr);
        let u12_cu = f.sub(three_r0_r1_r2, r1_cu, tr);
        let t3 = f.mul(p0, u12_cu, tr);
        let r2_sq = f.sqr(r2, tr);
        let p1_sq = f.sqr(p1, tr);
        let p1_sq_r0 = f.mul(p1_sq, r0, tr);
        let t4 = f.mul(p1_sq_r0, r2_sq, tr);
        let p0_p1 = f.mul(p0, p1, tr);
        let r1_r2_sq = f.mul(r1, r2_sq, tr);
        let t5_pos = f.mul(p0_p1, r1_r2_sq, tr);
        let p0_sq = f.sqr(p0, tr);
        let r2_cu = f.mul(r2_sq, r2, tr);
        let t6 = f.mul(p0_sq, r2_cu, tr);
        let acc1 = f.add(t1, t2, tr);
        let acc2 = f.add(acc1, t3, tr);
        let acc3 = f.add(acc2, t4, tr);
        let acc4 = f.sub(acc3, t5_pos, tr);
        f.add(acc4, t6, tr)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SymPairState {
    pub i: u32,
    pub j: u32,
    pub sum_ij: u64,
    pub prod_ij: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SignedDecomposition3 {
    pub indices: [usize; 3],
    pub signs: [i8; 3],
}

pub struct SymmetricM3Table {
    pub pairs: Vec<SymPairState>,
    pub fb_x: Vec<u64>,
}

impl SymmetricM3Table {
    pub fn build(curve: &ShortWeierstrass, fb: &[Point], tr: &mut OpTrace) -> Self {
        let fb_x: Vec<u64> = fb
            .iter()
            .map(|pt| match *pt {
                Point::Affine { x, .. } => x,
                Point::Infinity => panic!("Infinity not allowed as factor-base atom"),
            })
            .collect();
        let n = fb_x.len();
        let mut pairs = Vec::with_capacity(n * (n + 1) / 2);
        for i in 0..n {
            for j in i..n {
                let sum_ij = curve.f.add(fb_x[i], fb_x[j], tr);
                let prod_ij = curve.f.mul(fb_x[i], fb_x[j], tr);
                pairs.push(SymPairState {
                    i: i as u32,
                    j: j as u32,
                    sum_ij,
                    prod_ij,
                });
            }
        }
        Self { pairs, fb_x }
    }
    /// Scans i <= j <= k once each; on S_4 = 0, recovers signs by explicit point addition.
    pub fn decompose_target(
        &self,
        curve: &ShortWeierstrass,
        fb: &[Point],
        target: Point,
        tr: &mut OpTrace,
    ) -> Option<SignedDecomposition3> {
        let Point::Affine { x: xr, .. } = target else {
            return None;
        };
        let n = self.fb_x.len();
        {
            for pair in &self.pairs {
                let i = pair.i as usize;
                let j = pair.j as usize;
                for k in j..n {
                    let xk = self.fb_x[k];
                    let e1 = curve.f.add(pair.sum_ij, xk, tr);
                    let s_xk = curve.f.mul(pair.sum_ij, xk, tr);
                    let e2 = curve.f.add(pair.prod_ij, s_xk, tr);
                    let e3 = curve.f.mul(pair.prod_ij, xk, tr);
                    if curve.eval_s4_symmetric(e1, e2, e3, xr, tr) == 0 {
                        if let Some(decomp) = Self::recover_signs(curve, fb, [i, j, k], target, tr)
                        {
                            return Some(decomp);
                        }
                    }
                }
            }
        }
        None
    }
    fn recover_signs(
        curve: &ShortWeierstrass,
        fb: &[Point],
        idxs: [usize; 3],
        target: Point,
        tr: &mut OpTrace,
    ) -> Option<SignedDecomposition3> {
        tr.ec_verifies += 1;
        let p0 = fb[idxs[0]];
        let p1 = fb[idxs[1]];
        let p2 = fb[idxs[2]];
        for s0 in [1i8, -1i8] {
            let q0 = if s0 == 1 { p0 } else { curve.negate(p0) };
            for s1 in [1i8, -1i8] {
                let q1 = if s1 == 1 { p1 } else { curve.negate(p1) };
                let q01 = curve.add(q0, q1, tr);
                for s2 in [1i8, -1i8] {
                    let q2 = if s2 == 1 { p2 } else { curve.negate(p2) };
                    let sum = curve.add(q01, q2, tr);
                    if sum == target {
                        return Some(SignedDecomposition3 {
                            indices: idxs,
                            signs: [s0, s1, s2],
                        });
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_swu_and_symmetric_s4_parabola_resultant() {
        let curve = ShortWeierstrass::new(10007, 3, 5);
        let mut tr = OpTrace::default();
        let fb = curve.build_swu_orbit_factor_base(24, &mut tr);
        assert_eq!(fb.len(), 24);
        for &pt in &fb {
            assert!(curve.is_on_curve(pt));
        }
        let f2 = fb[2];
        let neg_f7 = curve.negate(fb[7]);
        let f15 = fb[15];
        let r01 = curve.add(f2, neg_f7, &mut tr);
        let target = curve.add(r01, f15, &mut tr);
        let table = SymmetricM3Table::build(&curve, &fb, &mut tr);
        let mut solve_tr = OpTrace::default();
        let found = table
            .decompose_target(&curve, &fb, target, &mut solve_tr)
            .expect("must recover planted 3-decomposition");
        let mut acc = Point::Infinity;
        for (&idx, &sgn) in found.indices.iter().zip(found.signs.iter()) {
            let term = if sgn == 1 {
                fb[idx]
            } else {
                curve.negate(fb[idx])
            };
            acc = curve.add(acc, term, &mut solve_tr);
        }
        assert_eq!(acc, target);
    }

    struct Lcg(u64);
    impl Lcg {
        fn next(&mut self) -> u64 {
            self.0 = self
                .0
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            self.0 >> 11
        }
    }

    fn rand_point(c: &ShortWeierstrass, rng: &mut Lcg) -> Point {
        let mut t = OpTrace::default();
        loop {
            let x = rng.next() % c.f.p;
            if let Some(y) = c.f.sqrt(c.rhs(x, &mut t), &mut t) {
                if y != 0 {
                    let y = if rng.next() & 1 == 1 { c.f.neg(y) } else { y };
                    return Point::Affine { x, y };
                }
            }
        }
    }

    fn xof(p: Point) -> u64 {
        match p {
            Point::Affine { x, .. } => x,
            Point::Infinity => unreachable!(),
        }
    }

    fn s4_of(c: &ShortWeierstrass, x: [u64; 3], xr: u64) -> u64 {
        let mut t = OpTrace::default();
        let f = &c.f;
        let e1 = f.add(f.add(x[0], x[1], &mut t), x[2], &mut t);
        let p01 = f.mul(x[0], x[1], &mut t);
        let e2 = f.add(
            f.add(p01, f.mul(x[1], x[2], &mut t), &mut t),
            f.mul(x[0], x[2], &mut t),
            &mut t,
        );
        let e3 = f.mul(p01, x[2], &mut t);
        c.eval_s4_symmetric(e1, e2, e3, xr, &mut t)
    }

    /// Reference S_4 = Res_u(S_3(x1,x2,u), S_3(x3,x4,u)) by a 4x4 Sylvester determinant, with
    /// S_3(x1,x2,u) = (x1-x2)^2 u^2 - 2((x1+x2)(x1x2+a) + 2b) u + (x1x2-a)^2 - 4b(x1+x2).
    fn semaev_s4(c: &ShortWeierstrass, x: [u64; 4]) -> u64 {
        let f = &c.f;
        let mut t = OpTrace::default();
        let s3 = |x1: u64, x2: u64, t: &mut OpTrace| -> [u64; 3] {
            let d = f.sub(x1, x2, t);
            let s = f.add(x1, x2, t);
            let pr = f.mul(x1, x2, t);
            let mid = f.add(f.mul(s, f.add(pr, c.a, t), t), f.add(c.b, c.b, t), t);
            let c1 = f.neg(f.add(mid, mid, t));
            let q = f.sub(pr, c.a, t);
            [
                f.sqr(d, t),
                c1,
                f.sub(f.sqr(q, t), f.mul(f.mul(4, c.b, t), s, t), t),
            ]
        };
        let [a2, a1, a0] = s3(x[0], x[1], &mut t);
        let [b2, b1, b0] = s3(x[2], x[3], &mut t);
        let mut m = [
            [a2, a1, a0, 0],
            [0, a2, a1, a0],
            [b2, b1, b0, 0],
            [0, b2, b1, b0],
        ];
        let mut det = 1u64;
        for col in 0..4 {
            let Some(pr) = (col..4).find(|&r| m[r][col] != 0) else {
                return 0;
            };
            if pr != col {
                m.swap(pr, col);
                det = f.neg(det);
            }
            det = f.mul(det, m[col][col], &mut t);
            let inv = f.inv(m[col][col], &mut t);
            for r in col + 1..4 {
                let k = f.mul(m[r][col], inv, &mut t);
                for cc in col..4 {
                    let v = f.mul(k, m[col][cc], &mut t);
                    m[r][cc] = f.sub(m[r][cc], v, &mut t);
                }
            }
        }
        det
    }

    /// The parabola resultant is one fixed multiple of Semaev's S_4, vanishes on every planted
    /// relation (including the vertical-pair locus x1 = x2, x3 = xr), and has no false zeros.
    #[test]
    fn parabola_resultant_matches_semaev_s4() {
        for &(p, a, b) in &[
            (10007u64, 3u64, 5u64),
            (10009, 3, 5),
            (2147483647, 7, 11),
            (2147483629, 7, 11),
        ] {
            let c = ShortWeierstrass::new(p, a, b);
            let mut rng = Lcg(p);
            let mut t = OpTrace::default();
            let mut ratio = None;
            for _ in 0..300 {
                let xs = [
                    rng.next() % p,
                    rng.next() % p,
                    rng.next() % p,
                    rng.next() % p,
                ];
                let sv = semaev_s4(&c, xs);
                let pv = s4_of(&c, [xs[0], xs[1], xs[2]], xs[3]);
                if sv == 0 {
                    assert_eq!(pv, 0);
                    continue;
                }
                let r = c.f.mul(pv, c.f.inv(sv, &mut t), &mut t);
                assert_ne!(r, 0);
                assert_eq!(
                    *ratio.get_or_insert(r),
                    r,
                    "not a constant multiple at p = {p}"
                );
            }
            for _ in 0..500 {
                let ps = [
                    rand_point(&c, &mut rng),
                    rand_point(&c, &mut rng),
                    rand_point(&c, &mut rng),
                ];
                let r = c.add(c.add(ps[0], c.negate(ps[1]), &mut t), ps[2], &mut t);
                if r != Point::Infinity {
                    assert_eq!(s4_of(&c, [xof(ps[0]), xof(ps[1]), xof(ps[2])], xof(r)), 0);
                }
                let q = rand_point(&c, &mut rng);
                assert_eq!(
                    s4_of(&c, [xof(ps[0]), xof(ps[0]), xof(q)], xof(q)),
                    0,
                    "vertical pair"
                );
            }
        }
    }

    #[test]
    fn s4_eval_cost_is_fixed() {
        let c = ShortWeierstrass::new(2147483647, 7, 11);
        let mut tr = OpTrace::default();
        c.eval_s4_symmetric(1, 2, 3, 4, &mut tr);
        assert_eq!(
            (tr.field_mul, tr.field_add, tr.field_inv, tr.field_exp),
            (25, 21, 0, 0)
        );
    }

    #[test]
    fn swu_prime31_and_p_1_mod_4() {
        for &(p, a, b) in &[(2147483647u64, 7u64, 11u64), (2147483629, 7, 11)] {
            let c = ShortWeierstrass::new(p, a, b);
            let mut t = OpTrace::default();
            let fb = c.build_swu_orbit_factor_base(2000, &mut t);
            assert_eq!(fb.len(), 2000);
            assert!(fb.iter().all(|&q| c.is_on_curve(q)));
        }
    }

    /// Cross-check against the repository's own S_4 (`semaev_higher::s4_in_last_proper`).
    #[test]
    fn matches_semaev_higher_s4_up_to_constant() {
        use crate::cryptanalysis::semaev_higher::s4_in_last_proper;
        use crate::ecc::field::FieldElement;
        use num_bigint::BigUint;
        use num_traits::ToPrimitive;
        for &(p, a, b) in &[(10007u64, 3u64, 5u64), (2147483647, 7, 11)] {
            let c = ShortWeierstrass::new(p, a, b);
            let fe = |v: u64| FieldElement::new(BigUint::from(v), BigUint::from(p));
            let mut rng = Lcg(p ^ 0x5eed);
            let mut t = OpTrace::default();
            let mut ratio = None;
            for _ in 0..100 {
                let xs = [
                    rng.next() % p,
                    rng.next() % p,
                    rng.next() % p,
                    rng.next() % p,
                ];
                let reference =
                    s4_in_last_proper(&fe(xs[0]), &fe(xs[1]), &fe(xs[2]), &fe(a), &fe(b))
                        .eval(&fe(xs[3]))
                        .value
                        .to_u64()
                        .unwrap();
                let ours = s4_of(&c, [xs[0], xs[1], xs[2]], xs[3]);
                if reference == 0 {
                    assert_eq!(ours, 0);
                    continue;
                }
                let r = c.f.mul(ours, c.f.inv(reference, &mut t), &mut t);
                assert_ne!(r, 0);
                assert_eq!(
                    *ratio.get_or_insert(r),
                    r,
                    "not a constant multiple at p = {p}"
                );
            }
        }
    }
}
