//! Private public-input workspace for the <=131-bit trace-zero solver.
//! General F2mElement storage and other algorithms are unchanged. Products and
//! reductions reuse the production word kernels; exact maps are prepared once.
use crate::binary_ecc::{
    curve::{point_add, point_neg},
    f2m::{compact_mod131_hw_available, mul_words_into, reduce_words, spread32, CompactLinearMaps},
    operation_tally, BinaryCurve, BinaryPoint, F2mElement, IrreduciblePoly,
};
pub type Element = [u64; 3];
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WordPoint {
    Infinity,
    Affine { x: Element, y: Element },
}
pub fn element(x: &F2mElement) -> Element {
    let mut a = [0; 3];
    a[..x.raw_bits().len()].copy_from_slice(x.raw_bits());
    a
}
impl WordPoint {
    pub fn from_point(p: &BinaryPoint) -> Self {
        match p {
            BinaryPoint::Infinity => Self::Infinity,
            BinaryPoint::Affine { x, y } => Self::Affine {
                x: element(x),
                y: element(y),
            },
        }
    }
    pub fn point(self, m: u32) -> BinaryPoint {
        match self {
            Self::Infinity => BinaryPoint::Infinity,
            Self::Affine { x, y } => BinaryPoint::Affine {
                x: F2mElement::from_words(&x, m),
                y: F2mElement::from_words(&y, m),
            },
        }
    }
}
pub struct WordField {
    pub m: u32,
    nw: usize,
    irr: IrreduciblePoly,
    trace_mask: Element,
    linear_maps: CompactLinearMaps,
    reduction: u8,
    hardware131: bool,
    tail: Element,
}
impl WordField {
    pub fn new(c: &BinaryCurve) -> Self {
        assert!(
            c.m <= 192 && c.m >= 3 && c.m % 2 == 1 && c.a.is_zero() && c.b == F2mElement::one(c.m)
        );
        let n = c.m as usize;
        let linear_maps = CompactLinearMaps::new(&c.irreducible);
        let trace_mask = linear_maps.trace_mask();
        let mut tail = [0; 3];
        for &t in &c.irreducible.low_terms {
            tail[t as usize / 64] |= 1u64 << (t % 64);
        }
        // Select only by the complete modulus, never by degree alone.
        let reduction = if c.m == 131 && tail == [0x2007, 0, 0] {
            131
        } else if c.m == 83 && tail == [(1u64 << 45) | 7, 0, 0] {
            83
        } else {
            0
        };
        Self {
            m: c.m,
            nw: n.div_ceil(64),
            irr: c.irreducible.clone(),
            trace_mask,
            linear_maps,
            reduction,
            hardware131: reduction == 131 && compact_mod131_hw_available(),
            tail,
        }
    }
    #[inline]
    pub fn add(&self, a: Element, b: Element) -> Element {
        operation_tally::tick(self.m, "add");
        [a[0] ^ b[0], a[1] ^ b[1], a[2] ^ b[2]]
    }
    #[inline(always)]
    pub fn mul(&self, a: Element, b: Element) -> Element {
        operation_tally::tick(self.m, "mul");
        #[cfg(target_arch = "x86_64")]
        if self.hardware131 {
            // SAFETY: private bit set only after full-modulus and ISA checks.
            return unsafe { crate::binary_ecc::f2m::mul_mod131_hw(&a, &b) };
        }
        let mut out = [0u64; 7];
        mul_words_into(&a[..self.nw], &b[..self.nw], &mut out);
        self.reduce(out)
    }
    #[inline(always)]
    pub fn square(&self, a: Element) -> Element {
        operation_tally::tick(self.m, "square");
        #[cfg(target_arch = "x86_64")]
        if self.hardware131 {
            // SAFETY: the same private full-modulus/ISA dispatch guarantee.
            return unsafe { crate::binary_ecc::f2m::square_mod131_hw(&a) };
        }
        let mut out = [0u64; 7];
        for (i, &w) in a[..self.nw].iter().enumerate() {
            out[2 * i] = spread32(w);
            out[2 * i + 1] = spread32(w >> 32);
        }
        self.reduce(out)
    }
    #[inline(always)]
    fn reduce(&self, mut p: [u64; 7]) -> Element {
        match self.reduction {
            83 => {
                // deg(product)<=164. Three whole-high-part folds suffice:
                // top degrees <=81,43,5 and tail degree45.
                let mut lo = p[0] as u128 | ((p[1] as u128) << 64);
                let hi = p[2] as u128 | ((p[3] as u128) << 64);
                let mask = (1u128 << 83) - 1;
                let mut top = (lo >> 83) | (hi << 45);
                for _ in 0..3 {
                    lo = (lo & mask) ^ top ^ (top << 1) ^ (top << 2) ^ (top << 45);
                    top = lo >> 83;
                }
                debug_assert_eq!(top, 0);
                [lo as u64, (lo >> 64) as u64, 0]
            }
            131 => {
                // product = low + z^131*s, deg(s)<=129. Fold s*(1+z+z²+z^13),
                // then its <=12-bit overflow once more; the latter ends below z^26.
                let s0 = (p[2] >> 3) | (p[3] << 61);
                let s1 = (p[3] >> 3) | (p[4] << 61);
                let s2 = (p[4] >> 3) | (p[5] << 61);
                let lo = s0 ^ (s0 << 1) ^ (s0 << 2) ^ (s0 << 13);
                let mid =
                    s1 ^ (s1 << 1) ^ (s1 << 2) ^ (s1 << 13) ^ (s0 >> 63) ^ (s0 >> 62) ^ (s0 >> 51);
                let high =
                    s2 ^ (s2 << 1) ^ (s2 << 2) ^ (s2 << 13) ^ (s1 >> 63) ^ (s1 >> 62) ^ (s1 >> 51);
                let last = (p[2] & 7) ^ high;
                let top = last >> 3;
                [
                    p[0] ^ lo ^ top ^ (top << 1) ^ (top << 2) ^ (top << 13),
                    p[1] ^ mid,
                    last & 7,
                ]
            }
            _ => {
                reduce_words(&mut p[..2 * self.nw + 1], &self.irr);
                [p[0], p[1], p[2]]
            }
        }
    }
    /// Multiply by the polynomial-basis generator, including exact reduction.
    #[inline]
    pub fn times_x(&self, a: Element) -> Element {
        operation_tally::tick(self.m, "mul");
        let high = (a[(self.m as usize - 1) / 64] >> ((self.m - 1) % 64)) & 1;
        let mut out = [
            a[0] << 1,
            (a[1] << 1) | (a[0] >> 63),
            (a[2] << 1) | (a[1] >> 63),
        ];
        out[self.nw - 1] &= (1u64 << (self.m % 64)) - 1;
        for (x, t) in out.iter_mut().zip(self.tail) {
            *x ^= t & 0u64.wrapping_sub(high);
        }
        out
    }
    pub fn inv(&self, a: Element) -> Option<Element> {
        operation_tally::tick(self.m, "inverse_informational");
        if a == [0; 3] {
            return None;
        }
        let n = self.m - 1;
        let mut beta = a;
        let mut len = 1;
        for bit in (0..31 - n.leading_zeros()).rev() {
            let mut shifted = beta;
            for _ in 0..len {
                shifted = self.square(shifted);
            }
            beta = self.mul(beta, shifted);
            len *= 2;
            if n >> bit & 1 != 0 {
                beta = self.mul(self.square(beta), a);
                len += 1;
            }
        }
        Some(self.square(beta))
    }
    #[inline]
    pub fn trace(&self, a: Element) -> u8 {
        (((a[0] & self.trace_mask[0]) ^ (a[1] & self.trace_mask[1]) ^ (a[2] & self.trace_mask[2]))
            .count_ones()
            & 1) as u8
    }
    #[inline]
    pub fn half(&self, a: Element) -> Element {
        self.linear_maps.half(a)
    }
    pub fn admit(&self, p: &BinaryPoint) -> bool {
        let BinaryPoint::Affine { x, y } = p else {
            return true;
        };
        if x.m_value() != self.m || y.m_value() != self.m {
            return false;
        }
        let (x, y) = (element(x), element(y));
        if self.add(self.square(y), self.mul(x, y))
            != self.add(self.mul(self.square(x), x), [1, 0, 0])
        {
            return false;
        }
        self.trace(x) == 0 && self.trace(self.add(y, self.mul(x, self.half(x)))) == 0
    }
    pub fn residuals(
        &self,
        c: &BinaryCurve,
        target: &BinaryPoint,
        base: &[BinaryPoint],
    ) -> Vec<WordPoint> {
        let WordPoint::Affine { x: tx, y: ty } = WordPoint::from_point(target) else {
            return base
                .iter()
                .map(|p| WordPoint::from_point(&point_neg(p)))
                .collect();
        };
        let mut out = vec![WordPoint::Infinity; base.len()];
        let mut ordinary = Vec::new();
        let mut product = [1, 0, 0];
        for (i, p) in base.iter().enumerate() {
            match WordPoint::from_point(p) {
                WordPoint::Affine { x, y } if x != tx => {
                    let den = self.add(tx, x);
                    // Consecutive equal abscissas have the same nonzero
                    // denominator. Keep each ordinate/index but invert it once.
                    let prefix = if let Some((_, prefix, previous_den, _)) = ordinary.last() {
                        if *previous_den == den {
                            *prefix
                        } else {
                            let prefix = product;
                            product = self.mul(product, den);
                            prefix
                        }
                    } else {
                        let prefix = product;
                        product = self.mul(product, den);
                        prefix
                    };
                    ordinary.push((i, prefix, den, self.add(ty, self.add(y, x))));
                }
                _ => out[i] = WordPoint::from_point(&point_add(c, target, &point_neg(p))),
            }
        }
        if ordinary.is_empty() {
            return out;
        }
        let mut inverse = self.inv(product).unwrap();
        let mut previous_den = [0; 3];
        let mut inv = [0; 3];
        for (i, prefix, den, num) in ordinary.into_iter().rev() {
            if den != previous_den {
                inv = self.mul(inverse, prefix);
                inverse = self.mul(inverse, den);
                previous_den = den;
            }
            let lambda = self.mul(num, inv);
            let x = self.add(self.add(self.square(lambda), lambda), den);
            let y = self.add(self.add(self.mul(lambda, self.add(tx, x)), x), ty);
            crate::cryptanalysis::group_op_count::count_adds(1);
            out[i] = WordPoint::Affine { x, y };
        }
        out
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn fixture(n: u32) -> BinaryCurve {
        if n == 131 {
            return BinaryCurve::certicom_challenge("ecc2k-130").unwrap().0;
        }
        BinaryCurve {
            m: n,
            irreducible: IrreduciblePoly {
                degree: n,
                low_terms: if n == 5 {
                    vec![0, 2]
                } else {
                    vec![0, 1, 2, 45]
                },
            },
            a: F2mElement::zero(n),
            b: F2mElement::one(n),
            generator: BinaryPoint::Infinity,
            order: if n == 5 {
                11u32.into()
            } else {
                num_bigint::BigUint::parse_bytes(b"2417851639230796216685689", 10).unwrap()
            },
            cofactor: 4u32.into(),
        }
    }
    #[test]
    fn compact_arithmetic_matches_general_backend_small_exhaustive_and_wide_samples() {
        let mut seed = 20261115u64;
        for n in [5, 83, 131] {
            let c = fixture(n);
            let f = WordField::new(&c);
            let count = if n == 5 { 32 } else { 128 };
            for i in 0..count {
                let a = if n == 5 {
                    F2mElement::from_biguint(&(i as u32).into(), n)
                } else {
                    let mut raw = [0; 3];
                    for w in &mut raw {
                        seed ^= seed << 13;
                        seed ^= seed >> 7;
                        seed ^= seed << 17;
                        *w = seed;
                    }
                    F2mElement::from_words(&raw, n)
                };
                let ar = element(&a);
                assert_eq!(f.square(ar), element(&a.square(&c.irreducible)));
                assert_eq!(f.half(ar), element(&a.half_trace(&c.irreducible).unwrap()));
                assert_eq!(f.trace(ar), a.trace(&c.irreducible));
                assert_eq!(
                    f.inv(ar),
                    a.flt_inverse(&c.irreducible).map(|x| element(&x))
                );
                for j in 0..if n == 5 { 32 } else { 4 } {
                    let b = if n == 5 {
                        F2mElement::from_biguint(&(j as u32).into(), n).add(&a)
                    } else {
                        let mut raw = [0; 3];
                        for w in &mut raw {
                            seed ^= seed << 13;
                            seed ^= seed >> 7;
                            seed ^= seed << 17;
                            *w = seed;
                        }
                        F2mElement::from_words(&raw, n)
                    };
                    assert_eq!(f.add(ar, element(&b)), element(&a.add(&b)));
                    assert_eq!(f.mul(ar, element(&b)), element(&a.mul(&b, &c.irreducible)));
                }
            }
        }
    }
    #[test]
    fn compact_admission_and_batch_residuals_match_all_small_curve_points_and_offcurve_inputs() {
        let c = fixture(5);
        let f = WordField::new(&c);
        let mut points = vec![BinaryPoint::Infinity];
        for x in 0..32u32 {
            for y in 0..32u32 {
                let p = BinaryPoint::Affine {
                    x: F2mElement::from_biguint(&x.into(), 5),
                    y: F2mElement::from_biguint(&y.into(), 5),
                };
                assert_eq!(f.admit(&p), c.is_in_4e(&p) == Some(true));
                if c.is_on_curve(&p) {
                    points.push(p);
                }
            }
        }
        for target in &points {
            let got = f.residuals(&c, target, &points);
            let want: Vec<_> = points
                .iter()
                .map(|p| WordPoint::from_point(&point_add(&c, target, &point_neg(p))))
                .collect();
            assert_eq!(got, want);
        }
    }
}
