//! Admitted original S3-chain construction. Not a dispatched solver profile.
//!
//! The caller owns field setup, public/domain validation and all later phases.
//! This child deliberately has no group search, native invocation or JSON tree.
#![allow(dead_code)] // Incremental private foundation; activation has separate gates.

use crate::cryptanalysis::f4_tower_generic::dynamic::{CanonicalTerm, InputPolynomial, Mono28};
use crate::cryptanalysis::fp_backend::Fp64Arithmetic;
use std::alloc::Layout;
use std::cmp::Ordering;
use std::mem::size_of;
use std::time::Instant;

const MAX_V: usize = 28;
const MAX_K: usize = 32;
const MAX_TERMS: usize = 33;
const COPY_WORK: u64 = 8 * (size_of::<CanonicalTerm>().div_ceil(8) as u64 + 1);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Stop {
    InvalidInput,
    WorkLimit,
    StorageLimit,
    Deadline,
    Allocation,
    ArithmeticOverflow,
    TicketExhausted,
}

/// One pool of the parent attempt. Callers retain it across all invocations.
/// Reserved work is nonrefundable; performed work is the executed tariff,
/// not a hardware instruction counter. Requested bytes are cumulative.
#[derive(Debug)]
pub(super) struct Ledger {
    work_limit: u64,
    bytes_limit: u64,
    pub reserved_work: u64,
    pub performed_work: u64,
    pub reserved_bytes: u64,
    deadline: Instant,
}
impl Ledger {
    pub fn new(work_limit: u64, bytes_limit: u64, deadline: Instant) -> Self {
        Self {
            work_limit,
            bytes_limit,
            reserved_work: 0,
            performed_work: 0,
            reserved_bytes: 0,
            deadline,
        }
    }
    fn check(&self) -> Result<(), Stop> {
        if Instant::now() >= self.deadline {
            Err(Stop::Deadline)
        } else {
            Ok(())
        }
    }
    fn reserve(&mut self, work: u64, bytes: u64) -> Result<Ticket<'_>, Stop> {
        self.check()?;
        let next_work = self
            .reserved_work
            .checked_add(work)
            .ok_or(Stop::ArithmeticOverflow)?;
        let next_bytes = self
            .reserved_bytes
            .checked_add(bytes)
            .ok_or(Stop::ArithmeticOverflow)?;
        if next_work > self.work_limit {
            return Err(Stop::WorkLimit);
        }
        if next_bytes > self.bytes_limit {
            return Err(Stop::StorageLimit);
        }
        // Both admissions are atomic; no partially charged rejected reservation.
        self.reserved_work = next_work;
        self.reserved_bytes = next_bytes;
        Ok(Ticket {
            remaining: work,
            performed: &mut self.performed_work,
            deadline: self.deadline,
        })
    }
}
struct Ticket<'a> {
    remaining: u64,
    performed: &'a mut u64,
    deadline: Instant,
}
impl Ticket<'_> {
    fn spend(&mut self, work: u64) -> Result<(), Stop> {
        if Instant::now() >= self.deadline {
            return Err(Stop::Deadline);
        }
        self.remaining = self
            .remaining
            .checked_sub(work)
            .ok_or(Stop::TicketExhausted)?;
        *self.performed = self
            .performed
            .checked_add(work)
            .ok_or(Stop::ArithmeticOverflow)?;
        Ok(())
    }
}

/// Already bounded canonical public scalars. Sorted codes are distinct x values,
/// not signed points: P and -P share a code. No silent reduction or truncation.
#[derive(Clone, Copy)]
pub(super) struct Parameters<'a> {
    pub m: usize,
    pub a: u64,
    pub b: u64,
    pub target_x: u64,
    pub codes: &'a [u64],
}
struct Scratch {
    membership: [u64; MAX_TERMS],
    next: [u64; MAX_TERMS],
    terms: [CanonicalTerm; MAX_TERMS],
    exponents: [u16; MAX_V],
}
// Additional live mathematical values: raw S3 template, coordinate map,
// canonical zero/key temporaries, parameters and bounded scalar cursors. This
// is an explicit requested-layout envelope, not compiler stack-frame/RSS size.
const AUXILIARY_BYTES: usize = size_of::<[([u16; 3], u64); 13]>()
    + size_of::<[Option<usize>; 3]>()
    + 2 * size_of::<CanonicalTerm>()
    + size_of::<Parameters<'static>>()
    + size_of::<[usize; 16]>()
    + size_of::<[u64; 16]>();
