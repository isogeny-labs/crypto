//! d2 rational interpolation, with structural common-factor detection.
//! Preconditions, derivation and exactness gate: LINEAR-PROTOCOL.md.
use crate::rational::Primitive;
use hec_native_ic::{
    fp::Fp,
    jac::Curve,
    poly::{Poly, CAP},
};

pub struct Kernel {
    pub pairs: Vec<(Poly, Poly)>,
    common: Vec<Option<usize>>,
    anchors: Poly,
    xs: Vec<u64>,
    g: usize,
}

fn quotient(c: &Curve, a: &Poly, b: &Poly, anchors: &Poly, bound: usize) -> Poly {
    let f = &c.fp;
    let l = anchors.deg() as usize;
    let bsq = b.mul(b, f);
    let mut q = [0; CAP];
    for k in (0..=bound).rev() {
        let n = l + k;
        let mut v = 0;
        for i in 0..a.n {
            if n < i {
                continue;
            }
            let j = n - i;
            if j < i || j >= a.n {
                continue;
            }
            let mut t = f.mul(a.c[i], a.c[j]);
            if i != j {
                t = f.add(t, t);
            }
            v = f.add(v, t);
        }
        for i in 0..bsq.n {
            if n >= i && n - i < c.f.n {
                v = f.sub(v, f.mul(bsq.c[i], c.f.c[n - i]));
            }
        }
        for j in k + 1..=bound {
            let index = l + k - j;
            if index < anchors.n {
                v = f.sub(v, f.mul(anchors.c[index], q[j]));
            }
        }
        q[k] = v;
    }
    Poly::from_coeffs(&q[..=bound])
}

impl Kernel {
    pub fn new(g: usize, points: &[(u64, u64)], f: &Fp) -> Self {
        let m = g + 4;
        assert!((2..=4).contains(&g));
        assert_eq!(points.len(), m);
        let xs: Vec<_> = points.iter().map(|p| p.0).collect();
        let mut anchors = Poly::one();
        for (i, &x) in xs.iter().enumerate() {
            assert!(!xs[..i].contains(&x));
            anchors = anchors.mul(&Poly::x_minus(x, f), f);
        }
        let mut basis = Vec::new();
        let mut probe = vec![vec![0; m]; m];
        let mut h = Poly::zero();
        let mut values = vec![0; m];
        for (j, &(x, y)) in points.iter().enumerate() {
            let li = anchors.div_exact(&Poly::x_minus(x, f), f);
            let w = f.mul(y, f.inv(li.eval(x, f)));
            let v = li.scale(w, f);
            h = h.add(&v, f);
            basis.push(v);
            for i in 0..m {
                probe[j][i] = f.mul(w, f.sub(xs[i], x));
                values[i] = f.add(values[i], probe[j][i]);
            }
        }
        let len = 1 << (m - 1);
        let mut pairs = vec![(Poly::zero(), Poly::zero()); len];
        let mut common = vec![None; len];
        let mut negative = vec![false; m];
        for step in 0usize..len {
            if step > 0 {
                let j = step.trailing_zeros() as usize + 1;
                let twice = basis[j].add(&basis[j], f);
                h = if negative[j] {
                    h.add(&twice, f)
                } else {
                    h.sub(&twice, f)
                };
                for i in 0..m {
                    let twice = f.add(probe[j][i], probe[j][i]);
                    values[i] = if negative[j] {
                        f.add(values[i], twice)
                    } else {
                        f.sub(values[i], twice)
                    };
                }
                negative[j] = !negative[j];
            }
            let s0 = h.c[m - 1];
            let s1 = f.sub(h.c[m - 2], f.mul(anchors.c[m - 1], s0));
            let b = Poly::from_coeffs(&[f.neg(s1), s0]);
            let square = f.mul(s0, s0);
            let mut a = [0; CAP];
            for k in 0..=m {
                let mut v = if k > 0 { f.mul(s0, h.c[k - 1]) } else { 0 };
                v = f.add(v, f.mul(f.neg(s1), h.c[k]));
                a[k] = f.sub(v, f.mul(square, anchors.c[k]));
            }
            debug_assert_eq!(a[m], 0);
            debug_assert_eq!(a[m - 1], 0);
            let a = Poly::from_coeffs(&a[..m - 1]);
            let signs = step ^ (step >> 1);
            if !b.is_zero() {
                for i in 0..m {
                    debug_assert_eq!(values[i], b.eval(xs[i], f));
                    if values[i] == 0 {
                        assert!(common[signs].is_none());
                        common[signs] = Some(i);
                    }
                }
            } else {
                assert!(a.is_zero());
            }
            pairs[signs] = (a, b);
        }
        Self {
            pairs,
            common,
            anchors,
            xs,
            g,
        }
    }

