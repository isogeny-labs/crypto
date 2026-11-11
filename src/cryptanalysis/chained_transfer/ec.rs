//! Elliptic curves `y² = x³ + a2·x² + a4·x + a6` over `F_p`, with points over
//! `F_{p²}` (the transfer maps need conjugate pairs of points).

use super::field::{Fp, F2};

/// A point; `None` is the point at infinity.
pub type Pt = Option<(F2, F2)>;

/// `y² = x³ + a2 x² + a4 x + a6` over `F_p`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Curve {
    pub k: Fp,
    pub a2: u64,
    pub a4: u64,
    pub a6: u64,
}

impl Curve {
    /// The curve, or `None` when it is singular.
    pub fn new(k: Fp, a2: u64, a4: u64, a6: u64) -> Option<Curve> {
        let c = Curve {
            k,
            a2: a2 % k.p,
            a4: a4 % k.p,
            a6: a6 % k.p,
        };
        if c.discriminant() == 0 {
            None
        } else {
            Some(c)
        }
    }

    /// Discriminant of the cubic (zero iff singular).
    pub fn discriminant(&self) -> u64 {
        // disc(x³+a x²+b x+c) = a²b² − 4b³ − 4a³c − 27c² + 18abc
        let k = &self.k;
        let (a, b, c) = (self.a2, self.a4, self.a6);
        let t1 = k.mul(k.mul(a, a), k.mul(b, b));
        let t2 = k.mul(4, k.mul(b, k.mul(b, b)));
        let t3 = k.mul(4, k.mul(k.mul(a, k.mul(a, a)), c));
        let t4 = k.mul(27, k.mul(c, c));
        let t5 = k.mul(18, k.mul(a, k.mul(b, c)));
        k.add(k.sub(k.sub(k.sub(t1, t2), t3), t4), t5)
    }

    /// j-invariant (as an element of `F_p`).
    pub fn j_invariant(&self) -> u64 {
        // b2 = 4 a2, b4 = 2 a4, b6 = 4 a6; c4 = b2² − 24 b4; Δ = 16·disc
        let k = &self.k;
        let b2 = k.mul(4, self.a2);
        let b4 = k.mul(2, self.a4);
        let c4 = k.sub(k.mul(b2, b2), k.mul(24, b4));
        let delta = k.mul(16, self.discriminant());
        k.mul(k.mul(c4, k.mul(c4, c4)), k.inv(delta))
    }

    /// `x³ + a2 x² + a4 x + a6` at `x ∈ F_{p²}`.
    pub fn rhs(&self, x: F2) -> F2 {
        let k = &self.k;
        let mut r = k.add2(x, k.e(self.a2));
        r = k.add2(k.mul2(r, x), k.e(self.a4));
        k.add2(k.mul2(r, x), k.e(self.a6))
    }

    pub fn on_curve(&self, p: &Pt) -> bool {
        match p {
            None => true,
            Some((x, y)) => self.k.sqr2(*y) == self.rhs(*x),
        }
    }

    /// Whether the point is `F_p`-rational.
    pub fn rational(p: &Pt) -> bool {
        p.map_or(true, |(x, y)| x.in_base() && y.in_base())
    }

    pub fn neg(&self, p: &Pt) -> Pt {
        p.map(|(x, y)| (x, self.k.neg2(y)))
    }

    pub fn add(&self, p: &Pt, q: &Pt) -> Pt {
        let k = &self.k;
        let (x1, y1) = match p {
            None => return *q,
            Some(v) => *v,
        };
        let (x2, y2) = match q {
            None => return *p,
            Some(v) => *v,
        };
        let m = if x1 == x2 {
            if k.add2(y1, y2).is_zero() {
                return None;
            }
            // (3x² + 2 a2 x + a4) / 2y
            let num = k.add2(
                k.add2(
                    k.mul2(k.e(3), k.sqr2(x1)),
                    k.mul2(k.e(2 * self.a2 % k.p), x1),
                ),
                k.e(self.a4),
            );
            k.mul2(num, k.inv2(k.add2(y1, y1)))
        } else {
            k.mul2(k.sub2(y2, y1), k.inv2(k.sub2(x2, x1)))
        };
        let x3 = k.sub2(k.sub2(k.sub2(k.sqr2(m), k.e(self.a2)), x1), x2);
        let y3 = k.sub2(k.mul2(m, k.sub2(x1, x3)), y1);
        Some((x3, y3))
    }