fn add(a: u64, b: u64, p: u64) -> u64 {
    ((u128::from(a) + u128::from(b)) % u128::from(p)) as u64
}
fn mul(a: u64, b: u64, p: u64) -> u64 {
    ((u128::from(a) * u128::from(b)) % u128::from(p)) as u64
}
fn neg(a: u64, p: u64) -> u64 {
    if a == 0 {
        0
    } else {
        p - a
    }
}
fn request_bytes<T>(n: usize) -> Result<u64, Stop> {
    let layout = Layout::array::<T>(n).map_err(|_| Stop::ArithmeticOverflow)?;
    u64::try_from(layout.size())
        .map_err(|_| Stop::ArithmeticOverflow)?
        .checked_add(128)
        .ok_or(Stop::ArithmeticOverflow)
}
fn requested_vec<T>(n: usize, ledger: &mut Ledger) -> Result<Vec<T>, Stop> {
    let mut ticket = ledger.reserve(32, request_bytes::<T>(n)?)?;
    ticket.spend(32)?;
    let mut result = Vec::new();
    result.try_reserve_exact(n).map_err(|_| Stop::Allocation)?;
    // Capacity may exceed n: all callers use their admitted logical n only.
    Ok(result)
}
fn cmp(a: &CanonicalTerm, b: &CanonicalTerm, v: usize) -> Ordering {
    let a = a.mono.exponents(v).expect("validated variable count");
    let b = b.mono.exponents(v).expect("validated variable count");
    let da: u32 = a.iter().map(|&e| u32::from(e)).sum();
    let db: u32 = b.iter().map(|&e| u32::from(e)).sum();
    da.cmp(&db).then_with(|| {
        (0..v)
            .rev()
            .find_map(|i| {
                let c = b[i].cmp(&a[i]);
                (c != Ordering::Equal).then_some(c)
            })
            .unwrap_or(Ordering::Equal)
    })
}
fn canonicalize(
    s: &mut Scratch,
    n: usize,
    v: usize,
    p: u64,
    ledger: &mut Ledger,
) -> Result<usize, Stop> {
    let comparison = 16 * v as u64 + 8;
    let steps = n * n.saturating_sub(1) / 2;
    // Insertion comparisons/shifts, key reads/writes, then full adjacent merge.
    let reserve = steps as u64 * (comparison + COPY_WORK)
        + n as u64 * (2 * COPY_WORK + comparison + COPY_WORK + 32);
    let mut ticket = ledger.reserve(reserve, 0)?;
    for i in 1..n {
        ticket.spend(COPY_WORK)?;
        let key = s.terms[i];
        let mut j = i;
        while j > 0 {
            ticket.spend(comparison)?;
            if cmp(&key, &s.terms[j - 1], v) != Ordering::Greater {
                break;
            }
            ticket.spend(COPY_WORK)?;
            s.terms[j] = s.terms[j - 1];
            j -= 1;
        }
        ticket.spend(COPY_WORK)?;
        s.terms[j] = key;
    }
    // Merge before filtering so a cancellation cannot leave duplicate terms.
    let mut merged = 0;
    for i in 0..n {
        ticket.spend(comparison + COPY_WORK + 32)?;
        let term = s.terms[i];
        if merged > 0 && s.terms[merged - 1].mono == term.mono {
            s.terms[merged - 1].coefficient =
                add(s.terms[merged - 1].coefficient, term.coefficient, p);
        } else {
            s.terms[merged] = term;
            merged += 1;
        }
    }
    let mut ticket = ledger.reserve(merged as u64 * (COPY_WORK + 8), 0)?;
    let mut kept = 0;
    for i in 0..merged {
        ticket.spend(COPY_WORK + 8)?;
        if s.terms[i].coefficient != 0 {
            s.terms[kept] = s.terms[i];
            kept += 1;
        }
    }
    Ok(kept)
}
fn retain(s: &Scratch, n: usize, ledger: &mut Ledger) -> Result<InputPolynomial, Stop> {
    let mut terms = requested_vec::<CanonicalTerm>(n, ledger)?;
    let mut ticket = ledger.reserve(n as u64 * COPY_WORK, 0)?;
    for term in &s.terms[..n] {
        ticket.spend(COPY_WORK)?;
        terms.push(*term);
    }
    Ok(InputPolynomial { terms })
}

