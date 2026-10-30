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
    OutputLimit,
    Serialization,
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

#[derive(Clone, Copy, Debug)]
pub(super) struct AffinePoint {
    pub x: u64,
    pub y: u64,
}
/// Producer-owned original input view. Structural validation is mandatory in
/// encode_input; an untrusted verifier must additionally reconstruct equations
/// and validate the curve/domain instead of trusting this representation.
pub(super) struct ChainInputView<'a> {
    pub field: &'a Fp64Arithmetic,
    pub parameters: Parameters<'a>,
    pub target: AffinePoint,
    pub signed_domain: &'a [AffinePoint],
    pub polynomials: &'a [InputPolynomial],
}

struct Decimal(u64);
impl serde::Serialize for Decimal {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut digits = [0u8; 20];
        let mut start = digits.len();
        let mut value = self.0;
        loop {
            start -= 1;
            digits[start] = b'0' + (value % 10) as u8;
            value /= 10;
            if value == 0 {
                break;
            }
        }
        serializer.serialize_str(std::str::from_utf8(&digits[start..]).expect("decimal digits"))
    }
}
struct VariableName {
    index: usize,
    m: usize,
}
impl serde::Serialize for VariableName {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut name = [0u8; 3];
        let (prefix, index) = if self.index < self.m {
            (b'x', self.index)
        } else {
            (b'u', self.index - self.m)
        };
        name[0] = prefix;
        let len = if index < 10 {
            name[1] = b'0' + index as u8;
            2
        } else {
            name[1] = b'0' + (index / 10) as u8;
            name[2] = b'0' + (index % 10) as u8;
            3
        };
        serializer.serialize_str(std::str::from_utf8(&name[..len]).expect("bounded variable name"))
    }
}
struct Sequence<F> {
    len: usize,
    item: F,
}
impl<F, T> serde::Serialize for Sequence<F>
where
    F: Fn(usize) -> T,
    T: serde::Serialize,
{
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(self.len))?;
        for i in 0..self.len {
            seq.serialize_element(&(self.item)(i))?;
        }
        seq.end()
    }
}
#[derive(serde::Serialize)]
struct PointView {
    x: Decimal,
    y: Decimal,
}
impl From<AffinePoint> for PointView {
    fn from(point: AffinePoint) -> Self {
        Self {
            x: Decimal(point.x),
            y: Decimal(point.y),
        }
    }
}
#[derive(serde::Serialize)]
struct CurveView {
    a: Decimal,
    b: Decimal,
}
#[derive(serde::Serialize)]
struct RoleView {
    index: usize,
    name: VariableName,
    role: &'static str,
    domain: &'static str,
}
#[derive(serde::Serialize)]
struct TermView<'a> {
    exponents: &'a [u16],
    coefficient: Decimal,
}
impl serde::Serialize for ChainInputView<'_> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let Parameters {
            m,
            a,
            b,
            target_x,
            codes,
        } = self.parameters;
        let v = 2 * m - 2;
        let mut state = serializer.serialize_struct("OriginalPrimeChain", 24)?;
        state.serialize_field("schema", "ml-dataset.prime-f4-chain-input/v1")?;
        state.serialize_field("strategy", super::PRIME_CHAIN_STRATEGY)?;
        state.serialize_field("m", &m)?;
        state.serialize_field("n_vars", &v)?;
        state.serialize_field("modulus", &Decimal(self.field.p()))?;
        state.serialize_field(
            "curve",
            &CurveView {
                a: Decimal(a),
                b: Decimal(b),
            },
        )?;
        state.serialize_field("target", &PointView::from(self.target))?;
        state.serialize_field("target_x", &Decimal(target_x))?;
        state.serialize_field("ring", "Fp[x0,...,x(m-1),u0,...,u(m-3)]")?;
        state.serialize_field(
            "variables",
            &Sequence {
                len: v,
                item: |index: usize| VariableName { index, m },
            },
        )?;
        state.serialize_field(
            "variable_roles",
            &Sequence {
                len: v,
                item: |index: usize| RoleView {
                    index,
                    name: VariableName { index, m },
                    role: if index < m {
                        "factor_base_leaf"
                    } else {
                        "unrestricted_affine_auxiliary"
                    },
                    domain: if index < m { "factor_base_x" } else { "all_Fp" },
                },
            },
        )?;
        state.serialize_field("monomial_order", "degrevlex")?;
        state.serialize_field(
            "domains",
            &Sequence {
                len: v,
                item: |i: usize| (i < m).then_some(codes),
            },
        )?;
        state.serialize_field(
            "factor_base_x",
            &Sequence {
                len: codes.len(),
                item: |i: usize| Decimal(codes[i]),
            },
        )?;
        state.serialize_field(
            "signed_factor_base",
            &Sequence {
                len: self.signed_domain.len(),
                item: |i: usize| PointView::from(self.signed_domain[i]),
            },
        )?;
        state.serialize_field(
            "polynomials",
            &Sequence {
                len: self.polynomials.len(),
                item: |i: usize| {
                    let poly = &self.polynomials[i];
                    Sequence {
                        len: poly.terms.len(),
                        item: move |j: usize| TermView {
                            exponents: poly.terms[j]
                                .mono
                                .exponents(v)
                                .expect("validated variable count"),
                            coefficient: Decimal(poly.terms[j].coefficient),
                        },
                    }
                },
            },
        )?;
        state.serialize_field("chain_equation_count", &(m - 1))?;
        state.serialize_field("membership_equation_count", &m)?;
        state.serialize_field(
            "membership_variables",
            &Sequence {
                len: m,
                item: |i: usize| i,
            },
        )?;
        state.serialize_field("native_normalization_applied", &false)?;
        state.serialize_field(
            "coefficient_arithmetic",
            "u128 modular intermediates; merged nonzero terms, leading-first, no monic rescaling",
        )?;
        state.serialize_field("native_modulus_limit_exclusive", "4294967296")?;
        state.serialize_field("affine_chain_exceptional_infinity_branches_covered", &false)?;
        state.serialize_field(
            "negative_scope",
            "encoding-relative only; no exhaustive PDP nonexistence claim",
        )?;
        state.end()
    }
}
fn validate_view(view: &ChainInputView<'_>, ledger: &mut Ledger) -> Result<(), Stop> {
    ledger.check()?;
    let Parameters {
        m,
        a,
        b,
        target_x,
        codes,
    } = view.parameters;
    let p = view.field.p();
    if !(4..=15).contains(&m)
        || p < 5
        || codes.is_empty()
        || codes.len() > 32
        || view.signed_domain.is_empty()
        || view.signed_domain.len() > 64
        || view.polynomials.len() != 2 * m - 1
        || [a, b, target_x, view.target.x, view.target.y]
            .iter()
            .any(|&x| x >= p)
        || target_x != view.target.x
    {
        return Err(Stop::InvalidInput);
    }
    {
        let mut ticket = ledger.reserve(128 + codes.len() as u64 * 32, 0)?;
        ticket.spend(128)?;
        for (i, &x) in codes.iter().enumerate() {
            ticket.spend(32)?;
            if x >= p || (i > 0 && codes[i - 1] >= x) {
                return Err(Stop::InvalidInput);
            }
        }
    }
    for point in view.signed_domain {
        let mut ticket = ledger.reserve(64 + codes.len() as u64 * 16, 0)?;
        ticket.spend(64 + codes.len() as u64 * 16)?;
        if point.x >= p || point.y >= p || !codes.contains(&point.x) {
            return Err(Stop::InvalidInput);
        }
    }
    for x in codes {
        let mut ticket = ledger.reserve(view.signed_domain.len() as u64 * 16, 0)?;
        ticket.spend(view.signed_domain.len() as u64 * 16)?;
        if !view.signed_domain.iter().any(|point| point.x == *x) {
            return Err(Stop::InvalidInput);
        }
    }
    let v = 2 * m - 2;
    for (i, poly) in view.polynomials.iter().enumerate() {
        let limit = if i < m - 1 { 13 } else { codes.len() + 1 };
        if poly.terms.len() > limit {
            return Err(Stop::InvalidInput);
        }
        for (j, term) in poly.terms.iter().enumerate() {
            let work = 32 * v as u64 + 128;
            let mut ticket = ledger.reserve(work, 0)?;
            ticket.spend(work)?;
            if term.coefficient == 0
                || term.coefficient >= p
                || term
                    .mono
                    .exponents(v)
                    .ok_or(Stop::InvalidInput)?
                    .iter()
                    .any(|&e| e > 32)
                || term.mono.exponents(MAX_V).ok_or(Stop::InvalidInput)?[v..]
                    .iter()
                    .any(|&e| e != 0)
                || (j > 0 && cmp(&poly.terms[j - 1], term, v) != Ordering::Greater)
            {
                return Err(Stop::InvalidInput);
            }
        }
    }
    Ok(())
}