    pub fn mul(&self, p: &Pt, mut n: u128) -> Pt {
        let mut acc = None;
        let mut b = *p;
        while n > 0 {
            if n & 1 == 1 {
                acc = self.add(&acc, &b);
            }
            b = self.add(&b, &b);
            n >>= 1;
        }
        acc
    }

    /// `[n]P` for a signed `n`.
    pub fn mul_i(&self, p: &Pt, n: i128) -> Pt {
        if n < 0 {
            self.neg(&self.mul(p, n.unsigned_abs()))
        } else {
            self.mul(p, n as u128)
        }
    }

    /// `#E(F_p)` by the Legendre sum (`O(p)`).
    pub fn order(&self) -> u64 {
        let k = &self.k;
        let mut n: i64 = 1 + k.p as i64;
        for x in 0..k.p {
            n += k.legendre(self.rhs(F2::base(x)).a) as i64;
        }
        n as u64
    }

    /// Every `F_p`-rational affine point (small `p` only).
    pub fn rational_points(&self) -> Vec<Pt> {
        let k = &self.k;
        let mut out = vec![None];
        for x in 0..k.p {
            let r = self.rhs(F2::base(x)).a;
            if let Some(y) = k.sqrt(r) {
                out.push(Some((F2::base(x), F2::base(y))));
                if y != 0 {
                    out.push(Some((F2::base(x), F2::base(k.neg(y)))));
                }
            }
        }
        out
    }

    /// A deterministic pseudo-random rational point from a seed.
    pub fn point_from_seed(&self, seed: u64) -> Pt {
        let k = &self.k;
        let mut x = seed % k.p;
        loop {
            let r = self.rhs(F2::base(x)).a;
            if let Some(y) = k.sqrt(r) {
                let y = if (seed >> 40) & 1 == 1 { k.neg(y) } else { y };
                return Some((F2::base(x), F2::base(y)));
            }
            x = (x + 1) % k.p;
        }
    }

    /// The `F_p`-rational 2-torsion points (roots of the cubic).
    pub fn rational_two_torsion(&self) -> Vec<Pt> {
        (0..self.k.p)
            .filter(|&x| self.rhs(F2::base(x)).is_zero())
            .map(|x| Some((F2::base(x), F2::ZERO)))
            .collect()
    }
}

/// A separable isogeny with an explicit `F_p`-rational kernel, evaluated by
/// Vélu's kernel-sum formula
/// `φ(P) = (x(P) + Σ_{Q≠O} [x(P+Q) − x(Q)], y(P) + Σ_{Q≠O} [y(P+Q) − y(Q)])`,
/// valid for `a1 = a3 = 0` models; the codomain keeps `a2` and its `a4, a6`
/// are fitted from images and then checked.
#[derive(Clone, Debug)]
pub struct VeluIsogeny {
    pub domain: Curve,
    pub codomain: Curve,
    /// All kernel points except `O`.
    pub kernel: Vec<Pt>,
}