/// Original S3 equations followed by m membership equations, leading-first
/// grevlex, merged nonzero canonical coefficients, without monic rescaling.
pub(super) fn build_original_chain(
    field: &Fp64Arithmetic,
    params: Parameters<'_>,
    ledger: &mut Ledger,
) -> Result<Vec<InputPolynomial>, Stop> {
    ledger.check()?;
    let p = field.p();
    let Parameters {
        m,
        a,
        b,
        target_x,
        codes,
    } = params;
    // Lengths first, so even adversarial borrowed slices are never scanned
    // outside the finite admitted validation schedule.
    if !(4..=15).contains(&m)
        || !(5..=0x1fff_ffff_ffff_ffff).contains(&p)
        || codes.is_empty()
        || codes.len() > MAX_K
    {
        return Err(Stop::InvalidInput);
    }
    {
        let mut ticket = ledger.reserve(64 + codes.len() as u64 * 32, 0)?;
        ticket.spend(64)?;
        if [a, b, target_x].iter().any(|&x| x >= p) {
            return Err(Stop::InvalidInput);
        }
        let mut previous = None;
        for &x in codes {
            ticket.spend(32)?;
            if x >= p || previous.is_some_and(|old| old >= x) {
                return Err(Stop::InvalidInput);
            }
            previous = Some(x);
        }
    }
    let v = 2 * m - 2;
    let equations = 2 * m - 1;
    // Fixed scratch admission precedes initialization, independently of K.
    {
        let mut ticket = ledger.reserve(
            ((size_of::<Scratch>() + AUXILIARY_BYTES) as u64) * 8,
            (size_of::<Scratch>() + AUXILIARY_BYTES) as u64,
        )?;
        ticket.spend(((size_of::<Scratch>() + AUXILIARY_BYTES) as u64) * 8)?;
    }
    let zero = CanonicalTerm {
        mono: Mono28::new(&[0; MAX_V][..v], v).ok_or(Stop::InvalidInput)?,
        coefficient: 0,
    };
    let mut s = Scratch {
        membership: [0; MAX_TERMS],
        next: [0; MAX_TERMS],
        terms: [zero; MAX_TERMS],
        exponents: [0; MAX_V],
    };
    let mut out = requested_vec::<InputPolynomial>(equations, ledger)?;
    let raw = {
        let mut ticket = ledger.reserve(512, 0)?;
        ticket.spend(512)?;
        [
            ([2, 2, 0], 1),
            ([2, 0, 2], 1),
            ([0, 2, 2], 1),
            ([2, 1, 1], neg(2, p)),
            ([1, 2, 1], neg(2, p)),
            ([1, 1, 2], neg(2, p)),
            ([1, 1, 0], neg(mul(2, a, p), p)),
            ([1, 0, 1], neg(mul(2, a, p), p)),
            ([0, 1, 1], neg(mul(2, a, p), p)),
            ([1, 0, 0], neg(mul(4, b, p), p)),
            ([0, 1, 0], neg(mul(4, b, p), p)),
            ([0, 0, 1], neg(mul(4, b, p), p)),
            ([0, 0, 0], mul(a, a, p)),
        ]
    };
    for equation in 0..m - 1 {
        let variables = if equation == 0 {
            [Some(0), Some(1), Some(m)]
        } else if equation == m - 2 {
            [Some(2 * m - 3), Some(m - 1), None]
        } else {
            [
                Some(m + equation - 1),
                Some(equation + 1),
                Some(m + equation),
            ]
        };
        for (i, &(powers, mut coefficient)) in raw.iter().enumerate() {
            // <= 2V coordinate copies/zeros, three updates, <=two target muls.
            let work = 8 * (MAX_V + 2 * v) as u64 + 128;
            let mut ticket = ledger.reserve(work, 0)?;
            ticket.spend(work)?;
            s.exponents[..v].fill(0);
            for (axis, power) in powers.into_iter().enumerate() {
                if let Some(variable) = variables[axis] {
                    s.exponents[variable] = s.exponents[variable]
                        .checked_add(power)
                        .ok_or(Stop::ArithmeticOverflow)?;
                } else {
                    for _ in 0..power {
                        coefficient = mul(coefficient, target_x, p);
                    }
                }
            }
            s.terms[i] = CanonicalTerm {
                mono: Mono28::new(&s.exponents[..v], v).ok_or(Stop::InvalidInput)?,
                coefficient,
            };
        }
        let n = canonicalize(&mut s, 13, v, p, ledger)?;
        out.push(retain(&s, n, ledger)?);
    }
    s.membership[0] = 1;
    for (i, &x) in codes.iter().enumerate() {
        {
            let mut ticket = ledger.reserve((i + 2) as u64 * 8, 0)?;
            ticket.spend((i + 2) as u64 * 8)?;
            s.next[..i + 2].fill(0);
        }
        for power in 0..=i {
            let mut ticket = ledger.reserve(128, 0)?;
            ticket.spend(128)?;
            let coefficient = s.membership[power];
            s.next[power] = add(s.next[power], neg(mul(coefficient, x, p), p), p);
            s.next[power + 1] = add(s.next[power + 1], coefficient, p);
        }
        let mut ticket = ledger.reserve(2 * MAX_TERMS as u64 * 8, 0)?;
        ticket.spend(2 * MAX_TERMS as u64 * 8)?;
        std::mem::swap(&mut s.membership, &mut s.next);
    }
    for leaf in 0..m {
        for power in 0..=codes.len() {
            let work = 8 * (MAX_V + 2 * v) as u64 + 128;
            let mut ticket = ledger.reserve(work, 0)?;
            ticket.spend(work)?;
            s.exponents[..v].fill(0);
            s.exponents[leaf] = power as u16;
            s.terms[power] = CanonicalTerm {
                mono: Mono28::new(&s.exponents[..v], v).ok_or(Stop::InvalidInput)?,
                coefficient: s.membership[power],
            };
        }
        let n = canonicalize(&mut s, codes.len() + 1, v, p, ledger)?;
        out.push(retain(&s, n, ledger)?);
    }
    ledger.check()?;
    debug_assert_eq!(out.len(), equations);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    fn field(p: u64) -> crate::cryptanalysis::fp_backend::FieldSetup {
        Fp64Arithmetic::try_new(
            p,
            1_000_000,
            4096,
            Instant::now() + Duration::from_secs(30),
            &mut |_| Ok(()),
        )
        .unwrap()
    }
    fn ledger() -> Ledger {
        Ledger::new(
            100_000_000,
            1_000_000,
            Instant::now() + Duration::from_secs(30),
        )
    }
    fn legacy(typed: &[InputPolynomial], v: usize) -> Vec<crate::cryptanalysis::f4_fp::Poly> {
        typed
            .iter()
            .map(|f| {
                f.terms
                    .iter()
                    .map(|t| {
                        (
                            t.mono
                                .exponents(v)
                                .unwrap()
                                .iter()
                                .map(|&e| u32::from(e))
                                .collect(),
                            t.coefficient,
                        )
                    })
                    .collect()
            })
            .collect()
    }
    #[test]
    fn all_arities_original_equations_match_legacy_target_and_cancellation_cases() {
        let setup = field(101);
        for m in 4..=15 {
            for (a, b, target_x) in [(0, 0, 0), (1, 0, 1), (0, 1, 100), (100, 100, 42)] {
                for codes in [&[0][..], &[0, 1, 100][..], &[2, 3][..]] {
                    let typed = build_original_chain(
                        &setup.field,
                        Parameters {
                            m,
                            a,
                            b,
                            target_x,
                            codes,
                        },
                        &mut ledger(),
                    )
                    .unwrap();
                    assert_eq!(
                        legacy(&typed, 2 * m - 2),
                        super::super::prime_chain_polynomials(m, 101, a, b, target_x, codes)
                            .unwrap()
                    );
                    assert!(typed.iter().all(|f| f
                        .terms
                        .iter()
                        .all(|t| t.coefficient > 0 && t.coefficient < 101)));
                }
            }
        }
    }
    #[test]
    fn greatest_shape_m15_k32_wide_prime_matches_original() {
        let p = (1u64 << 61) - 1;
        let setup = field(p);
        let codes: [u64; 32] = std::array::from_fn(|i| p - 32 + i as u64);
        let params = Parameters {
            m: 15,
            a: p - 1,
            b: p - 2,
            target_x: p - 3,
            codes: &codes,
        };
        // Actual approved algebra pool and mathematical requested-byte ceiling.
        // This does not promise the remaining native/extraction phases fit.
        let mut budget = Ledger::new(
            3 * 16_777_216 / 4,
            64 * 1024 * 1024,
            Instant::now() + Duration::from_secs(30),
        );
        let typed = build_original_chain(&setup.field, params, &mut budget).unwrap();
        assert!(budget.reserved_work <= 3 * 16_777_216 / 4);
        assert!(budget.reserved_bytes <= 64 * 1024 * 1024);
        assert_eq!(typed.len(), 29);
        assert!(typed.iter().map(|f| f.terms.len()).sum::<usize>() <= 677);
        assert_eq!(
            legacy(&typed, 28),
            super::super::prime_chain_polynomials(15, p, p - 1, p - 2, p - 3, &codes).unwrap()
        );
        assert!(budget.performed_work <= budget.reserved_work);
    }
    #[test]
    fn exact_and_one_less_total_work_and_requested_storage() {
        let setup = field(101);
        let params = Parameters {
            m: 4,
            a: 1,
            b: 2,
            target_x: 3,
            codes: &[1, 3],
        };
        let mut measured = ledger();
        let expected = build_original_chain(&setup.field, params, &mut measured).unwrap();
        let w = measured.reserved_work;
        let b = measured.reserved_bytes;
        for (work, bytes, stop) in [
            (w, b, None),
            (w - 1, b, Some(Stop::WorkLimit)),
            (w, b - 1, Some(Stop::StorageLimit)),
        ] {
            let mut budget = Ledger::new(work, bytes, Instant::now() + Duration::from_secs(30));
            let result = build_original_chain(&setup.field, params, &mut budget);
            if let Some(stop) = stop {
                assert_eq!(result.unwrap_err(), stop);
            } else {
                assert_eq!(result.unwrap(), expected);
            }
            assert!(budget.reserved_work <= work && budget.reserved_bytes <= bytes);
        }
    }
    #[test]
    fn expired_deadline_and_invalid_canonical_input_fail_closed() {
        let setup = field(101);
        let params = Parameters {
            m: 4,
            a: 1,
            b: 2,
            target_x: 3,
            codes: &[1],
        };
        let mut expired = Ledger::new(u64::MAX, u64::MAX, Instant::now());
        assert_eq!(
            build_original_chain(&setup.field, params, &mut expired).unwrap_err(),
            Stop::Deadline
        );
        assert_eq!((expired.reserved_work, expired.reserved_bytes), (0, 0));
        for bad in [
            Parameters { m: 3, ..params },
            Parameters { m: 16, ..params },
            Parameters { a: 101, ..params },
            Parameters { b: 101, ..params },
            Parameters {
                target_x: 101,
                ..params
            },
            Parameters {
                codes: &[],
                ..params
            },
            Parameters {
                codes: &[101],
                ..params
            },
            Parameters {
                codes: &[1, 1],
                ..params
            },
            Parameters {
                codes: &[2, 1],
                ..params
            },
            Parameters {
                codes: &[0; 33],
                ..params
            },
        ] {
            assert_eq!(
                build_original_chain(&setup.field, bad, &mut ledger()).unwrap_err(),
                Stop::InvalidInput
            );
        }
    }
    #[test]
    fn parent_reservations_are_nonrefundable_and_repeated_calls_share_allowance() {
        let setup = field(101);
        let params = Parameters {
            m: 4,
            a: 1,
            b: 2,
            target_x: 3,
            codes: &[1],
        };
        let mut budget = ledger();
        build_original_chain(&setup.field, params, &mut budget).unwrap();
        let first = (
            budget.reserved_work,
            budget.reserved_bytes,
            budget.performed_work,
        );
        build_original_chain(&setup.field, params, &mut budget).unwrap();
        assert_eq!(
            (
                budget.reserved_work,
                budget.reserved_bytes,
                budget.performed_work
            ),
            (2 * first.0, 2 * first.1, 2 * first.2)
        );
        let mut only_one = Ledger::new(first.0, first.1, Instant::now() + Duration::from_secs(30));
        build_original_chain(&setup.field, params, &mut only_one).unwrap();
        assert_eq!(
            build_original_chain(&setup.field, params, &mut only_one).unwrap_err(),
            Stop::WorkLimit
        );
        let mut admission = Ledger::new(10, 10, Instant::now() + Duration::from_secs(30));
        {
            let mut ticket = admission.reserve(10, 10).unwrap();
            ticket.spend(2).unwrap();
        }
        assert_eq!(
            (
                admission.reserved_work,
                admission.performed_work,
                admission.reserved_bytes
            ),
            (10, 2, 10)
        );
        assert!(matches!(admission.reserve(1, 0), Err(Stop::WorkLimit)));
    }
    #[test]
    fn atomic_admission_and_local_ticket_cannot_overspend() {
        let mut budget = Ledger::new(10, 10, Instant::now() + Duration::from_secs(30));
        assert!(matches!(budget.reserve(5, 11), Err(Stop::StorageLimit)));
        assert_eq!((budget.reserved_work, budget.reserved_bytes), (0, 0));
        let mut ticket = budget.reserve(10, 10).unwrap();
        assert_eq!(ticket.spend(11), Err(Stop::TicketExhausted));
        assert_eq!(*ticket.performed, 0);
    }
}
