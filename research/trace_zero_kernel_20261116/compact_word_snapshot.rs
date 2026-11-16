//! Private public-input workspace for the <=131-bit trace-zero solver.
//! General F2mElement storage and other algorithms are unchanged. Products and
//! reductions reuse the production word kernels; exact maps are prepared once.
use crate::binary_ecc::{
    curve::{point_add, point_neg},
    f2m::{mul_words_into, reduce_words, spread32},
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
    half: Vec<Element>,
    nbytes: usize,
}
impl WordField {
    pub fn new(c: &BinaryCurve) -> Self {
        assert!(
            c.m <= 192 && c.m >= 3 && c.m % 2 == 1 && c.a.is_zero() && c.b == F2mElement::one(c.m)
        );
        let n = c.m as usize;
        let mut trace_mask = [0; 3];
        let mut halves = Vec::new();
        for bit in 0..n {
            let e = F2mElement::from_bit_positions(&[bit as u32], c.m);
            if e.trace(&c.irreducible) != 0 {
                trace_mask[bit / 64] |= 1u64 << (bit % 64);
            }
            halves.push(element(&e.half_trace(&c.irreducible).unwrap()));
        }
        let nbytes = n.div_ceil(8);
        let mut half = vec![[0; 3]; nbytes * 256];
        for byte in 0..nbytes {
            for value in 1..256usize {
                let bit = value.trailing_zeros() as usize;
                let prev = half[byte * 256 + (value & (value - 1))];
                let column = halves.get(byte * 8 + bit).copied().unwrap_or([0; 3]);
                half[byte * 256 + value] = [
                    prev[0] ^ column[0],
                    prev[1] ^ column[1],
                    prev[2] ^ column[2],
                ];
            }
        }
        Self {
            m: c.m,
            nw: n.div_ceil(64),
            irr: c.irreducible.clone(),
            trace_mask,
            half,
            nbytes,
        }
    }
    #[inline]
    pub fn add(&self, a: Element, b: Element) -> Element {
        operation_tally::tick(self.m, "add");
        [a[0] ^ b[0], a[1] ^ b[1], a[2] ^ b[2]]
    }
    #[inline]
    pub fn mul(&self, a: Element, b: Element) -> Element {
        operation_tally::tick(self.m, "mul");
        let mut out = [0u64; 7];
        mul_words_into(&a[..self.nw], &b[..self.nw], &mut out);
        reduce_words(&mut out[..2 * self.nw + 1], &self.irr);
        [out[0], out[1], out[2]]
    }
    #[inline]
    pub fn square(&self, a: Element) -> Element {
        operation_tally::tick(self.m, "square");
        let mut out = [0u64; 7];
        for (i, &w) in a[..self.nw].iter().enumerate() {
            out[2 * i] = spread32(w);
            out[2 * i + 1] = spread32(w >> 32);
        }
        reduce_words(&mut out[..2 * self.nw + 1], &self.irr);
        [out[0], out[1], out[2]]
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
        let mut out = [0; 3];
        for byte in 0..self.nbytes {
            let value = ((a[byte / 8] >> (8 * (byte % 8))) & 255) as usize;
            let h = self.half[byte * 256 + value];
            out[0] ^= h[0];
            out[1] ^= h[1];
            out[2] ^= h[2];
        }
        out
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
                    ordinary.push((i, product, den, self.add(ty, self.add(y, x))));
                    product = self.mul(product, den);
                }
                _ => out[i] = WordPoint::from_point(&point_add(c, target, &point_neg(p))),
            }
        }
        if ordinary.is_empty() {
            return out;
        }
        let mut inverse = self.inv(product).unwrap();
        for (i, prefix, den, num) in ordinary.into_iter().rev() {
            let inv = self.mul(inverse, prefix);
            inverse = self.mul(inverse, den);
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
