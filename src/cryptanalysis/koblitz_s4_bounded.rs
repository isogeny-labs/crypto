//! Fallible finite-selector construction of the existing b=1 S4 equation.
//! Reservations are conservative modeled totals, not CPU instructions or RSS.
//! This module neither solves the equations nor proves signed-point completeness.
use super::{
    mono_key, F2BoolMono, F2BoolPoly, F2mElement, FieldStructure, FieldSum, IrreduciblePoly, Mask,
};
use std::{mem::size_of, time::Instant};

pub const MAX_WORK: u64 = 16_777_216;
pub const MAX_BYTES: u64 = 64 * 1024 * 1024;
const SUPPORT: usize = 256;
const PAIRS: usize = 65_536;
const METADATA: u64 = 4096;

#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub work: u64,
    pub bytes: u64,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            work: MAX_WORK,
            bytes: MAX_BYTES,
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Reserved {
    pub work: u64,
    pub bytes: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stop {
    InvalidInput,
    WorkLimit,
    ByteLimit,
    SupportLimit,
    PairLimit,
    Allocation,
    Overflow,
    Deadline,
    Observer,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    Begin,
    Table,
    Selector,
    Add,
    Square,
    Product,
    Coordinates,
    Complete,
}
#[derive(Clone, Copy, Debug)]
pub struct Event {
    pub stage: Stage,
    pub reserved: Reserved,
}
#[derive(Clone, Copy, Debug)]
pub struct Failure {
    pub stop: Stop,
    pub reserved: Reserved,
}
#[derive(Debug)]
pub struct Output {
    pub n_vars: usize,
    pub equations: Vec<F2BoolPoly>,
    pub reserved: Reserved,
}

struct Gate<'a, O> {
    limits: Limits,
    reserved: Reserved,
    deadline: Instant,
    observer: &'a mut O,
}
impl<O: FnMut(Event) -> Result<(), ()>> Gate<'_, O> {
    fn check(&self) -> Result<(), Stop> {
        if Instant::now() >= self.deadline {
            Err(Stop::Deadline)
        } else {
            Ok(())
        }
    }
    fn admit(&mut self, work: u64, bytes: u64, stage: Stage) -> Result<(), Stop> {
        self.check()?;
        let w = self.reserved.work.checked_add(work).ok_or(Stop::Overflow)?;
        let b = self
            .reserved
            .bytes
            .checked_add(bytes)
            .ok_or(Stop::Overflow)?;
        if w > self.limits.work {
            return Err(Stop::WorkLimit);
        }
        if b > self.limits.bytes {
            return Err(Stop::ByteLimit);
        }
        self.reserved = Reserved { work: w, bytes: b };
        (self.observer)(Event {
            stage,
            reserved: self.reserved,
        })
        .map_err(|_| Stop::Observer)?;
        self.check()
    }
    fn vec<T>(&self, len: usize) -> Result<Vec<T>, Stop> {
        self.check()?;
        std::alloc::Layout::array::<T>(len).map_err(|_| Stop::Overflow)?;
        let mut v = Vec::new();
        v.try_reserve_exact(len).map_err(|_| Stop::Allocation)?;
        Ok(v)
    }
    fn finish_sum(&self, mut terms: Vec<(Mask, u64)>) -> Result<FieldSum, Stop> {
        self.check()?;
        terms.sort_unstable_by_key(|&(m, _)| std::cmp::Reverse(mono_key(F2BoolMono::from_mask(m))));
        let mut out = 0;
        for i in 0..terms.len() {
            if i % 256 == 0 {
                self.check()?;
            }
            let (m, c) = terms[i];
            if out > 0 && terms[out - 1].0 == m {
                terms[out - 1].1 ^= c;
            } else {
                terms[out] = (m, c);
                out += 1;
            }
        }
        terms.truncate(out);
        terms.retain(|&(_, c)| c != 0);
        if terms.len() > SUPPORT {
            return Err(Stop::SupportLimit);
        }
        Ok(FieldSum(terms))
    }
    fn constant(&mut self, value: u64) -> Result<FieldSum, Stop> {
        self.admit(256, 4096, Stage::Selector)?;
        let mut v = self.vec(usize::from(value != 0))?;
        if value != 0 {
            v.push((0, value));
        }
        Ok(FieldSum(v))
    }
    fn selector(&mut self, domain: &[u64], q: usize, offset: usize) -> Result<FieldSum, Stop> {
        self.admit(65_536, 65_536, Stage::Selector)?;
        let count = 1usize << q;
        let mut values = self.vec(count)?;
        for i in 0..count {
            values.push(domain[i % domain.len()]);
        }
        for bit in 0..q {
            self.check()?;
            for mask in 0..count {
                if mask & (1 << bit) != 0 {
                    values[mask] ^= values[mask ^ (1 << bit)];
                }
            }
        }
        let mut terms = self.vec(count)?;
        for (mask, value) in values.into_iter().enumerate() {
            if value != 0 {
                terms.push(((mask as Mask) << offset, value));
            }
        }
        self.finish_sum(terms)
    }
    fn table(&mut self, irr: &IrreduciblePoly) -> Result<FieldStructure, Stop> {
        let n = irr.degree as usize;
        self.admit(
            4096 * (2 * n - 2) as u64 + 32 * (n * n + 3 * n + 1) as u64,
            1 << 20,
            Stage::Table,
        )?;
        let mut powers = self.vec(2 * n - 1)?;
        let z = F2mElement::from_bit_positions(&[1], irr.degree);
        let mut p = F2mElement::one(irr.degree);
        powers.push(1);
        for _ in 1..2 * n - 1 {
            self.check()?;
            p = p.mul(&z, irr);
            powers.push(p.raw_bits().first().copied().unwrap_or(0));
        }
        let mut reduced = self.vec(n)?;
        let mut squares = self.vec(n)?;
        for i in 0..n {
            let mut row = self.vec(n)?;
            row.extend_from_slice(&powers[i..i + n]);
            reduced.push(row);
            squares.push(powers[2 * i]);
        }
        Ok(FieldStructure {
            n: irr.degree,
            reduced,
            squares,
        })
    }
    fn add(&mut self, a: &FieldSum, b: &FieldSum) -> Result<FieldSum, Stop> {
        let count = a.0.len() + b.0.len();
        self.admit(
            16 * (count + 1) as u64,
            (2 * size_of::<(Mask, u64)>() * (count + 1) + 256) as u64,
            Stage::Add,
        )?;
        let mut out = self.vec(count)?;
        // Merge canonical support, avoiding a second sorting tariff.
        let (mut i, mut j) = (0, 0);
        while i < a.0.len() || j < b.0.len() {
            self.check()?;
            if j == b.0.len()
                || (i < a.0.len()
                    && mono_key(F2BoolMono::from_mask(a.0[i].0))
                        > mono_key(F2BoolMono::from_mask(b.0[j].0)))
            {
                out.push(a.0[i]);
                i += 1;
            } else if i == a.0.len() || a.0[i].0 != b.0[j].0 {
                out.push(b.0[j]);
                j += 1;
            } else {
                let c = a.0[i].1 ^ b.0[j].1;
                if c != 0 {
                    out.push((a.0[i].0, c));
                }
                i += 1;
                j += 1;
            }
        }
        if out.len() > SUPPORT {
            return Err(Stop::SupportLimit);
        }
        Ok(FieldSum(out))
    }
    fn square(&mut self, a: &FieldSum, st: &FieldStructure) -> Result<FieldSum, Stop> {
        self.admit(
            16 * (a.0.len() * (st.n as usize + 1) + 1) as u64,
            (2 * size_of::<(Mask, u64)>() * (a.0.len() + 1) + 256) as u64,
            Stage::Square,
        )?;
        let mut out = self.vec(a.0.len())?;
        for &(mask, mut bits) in &a.0 {
            self.check()?;
            let mut c = 0;
            while bits != 0 {
                c ^= st.squares[bits.trailing_zeros() as usize];
                bits &= bits - 1;
            }
            if c != 0 {
                out.push((mask, c));
            }
        }
        Ok(FieldSum(out))
    }
    fn mul(&mut self, a: &FieldSum, b: &FieldSum, st: &FieldStructure) -> Result<FieldSum, Stop> {
        let n = st.n as usize;
        let q = a.0.len().checked_mul(b.0.len()).ok_or(Stop::Overflow)?;
        if q > PAIRS {
            return Err(Stop::PairLimit);
        }
        let l = q.max(2).next_power_of_two().trailing_zeros() as usize;
        let work = 32 * (b.0.len() * n * n + q * (n + 1) + q * l + q + 1) as u64;
        let bytes = (2 * size_of::<(Mask, u64)>() * (q + 1) + 16 * b.0.len() * n + 512) as u64;
        self.admit(work, bytes, Stage::Product)?;
        let mut out = self.vec(q)?;
        let mut rows = self.vec(b.0.len() * n)?;
        rows.resize(b.0.len() * n, 0u64);
        for (row, &(_, f)) in rows.chunks_exact_mut(n).zip(&b.0) {
            self.check()?;
            for (r, reduced) in row.iter_mut().zip(&st.reduced) {
                let mut bits = f;
                while bits != 0 {
                    *r ^= reduced[bits.trailing_zeros() as usize];
                    bits &= bits - 1;
                }
            }
        }
        for &(s, f) in &a.0 {
            self.check()?;
            for (&(t, _), row) in b.0.iter().zip(rows.chunks_exact(n)) {
                let mut bits = f;
                let mut c = 0;
                while bits != 0 {
                    c ^= row[bits.trailing_zeros() as usize];
                    bits &= bits - 1;
                }
                out.push((s | t, c));
            }
        }
        self.finish_sum(out)
    }
    fn coordinates(
        &mut self,
        a: &FieldSum,
        n: usize,
        n_vars: usize,
    ) -> Result<Vec<F2BoolPoly>, Stop> {
        self.admit(16 * (a.0.len() * n + 1) as u64, 0, Stage::Coordinates)?;
        let count: usize = a.0.iter().map(|&(_, c)| c.count_ones() as usize).sum();
        self.admit(
            0,
            (2 * size_of::<F2BoolMono>() * (count + 1) + 128 * (n + 1)) as u64,
            Stage::Coordinates,
        )?;
        let mut out = self.vec(n)?;
        for bit in 0..n {
            let len = a.0.iter().filter(|&&(_, c)| c & (1 << bit) != 0).count();
            let mut terms = self.vec(len)?;
            for &(mask, c) in &a.0 {
                if c & (1 << bit) != 0 {
                    terms.push(F2BoolMono::from_mask(mask));
                }
            }
            out.push(F2BoolPoly { terms, n_vars });
        }
        Ok(out)
    }
    fn run(
        &mut self,
        irr: &IrreduciblePoly,
        domain: &[u64],
        target: u64,
    ) -> Result<(usize, Vec<F2BoolPoly>), Stop> {
        self.admit(0, METADATA, Stage::Begin)?;
        let q = domain.len().next_power_of_two().trailing_zeros() as usize;
        let st = self.table(irr)?;
        let x1 = self.selector(domain, q, 0)?;
        let x2 = self.selector(domain, q, q)?;
        let x3 = self.selector(domain, q, 2 * q)?;
        let e12 = self.add(&x1, &x2)?;
        let e1 = self.add(&e12, &x3)?;
        let p12 = self.mul(&x1, &x2, &st)?;
        let p13 = self.mul(&x1, &x3, &st)?;
        let p23 = self.mul(&x2, &x3, &st)?;
        let e212 = self.add(&p12, &p13)?;
        let e2 = self.add(&e212, &p23)?;
        let e3 = self.mul(&p12, &x3, &st)?;
        let e1s = self.square(&e1, &st)?;
        let e2s = self.square(&e2, &st)?;
        let e3s = self.square(&e3, &st)?;
        let r1 = self.constant(target)?;
        let r2 = self.square(&r1, &st)?;
        let r3 = self.mul(&r2, &r1, &st)?;
        let r4 = self.square(&r2, &st)?;
        let t = self.square(&e1s, &st)?;
        let mut acc = self.add(&r4, &t)?;
        let t = self.square(&e3s, &st)?;
        acc = self.add(&acc, &t)?;
        let t = self.square(&e2s, &st)?;
        let t = self.mul(&t, &r4, &st)?;
        acc = self.add(&acc, &t)?;
        let t = self.mul(&e3s, &e3, &st)?;
        let t = self.mul(&t, &r1, &st)?;
        acc = self.add(&acc, &t)?;
        let t = self.mul(&e3, &e2s, &st)?;
        let t = self.mul(&t, &r3, &st)?;
        acc = self.add(&acc, &t)?;
        let t = self.mul(&e3, &e1s, &st)?;
        let t = self.mul(&t, &r1, &st)?;
        acc = self.add(&acc, &t)?;
        let t = self.mul(&e3, &r3, &st)?;
        acc = self.add(&acc, &t)?;
        let t = self.mul(&e1s, &e3s, &st)?;
        let t = self.mul(&t, &r2, &st)?;
        acc = self.add(&acc, &t)?;
        let t = self.mul(&e3s, &r4, &st)?;
        acc = self.add(&acc, &t)?;
        acc = self.add(&acc, &e3s)?;
        let t = self.mul(&e2s, &r2, &st)?;
        acc = self.add(&acc, &t)?;
        let equations = self.coordinates(&acc, irr.degree as usize, 3 * q)?;
        self.admit(0, 0, Stage::Complete)?;
        Ok((3 * q, equations))
    }
}

