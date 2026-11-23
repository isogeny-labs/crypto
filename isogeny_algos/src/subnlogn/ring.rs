//! Signed Kronecker packing for C[y]/(y^r+1), manuscript §6.
//!
//! Four concrete integer products compute the exact Gaussian numerators.
//! Packing and centered extraction each scan fixed-width fields once. In
//! particular, repeatedly shifting a growing evaluation integer is avoided.
//! Only the final division by r and grid truncation loses precision.

use super::certified::{Complex, Signed};
use super::fastint::{bit_slice, multiply};
use crate::bigint::Big;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Shape,
    Precision,
    SizeOverflow,
    OutsideDisk { index: usize },
    PackingInvariant,
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "signed ring product: {self:?}")
    }
}
impl std::error::Error for Error {}

fn pow2(bits: usize) -> Big {
    Big::from_u64(1).shl(bits)
}

fn write_field(out: &mut [u64], offset: usize, value: &Big) {
    let limb = offset / 64;
    let shift = offset % 64;
    for (j, &x) in value.0.iter().enumerate() {
        out[limb + j] |= x << shift;
        if shift != 0 && limb + j + 1 < out.len() {
            out[limb + j + 1] |= x >> (64 - shift);
        }
    }
}

fn pack<'a>(
    values: impl Iterator<Item = &'a Signed>,
    len: usize,
    width: usize,
) -> Result<Signed, Error> {
    let total = len.checked_mul(width).ok_or(Error::SizeOverflow)?;
    let mut words = vec![0; total.div_ceil(64)];
    let radix = pow2(width);
    let minus_one = Signed::from_i64(-1);
    let mut carry = false;
    for (j, value) in values.enumerate() {
        let value = if carry {
            value.add(&minus_one)
        } else {
            value.clone()
        };
        if value.magnitude >= radix {
            return Err(Error::PackingInvariant);
        }
        let field = if value.negative {
            radix.sub(&value.magnitude)
        } else {
            value.magnitude
        };
        carry = value.negative;
        write_field(&mut words, j * width, &field);
    }
    let fixed = Big::from_limbs(&words);
    Ok(Signed::new(
        carry,
        if carry {
            pow2(total).sub(&fixed)
        } else {
            fixed
        },
    ))
}

fn unpack(value: Signed, len: usize, width: usize) -> Result<Vec<Signed>, Error> {
    let total = len.checked_mul(width).ok_or(Error::SizeOverflow)?;
    if value.magnitude.bits() >= total {
        return Err(Error::PackingInvariant);
    }
    let fixed = if value.negative {
        pow2(total).sub(&value.magnitude)
    } else {
        value.magnitude
    };
    let radix = pow2(width);
    let half = pow2(width - 1);
    let mut carry = false;
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let field = bit_slice(&fixed, j * width, width).add_small(u64::from(carry));
        if field >= half {
            out.push(Signed::new(true, radix.sub(&field)));
            carry = true;
        } else {
            out.push(Signed::new(false, field));
            carry = false;
        }
    }
    if carry != value.negative || !out.last().is_some_and(|x| x.magnitude.is_zero()) {
        return Err(Error::PackingInvariant);
    }
    Ok(out)
}

fn product(a: &Signed, b: &Signed) -> Signed {
    Signed::new(
        a.negative ^ b.negative,
        multiply(&a.magnitude, &b.magnitude),
    )
}

