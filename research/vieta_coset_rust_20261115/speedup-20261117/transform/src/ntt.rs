//! Single-field GS/CT transform schedule adapted from the owner's
//! src/cryptanalysis/isogeny_eval/ntt.rs. No CRT: both study primes support NTT.
//! Every root search, table multiplication, butterfly, pointwise product and
//! inverse scaling is charged; tables belong to one cold field, never global.
use crate::Encoded;
use num_bigint::BigUint;
use num_traits::One;
use std::time::Instant;
use vieta_coset_lab::vieta_coset::{Error, Limits, Result, Work};
use vieta_shared_fp::fp_backend::PrimeField;
#[derive(Debug)]
pub struct Tables<E> {
    pub products: u64,
    roots: Option<(E, E, u32)>,
    fwd: Vec<Vec<E>>,
    inv: Vec<Vec<E>>,
}
impl<E> Default for Tables<E> {
    fn default() -> Self {
        Self {
            products: 0,
            roots: None,
            fwd: vec![],
            inv: vec![],
        }
    }
}
fn pow<F: PrimeField>(f: &F, a: F::Elem, e: &BigUint, w: &mut Work) -> F::Elem {
    let mut r = f.one();
    for i in (0..e.bits()).rev() {
        r = f.mul(r, r);
        w.muls += 1;
        if e.bit(i) {
            r = f.mul(r, a);
            w.muls += 1;
        }
    }
    r
}
fn ensure<F: PrimeField>(f: &F, t: &mut Tables<F::Elem>, k: u32, w: &mut Work) -> Result<()> {
    let pm1 = f.modulus() - 1u8;
    if t.roots.is_none() {
        let mut odd = pm1.clone();
        let mut s = 0u32;
        while !odd.bit(0) {
            odd >>= 1usize;
            s += 1;
        }
        if k > s {
            return Err(Error::Invalid("field lacks requested transform roots"));
        }
        let mut found = None;
        for g in 2..=64 {
            let root = pow(f, f.from_u64(g), &odd, w);
            if pow(f, root, &(BigUint::one() << (s - 1)), w) != f.one()
                && pow(f, root, &(BigUint::one() << s), w) == f.one()
            {
                found = Some(root);
                break;
            }
        }
        let root = found.ok_or(Error::Invalid(
            "no certified transform generator in frozen search",
        ))?;
        let inv = pow(f, root, &(f.modulus() - 2u8), w);
        t.roots = Some((root, inv, s));
    }
    let (root, inverse, s) = t.roots.unwrap();
    if k > s {
        return Err(Error::Invalid("transform length exceeds certified order"));
    }
    while t.fwd.len() < k as usize {
        let stage = t.fwd.len() as u32;
        let h = 1usize << stage;
        let exp = BigUint::one() << (s - stage - 1);
        let wr = pow(f, root, &exp, w);
        let wi = pow(f, inverse, &exp, w);
        let (mut a, mut b) = (f.one(), f.one());
        let (mut v, mut u) = (Vec::with_capacity(h), Vec::with_capacity(h));
        for _ in 0..h {
            v.push(a);
            u.push(b);
            a = f.mul(a, wr);
            b = f.mul(b, wi);
            w.muls += 2;
        }
        t.fwd.push(v);
        t.inv.push(u);
    }
    Ok(())
}
fn transform<F: PrimeField>(
    f: &F,
    t: &Tables<F::Elem>,
    a: &mut [F::Elem],
    inverse: bool,
    w: &mut Work,
) {
    let n = a.len();
    let log = n.trailing_zeros() as usize;
    for step in 0..log {
        let stage = if inverse { step } else { log - 1 - step };
        let h = 1usize << stage;
        let roots = if inverse {
            &t.inv[stage]
        } else {
            &t.fwd[stage]
        };
        for block in a.chunks_exact_mut(2 * h) {
            let (lo, hi) = block.split_at_mut(h);
            for ((x, y), &root) in lo.iter_mut().zip(hi).zip(roots) {
                let u = *x;
                if inverse {
                    let v = f.mul(*y, root);
                    *x = f.add(u, v);
                    *y = f.sub(u, v);
                } else {
                    let v = *y;
                    *x = f.add(u, v);
                    *y = f.mul(f.sub(u, v), root);
                }
            }
        }
        w.muls += (n / 2) as u64;
        w.adds += n as u64;
    }
}
pub fn convolution<F: PrimeField>(
    f: &F,
    t: &mut Tables<F::Elem>,
    a: &[Encoded<F::Elem>],
    b: &[Encoded<F::Elem>],
    limits: &Limits,
) -> Result<(Vec<Encoded<F::Elem>>, Work)> {
    let start = Instant::now();
    let mut w = Work::default();
    let out_len = a
        .len()
        .checked_add(b.len())
        .and_then(|n| n.checked_sub(1))
        .ok_or(Error::Invalid("transform length overflow"))?;
    let n = out_len
        .checked_next_power_of_two()
        .ok_or(Error::Invalid("transform length overflow"))?;
    if n > (1 << 21) {
        return Err(Error::Budget("transform allocation cap"));
    }
    ensure(f, t, n.trailing_zeros(), &mut w)?;
    let mut fa = vec![f.zero(); n];
    for (x, y) in fa.iter_mut().zip(a) {
        *x = y.0;
    }
    transform(f, t, &mut fa, false, &mut w);
    if std::ptr::eq(a, b) {
        for x in &mut fa {
            *x = f.mul(*x, *x);
        }
    } else {
        let mut fb = vec![f.zero(); n];
        for (x, y) in fb.iter_mut().zip(b) {
            *x = y.0;
        }
        transform(f, t, &mut fb, false, &mut w);
        for (x, y) in fa.iter_mut().zip(fb) {
            *x = f.mul(*x, y);
        }
    }
    w.muls += n as u64;
    transform(f, t, &mut fa, true, &mut w);
    fa.truncate(out_len);
    let scale = pow(f, f.from_u64(n as u64), &(f.modulus() - 2u8), &mut w);
    for x in &mut fa {
        *x = f.mul(*x, scale);
    }
    w.muls += out_len as u64;
    if w.muls > limits.max_muls || start.elapsed() > limits.timeout {
        return Err(Error::Budget("transform local budget"));
    }
    t.products += 1;
    Ok((fa.into_iter().map(Encoded).collect(), w))
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use vieta_shared_fp::fp_backend::{Fp224, Fp64};
    fn check<F: PrimeField>(f: F) {
        let limits = Limits {
            max_muls: 1_000_000_000,
            max_split_attempts: 100,
            timeout: Duration::from_secs(20),
        };
        let mut tables = Tables::default();
        for (la, lb) in [
            (1, 1),
            (127, 128),
            (128, 129),
            (256, 256),
            (511, 257),
            (1024, 128),
        ] {
            let a: Vec<_> = (0..la)
                .map(|i| Encoded(f.from_u64((i * i + 3 * i + 1) as u64)))
                .collect();
            let b: Vec<_> = (0..lb)
                .map(|i| Encoded(f.from_u64((11 * i + 7) as u64)))
                .collect();
            for square in [false, true] {
                let b = if square { &a } else { &b };
                let (c, w) = convolution(&f, &mut tables, &a, b, &limits).unwrap();
                let mut expected = vec![f.zero(); a.len() + b.len() - 1];
                for (i, x) in a.iter().enumerate() {
                    for (j, y) in b.iter().enumerate() {
                        expected[i + j] = f.add(expected[i + j], f.mul(x.0, y.0));
                    }
                }
                assert_eq!(c.iter().map(|x| x.0).collect::<Vec<_>>(), expected);
                assert!(w.muls > 0);
            }
        }
        // Certified roots and inverse are checked against actual field arithmetic.
        let (root, inv, s) = tables.roots.unwrap();
        let mut w = Work::default();
        assert_eq!(pow(&f, root, &(BigUint::one() << s), &mut w), f.one());
        assert_ne!(pow(&f, root, &(BigUint::one() << (s - 1)), &mut w), f.one());
        assert_eq!(f.mul(root, inv), f.one());
    }
    #[test]
    fn word_transform_matches_literal_schoolbook() {
        check(Fp64::new(4179340454199820289));
    }
    #[test]
    fn p224_transform_matches_literal_schoolbook() {
        check(Fp224::new(
            &((BigUint::one() << 224usize) - (BigUint::one() << 96usize) + 1u8),
        ));
    }
    #[test]
    fn unsupported_roots_are_rejected() {
        let f = Fp64::new(17);
        let mut t = Tables::default();
        let mut w = Work::default();
        assert!(ensure(&f, &mut t, 5, &mut w).is_err());
    }
}