/// One caller-admitted slot; physical spare capacity is never usable budget.
pub(super) struct InputBuffer {
    bytes: Vec<u8>,
    logical_cap: usize,
}
impl InputBuffer {
    pub fn new(cap: usize, ledger: &mut Ledger) -> Result<Self, Stop> {
        if cap > 4 * 1024 * 1024 {
            return Err(Stop::StorageLimit);
        }
        Ok(Self {
            bytes: requested_vec::<u8>(cap, ledger)?,
            logical_cap: cap,
        })
    }
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}
struct MeteredWriter<'a> {
    ledger: &'a mut Ledger,
    output: Option<&'a mut Vec<u8>>,
    count: usize,
    cap: usize,
    stop: Option<Stop>,
}
impl std::io::Write for MeteredWriter<'_> {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        if self.stop.is_some() {
            return Err(std::io::ErrorKind::Other.into());
        }
        let result = (|| {
            let next = self
                .count
                .checked_add(bytes.len())
                .ok_or(Stop::ArithmeticOverflow)?;
            if next > self.cap {
                return Err(Stop::OutputLimit);
            }
            // At most 256 output bytes between original deadline checks.
            // Both count and write consume this schedule independently.
            for chunk in bytes.chunks(256) {
                let work = 8 * chunk.len() as u64 + 32;
                let mut ticket = self.ledger.reserve(work, 0)?;
                ticket.spend(work)?;
                if let Some(output) = self.output.as_mut() {
                    output.extend_from_slice(chunk);
                }
                self.count += chunk.len();
            }
            Ok(bytes.len())
        })();
        match result {
            Ok(n) => Ok(n),
            Err(stop) => {
                self.stop = Some(stop);
                Err(std::io::ErrorKind::Other.into())
            }
        }
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
/// Two independent metered passes. The buffer is logically empty on every
/// failure, so a censored prefix cannot be mistaken for a complete artifact.
pub(super) fn encode_input(
    view: &ChainInputView<'_>,
    output: &mut InputBuffer,
    ledger: &mut Ledger,
) -> Result<(), Stop> {
    output.bytes.clear();
    let result = (|| {
        validate_view(view, ledger)?;
        // Account fixed formatting/serializer/sink mathematical scratch. serde
        // machinery and errors remain within the existing outer process gate.
        let scratch = size_of::<[u8; 20]>()
            + size_of::<[u8; 3]>()
            + size_of::<MeteredWriter<'static>>()
            + size_of::<[usize; 32]>();
        {
            let mut ticket = ledger.reserve(scratch as u64 * 8, scratch as u64)?;
            ticket.spend(scratch as u64 * 8)?;
        }
        let mut counter = MeteredWriter {
            ledger,
            output: None,
            count: 0,
            cap: output.logical_cap,
            stop: None,
        };
        if serde_json::to_writer(&mut counter, view).is_err() {
            return Err(counter.stop.unwrap_or(Stop::Serialization));
        }
        let count = counter.count;
        let mut writer = MeteredWriter {
            ledger,
            output: Some(&mut output.bytes),
            count: 0,
            cap: output.logical_cap,
            stop: None,
        };
        if serde_json::to_writer(&mut writer, view).is_err() {
            return Err(writer.stop.unwrap_or(Stop::Serialization));
        }
        if writer.count != count {
            return Err(Stop::Serialization);
        }
        ledger.check()?;
        Ok(())
    })();
    if result.is_err() {
        output.bytes.clear();
    }
    result
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
    fn typed_view<'a>(
        field: &'a Fp64Arithmetic,
        polynomials: &'a [InputPolynomial],
        m: usize,
    ) -> ChainInputView<'a> {
        ChainInputView {
            field,
            parameters: Parameters {
                m,
                a: 1,
                b: 2,
                target_x: 1,
                codes: &[1],
            },
            target: AffinePoint { x: 1, y: 2 },
            signed_domain: &[AffinePoint { x: 1, y: 2 }, AffinePoint { x: 1, y: 99 }],
            polynomials,
        }
    }
    #[test]
    fn typed_input_schema_matches_legacy_for_every_arity() {
        use num_bigint::BigUint;
        let setup = field(101);
        let curve = super::super::curves::ExactPrimeCurve {
            p: 101u32.into(),
            a: 1u32.into(),
            b: 2u32.into(),
            gx: 1u32.into(),
            gy: 2u32.into(),
            n: 1u32.into(),
            cofactor: 1,
        };
        let point = |y: u64| {
            super::super::Element::Prime(crate::ecc::Point::Affine {
                x: curve.fe(BigUint::from(1u64)),
                y: curve.fe(BigUint::from(y)),
            })
        };
        let target = point(2);
        let domain = [point(2), point(99)];
        let group = super::super::Group::Prime(curve.clone());
        for m in 4..=15 {
            let params = Parameters {
                m,
                a: 1,
                b: 2,
                target_x: 1,
                codes: &[1],
            };
            let polys = build_original_chain(&setup.field, params, &mut ledger()).unwrap();
            let view = typed_view(&setup.field, &polys, m);
            let mut budget = ledger();
            let mut output = InputBuffer::new(131072, &mut budget).unwrap();
            encode_input(&view, &mut output, &mut budget).unwrap();
            let actual: serde_json::Value = serde_json::from_slice(output.bytes()).unwrap();
            let expected = super::super::prime_chain_input(m, &group, &domain, &target)
                .unwrap()
                .artifact;
            assert_eq!(actual, expected);
            assert_eq!(
                serde_json::to_vec(&actual).unwrap(),
                serde_json::to_vec(&expected).unwrap()
            );
        }
    }
    #[test]
    fn typed_input_exact_one_less_bytes_work_and_empty_failure() {
        let setup = field(101);
        let params = Parameters {
            m: 4,
            a: 1,
            b: 2,
            target_x: 1,
            codes: &[1],
        };
        let polys = build_original_chain(&setup.field, params, &mut ledger()).unwrap();
        let view = typed_view(&setup.field, &polys, 4);
        let mut initial = ledger();
        let mut generous = InputBuffer::new(131072, &mut initial).unwrap();
        encode_input(&view, &mut generous, &mut initial).unwrap();
        let size = generous.bytes().len();
        let mut measured = ledger();
        let mut exact = InputBuffer::new(size, &mut measured).unwrap();
        encode_input(&view, &mut exact, &mut measured).unwrap();
        assert_eq!(exact.bytes(), generous.bytes());
        let work = measured.reserved_work;
        let bytes = measured.reserved_bytes;
        let mut exact_budget = Ledger::new(work, bytes, Instant::now() + Duration::from_secs(30));
        let mut exact_again = InputBuffer::new(size, &mut exact_budget).unwrap();
        encode_input(&view, &mut exact_again, &mut exact_budget).unwrap();
        assert_eq!(exact_again.bytes(), exact.bytes());
        let mut low_work = Ledger::new(work - 1, bytes, Instant::now() + Duration::from_secs(30));
        let mut failed = InputBuffer::new(size, &mut low_work).unwrap();
        assert_eq!(
            encode_input(&view, &mut failed, &mut low_work),
            Err(Stop::WorkLimit)
        );
        assert!(failed.bytes().is_empty());
        assert!(low_work.reserved_work > 0);
        // Physical storage is large enough; the logical cap still wins.
        generous.logical_cap = size - 1;
        assert_eq!(
            encode_input(&view, &mut generous, &mut ledger()),
            Err(Stop::OutputLimit)
        );
        assert!(generous.bytes().is_empty());
        let mut low_bytes = Ledger::new(work, bytes - 1, Instant::now() + Duration::from_secs(30));
        let mut output = InputBuffer::new(size, &mut low_bytes).unwrap();
        assert_eq!(
            encode_input(&view, &mut output, &mut low_bytes),
            Err(Stop::StorageLimit)
        );
        assert!(output.bytes().is_empty());
    }
    #[test]
    fn typed_input_rejects_dimensions_coordinates_coverage_and_coefficients() {
        let setup = field(101);
        let params = Parameters {
            m: 4,
            a: 1,
            b: 2,
            target_x: 1,
            codes: &[1],
        };
        let mut polys = build_original_chain(&setup.field, params, &mut ledger()).unwrap();
        let mut output = InputBuffer::new(131072, &mut ledger()).unwrap();
        let mut view = typed_view(&setup.field, &polys, 4);
        view.target.x = 2;
        assert_eq!(
            encode_input(&view, &mut output, &mut ledger()),
            Err(Stop::InvalidInput)
        );
        view.target.x = 1;
        view.signed_domain = &[AffinePoint { x: 2, y: 2 }];
        assert_eq!(
            encode_input(&view, &mut output, &mut ledger()),
            Err(Stop::InvalidInput)
        );
        view.signed_domain = &[AffinePoint { x: 1, y: 101 }];
        assert_eq!(
            encode_input(&view, &mut output, &mut ledger()),
            Err(Stop::InvalidInput)
        );
        view.signed_domain = &[AffinePoint { x: 1, y: 2 }; 65];
        assert_eq!(
            encode_input(&view, &mut output, &mut ledger()),
            Err(Stop::InvalidInput)
        );
        view.signed_domain = &[AffinePoint { x: 1, y: 2 }];
        view.parameters.codes = &[1, 2];
        assert_eq!(
            encode_input(&view, &mut output, &mut ledger()),
            Err(Stop::InvalidInput)
        );
        view.parameters.m = usize::MAX;
        assert_eq!(
            encode_input(&view, &mut output, &mut ledger()),
            Err(Stop::InvalidInput)
        );
        polys[0].terms[0].coefficient = 101;
        let view = typed_view(&setup.field, &polys, 4);
        assert_eq!(
            encode_input(&view, &mut output, &mut ledger()),
            Err(Stop::InvalidInput)
        );
        assert!(output.bytes().is_empty());
    }
    #[test]
    fn typed_input_maximum_shape_fits_original_algebra_allowance() {
        let p = (1u64 << 61) - 1;
        let setup = field(p);
        let codes: [u64; 32] = std::array::from_fn(|i| p - 32 + i as u64);
        // Structural canonical schema fixture only; arbitrary y values are not
        // claimed to be on-curve. The all-arity test above uses actual points.
        let signed: [AffinePoint; 64] = std::array::from_fn(|i| AffinePoint {
            x: codes[i / 2],
            y: if i % 2 == 0 { 1 } else { p - 1 },
        });
        let params = Parameters {
            m: 15,
            a: p - 1,
            b: p - 2,
            target_x: p - 3,
            codes: &codes,
        };
        let mut budget = Ledger::new(
            3 * 16_777_216 / 4,
            64 * 1024 * 1024,
            Instant::now() + Duration::from_secs(30),
        );
        let polys = build_original_chain(&setup.field, params, &mut budget).unwrap();
        let view = ChainInputView {
            field: &setup.field,
            parameters: params,
            target: AffinePoint { x: p - 3, y: 1 },
            signed_domain: &signed,
            polynomials: &polys,
        };
        // Approved maximum E input slot, including its fixed R/S reservations.
        let cap = (4 * 1024 * 1024 - 65536 - 4096) / 3;
        let mut output = InputBuffer::new(cap, &mut budget).unwrap();
        encode_input(&view, &mut output, &mut budget).unwrap();
        assert!(output.bytes().len() <= cap);
        eprintln!(
            "dynamic_input_max_shape bytes={} reserved_work={} reserved_bytes={}",
            output.bytes().len(),
            budget.reserved_work,
            budget.reserved_bytes
        );
        assert!(budget.reserved_work <= 3 * 16_777_216 / 4);
        assert!(budget.reserved_bytes <= 64 * 1024 * 1024);
        let value: serde_json::Value = serde_json::from_slice(output.bytes()).unwrap();
        assert_eq!(value["polynomials"].as_array().unwrap().len(), 29);
        assert_eq!(value["signed_factor_base"].as_array().unwrap().len(), 64);
    }
}