/// Return Q_p(f*g/r) in the negacyclic ring. Inputs and outputs use integer
/// numerators on the 2^-p grid. The componentwise truncation error is less than
/// sqrt(2)*2^-p, and the normalized product is a disk contraction.
pub fn normalized_product(a: &[Complex], b: &[Complex], p: usize) -> Result<Vec<Complex>, Error> {
    let r = a.len();
    if r != b.len() || r < 2 || !r.is_power_of_two() {
        return Err(Error::Shape);
    }
    let ell = r.trailing_zeros() as usize;
    if p < 2 || ell >= p {
        return Err(Error::Precision);
    }
    for (index, value) in a.iter().chain(b).enumerate() {
        if !value.is_disk(p) {
            return Err(Error::OutsideDisk { index });
        }
    }
    let width = p.checked_mul(4).ok_or(Error::SizeOverflow)?;
    let j = r.checked_mul(2).ok_or(Error::SizeOverflow)?;
    let ar = pack(a.iter().map(|x| &x.re), r, width)?;
    let ai = pack(a.iter().map(|x| &x.im), r, width)?;
    let br = pack(b.iter().map(|x| &x.re), r, width)?;
    let bi = pack(b.iter().map(|x| &x.im), r, width)?;
    let ac = unpack(product(&ar, &br), j, width)?;
    let be = unpack(product(&ai, &bi), j, width)?;
    let ae = unpack(product(&ar, &bi), j, width)?;
    let bc = unpack(product(&ai, &br), j, width)?;
    let mut output = Vec::with_capacity(r);
    for k in 0..r {
        let re = ac[k].sub(&be[k]).sub(&ac[k + r]).add(&be[k + r]);
        let im = ae[k].add(&bc[k]).sub(&ae[k + r]).sub(&bc[k + r]);
        let value = Complex { re, im }.shr_trunc(p + ell);
        if !value.is_disk(p) {
            return Err(Error::PackingInvariant);
        }
        output.push(value);
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn oracle(a: &[Complex], b: &[Complex], p: usize) -> Vec<Complex> {
        let r = a.len();
        let mut out = vec![Complex::zero(); r];
        for (i, a) in a.iter().enumerate() {
            for (j, b) in b.iter().enumerate() {
                let value = a.mul_exact(b);
                let k = (i + j) % r;
                out[k] = if i + j >= r {
                    out[k].sub(&value)
                } else {
                    out[k].add(&value)
                };
            }
        }
        out.into_iter()
            .map(|x| x.shr_trunc(p + r.trailing_zeros() as usize))
            .collect()
    }

    #[test]
    fn signed_fields_with_negative_leading_coefficients() {
        for input in [vec![-3, 0, 7, -1], vec![5, -6, 0, 0], vec![0, 0, 0, 0]] {
            let values: Vec<_> = input.iter().map(|&x| Signed::from_i64(x)).collect();
            let packed = pack(values.iter(), 4, 16).unwrap();
            let mut expected = Signed::zero();
            for (j, value) in values.iter().enumerate() {
                expected = expected.add(&value.shl(16 * j));
            }
            assert_eq!(packed, expected);
            // unpack expects one final zero coefficient, as every product has.
            let recovered = unpack(packed, 5, 16).unwrap();
            assert_eq!(&recovered[..4], &values);
        }
    }

    #[test]
    fn complex_products_and_negacyclic_wrap() {
        for p in [8, 19, 64, 128] {
            for r in [2, 4, 8, 16] {
                let a: Vec<_> = (0..r)
                    .map(|j| Complex {
                        re: Signed::from_i64(j as i64 % 7 - 3).shl(p - 5),
                        im: Signed::from_i64(j as i64 % 5 - 2).shl(p - 5),
                    })
                    .collect();
                let b: Vec<_> = (0..r)
                    .map(|j| Complex {
                        re: Signed::from_i64(2 - j as i64 % 5).shl(p - 5),
                        im: Signed::from_i64(j as i64 % 3 - 1).shl(p - 5),
                    })
                    .collect();
                assert_eq!(normalized_product(&a, &b, p).unwrap(), oracle(&a, &b, p));
            }
        }
    }

    #[test]
    fn rejects_outside_disk_and_invalid_dimensions() {
        let bad = Complex {
            re: Signed::from_i64(257),
            im: Signed::zero(),
        };
        assert!(matches!(
            normalized_product(&[bad.clone(), bad], &vec![Complex::zero(); 2], 8),
            Err(Error::OutsideDisk { .. })
        ));
        assert!(normalized_product(&[], &[], 8).is_err());
    }
}
