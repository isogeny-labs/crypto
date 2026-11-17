//! Bounded packed instantiation of the maintained constructor and provenance kernels.
//! Returns unverified complete wire bytes; acceptance always requires separate replay.
#![allow(dead_code, private_interfaces)]
use super::monomial::MonomialOps;
#[cfg(feature = "count-field-ops")]
use super::COUNTERS;
use super::{inv, minus, mul, native_kernel_bits, prime, with_native_kernel_bits};
use serde::Serialize;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::ops::Index;

/// Only this typed refusal is caught at the public boundary. Unrelated panics propagate.
#[derive(Debug)]
struct PackingRefusal(&'static str);
fn refuse(reason: &'static str) -> ! {
    std::panic::panic_any(PackingRefusal(reason))
}

/// Low112 bits contain16 seven-bit lanes, x0 most significant. Cached degree and
/// dimension occupy separate high bytes. No lane arithmetic touches metadata.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Mono(u128);
const EXP_MASK: u128 = (1u128 << 112) - 1;
const VALUES: [usize; 128] = {
    let mut a = [0; 128];
    let mut i = 0;
    while i < 128 {
        a[i] = i;
        i += 1;
    }
    a
};
impl Mono {
    fn from_parts(bits: u128, n: usize, d: usize) -> Self {
        if n > 16 || d > 64 || bits & !EXP_MASK != 0 {
            refuse("packed monomial range");
        }
        Self(bits | ((d as u128) << 112) | ((n as u128) << 120))
    }
    fn len(&self) -> usize {
        (self.0 >> 120) as usize
    }
    fn bits(&self) -> u128 {
        self.0 & EXP_MASK
    }
    fn iter(&self) -> impl DoubleEndedIterator<Item = &'static usize> + ExactSizeIterator + '_ {
        (0..self.len()).map(|i| &VALUES[((self.bits() >> (7 * (15 - i))) & 127) as usize])
    }
    fn unpack(&self) -> Vec<usize> {
        self.iter().copied().collect()
    }
    fn try_pack(v: &[usize]) -> Result<Self, String> {
        if v.len() > 16 {
            return Err("packed monomial dimension cap".into());
        }
        let d = v
            .iter()
            .try_fold(0usize, |s, e| s.checked_add(*e))
            .ok_or("packed degree overflow")?;
        if d > 64 {
            return Err("packed monomial degree cap".into());
        }
        let bits = v
            .iter()
            .enumerate()
            .fold(0u128, |s, (i, e)| s | ((*e as u128) << (7 * (15 - i))));
        Ok(Self::from_parts(bits, v.len(), d))
    }
}
impl Index<usize> for Mono {
    type Output = usize;
    fn index(&self, i: usize) -> &usize {
        assert!(i < self.len());
        &VALUES[((self.bits() >> (7 * (15 - i))) & 127) as usize]
    }
}
impl Ord for Mono {
    fn cmp(&self, other: &Self) -> Ordering {
        self.bits()
            .cmp(&other.bits())
            .then_with(|| self.len().cmp(&other.len()))
    }
}
impl PartialOrd for Mono {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
struct MonoIter<'a> {
    m: &'a Mono,
    range: std::ops::Range<usize>,
}
impl Iterator for MonoIter<'_> {
    type Item = &'static usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.range
            .next()
            .map(|i| &VALUES[((self.m.bits() >> (7 * (15 - i))) & 127) as usize])
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }
}
impl DoubleEndedIterator for MonoIter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.range
            .next_back()
            .map(|i| &VALUES[((self.m.bits() >> (7 * (15 - i))) & 127) as usize])
    }
}
impl ExactSizeIterator for MonoIter<'_> {}
impl<'a> IntoIterator for &'a Mono {
    type Item = &'static usize;
    type IntoIter = MonoIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        MonoIter {
            m: self,
            range: 0..self.len(),
        }
    }
}
impl FromIterator<usize> for Mono {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        let mut bits = 0u128;
        let mut n = 0;
        let mut d = 0usize;
        for e in iter {
            if n >= 16 || e > 64 {
                refuse("packed monomial range");
            }
            d += e;
            if d > 64 {
                refuse("packed monomial degree cap");
            }
            bits |= (e as u128) << (7 * (15 - n));
            n += 1;
        }
        Self::from_parts(bits, n, d)
    }
}
impl MonomialOps for Mono {
    const PACKED: bool = true;
    fn zero(n: usize) -> Self {
        Self::from_parts(0, n, 0)
    }
    fn set_exp(&mut self, i: usize, v: usize) {
        assert!(i < self.len());
        if v > 64 {
            refuse("packed exponent cap");
        }
        let d = self.total_degree() - self[i] + v;
        if d > 64 {
            refuse("packed monomial degree cap");
        }
        let shift = 7 * (15 - i);
        let bits = (self.bits() & !(127u128 << shift)) | ((v as u128) << shift);
        *self = Self::from_parts(bits, self.len(), d);
    }
    fn swap_exp(&mut self, a: usize, b: usize) {
        let x = self[a] as u128;
        let y = self[b] as u128;
        let sa = 7 * (15 - a);
        let sb = 7 * (15 - b);
        *self = Self::from_parts(
            (self.bits() & !(127u128 << sa) & !(127u128 << sb)) | (x << sb) | (y << sa),
            self.len(),
            self.total_degree(),
        );
    }
    fn total_degree(&self) -> usize {
        ((self.0 >> 112) & 255) as usize
    }
    fn reverse_lex_cmp(&self, b: &Self) -> Ordering {
        self.iter().rev().cmp(b.iter().rev())
    }
    fn add_exp(&self, b: &Self) -> Self {
        if self.len() != b.len() {
            refuse("packed dimension mismatch");
        }
        let d = self.total_degree() + b.total_degree();
        if d > 64 {
            refuse("packed product degree cap");
        }
        // Every exponent sum <=64: no seven-bit lane can carry.
        Self::from_parts(self.bits() + b.bits(), self.len(), d)
    }
    fn sub_exp(&self, b: &Self) -> Self {
        if self.len() != b.len() || !b.divides_exp(self) {
            refuse("packed invalid monomial quotient");
        }
        Self::from_parts(
            self.bits() - b.bits(),
            self.len(),
            self.total_degree() - b.total_degree(),
        )
    }
    fn divides_exp(&self, b: &Self) -> bool {
        self.len() == b.len() && self.iter().zip(b.iter()).all(|(x, y)| x <= y)
    }
}
type Poly = BTreeMap<Mono, u64>;
type Proof = BTreeMap<(usize, Mono), u64>;
#[derive(Clone, Debug, Eq, PartialEq)]
struct Row {
    poly: Poly,
    proof: Proof,
}
include!("kernels/arithmetic.rs");
include!("kernels/check.rs");
mod cheap {
    include!("kernels/cheap.rs");
}
mod projection {
    include!("kernels/projection.rs");
}
mod quotient {
    include!("kernels/quotient.rs");
    pub mod compact {
        include!("kernels/compact.rs");
    }
}
fn certificate_degree(row: &Row, inputs: &[Poly]) -> usize {
    row.proof
        .keys()
        .map(|(i, m)| degree(&inputs[*i]) + m.total_degree())
        .max()
        .unwrap_or(0)
}
fn monomials(n: usize, d: usize, exact: bool) -> Vec<Mono> {
    fn emit(index: usize, left: usize, m: &mut Mono, out: &mut Vec<Mono>) {
        if index == 0 {
            m.set_exp(0, left);
            out.push(*m);
            return;
        }
        for e in 0..=left {
            m.set_exp(index, e);
            emit(index - 1, left - e, m, out);
        }
    }
    assert!(n > 0);
    let mut out = Vec::new();
    let mut m = Mono::zero(n);
    for k in (if exact { d..=d } else { 0..=d }).rev() {
        emit(n - 1, k, &mut m, &mut out);
    }
    out
}
fn polynomial(f: &Poly) -> Vec<super::replay::CanonicalTerm> {
    f.iter()
        .map(|(m, c)| super::replay::CanonicalTerm {
            exponents: m.unpack(),
            coefficient: *c,
        })
        .collect()
}
fn proof(f: &Proof) -> Vec<super::replay::CofactorTerm> {
    f.iter()
        .map(|((i, m), c)| super::replay::CofactorTerm {
            generator: *i,
            exponents: m.unpack(),
            coefficient: *c,
        })
        .collect()
}
fn member(row: &Row) -> super::replay::Member {
    super::replay::Member {
        polynomial: polynomial(&row.poly),
        proof: proof(&row.proof),
    }
}
fn quotient_witness(
    target: Poly,
    reducers: &[Row],
    p: u64,
) -> Result<Vec<super::replay::CofactorTerm>, String> {
    let r = normal_form(
        Row {
            poly: target,
            proof: Proof::new(),
        },
        reducers,
        p,
    );
    if !r.poly.is_empty() {
        return Err("packed completion: nonzero remainder".into());
    }
    let coefficients = r
        .proof
        .into_iter()
        .map(|(key, c)| (key, minus(c, p)))
        .collect();
    Ok(proof(&coefficients))
}
/// Opt-in full packed constructor. Same complete existing v2 wire, same checks.
/// Unsupported shapes and packing bounds return Err; no hidden oracle/fallback.
pub fn produce_dag(
    system: &super::System,
    kernels: super::replay::NativeKernels,
) -> Result<(Vec<u8>, serde_json::Value), String> {
    produce(system, kernels, true)
}
/// Version1 sparse form, including the same charged subset-fallback path.
pub fn produce_sparse(
    system: &super::System,
    kernels: super::replay::NativeKernels,
) -> Result<(Vec<u8>, serde_json::Value), String> {
    produce(system, kernels, false)
}
fn produce(
    system: &super::System,
    kernels: super::replay::NativeKernels,
    dag: bool,
) -> Result<(Vec<u8>, serde_json::Value), String> {
    // Keep kernel choice lexical and restore it even on typed range refusal.
    let bits = match kernels {
        super::replay::NativeKernels::BothUpdatesFrontier => 103,
        super::replay::NativeKernels::FrontierPackedProducts => 231,
        super::replay::NativeKernels::FrontierPackedBatch => 487,
        _ => return Err("packed constructor requires a qualified frontier kernel".into()),
    };
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        with_native_kernel_bits(bits, || produce_impl(system, dag))
    }));
    match result {
        Ok(v) => v,
        Err(e) => match e.downcast::<PackingRefusal>() {
            Ok(v) => Err(v.0.into()),
            Err(e) => std::panic::resume_unwind(e),
        },
    }
}
fn produce_impl(system: &super::System, dag: bool) -> Result<(Vec<u8>, serde_json::Value), String> {
    use super::replay::{PairReduction, Witness};
    let p = system.prime;
    let n = system.variables.len();
    if ![2147483647, 9007199254740881].contains(&p)
        || !(3..=quotient::MAX_VARIABLES.min(16)).contains(&n)
    {
        return Err("packed constructor field/dimension admission".into());
    }
    let input = system
        .decode()?
        .into_iter()
        .map(|f| {
            f.into_iter()
                .map(|(m, c)| Ok((Mono::try_pack(&m)?, c)))
                .collect::<Result<Poly, String>>()
        })
        .collect::<Result<Vec<_>, _>>()?;
    let (certificate, metadata) = if dag {
        let out = quotient::compact::solve_sparse_dag_product(&input, n, p)?;
        (
            out.certificate,
            serde_json::to_value(out.info).map_err(|e| e.to_string())?,
        )
    } else {
        let out = quotient::compact::solve_sparse_product(&input, n, p)?;
        (
            out.output.certificate,
            serde_json::json!({"info":out.output.info,"subset_succeeded":out.subset_succeeded,"subset_error":out.subset_error}),
        )
    };
    let basis: Vec<_> = certificate.basis.iter().map(|r| r.poly.clone()).collect();
    let reducers = input_rows(&basis, n);
    let input_reductions = input
        .into_iter()
        .map(|f| quotient_witness(f, &reducers, p))
        .collect::<Result<Vec<_>, _>>()?;
    let mut pairs = Vec::new();
    for right in 0..reducers.len() {
        for left in 0..right {
            let coprime = lead(&reducers[left].poly)
                .iter()
                .zip(lead(&reducers[right].poly).iter())
                .all(|(a, b)| *a == 0 || *b == 0);
            let coefficients = if coprime {
                Vec::new()
            } else {
                quotient_witness(
                    s_polynomial(&reducers[left], &reducers[right], p).poly,
                    &reducers,
                    p,
                )?
            };
            pairs.push(PairReduction {
                left,
                right,
                product_criterion: coprime,
                proof: coefficients,
            });
        }
    }
    let w = Witness {
        version: if dag { 2 } else { 1 },
        definitions: certificate.definitions.iter().map(member).collect(),
        basis: certificate.basis.iter().map(member).collect(),
        input_reductions,
        pair_reductions: pairs,
    };
    let bytes = serde_json::to_vec(&w).map_err(|e| e.to_string())?;
    if bytes.len() > 16 * 1024 * 1024 {
        return Err("packed serialized proof byte cap".into());
    }
    Ok((bytes, metadata))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn unpack_map(f: &Poly) -> super::super::Poly {
        f.iter().map(|(m, c)| (m.unpack(), *c)).collect()
    }
    #[test]
    fn bounded_key_order_add_division_and_swap_equal_vectors() {
        assert_eq!(std::mem::size_of::<Mono>(), 16);
        let mut state = 0x72a1u64;
        for n in [1, 3, 4, 6, 16] {
            let mut vectors = Vec::new();
            for _ in 0..64 {
                let mut v = vec![0; n];
                for _ in 0..8 {
                    state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
                    v[state as usize % n] += 1;
                }
                vectors.push(v);
            }
            for a in &vectors {
                for b in &vectors {
                    let x = Mono::try_pack(a).unwrap();
                    let y = Mono::try_pack(b).unwrap();
                    assert_eq!(x.unpack(), *a);
                    assert_eq!(x.total_degree(), a.total_degree());
                    assert_eq!(x.cmp(&y), a.cmp(b));
                    assert_eq!(cmp(&x, &y), super::super::cmp(a, b));
                    assert_eq!(plus(&x, &y).unpack(), super::super::plus(a, b));
                    assert_eq!(divides(&x, &y), super::super::divides(a, b));
                    assert_eq!(difference(&plus(&x, &y), &y), x);
                    let mut packed = x;
                    let mut plain = a.clone();
                    packed.swap_exp(0, n - 1);
                    plain.swap(0, n - 1);
                    assert_eq!(packed.unpack(), plain);
                }
            }
        }
        let small = [
            vec![],
            vec![0],
            vec![0, 0],
            vec![1],
            vec![1, 0],
            vec![0, 1],
            vec![0, 0, 1],
        ];
        for a in &small {
            for b in &small {
                assert_eq!(
                    Mono::try_pack(a).unwrap().cmp(&Mono::try_pack(b).unwrap()),
                    a.cmp(b)
                );
            }
        }
    }
    #[test]
    fn packed_products_provenance_and_cancellation_equal_vector_kernels() {
        for p in [2147483647, 9007199254740881] {
            for n in [3, 4, 6, 16] {
                let mut a = vec![0; n];
                a[0] = 1;
                let mut b = vec![0; n];
                b[n - 1] = 2;
                let plain = super::super::Poly::from([(a.clone(), 1), (b.clone(), p - 1)]);
                let packed: Poly = plain
                    .iter()
                    .map(|(m, c)| (Mono::try_pack(m).unwrap(), *c))
                    .collect();
                assert_eq!(
                    unpack_map(&projection::product(&packed, &packed, p)),
                    super::super::projection::product(&plain, &plain, p)
                );
                let row = Row {
                    poly: packed.clone(),
                    proof: BTreeMap::from([((0, Mono::zero(n)), 1)]),
                };
                let got = projection::times(&row, &packed, p);
                assert!(verify(&got, &[packed], p));
                assert_eq!(
                    unpack_map(&got.poly),
                    super::super::projection::product(&plain, &plain, p)
                );
            }
        }
    }
    #[test]
    fn range_refusals_are_explicit_and_unrelated_panics_are_not_swallowed() {
        assert!(Mono::try_pack(&vec![0; 17]).is_err());
        assert!(Mono::try_pack(&[usize::MAX, 1]).is_err());
        assert!(Mono::try_pack(&[64, 1]).is_err());
        let x = Mono::try_pack(&[64, 0, 0]).unwrap();
        let y = Mono::try_pack(&[0, 1, 0]).unwrap();
        let e = std::panic::catch_unwind(|| plus(&x, &y)).unwrap_err();
        assert!(e.is::<PackingRefusal>());
        let e = std::panic::catch_unwind(|| difference(&x, &y)).unwrap_err();
        assert!(e.is::<PackingRefusal>());
        let e = std::panic::catch_unwind(|| plus(&x, &Mono::zero(4))).unwrap_err();
        assert!(e.is::<PackingRefusal>());
        assert_eq!(
            Mono::try_pack(&[64, 0, 0]).unwrap().unpack(),
            vec![64, 0, 0]
        );
        assert_eq!(difference(&x, &x).unpack(), vec![0, 0, 0]);
        let bad = super::super::System {
            prime: 2147483647,
            variables: vec!["x".into(); 17],
            polynomials: vec![],
        };
        assert!(produce_dag(
            &bad,
            super::super::replay::NativeKernels::FrontierPackedBatch
        )
        .is_err());
        // The lexical kernel guard restores state during any panic.
        let before = native_kernel_bits();
        assert!(
            std::panic::catch_unwind(|| with_native_kernel_bits(487, || panic!(
                "unrelated sentinel"
            )))
            .unwrap_err()
            .is::<&str>()
        );
        assert_eq!(native_kernel_bits(), before);
    }
}