/// Build only the b=1 polynomial. The caller must certify the modulus is
/// irreducible and bind actual curve/twist/domain provenance separately.
/// Limits and absolute deadline must be derived from the original run envelope.
pub fn build<O: FnMut(Event) -> Result<(), ()>>(
    irr: &IrreduciblePoly,
    b: u64,
    domain: &[u64],
    target: u64,
    limits: Limits,
    deadline: Instant,
    observer: &mut O,
) -> Result<Output, Failure> {
    let n = irr.degree;
    let valid_n = (23..=31).contains(&n) || (47..=61).contains(&n);
    let valid = valid_n
        && b == 1
        && limits.work <= MAX_WORK
        && limits.bytes <= MAX_BYTES
        && !domain.is_empty()
        && domain.len() <= 64
        && irr.low_terms.len() <= n as usize
        && irr.low_terms.first() == Some(&0)
        && irr.low_terms.iter().all(|&i| i < n)
        && irr.low_terms.windows(2).all(|w| w[0] < w[1])
        && domain.windows(2).all(|w| w[0] < w[1])
        && domain.iter().all(|&x| x < (1u64 << n))
        && target < (1u64 << n);
    if !valid {
        return Err(Failure {
            stop: Stop::InvalidInput,
            reserved: Reserved::default(),
        });
    }
    let mut gate = Gate {
        limits,
        reserved: Reserved::default(),
        deadline,
        observer,
    };
    match gate.run(irr, domain, target) {
        Ok((n_vars, equations)) => Ok(Output {
            n_vars,
            equations,
            reserved: gate.reserved,
        }),
        Err(stop) => Err(Failure {
            stop,
            reserved: gate.reserved,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::{
        koblitz_index_calculus::find_irreducible_sparse as find_irreducible,
        pq_groebner_f2::mask_from64,
        semaev_decomp::{eval_f3, Gf2},
    };
    use std::time::Duration;
    fn deadline() -> Instant {
        Instant::now() + Duration::from_secs(5)
    }
    fn run(
        irr: &IrreduciblePoly,
        domain: &[u64],
        target: u64,
        limits: Limits,
    ) -> Result<Output, Failure> {
        build(irr, 1, domain, target, limits, deadline(), &mut |_| Ok(()))
    }
    fn eval(p: &[F2BoolPoly], assignment: u64) -> u64 {
        p.iter().enumerate().fold(0, |out, (i, p)| {
            out | ((p.eval(mask_from64(assignment)) as u64) << i)
        })
    }
    // Separate general-resultant expression, including repeated/zero inputs.
    fn resultant(x: u64, y: u64, z: u64, r: u64, g: &Gf2) -> u64 {
        let s = x ^ y;
        let t = z ^ r;
        let p = g.mul(x, y);
        let q = g.mul(z, r);
        let ss = g.mul(s, s);
        let tt = g.mul(t, t);
        let a = g.mul(ss, g.mul(q, q)) ^ g.mul(tt, g.mul(p, p)) ^ ss ^ tt;
        g.mul(a, a) ^ g.mul(g.mul(g.mul(ss, q) ^ g.mul(tt, p), p ^ q), g.mul(p, q) ^ 1)
    }
    #[test]
    fn bounded_s4_k2_requested_degrees_match_scalar_and_resultant() {
        for n in [23, 31, 47, 53, 61] {
            let irr = find_irreducible(n).unwrap();
            let gf = Gf2::new(&irr);
            let domain = [(1u64 << (n - 2)) | 5, (1u64 << (n - 1)) | 3];
            for target in [0, domain[0], 37] {
                let out = run(&irr, &domain, target, Limits::default()).unwrap();
                assert_eq!(out.n_vars, 3);
                assert!(out
                    .equations
                    .iter()
                    .any(|p| p.terms.iter().any(|m| m.mask != 0)));
                for assignment in 0..8u64 {
                    let x = domain[(assignment & 1) as usize];
                    let y = domain[((assignment >> 1) & 1) as usize];
                    let z = domain[((assignment >> 2) & 1) as usize];
                    assert_eq!(
                        eval(&out.equations, assignment),
                        eval_f3(x, y, z, target, &gf)
                    );
                    assert_eq!(
                        eval(&out.equations, assignment),
                        resultant(x, y, z, target, &gf)
                    );
                }
            }
        }
    }
    #[test]
    fn bounded_s4_power_tables_equal_legacy() {
        for n in [23, 31, 47, 53, 61] {
            let irr = find_irreducible(n).unwrap();
            let mut observer = |_| Ok(());
            let mut gate = Gate {
                limits: Limits::default(),
                reserved: Reserved::default(),
                deadline: deadline(),
                observer: &mut observer,
            };
            let got = gate.table(&irr).unwrap();
            let expected = FieldStructure::new(n, &irr);
            assert_eq!(got.reduced, expected.reduced);
            assert_eq!(got.squares, expected.squares);
        }
    }

    #[test]
    fn bounded_s4_add_allows_large_temporary_support_then_cancels() {
        let canonical = |range: std::ops::Range<u64>| {
            let mut terms: Vec<_> = range.map(|m| (mask_from64(m), 1u64)).collect();
            terms.sort_unstable_by_key(|&(m, _)| {
                std::cmp::Reverse(mono_key(F2BoolMono::from_mask(m)))
            });
            FieldSum(terms)
        };
        let a = canonical(0..256);
        let b = canonical(128..384);
        assert!(a.0.len() + b.0.len() > SUPPORT);
        let mut observer = |_| Ok(());
        let mut gate = Gate {
            limits: Limits::default(),
            reserved: Reserved::default(),
            deadline: deadline(),
            observer: &mut observer,
        };
        let sum = gate.add(&a, &b).unwrap();
        let mut expected = canonical(0..384).0;
        expected.retain(|&(m, _)| !(mask_from64(128)..mask_from64(256)).contains(&m));
        assert_eq!(sum.0.len(), SUPPORT);
        assert_eq!(sum.0, expected);
        assert!(gate.add(&a, &canonical(256..512)).is_err());
    }

    #[test]
    fn bounded_s4_canonical_coordinates_equal_legacy_builder() {
        use super::super::{sym_semaev_s4, SymElement};
        for n in [23, 31, 47, 53, 61] {
            let irr = find_irreducible(n).unwrap();
            let st = FieldStructure::new(n, &irr);
            let domain = [(1u64 << (n - 2)) | 5, (1u64 << (n - 1)) | 3];
            let constant = F2mElement::from_words(&[domain[0]], n);
            let delta = F2mElement::from_words(&[domain[0] ^ domain[1]], n);
            // Independently construct each affine K2 selector through legacy
            // coordinate APIs, rather than converting the new FieldSum output.
            let xs: Vec<_> = (0..3)
                .map(|offset| {
                    SymElement::from_subspace_vars(std::slice::from_ref(&delta), offset, n, 3)
                        .add(&SymElement::constant(&constant, n, 3))
                })
                .collect();
            for target in [0, domain[0], 37] {
                let actual = run(&irr, &domain, target, Limits::default()).unwrap();
                let target_element = F2mElement::from_words(&[target], n);
                let expected = sym_semaev_s4(&xs[0], &xs[1], &xs[2], &target_element, &st);
                assert_eq!(actual.equations.len(), expected.len());
                for (actual, expected) in actual.equations.iter().zip(&expected) {
                    assert_eq!(actual.n_vars, expected.n_vars);
                    let actual_masks: Vec<_> = actual.terms.iter().map(|t| t.mask).collect();
                    let expected_masks: Vec<_> = expected.terms.iter().map(|t| t.mask).collect();
                    assert_eq!(actual_masks, expected_masks, "n={n}, target={target}");
                }
            }
        }
    }
    #[test]
    fn bounded_s4_selector_modulo_padding_and_sparse_controls() {
        for k in [1usize, 2, 3, 4, 5, 64] {
            let domain: Vec<u64> = (0..k as u64).map(|v| (v << 3) ^ 3).collect();
            let q = k.next_power_of_two().trailing_zeros() as usize;
            let mut observer = |_| Ok(());
            let mut gate = Gate {
                limits: Limits::default(),
                reserved: Reserved::default(),
                deadline: deadline(),
                observer: &mut observer,
            };
            let s = gate.selector(&domain, q, 2 * q).unwrap();
            let polys = gate.coordinates(&s, 23, 3 * q).unwrap();
            for index in 0..1usize << q {
                assert_eq!(eval(&polys, (index as u64) << (2 * q)), domain[index % k]);
            }
        }
        let irr = find_irreducible(23).unwrap();
        for domain in [&[0, 1, 2][..], &[0, 1, 2, 3][..]] {
            match run(&irr, domain, 0, Limits::default()) {
                Ok(out) => {
                    let gf = Gf2::new(&irr);
                    for a in 0..64 {
                        assert_eq!(
                            eval(&out.equations, a),
                            eval_f3(
                                domain[(a & 3) as usize % domain.len()],
                                domain[((a >> 2) & 3) as usize % domain.len()],
                                domain[((a >> 4) & 3) as usize % domain.len()],
                                0,
                                &gf
                            )
                        );
                    }
                }
                Err(f) => assert!(matches!(
                    f.stop,
                    Stop::WorkLimit | Stop::SupportLimit | Stop::ByteLimit | Stop::PairLimit
                )),
            }
        }
    }
    #[test]
    fn bounded_s4_exact_budget_and_one_less() {
        let irr = find_irreducible(23).unwrap();
        let domain = [5, 9];
        let baseline = run(&irr, &domain, 37, Limits::default()).unwrap();
        let exact = Limits {
            work: baseline.reserved.work,
            bytes: baseline.reserved.bytes,
        };
        let repeat = run(&irr, &domain, 37, exact).unwrap();
        assert_eq!(repeat.reserved, baseline.reserved);
        assert_eq!(
            run(
                &irr,
                &domain,
                37,
                Limits {
                    work: exact.work - 1,
                    ..exact
                }
            )
            .unwrap_err()
            .stop,
            Stop::WorkLimit
        );
        assert_eq!(
            run(
                &irr,
                &domain,
                37,
                Limits {
                    bytes: exact.bytes - 1,
                    ..exact
                }
            )
            .unwrap_err()
            .stop,
            Stop::ByteLimit
        );
        let mut calls = 0;
        let err = build(
            &irr,
            1,
            &domain,
            37,
            Limits {
                bytes: METADATA - 1,
                ..Limits::default()
            },
            deadline(),
            &mut |_| {
                calls += 1;
                Ok(())
            },
        )
        .unwrap_err();
        assert_eq!(err.stop, Stop::ByteLimit);
        assert_eq!(err.reserved, Reserved::default());
        assert_eq!(calls, 0);
        let err = build(
            &irr,
            1,
            &domain,
            37,
            Limits {
                bytes: METADATA,
                ..Limits::default()
            },
            deadline(),
            &mut |_| {
                calls += 1;
                Err(())
            },
        )
        .unwrap_err();
        assert_eq!(err.stop, Stop::Observer);
        assert_eq!(calls, 1);
        assert_eq!(err.reserved.bytes, METADATA);
    }
    #[test]
    fn bounded_s4_sticky_observer_and_deadline() {
        let irr = find_irreducible(23).unwrap();
        let mut calls = 0;
        let err = build(
            &irr,
            1,
            &[5, 9],
            37,
            Limits::default(),
            deadline(),
            &mut |_| {
                calls += 1;
                if calls == 4 {
                    Err(())
                } else {
                    Ok(())
                }
            },
        )
        .unwrap_err();
        assert_eq!(err.stop, Stop::Observer);
        assert_eq!(calls, 4);
        calls = 0;
        let err = build(
            &irr,
            1,
            &[5, 9],
            37,
            Limits::default(),
            Instant::now(),
            &mut |_| {
                calls += 1;
                Ok(())
            },
        )
        .unwrap_err();
        assert_eq!(err.stop, Stop::Deadline);
        assert_eq!(calls, 0);
    }
    #[test]
    fn bounded_s4_rejects_malformed_before_callback() {
        let irr = find_irreducible(23).unwrap();
        let mut calls = 0;
        for (b, domain, target, limits) in [
            (2, &[1, 2][..], 0, Limits::default()),
            (1, &[2, 1][..], 0, Limits::default()),
            (1, &[1, 1][..], 0, Limits::default()),
            (1, &[][..], 0, Limits::default()),
            (1, &[1, 2][..], 1 << 23, Limits::default()),
            (
                1,
                &[1, 2][..],
                0,
                Limits {
                    work: u64::MAX,
                    bytes: MAX_BYTES,
                },
            ),
        ] {
            assert_eq!(
                build(&irr, b, domain, target, limits, deadline(), &mut |_| {
                    calls += 1;
                    Ok(())
                })
                .unwrap_err()
                .stop,
                Stop::InvalidInput
            );
        }
        for bad in [
            IrreduciblePoly {
                degree: 64,
                low_terms: vec![0, 1],
            },
            IrreduciblePoly {
                degree: 23,
                low_terms: vec![0, 0, 1],
            },
            IrreduciblePoly {
                degree: 23,
                low_terms: vec![1],
            },
            IrreduciblePoly {
                degree: 23,
                low_terms: vec![0, 23],
            },
        ] {
            assert_eq!(
                build(
                    &bad,
                    1,
                    &[1, 2],
                    0,
                    Limits::default(),
                    deadline(),
                    &mut |_| {
                        calls += 1;
                        Ok(())
                    }
                )
                .unwrap_err()
                .stop,
                Stop::InvalidInput
            );
        }
        assert_eq!(calls, 0);
    }

    #[test]
    fn bounded_s4_checked_accounting_and_raw_pair_refusal() {
        let mut calls = 0;
        let mut observer = |_| {
            calls += 1;
            Ok(())
        };
        let mut gate = Gate {
            limits: Limits::default(),
            reserved: Reserved {
                work: u64::MAX,
                bytes: 0,
            },
            deadline: deadline(),
            observer: &mut observer,
        };
        assert_eq!(gate.admit(1, 0, Stage::Product), Err(Stop::Overflow));
        gate.reserved = Reserved {
            work: 0,
            bytes: u64::MAX,
        };
        assert_eq!(gate.admit(0, 1, Stage::Product), Err(Stop::Overflow));
        gate.reserved = Reserved::default();
        // Test rejection before touching the deliberately absent field table.
        let a = FieldSum(vec![(0, 1); 257]);
        let st = FieldStructure {
            n: 23,
            reduced: Vec::new(),
            squares: Vec::new(),
        };
        assert!(matches!(gate.mul(&a, &a, &st), Err(Stop::PairLimit)));
        assert_eq!(gate.reserved, Reserved::default());
        assert_eq!(calls, 0);
    }
}