impl VeluIsogeny {
    /// The isogeny with kernel generated by `g` (a rational point of small
    /// order); `None` when the fitted codomain is inconsistent.
    pub fn from_generator(e: &Curve, g: &Pt) -> Option<VeluIsogeny> {
        let mut kernel = vec![];
        let mut q = *g;
        while q.is_some() {
            kernel.push(q);
            q = e.add(&q, g);
            if kernel.len() > 1000 {
                return None;
            }
        }
        let k = e.k;
        let mut iso = VeluIsogeny {
            domain: *e,
            codomain: *e,
            kernel,
        };
        // Fit a4', a6' from two images with distinct x (y² − x³ − a2 x² = a4 x + a6).
        let mut imgs = vec![];
        let mut s = 1u64;
        while imgs.len() < 2 && s < 4 * k.p {
            let p = e.point_from_seed(s.wrapping_mul(0x9E37_79B9_7F4A_7C15));
            s += 1;
            if let Some((x, y)) = iso.eval_raw(&p) {
                if imgs.iter().all(|&(x0, _): &(F2, F2)| x0 != x) {
                    imgs.push((x, y));
                }
            }
        }
        if imgs.len() < 2 {
            return None;
        }
        let w = |(x, y): (F2, F2)| {
            k.sub2(
                k.sqr2(y),
                k.add2(k.mul2(k.sqr2(x), x), k.mul2(k.e(e.a2), k.sqr2(x))),
            )
        };
        let (w1, w2) = (w(imgs[0]), w(imgs[1]));
        let a4 = k.mul2(k.sub2(w1, w2), k.inv2(k.sub2(imgs[0].0, imgs[1].0)));
        let a6 = k.sub2(w1, k.mul2(a4, imgs[0].0));
        if !a4.in_base() || !a6.in_base() {
            return None;
        }
        iso.codomain = Curve::new(k, e.a2, a4.a, a6.a)?;
        // Check a third batch of images lies on the codomain.
        for t in 0..16u64 {
            let p = e.point_from_seed(0xABCD + 7919 * t);
            if !iso.codomain.on_curve(&iso.eval(&p)) {
                return None;
            }
        }
        Some(iso)
    }

    fn eval_raw(&self, p: &Pt) -> Pt {
        let e = &self.domain;
        let k = &e.k;
        let (mut x, mut y) = (*p)?;
        for q in &self.kernel {
            let s = e.add(p, q);
            let (xs, ys) = s?; // P ∈ ker ⇒ φ(P) = O
            let (xq, yq) = q.unwrap();
            x = k.add2(x, k.sub2(xs, xq));
            y = k.add2(y, k.sub2(ys, yq));
        }
        Some((x, y))
    }

    /// `φ(P)`.
    pub fn eval(&self, p: &Pt) -> Pt {
        self.eval_raw(p)
    }

    /// Degree `#ker`.
    pub fn degree(&self) -> usize {
        self.kernel.len() + 1
    }

    /// Field multiplications charged per evaluation (one affine addition
    /// is charged at 1 inversion ≈ counted separately; here: additions).
    pub fn group_ops_per_eval(&self) -> usize {
        self.kernel.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group_law_and_velu_homomorphism() {
        let k = Fp::new(1009).unwrap();
        // Search for a curve whose order has a small prime factor (so a rational
        // ℓ-torsion kernel exists); a prime-order curve has no ℓ-isogeny here.
        let e = (0..200u64)
            .find_map(|s| {
                let c = Curve::new(k, 1 + s % 100, 3 + 7 * s % 400, 2 + 11 * s % 500)?;
                let n = c.order();
                if [2u64, 3, 5, 7].iter().any(|&l| n % l == 0) {
                    Some(c)
                } else {
                    None
                }
            })
            .expect("a composite-order curve over F_1009");
        let n = e.order() as u128;
        for s in 0..50u64 {
            let p = e.point_from_seed(s * 977 + 1);
            assert!(e.on_curve(&p));
            assert_eq!(e.mul(&p, n), None);
        }
        // An isogeny from every rational point of order 2, 3 or 5 we find.
        let mut found = 0;
        for s in 0..400u64 {
            let p = e.point_from_seed(s * 31 + 5);
            for l in [2u128, 3, 5] {
                if n % l != 0 {
                    continue;
                }
                let g = e.mul(&p, n / l);
                if g.is_none() || e.mul(&g, l).is_some() {
                    continue; // need a point of exact order l
                }
                let Some(iso) = VeluIsogeny::from_generator(&e, &g) else {
                    continue;
                };
                assert_eq!(iso.codomain.order() as u128, n, "isogenous ⇒ same order");
                let q = e.point_from_seed(s * 13 + 2);
                let r = e.point_from_seed(s * 17 + 3);
                assert_eq!(
                    iso.eval(&e.add(&q, &r)),
                    iso.codomain.add(&iso.eval(&q), &iso.eval(&r))
                );
                found += 1;
            }
        }
        assert!(found > 0);
    }
}