    /// Requires the constructor's points to lie on c. No generic GCD or full norm division.
    pub fn primitive(&self, c: &Curve, signs: usize) -> Option<Primitive> {
        assert_eq!(c.g, self.g);
        let f = &c.fp;
        let (mut a, mut b) = self.pairs[signs];
        if b.is_zero() {
            assert!(a.is_zero());
            return None;
        }
        let mut anchors = self.anchors;
        let mut keep_mask = (1 << self.xs.len()) - 1;
        let removed;
        if let Some(i) = self.common[signs] {
            let factor = Poly::x_minus(self.xs[i], f);
            a = a.div_exact(&factor, f);
            b = Poly::constant(b.c[1]);
            anchors = anchors.div_exact(&factor, f);
            keep_mask ^= 1 << i;
            removed = 1;
        } else {
            removed = 0;
        }
        let q = quotient(c, &a, &b, &anchors, c.g - removed);
        Some(Primitive {
            a,
            b,
            q,
            keep_mask,
            common_degree: removed,
            removed,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rational::{primitive, SignedFamily};
    use hec_native_ic::instance::{factor_base, thread_curve};
    #[test]
    fn exact_pairs_normalization_and_norm_quotients() {
        for g in 2..=4 {
            let (mut n, mut shared, mut zero, mut constant) = (0, 0, 0, 0);
            for p in [101, 127, 503] {
                let c = (1..60).find_map(|k| thread_curve(p, k, g)).unwrap();
                let fb = factor_base(&c);
                for offset in [0, 3, 6, 9] {
                    let points = &fb[offset..offset + g + 4];
                    let k = Kernel::new(g, points, &c.fp);
                    let reference = SignedFamily::new(g, 2, points, &c.fp).walsh(&c.fp);
                    for (s, &(a, b)) in reference.iter().enumerate() {
                        assert_eq!(
                            k.pairs[s],
                            (a, b),
                            "raw g={g} p={p} offset={offset} signs={s}"
                        );
                        let fast = k.primitive(&c, s);
                        let slow = primitive(&c, points, a, b);
                        match (fast, slow) {
                            (None, None) => {
                                zero += 1;
                            }
                            (Some(x), Some(y)) => {
                                assert_eq!(
                                    (x.a, x.b, x.q, x.keep_mask, x.common_degree, x.removed),
                                    (y.a, y.b, y.q, y.keep_mask, y.common_degree, y.removed)
                                );
                                shared += (x.common_degree > 0) as usize;
                                constant += (b.deg() == 0) as usize;
                            }
                            _ => panic!("zero mismatch"),
                        };
                        n += 1;
                    }
                }
            }
            assert!(shared > 0);
            println!("linear g={g} cases={n} common={shared} zero={zero} constant_denominator={constant}");
        }
    }
    #[test]
    fn zero_and_constant_denominator_controls() {
        let f = Fp::new(101);
        let mut pts: Vec<_> = (0..6).map(|x| (x, 0)).collect();
        let zero = Kernel::new(2, &pts, &f);
        assert!(zero.pairs.iter().all(|(a, b)| a.is_zero() && b.is_zero()));
        for i in 0..2 {
            let mut der = 1;
            for j in 0..6 {
                if i != j {
                    der = f.mul(der, f.sub(pts[i].0, pts[j].0));
                }
            }
            pts[i].1 = if i == 0 { der } else { f.neg(der) };
        }
        let k = Kernel::new(2, &pts, &f);
        let r = SignedFamily::new(2, 2, &pts, &f).walsh(&f);
        assert_eq!(k.pairs, r);
        assert_eq!(k.pairs[0].1.deg(), 0);
        assert!(!k.pairs[0].1.is_zero());
    }
}
