//! Admitted original S3-chain construction. Not a dispatched solver profile.
//!
//! The caller owns field setup, public/domain validation and all later phases.
//! This child has no group search, runtime dispatch or intermediate JSON tree.
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
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
    Observer,
    Setup,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
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

// Native execution is a private phase, not dispatcher activation.
use super::super::capture::TypedEventBuffer;
use crate::cryptanalysis::f4_tower_generic::dynamic;
use crate::cryptanalysis::fp_backend::{FieldSetup, FieldSetupFailure, FieldSetupStage};
use std::time::Duration;
const TERMINAL_WORK: u64 = 262_144;
const CALLBACK_WORK: u64 = 81_920; // two bounded capture passes, <=1024 bytes each
const CAPTURE_CAP: u64 = 1024;

#[derive(Clone, Copy)]
pub(super) struct AttemptLimits {
    pub work_limit: u64,
    pub memory_bytes: u64,
    pub trace_bytes: u64,
    pub degree_bound: u32,
    pub original_deadline: Instant,
}
#[derive(Clone, Copy, Debug, serde::Serialize)]
pub(super) struct Supply {
    pub work: u64,
    pub bytes: u64,
}
#[derive(Clone, Copy, Debug, serde::Serialize)]
pub(super) struct Settlement {
    pub supplied: Supply,
    pub consumed: Supply,
}
struct Escrow {
    supplied: Supply,
    settled: bool,
}
impl Escrow {
    fn new(supplied: Supply) -> Self {
        Self {
            supplied,
            settled: false,
        }
    }
}
/// Borrowing the owner exclusively prevents concurrent spending of this escrow.
/// Settlement intentionally has no clock check: expired work is still charged.
fn settle(
    ledger: &mut Ledger,
    escrow: &mut Escrow,
    consumed: Supply,
    performed: u64,
) -> Result<Settlement, Stop> {
    if escrow.settled {
        return Err(Stop::InvalidInput);
    }
    let supplied = escrow.supplied;
    if consumed.work > supplied.work || consumed.bytes > supplied.bytes || performed > consumed.work
    {
        return Err(Stop::InvalidInput);
    }
    let w = ledger
        .reserved_work
        .checked_add(consumed.work)
        .ok_or(Stop::ArithmeticOverflow)?;
    let b = ledger
        .reserved_bytes
        .checked_add(consumed.bytes)
        .ok_or(Stop::ArithmeticOverflow)?;
    let done = ledger
        .performed_work
        .checked_add(performed)
        .ok_or(Stop::ArithmeticOverflow)?;
    if w > ledger.work_limit || b > ledger.bytes_limit {
        return Err(Stop::InvalidInput);
    }
    ledger.reserved_work = w;
    ledger.reserved_bytes = b;
    ledger.performed_work = done;
    escrow.settled = true;
    Ok(Settlement { supplied, consumed })
}
fn remaining(ledger: &Ledger) -> Supply {
    Supply {
        work: ledger.work_limit - ledger.reserved_work,
        bytes: ledger.bytes_limit - ledger.reserved_bytes,
    }
}

struct Slots {
    buffers: [InputBuffer; 8],
}
impl Slots {
    fn new(e: usize, ledger: &mut Ledger) -> Result<Self, Stop> {
        if e <= 4 * 65536 || e > 4 * 1024 * 1024 {
            return Err(Stop::InvalidInput);
        }
        let a = e - 65536 - 4096;
        let caps = [a / 3, 4096, a / 4, a / 12, a / 12, a / 6, a / 12, 65536];
        Ok(Self {
            buffers: [
                InputBuffer::new(caps[0], ledger)?,
                InputBuffer::new(caps[1], ledger)?,
                InputBuffer::new(caps[2], ledger)?,
                InputBuffer::new(caps[3], ledger)?,
                InputBuffer::new(caps[4], ledger)?,
                InputBuffer::new(caps[5], ledger)?,
                InputBuffer::new(caps[6], ledger)?,
                InputBuffer::new(caps[7], ledger)?,
            ],
        })
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum SmallPoint {
    Infinity,
    Affine(AffinePoint),
}
struct GroupWorkspace {
    domain: [SmallPoint; 64],
    cursor: [usize; 15],
    tuple: [usize; 15],
    prefix: [SmallPoint; 15],
    coordinates: [u64; 28],
    residues: [u64; 29],
    first_certificate: [usize; 15],
    exceptional: [usize; 15],
    assignments: [Option<u64>; 28],
    sources: [usize; 28],
    transitions: [Option<LinearTransition>; 29],
}
struct Workspace {
    linear_left: Vec<CanonicalTerm>,
    linear_right: Vec<CanonicalTerm>,
    group: GroupWorkspace,
}
impl Workspace {
    fn new(domain: &[AffinePoint], ledger: &mut Ledger) -> Result<Self, Stop> {
        if domain.len() > 64 {
            return Err(Stop::InvalidInput);
        }
        let size = size_of::<GroupWorkspace>() as u64;
        {
            let mut t = ledger.reserve(size * 8, size)?;
            t.spend(size * 8)?;
        }
        let mut group = GroupWorkspace {
            domain: [SmallPoint::Infinity; 64],
            cursor: [0; 15],
            tuple: [0; 15],
            prefix: [SmallPoint::Infinity; 15],
            coordinates: [0; 28],
            residues: [0; 29],
            first_certificate: [0; 15],
            exceptional: [0; 15],
            assignments: [None; 28],
            sources: [0; 28],
            transitions: [None; 29],
        };
        for (i, &point) in domain.iter().enumerate() {
            group.domain[i] = SmallPoint::Affine(point);
        }
        Ok(Self {
            linear_left: requested_vec(8192, ledger)?,
            linear_right: requested_vec(8192, ledger)?,
            group,
        })
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum EventPersistence {
    LocalOnly,
    BoundCapture,
}
#[derive(Clone, Copy, Debug, serde::Serialize)]
pub(super) struct PhaseEvent {
    schema: &'static str,
    source: PhaseSource,
    global_sequence: usize,
    local_sequence: usize,
    stage: &'static str,
    reserved_work: u64,
    reserved_bytes: u64,
    persistence: EventPersistence,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
enum PhaseSource {
    Setup,
    Native,
    Linear,
    Group,
}
struct ObserverState {
    acknowledged: usize,
    attempted: usize,
    setup: usize,
    native: usize,
    linear: usize,
    group: usize,
    durable: usize,
    pending: Option<PhaseEvent>,
    stop: Option<Stop>,
    last_native: Supply,
    terminal_acknowledged: bool,
    persistence: EventPersistence,
}
/// Check only information actually present in native callback snapshots.
/// No clock check: a censored call may legitimately return after phase expiry.
fn validate_final_snapshot(report: &dynamic::Report, observer: &ObserverState) -> Result<(), Stop> {
    let final_work = report.reserved.work;
    let final_bytes = report.reserved.bytes;
    if report.acknowledged_events != observer.native
        || final_work < observer.last_native.work
        || final_bytes < observer.last_native.bytes
        || (observer.terminal_acknowledged
            && (final_work != observer.last_native.work
                || final_bytes != observer.last_native.bytes))
    {
        return Err(Stop::InvalidInput);
    }
    Ok(())
}
fn encode_record<T: serde::Serialize>(
    record: &T,
    slot: &mut InputBuffer,
    ledger: &mut Ledger,
) -> Result<(), Stop> {
    slot.bytes.clear();
    let result = (|| {
        let mut count = MeteredWriter {
            ledger,
            output: None,
            count: 0,
            cap: slot.logical_cap,
            stop: None,
        };
        if serde_json::to_writer(&mut count, record).is_err() {
            return Err(count.stop.unwrap_or(Stop::Serialization));
        }
        let expected = count.count;
        let mut writer = MeteredWriter {
            ledger,
            output: Some(&mut slot.bytes),
            count: 0,
            cap: slot.logical_cap,
            stop: None,
        };
        if serde_json::to_writer(&mut writer, record).is_err() {
            return Err(writer.stop.unwrap_or(Stop::Serialization));
        }
        if writer.count != expected {
            return Err(Stop::Serialization);
        }
        ledger.check()?;
        Ok(())
    })();
    if result.is_err() {
        slot.bytes.clear();
    }
    result
}
fn append_observation<F: FnMut(&PhaseEvent, &mut TypedEventBuffer) -> Result<(), ()>>(
    event: PhaseEvent,
    state: &mut ObserverState,
    slot: &mut InputBuffer,
    budget: &mut Ledger,
    capture: &mut TypedEventBuffer,
    forward: &mut F,
) -> Result<(), ()> {
    let start = slot.bytes.len();
    let result = (|| {
        if state.stop.is_some() || state.acknowledged >= 4096 {
            return Err(Stop::OutputLimit);
        }
        budget.check()?;
        state.attempted += 1;
        let cap = slot
            .logical_cap
            .checked_sub(start)
            .and_then(|n| n.checked_sub(1))
            .ok_or(Stop::OutputLimit)?;
        let mut count = MeteredWriter {
            ledger: budget,
            output: None,
            count: 0,
            cap,
            stop: None,
        };
        if serde_json::to_writer(&mut count, &event).is_err() {
            return Err(count.stop.unwrap_or(Stop::Serialization));
        }
        let expected = count.count;
        let mut writer = MeteredWriter {
            ledger: budget,
            output: Some(&mut slot.bytes),
            count: 0,
            cap,
            stop: None,
        };
        if serde_json::to_writer(&mut writer, &event).is_err() {
            return Err(writer.stop.unwrap_or(Stop::Serialization));
        }
        if writer.count != expected {
            return Err(Stop::Serialization);
        }
        {
            let mut t = budget.reserve(40, 0)?;
            t.spend(40)?;
        }
        slot.bytes.push(b'\n');
        if state.persistence == EventPersistence::BoundCapture {
            // This is consumed once before entering caller serialization; the
            // callback must use this exact1024-byte admitted typed buffer.
            let mut t = budget.reserve(CALLBACK_WORK, 0)?;
            t.spend(CALLBACK_WORK)?;
            forward(&event, capture).map_err(|_| Stop::Observer)?;
        }
        // NO deadline check after durable acknowledgement. Native checks after
        // its callback and must count this already committed row before expiry.
        state.acknowledged += 1;
        if state.persistence == EventPersistence::BoundCapture {
            state.durable += 1;
        }
        match event.source {
            PhaseSource::Setup => state.setup += 1,
            PhaseSource::Native => state.native += 1,
            PhaseSource::Linear => state.linear += 1,
            PhaseSource::Group => state.group += 1,
        }
        Ok(())
    })();
    if let Err(stop) = result {
        slot.bytes.truncate(start);
        state.pending = Some(event);
        state.stop = Some(stop);
        Err(())
    } else {
        Ok(())
    }
}
fn stage_name(stage: dynamic::Stage) -> &'static str {
    match stage {
        dynamic::Stage::Begin => "begin",
        dynamic::Stage::Initial => "initial",
        dynamic::Stage::Basis => "basis",
        dynamic::Stage::Batch => "batch",
        dynamic::Stage::Terminal => "terminal",
    }
}
fn stop_name(stop: dynamic::Stop) -> &'static str {
    match stop {
        dynamic::Stop::Complete => "complete",
        dynamic::Stop::UnitDerived => "unit_derived",
        dynamic::Stop::PairDegreeLimit => "pair_degree_limit",
        dynamic::Stop::WorkLimit => "work_limit",
        dynamic::Stop::StorageLimit => "storage_limit",
        dynamic::Stop::RowLimit => "row_limit",
        dynamic::Stop::ColumnLimit => "column_limit",
        dynamic::Stop::BasisLimit => "basis_limit",
        dynamic::Stop::PairLimit => "pair_limit",
        dynamic::Stop::StepLimit => "step_limit",
        dynamic::Stop::EventLimit => "event_limit",
        dynamic::Stop::ExponentOverflow => "exponent_overflow",
        dynamic::Stop::Allocation => "allocation",
        dynamic::Stop::Deadline => "deadline",
        dynamic::Stop::Observer => "observer",
        dynamic::Stop::InvalidInput => "invalid_input",
    }
}
#[derive(serde::Serialize)]
struct NativeLimitRecord {
    work: u64,
    bytes: u64,
    rows: usize,
    columns: usize,
    basis: usize,
    pairs: usize,
    steps: usize,
    events: usize,
    max_pair_degree: u32,
}
impl From<&dynamic::Limits> for NativeLimitRecord {
    fn from(l: &dynamic::Limits) -> Self {
        Self {
            work: l.work,
            bytes: l.bytes,
            rows: l.rows,
            columns: l.columns,
            basis: l.basis,
            pairs: l.pairs,
            steps: l.steps,
            events: l.events,
            max_pair_degree: l.max_pair_degree,
        }
    }
}
#[derive(serde::Serialize)]
struct NativeMetadata {
    schema: &'static str,
    m: usize,
    variables: usize,
    stop: &'static str,
    supplied: Supply,
    consumed: Supply,
    limits: NativeLimitRecord,
    acknowledged_events: usize,
    native_backend_invoked: bool,
    native_statistics_complete: bool,
    matrix_rows_reduced: u64,
    pivot_updates: u64,
    pairs_pending: usize,
    pairs_selected_unfinished: usize,
    maximum_rows_admitted: usize,
    maximum_columns_admitted: usize,
    maximum_pending_pairs: usize,
    completed_steps: usize,
    maximum_input_degree: u32,
    maximum_processed_row_degree: u32,
    basis_retained: bool,
    basis_retention_stop: Option<Stop>,
    native_basis_derivation_verified: bool,
    independent_derivation_verified: bool,
    linear_extraction_complete: bool,
    group_domain_complete: bool,
    pdp_witness_verified: bool,
    exhaustive_no_decomposition: bool,
    solver_completed: bool,
    exact_instruction_count_verified: bool,
    full_execution_trace_verified: bool,
    fall_degree_verified: bool,
}
impl NativeMetadata {
    fn new(
        m: usize,
        r: &dynamic::Report,
        limits: &dynamic::Limits,
        retained: bool,
        stop: Option<Stop>,
    ) -> Self {
        Self {
            schema: "crypto.ml-dataset.wide-prime-dynamic-native/v1",
            m,
            variables: 2 * m - 2,
            stop: stop_name(r.stop),
            supplied: Supply {
                work: limits.work,
                bytes: limits.bytes,
            },
            limits: NativeLimitRecord::from(limits),
            consumed: Supply {
                work: r.reserved.work,
                bytes: r.reserved.bytes,
            },
            acknowledged_events: r.acknowledged_events,
            native_backend_invoked: r.native_backend_invoked,
            native_statistics_complete: r.native_statistics_complete,
            matrix_rows_reduced: r.matrix_rows_reduced,
            pivot_updates: r.pivot_updates,
            pairs_pending: r.pairs_pending,
            pairs_selected_unfinished: r.pairs_selected_unfinished,
            maximum_rows_admitted: r.maximum_rows_admitted,
            maximum_columns_admitted: r.maximum_columns_admitted,
            maximum_pending_pairs: r.maximum_pending_pairs,
            completed_steps: r.completed_steps,
            maximum_input_degree: r.maximum_input_degree,
            maximum_processed_row_degree: r.maximum_processed_row_degree,
            basis_retained: retained,
            basis_retention_stop: stop,
            native_basis_derivation_verified: false,
            independent_derivation_verified: false,
            linear_extraction_complete: false,
            group_domain_complete: false,
            pdp_witness_verified: false,
            exhaustive_no_decomposition: false,
            solver_completed: false,
            exact_instruction_count_verified: false,
            full_execution_trace_verified: false,
            fall_degree_verified: false,
        }
    }
}
struct NativeRecord<'a> {
    metadata: NativeMetadata,
    basis: Option<&'a [InputPolynomial]>,
}
impl serde::Serialize for NativeRecord<'_> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut out = s.serialize_struct("NativeRecord", 2)?;
        out.serialize_field("metadata", &self.metadata)?;
        if let Some(basis) = self.basis {
            let v = self.metadata.variables;
            out.serialize_field(
                "basis",
                &Sequence {
                    len: basis.len(),
                    item: |i: usize| {
                        let p = &basis[i];
                        Sequence {
                            len: p.terms.len(),
                            item: move |j: usize| TermView {
                                exponents: p.terms[j].mono.exponents(v).expect("native valid V"),
                                coefficient: Decimal(p.terms[j].coefficient),
                            },
                        }
                    },
                },
            )?;
        } else {
            out.serialize_field("basis", &Option::<u8>::None)?;
        }
        out.end()
    }
}
#[derive(serde::Serialize)]
struct SetupRecord {
    schema: &'static str,
    modulus: Decimal,
    prime_verified: bool,
    reserved_work: u64,
    reserved_bytes: u64,
    acknowledged_events: usize,
}
pub(super) struct PreparedOriginal {
    setup: FieldSetup,
    input: Vec<InputPolynomial>,
    slots: Slots,
    workspace: Workspace,
    capture: TypedEventBuffer,
    ledger: Ledger,
    checking_work: u64,
    checking_reserved_work: u64,
    checking_performed_work: u64,
    terminal: Ledger,
    observer: ObserverState,
    phase_deadline: Instant,
    original_deadline: Instant,
    m: usize,
    degree_bound: u32,
    group_control: GroupControl,
    linear_retained: bool,
    step4: Step4State,
}
pub(super) struct PrepareFailure {
    pub stop: Stop,
    pub ledger: Ledger,
    pub setup_failure: Option<FieldSetupFailure>,
    retained_slots: Option<Slots>,
}
impl std::fmt::Debug for PrepareFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PrepareFailure")
            .field("stop", &self.stop)
            .field("ledger", &self.ledger)
            .field("setup_failure", &self.setup_failure)
            .field("retained_slots", &self.retained_slots.is_some())
            .finish()
    }
}
/// Public/group validation is the caller's precondition; the serializer enforces
/// canonical dimensions/domain coverage. This private API never activates CLI.
#[allow(clippy::result_large_err)] // Preserve admitted partial state without an error-path allocation.
pub(super) fn prepare_original<F: FnMut(&PhaseEvent, &mut TypedEventBuffer) -> Result<(), ()>>(
    p: u64,
    parameters: Parameters<'_>,
    target: AffinePoint,
    domain: &[AffinePoint],
    limits: AttemptLimits,
    persistence: EventPersistence,
    forward: &mut F,
) -> Result<PreparedOriginal, PrepareFailure> {
    let phase_deadline = limits
        .original_deadline
        .min(Instant::now() + Duration::from_secs(5));
    let w = match limits.work_limit.checked_mul(1024) {
        Some(w) => w.min(16_777_216),
        None => {
            return Err(PrepareFailure {
                stop: Stop::ArithmeticOverflow,
                ledger: Ledger::new(0, 0, phase_deadline),
                setup_failure: None,
                retained_slots: None,
            })
        }
    };
    let algebra = w * 3 / 4;
    let mut ledger = Ledger::new(
        algebra,
        (limits.memory_bytes / 4).min(64 * 1024 * 1024),
        phase_deadline,
    );
    let mut setup_failure = None;
    let mut retained_slots = None;
    let result = (|| {
        if w == 0
            || ledger.bytes_limit == 0
            || !(4..=15).contains(&parameters.m)
            || domain.is_empty()
            || domain.len() > 64
        {
            return Err(Stop::InvalidInput);
        }
        let fixed = (size_of::<ObserverState>()
            + size_of::<PhaseEvent>()
            + size_of::<NativeMetadata>()
            + size_of::<SetupRecord>()
            + size_of::<[u8; 60]>()
            + size_of::<[usize; 32]>()
            + size_of::<GroupControl>()
            + size_of::<Step4State>()
            + size_of::<BoundWitnessPublisher>()
            + 2 * size_of::<DynamicResult<'static>>()
            + GROUP_SCRATCH_BYTES) as u64;
        {
            let mut ticket = ledger.reserve(fixed * 8, fixed)?;
            ticket.spend(fixed * 8)?;
        }
        let e = usize::try_from((limits.trace_bytes / 32).min(4 * 1024 * 1024))
            .map_err(|_| Stop::ArithmeticOverflow)?;
        retained_slots = Some(Slots::new(e, &mut ledger)?);
        let slots = retained_slots.as_mut().expect("slots admitted");
        let workspace = Workspace::new(domain, &mut ledger)?;
        let admitted =
            TypedEventBuffer::requested_bytes(CAPTURE_CAP).map_err(|_| Stop::StorageLimit)?;
        {
            let _ = ledger.reserve(0, admitted)?;
        }
        let mut capture =
            TypedEventBuffer::try_new(CAPTURE_CAP, admitted).map_err(|_| Stop::Allocation)?;
        // Terminal allowance is consumed once. Its own ticket uses ONLY the
        // original process deadline, never extends native/phase execution.
        {
            let _ = ledger.reserve(TERMINAL_WORK, 0)?;
        }
        let terminal = Ledger::new(TERMINAL_WORK, 0, limits.original_deadline);
        let setup_event_allowance = 2 * (CALLBACK_WORK + 16_384);
        {
            let _ = ledger.reserve(setup_event_allowance, 0)?;
        }
        let mut setup_events = Ledger::new(setup_event_allowance, 0, phase_deadline);
        let mut observer = ObserverState {
            acknowledged: 0,
            attempted: 0,
            setup: 0,
            native: 0,
            linear: 0,
            group: 0,
            durable: 0,
            pending: None,
            stop: None,
            last_native: Supply { work: 0, bytes: 0 },
            terminal_acknowledged: false,
            persistence,
        };
        let supplied = remaining(&ledger);
        let mut setup_escrow = Escrow::new(supplied);
        let setup_result = Fp64Arithmetic::try_new(
            p,
            supplied.work,
            supplied.bytes,
            phase_deadline,
            &mut |event| {
                let row = PhaseEvent {
                    schema: "crypto.ml-dataset.wide-prime-dynamic-event/v1",
                    source: PhaseSource::Setup,
                    global_sequence: observer.acknowledged,
                    local_sequence: observer.setup,
                    stage: match event.stage {
                        FieldSetupStage::Admitted => "admitted",
                        FieldSetupStage::Complete => "complete",
                    },
                    reserved_work: event.reserved_work,
                    reserved_bytes: event.reserved_bytes,
                    persistence,
                };
                append_observation(
                    row,
                    &mut observer,
                    &mut slots.buffers[3],
                    &mut setup_events,
                    &mut capture,
                    forward,
                )
            },
        );
        ledger.performed_work = ledger
            .performed_work
            .checked_add(setup_events.performed_work)
            .ok_or(Stop::ArithmeticOverflow)?;
        let setup = match setup_result {
            Ok(setup) => {
                settle(
                    &mut ledger,
                    &mut setup_escrow,
                    Supply {
                        work: setup.reserved_work,
                        bytes: setup.reserved_bytes,
                    },
                    0,
                )?;
                setup
            }
            Err(failure) => {
                settle(
                    &mut ledger,
                    &mut setup_escrow,
                    Supply {
                        work: failure.reserved_work,
                        bytes: failure.reserved_bytes,
                    },
                    0,
                )?;
                setup_failure = Some(failure);
                return Err(Stop::Setup);
            }
        };
        let input = build_original_chain(&setup.field, parameters, &mut ledger)?;
        let view = ChainInputView {
            field: &setup.field,
            parameters,
            target,
            signed_domain: domain,
            polynomials: &input,
        };
        encode_input(&view, &mut slots.buffers[0], &mut ledger)?;
        encode_record(
            &SetupRecord {
                schema: "crypto.ml-dataset.wide-prime-dynamic-setup/v1",
                modulus: Decimal(p),
                prime_verified: setup.prime_verified,
                reserved_work: setup.reserved_work,
                reserved_bytes: setup.reserved_bytes,
                acknowledged_events: observer.setup,
            },
            &mut slots.buffers[1],
            &mut ledger,
        )?;
        Ok((setup, input, workspace, capture, terminal, observer))
    })();
    match result {
        Ok((setup, input, workspace, capture, terminal, observer)) => Ok(PreparedOriginal {
            setup,
            input,
            slots: retained_slots.take().expect("slots admitted"),
            workspace,
            capture,
            ledger,
            checking_work: w - algebra,
            checking_reserved_work: 0,
            checking_performed_work: 0,
            terminal,
            observer,
            phase_deadline,
            original_deadline: limits.original_deadline,
            m: parameters.m,
            degree_bound: limits.degree_bound.min(12),
            group_control: GroupControl {
                a: parameters.a,
                b: parameters.b,
                target,
                domain_len: domain.len(),
                addition_limit: limits.work_limit,
                tuple_limit: limits.work_limit.min(1024),
                started: false,
                additions_reserved: 0,
                additions_performed: 0,
                tuples: 0,
            },
            linear_retained: false,
            step4: Step4State::new(limits.original_deadline),
        }),
        Err(stop) => Err(PrepareFailure {
            stop,
            ledger,
            setup_failure,
            retained_slots,
        }),
    }
}
pub(super) struct NativePhase {
    pub prepared: PreparedOriginal,
    pub report: dynamic::Report,
    pub settlement: Settlement,
    pub observer_settlement: Settlement,
    pub basis_retained: bool,
    pub retention_stop: Option<Stop>,
    pub diagnostic_stop: Option<Stop>,
    pub independent_derivation_verified: bool,
}
#[allow(clippy::result_large_err)] // Preserve admitted partial state without an error-path allocation.
pub(super) fn invoke_original<F: FnMut(&PhaseEvent, &mut TypedEventBuffer) -> Result<(), ()>>(
    mut prepared: PreparedOriginal,
    forward: &mut F,
) -> Result<NativePhase, PrepareFailure> {
    // All data-sized storage was admitted by prepare. No parent access escapes
    // while the native and callback allowances are exclusively loaned below.
    let available = remaining(&prepared.ledger);
    let observer_supply = Supply {
        work: available.work / 4,
        bytes: 0,
    };
    let native_supply = Supply {
        work: available.work - observer_supply.work,
        bytes: available.bytes,
    };
    let mut native_escrow = Escrow::new(native_supply);
    let mut observer_escrow = Escrow::new(observer_supply);
    let mut observer_budget = Ledger::new(observer_supply.work, 0, prepared.phase_deadline);
    let native_limits = dynamic::Limits {
        work: native_supply.work,
        bytes: native_supply.bytes,
        events: 4096 - prepared.observer.acknowledged,
        max_pair_degree: prepared.degree_bound,
        ..dynamic::Limits::default()
    };
    let report = dynamic::run(
        &prepared.setup.field,
        (2 * prepared.m - 2) as u8,
        &prepared.input,
        &native_limits,
        prepared.phase_deadline,
        &mut |event| {
            if prepared.observer.terminal_acknowledged
                || event.sequence != prepared.observer.native
                || event.reserved.work < prepared.observer.last_native.work
                || event.reserved.bytes < prepared.observer.last_native.bytes
                || event.reserved.work > native_supply.work
                || event.reserved.bytes > native_supply.bytes
            {
                prepared.observer.stop = Some(Stop::InvalidInput);
                return Err(());
            }
            let row = PhaseEvent {
                schema: "crypto.ml-dataset.wide-prime-dynamic-event/v1",
                source: PhaseSource::Native,
                global_sequence: prepared.observer.acknowledged,
                local_sequence: event.sequence,
                stage: stage_name(event.stage),
                reserved_work: event.reserved.work,
                reserved_bytes: event.reserved.bytes,
                persistence: prepared.observer.persistence,
            };
            append_observation(
                row,
                &mut prepared.observer,
                &mut prepared.slots.buffers[3],
                &mut observer_budget,
                &mut prepared.capture,
                forward,
            )?;
            prepared.observer.last_native = Supply {
                work: event.reserved.work,
                bytes: event.reserved.bytes,
            };
            prepared.observer.terminal_acknowledged = event.stage == dynamic::Stage::Terminal;
            Ok(())
        },
    );
    prepared.step4.observer_performed = observer_budget.performed_work;
    // These are clock-independent and occur BEFORE fallible retention.
    let settled = (|| {
        let native = settle(
            &mut prepared.ledger,
            &mut native_escrow,
            Supply {
                work: report.reserved.work,
                bytes: report.reserved.bytes,
            },
            0,
        )?;
        let observer = settle(
            &mut prepared.ledger,
            &mut observer_escrow,
            Supply {
                work: observer_budget.reserved_work,
                bytes: 0,
            },
            observer_budget.performed_work,
        )?;
        validate_final_snapshot(&report, &prepared.observer)?;
        Ok((native, observer))
    })();
    let (settlement, observer_settlement) = match settled {
        Ok(v) => v,
        Err(stop) => {
            return Err(PrepareFailure {
                stop,
                ledger: prepared.ledger,
                setup_failure: None,
                retained_slots: Some(prepared.slots),
            })
        }
    };
    // Borrow canonical basis directly; never apply input's677-term cap here.
    let retention = (|| {
        for p in &report.basis {
            for chunk in p.terms.chunks(128) {
                let work = chunk.len() as u64 * (8 * (2 * prepared.m - 2) as u64 + 32);
                let mut t = prepared.ledger.reserve(work, 0)?;
                t.spend(work)?;
            }
        }
        let record = NativeRecord {
            metadata: NativeMetadata::new(prepared.m, &report, &native_limits, true, None),
            basis: Some(&report.basis),
        };
        encode_record(
            &record,
            &mut prepared.slots.buffers[2],
            &mut prepared.ledger,
        )
    })();
    let retention_stop = retention.err();
    let diagnostic_stop = if let Some(stop) = retention_stop {
        let record = NativeRecord {
            metadata: NativeMetadata::new(prepared.m, &report, &native_limits, false, Some(stop)),
            basis: None,
        };
        encode_record(
            &record,
            &mut prepared.slots.buffers[2],
            &mut prepared.terminal,
        )
        .err()
    } else {
        None
    };
    // Terminal budget was already charged in prepare; performed tariff only.
    prepared.ledger.performed_work += prepared.terminal.performed_work;
    Ok(NativePhase {
        prepared,
        report,
        settlement,
        observer_settlement,
        basis_retained: retention_stop.is_none(),
        retention_stop,
        diagnostic_stop,
        independent_derivation_verified: false,
    })
}

#[derive(Clone, Copy, Debug, serde::Serialize)]
struct LinearTransition {
    variable: usize,
    basis_id: usize,
    a: u64,
    b: u64,
    value: u64,
    algebra_reserved_work: u64,
    checking_reserved_work: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum LinearStop {
    ReportedBasisCandidate,
    NoRetainedBasis,
    NativeUnit,
    Contradiction,
    OutsideDomain,
    NonlinearOrUnderdetermined,
    MissingPivot,
    TermLimit,
    InvalidBasis,
    InvalidWorkspace,
    WorkLimit,
    Deadline,
    OriginalEquationMismatch,
    InvalidOriginal,
}
#[derive(Clone, Copy, Debug, serde::Serialize)]
pub(super) struct LinearReport {
    pub stop: LinearStop,
    pub assignments: usize,
    pub completed_passes: usize,
    pub original_equations_checked: usize,
    pub candidate_verified_original: bool,
    pub algebra_reserved_work: u64,
    pub checking_reserved_work: u64,
    pub independent_derivation_verified: bool,
    pub pdp_witness_verified: bool,
}
fn linear_budget(stop: Stop) -> LinearStop {
    match stop {
        Stop::Deadline => LinearStop::Deadline,
        Stop::WorkLimit | Stop::TicketExhausted => LinearStop::WorkLimit,
        _ => LinearStop::InvalidWorkspace,
    }
}
fn charge(ledger: &mut Ledger, units: u64) -> Result<(), LinearStop> {
    let mut ticket = ledger.reserve(units, 0).map_err(linear_budget)?;
    ticket.spend(units).map_err(linear_budget)
}

/// No allocations: both vectors retain their admitted logical8192 capacities.
/// Each source exponent uses all16bits; assigned coordinates are zeroed only
/// after their complete power has been applied through borrowed field arithmetic.
fn linear_remainder(
    source: &InputPolynomial,
    v: usize,
    field: &Fp64Arithmetic,
    assigned: &[Option<u64>; 28],
    left: &mut Vec<CanonicalTerm>,
    right: &mut Vec<CanonicalTerm>,
    ledger: &mut Ledger,
) -> Result<(), LinearStop> {
    if !(1..=28).contains(&v) || left.capacity() < 8192 || right.capacity() < 8192 {
        return Err(LinearStop::InvalidWorkspace);
    }
    if source.terms.len() > 8192 {
        return Err(LinearStop::TermLimit);
    }
    charge(ledger, 32)?;
    left.clear();
    right.clear();
    for term in &source.terms {
        charge(ledger, 8 * (MAX_V + v) as u64 + COPY_WORK + 64)?;
        if term.coefficient == 0 || term.coefficient >= field.p() {
            return Err(LinearStop::InvalidBasis);
        }
        let all = term.mono.exponents(MAX_V).ok_or(LinearStop::InvalidBasis)?;
        if all[v..].iter().any(|&x| x != 0) {
            return Err(LinearStop::InvalidBasis);
        }
        let mut powers = [0u16; 28];
        powers[..v].copy_from_slice(&all[..v]);
        let mut coefficient = field.from_u64(term.coefficient);
        for axis in 0..v {
            charge(ledger, 16)?;
            let exponent = powers[axis];
            if let Some(value) = assigned[axis] {
                if value >= field.p() {
                    return Err(LinearStop::InvalidBasis);
                }
                if exponent != 0 {
                    let bits = u16::BITS - exponent.leading_zeros();
                    charge(ledger, 64 + 48 * u64::from(bits))?;
                    let mut base = field.from_u64(value);
                    let mut power = exponent;
                    let mut factor = field.one();
                    while power != 0 {
                        if power & 1 != 0 {
                            factor = field.mul(factor, base);
                        }
                        power >>= 1;
                        if power != 0 {
                            base = field.sqr(base);
                        }
                    }
                    coefficient = field.mul(coefficient, factor);
                }
                powers[axis] = 0;
            }
        }
        charge(ledger, COPY_WORK + 8 * v as u64 + 32)?;
        left.push(CanonicalTerm {
            mono: Mono28::new(&powers[..v], v).ok_or(LinearStop::InvalidBasis)?,
            coefficient: field.to_u64(coefficient),
        });
    }
    let comparison = 16 * v as u64 + 8;
    let mut width = 1usize;
    while width < left.len() {
        charge(ledger, 16)?;
        right.clear();
        for start in (0..left.len()).step_by(width * 2) {
            let mid = (start + width).min(left.len());
            let end = (start + 2 * width).min(left.len());
            let mut i = start;
            let mut j = mid;
            while i < mid || j < end {
                charge(ledger, comparison + COPY_WORK + 32)?;
                if j == end || (i < mid && cmp(&left[i], &left[j], v) != Ordering::Less) {
                    right.push(left[i]);
                    i += 1;
                } else {
                    right.push(left[j]);
                    j += 1;
                }
            }
        }
        std::mem::swap(left, right);
        width *= 2;
    }
    let mut merged = 0;
    for i in 0..left.len() {
        charge(ledger, comparison + 2 * COPY_WORK + 32)?;
        let term = left[i];
        if merged > 0 && left[merged - 1].mono == term.mono {
            left[merged - 1].coefficient =
                add(left[merged - 1].coefficient, term.coefficient, field.p());
        } else {
            left[merged] = term;
            merged += 1;
        }
    }
    let mut kept = 0;
    for i in 0..merged {
        charge(ledger, COPY_WORK + 16)?;
        if left[i].coefficient != 0 {
            left[kept] = left[i];
            kept += 1;
        }
    }
    left.truncate(kept);
    ledger.check().map_err(linear_budget)
}
#[derive(Clone, Copy)]
struct LinearPivot {
    variable: usize,
    basis_id: usize,
    a: u64,
    b: u64,
}
/// None means zero or genuinely non-univariate-linear; nonzero constants are
/// contradictions. The caller distinguishes zero from unresolved polynomial.
fn pivot_from_remainder(
    poly: &[CanonicalTerm],
    v: usize,
    basis_id: usize,
    ledger: &mut Ledger,
) -> Result<Option<LinearPivot>, LinearStop> {
    let mut variable = None;
    let mut a = 0;
    let mut b = 0;
    for term in poly {
        charge(ledger, 16 * v as u64 + 32)?;
        let powers = term.mono.exponents(v).ok_or(LinearStop::InvalidBasis)?;
        let mut term_variable = None;
        for (axis, &power) in powers.iter().enumerate() {
            if power != 0 {
                if power != 1 || term_variable.is_some() {
                    return Ok(None);
                }
                term_variable = Some(axis);
            }
        }
        if let Some(axis) = term_variable {
            if variable.is_some_and(|old| old != axis) {
                return Ok(None);
            }
            variable = Some(axis);
            a = term.coefficient;
        } else {
            b = term.coefficient;
        }
    }
    match variable {
        Some(variable) => Ok(Some(LinearPivot {
            variable,
            basis_id,
            a,
            b,
        })),
        None if b != 0 => Err(LinearStop::Contradiction),
        None => Ok(None),
    }
}
fn invert_linear(
    pivot: LinearPivot,
    field: &Fp64Arithmetic,
    ledger: &mut Ledger,
) -> Result<u64, LinearStop> {
    if pivot.a == 0 || pivot.a >= field.p() || pivot.b >= field.p() {
        return Err(LinearStop::InvalidBasis);
    }
    // Single consumed ticket, per-bit deadline checks; no nested parent debit.
    let mut ticket = ledger.reserve(4096, 0).map_err(linear_budget)?;
    ticket.spend(128).map_err(linear_budget)?;
    let a = field.from_u64(pivot.a);
    let mut base = a;
    let mut power = field.p() - 2;
    let mut inverse = field.one();
    while power != 0 {
        ticket.spend(48).map_err(linear_budget)?;
        if power & 1 != 0 {
            inverse = field.mul(inverse, base);
        }
        power >>= 1;
        if power != 0 {
            base = field.sqr(base);
        }
    }
    ticket.spend(128).map_err(linear_budget)?;
    if field.mul(a, inverse) != field.one() {
        return Err(LinearStop::InvalidBasis);
    }
    Ok(field.to_u64(field.mul(field.neg(field.from_u64(pivot.b)), inverse)))
}
/// Independent canonical u128 evaluator of the ORIGINAL input, not reported
/// basis equations. No Montgomery multiplication, native root routine or group
/// oracle is used here. Inputs have p<=2^61-1, hence products fit in u128.
fn check_original_linear_candidate(
    input: &[InputPolynomial],
    v: usize,
    p: u64,
    workspace: &mut GroupWorkspace,
    checking: &mut Ledger,
    checked: &mut usize,
) -> Result<(), LinearStop> {
    if !(1..=28).contains(&v) || input.len() != v + 1 || !(5..=0x1fff_ffff_ffff_ffff).contains(&p) {
        return Err(LinearStop::InvalidOriginal);
    }
    for axis in 0..v {
        charge(checking, 16)?;
        workspace.coordinates[axis] =
            workspace.assignments[axis].ok_or(LinearStop::MissingPivot)?;
    }
    let mut terms_seen = 0usize;
    let mut mismatch = false;
    for (i, poly) in input.iter().enumerate() {
        if poly.terms.len() > 33 {
            return Err(LinearStop::InvalidOriginal);
        }
        terms_seen += poly.terms.len();
        if terms_seen > 677 {
            return Err(LinearStop::InvalidOriginal);
        }
        let mut residue = 0;
        for term in &poly.terms {
            charge(checking, 32)?;
            if term.coefficient == 0 || term.coefficient >= p {
                return Err(LinearStop::InvalidOriginal);
            }
            let mut product = term.coefficient;
            let all = term
                .mono
                .exponents(MAX_V)
                .ok_or(LinearStop::InvalidOriginal)?;
            charge(checking, 8 * MAX_V as u64)?;
            if all[v..].iter().any(|&e| e != 0) {
                return Err(LinearStop::InvalidOriginal);
            }
            let powers = &all[..v];
            for (axis, &exponent) in powers.iter().enumerate() {
                charge(checking, 16)?;
                if exponent > 32 {
                    return Err(LinearStop::InvalidOriginal);
                }
                if exponent != 0 {
                    let bits = u16::BITS - exponent.leading_zeros();
                    charge(checking, 32 + 48 * u64::from(bits))?;
                    let mut base = workspace.coordinates[axis];
                    let mut power = exponent;
                    let mut value = 1;
                    while power != 0 {
                        if power & 1 != 0 {
                            value = ((u128::from(value) * u128::from(base)) % u128::from(p)) as u64;
                        }
                        power >>= 1;
                        if power != 0 {
                            base = ((u128::from(base) * u128::from(base)) % u128::from(p)) as u64;
                        }
                    }
                    product = ((u128::from(product) * u128::from(value)) % u128::from(p)) as u64;
                }
            }
            charge(checking, 24)?;
            residue = ((u128::from(residue) + u128::from(product)) % u128::from(p)) as u64;
        }
        workspace.residues[i] = residue;
        *checked += 1;
        mismatch |= residue != 0;
    }
    if mismatch {
        Err(LinearStop::OriginalEquationMismatch)
    } else {
        checking.check().map_err(linear_budget)?;
        Ok(())
    }
}
/// Deterministic V+1 complete passes: select lowest eligible original variable,
/// then lowest basis ID. No assignment is committed until its whole pass has
/// completed without contradiction or resource refusal. Partial native basis
/// may supply a checked candidate, never an ideal-derivation/nonexistence proof.
pub(super) fn extract_reported_linear(phase: &mut NativePhase) -> LinearReport {
    let prepared = &mut phase.prepared;
    let mut report = LinearReport {
        stop: LinearStop::NoRetainedBasis,
        assignments: 0,
        completed_passes: 0,
        original_equations_checked: 0,
        candidate_verified_original: false,
        algebra_reserved_work: prepared.ledger.reserved_work,
        checking_reserved_work: prepared.checking_reserved_work,
        independent_derivation_verified: false,
        pdp_witness_verified: false,
    };
    if !phase.basis_retained {
        prepared.step4.linear_stop = Some(report.stop);
        return report;
    }
    if phase.report.stop == dynamic::Stop::UnitDerived {
        report.stop = LinearStop::NativeUnit;
        prepared.step4.linear_stop = Some(report.stop);
        return report;
    }
    let v = 2 * prepared.m - 2;
    let mut checking = Ledger::new(prepared.checking_work, 0, prepared.phase_deadline);
    let result = (|| {
        if !(1..=28).contains(&v) || phase.report.basis.len() > 256 {
            return Err(LinearStop::InvalidBasis);
        }
        // One nonrefundable reservation, with per-entry deadline checks and
        // performed charges only for visited entries. No borrowed basis clone.
        {
            let mut scan = prepared
                .ledger
                .reserve(32 + 16 * phase.report.basis.len() as u64, 0)
                .map_err(linear_budget)?;
            scan.spend(32).map_err(linear_budget)?;
            for poly in &phase.report.basis {
                scan.spend(16).map_err(linear_budget)?;
                if poly.terms.len() > 8192 {
                    return Err(LinearStop::TermLimit);
                }
            }
        }
        charge(
            &mut prepared.ledger,
            (size_of::<[Option<u64>; 28]>()
                + size_of::<[usize; 28]>()
                + size_of::<[Option<LinearTransition>; 29]>()) as u64
                * 8,
        )?;
        prepared.workspace.group.assignments.fill(None);
        prepared.workspace.group.sources.fill(0);
        prepared.workspace.group.transitions.fill(None);
        for _ in 0..=v {
            let mut best: Option<LinearPivot> = None;
            for (basis_id, poly) in phase.report.basis.iter().enumerate() {
                linear_remainder(
                    poly,
                    v,
                    &prepared.setup.field,
                    &prepared.workspace.group.assignments,
                    &mut prepared.workspace.linear_left,
                    &mut prepared.workspace.linear_right,
                    &mut prepared.ledger,
                )?;
                let pivot = pivot_from_remainder(
                    &prepared.workspace.linear_left,
                    v,
                    basis_id,
                    &mut prepared.ledger,
                )?;
                if let Some(pivot) = pivot {
                    if prepared.workspace.group.assignments[pivot.variable].is_some() {
                        return Err(LinearStop::InvalidBasis);
                    }
                    if best.is_none_or(|old| {
                        (pivot.variable, pivot.basis_id) < (old.variable, old.basis_id)
                    }) {
                        best = Some(pivot);
                    }
                }
            }
            report.completed_passes += 1;
            if report.assignments == v {
                check_original_linear_candidate(
                    &prepared.input,
                    v,
                    prepared.setup.field.p(),
                    &mut prepared.workspace.group,
                    &mut checking,
                    &mut report.original_equations_checked,
                )?;
                return Ok(());
            }
            let Some(pivot) = best else {
                return Err(LinearStop::NonlinearOrUnderdetermined);
            };
            let value = invert_linear(pivot, &prepared.setup.field, &mut prepared.ledger)?;
            if pivot.variable < prepared.m {
                charge(&mut checking, 64 * 16)?;
                if !prepared
                    .workspace
                    .group
                    .domain
                    .iter()
                    .any(|point| matches!(point,SmallPoint::Affine(point) if point.x==value))
                {
                    return Err(LinearStop::OutsideDomain);
                }
            }
            charge(&mut prepared.ledger, 128)?;
            let checking_reserved = prepared
                .checking_reserved_work
                .checked_add(checking.reserved_work)
                .ok_or(LinearStop::WorkLimit)?;
            prepared.workspace.group.assignments[pivot.variable] = Some(value);
            prepared.workspace.group.sources[pivot.variable] = pivot.basis_id;
            prepared.workspace.group.transitions[report.assignments] = Some(LinearTransition {
                variable: pivot.variable,
                basis_id: pivot.basis_id,
                a: pivot.a,
                b: pivot.b,
                value,
                algebra_reserved_work: prepared.ledger.reserved_work,
                checking_reserved_work: checking_reserved,
            });
            report.assignments += 1;
        }
        Err(LinearStop::MissingPivot)
    })();
    // No clock check at settlement: all consumed checking work survives refusal.
    prepared.checking_work -= checking.reserved_work;
    prepared.checking_reserved_work += checking.reserved_work;
    prepared.checking_performed_work += checking.performed_work;
    report.algebra_reserved_work = prepared.ledger.reserved_work;
    report.checking_reserved_work = prepared.checking_reserved_work;
    match result {
        Ok(()) => {
            report.stop = LinearStop::ReportedBasisCandidate;
            report.candidate_verified_original = true;
        }
        Err(stop) => report.stop = stop,
    }
    prepared.step4.linear_stop = Some(report.stop);
    report
}

// Wire counters/cursors use canonical decimal strings, including values above
// u64::MAX. Formatting uses caller-admitted fixed scratch, never a String/Value.
fn serialize_u128_decimal<S: serde::Serializer>(
    value: &u128,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let mut digits = [0u8; 40];
    let mut start = digits.len();
    let mut value = *value;
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
// Steps 1–3 only: local typed extraction and group prefixes. A computed match
// stops before independent certificate replay / bound payload publication.
const GROUP_SCRATCH_BYTES: usize = 2048;
const GROUP_EVENT_WORK: u64 = 163_880; // <=80*1024 typed bytes + callback81920 + newline40
#[derive(Clone, Copy)]
struct GroupControl {
    a: u64,
    b: u64,
    target: AffinePoint,
    domain_len: usize,
    addition_limit: u64,
    tuple_limit: u64,
    started: bool,
    additions_reserved: u64,
    additions_performed: u64,
    tuples: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum GroupOrigin {
    ReportedBasisLinearCandidate,
    ReportedBasisLeafLift,
    IndependentGroupPrefix,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum GroupStop {
    IndependentTraversalFinished,
    PendingCertificate,
    AlreadyStarted,
    InvalidDomain,
    InvalidCandidate,
    NativeContradiction,
    OriginalEquationMismatch,
    TupleLimit,
    AdditionLimit,
    CheckWorkLimit,
    GroupBytes,
    EventLimit,
    PersistenceFailure,
    Deadline,
    InvalidState,
}
#[derive(Clone, Copy, Debug, serde::Serialize)]
pub(super) struct GroupReport {
    pub stop: GroupStop,
    pub origin: GroupOrigin,
    #[serde(serialize_with = "serialize_u128_decimal")]
    pub cursor: u128,
    #[serde(serialize_with = "serialize_u128_decimal")]
    pub independent_visited: u128,
    #[serde(serialize_with = "serialize_u128_decimal")]
    pub native_lifts_visited: u128,
    #[serde(serialize_with = "serialize_u128_decimal")]
    pub independent_total: u128,
    pub completed_prefixes: usize,
    pub tuple_entered: bool,
    pub computed_match: bool,
    pub row_bound: usize,
    pub tuples_reserved: u64,
    pub additions_reserved: u64,
    pub additions_performed: u64,
    pub checking_reserved_work: u64,
    pub checking_performed_work: u64,
    pub group_domain_complete: bool,
    pub pdp_witness_verified: bool,
    pub certificate_durably_persisted: bool,
    pub independent_derivation_verified: bool,
}
#[derive(serde::Serialize)]
struct LinearRecord<'a> {
    schema: &'static str,
    report: &'a LinearReport,
    assignments: &'a [Option<u64>],
    source_ids: &'a [usize],
    transitions: &'a [Option<LinearTransition>],
    unresolved: Unresolved<'a>,
    residues: &'a [u64],
}
struct Unresolved<'a>(&'a [Option<u64>]);
impl serde::Serialize for Unresolved<'_> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for (i, x) in self.0.iter().enumerate() {
            seq.serialize_element(&x.is_none().then_some(i))?;
        }
        seq.end()
    }
}
pub(super) fn retain_linear(phase: &mut NativePhase, report: &LinearReport) -> Result<(), Stop> {
    let p = &mut phase.prepared;
    if p.group_control.started {
        return Err(Stop::InvalidInput);
    }
    p.linear_retained = false;
    let v = 2 * p.m - 2;
    if report.assignments > v || report.original_equations_checked > v + 1 {
        return Err(Stop::InvalidInput);
    }
    let g = &p.workspace.group;
    let record = LinearRecord {
        schema: "crypto.ml-dataset.wide-prime-dynamic-linear/v1",
        report,
        assignments: &g.assignments[..v],
        source_ids: &g.sources[..v],
        transitions: &g.transitions[..report.assignments],
        unresolved: Unresolved(&g.assignments[..v]),
        residues: &g.residues[..report.original_equations_checked],
    };
    encode_record(&record, &mut p.slots.buffers[4], &mut p.ledger)?;
    p.linear_retained = true;
    Ok(())
}
#[derive(serde::Serialize)]
struct GroupRow<'a> {
    schema: &'static str,
    origin: GroupOrigin,
    #[serde(serialize_with = "serialize_u128_decimal")]
    cursor: u128,
    indices: &'a [usize],
    prefixes: &'a [SmallPoint],
    completed_additions: usize,
    exceptional_auxiliary_positions: &'a [usize],
    coordinates: Option<&'a [u64]>,
    residues: Option<&'a [u64]>,
    total: Option<&'a SmallPoint>,
    computed_match: bool,
    candidate_coordinates_match: bool,
    complete: bool,
    tuples_reserved: u64,
    additions_reserved: u64,
    additions_performed: u64,
    checking_reserved_work: u64,
}
const _: () = assert!(
    size_of::<GroupRow<'static>>()
        + size_of::<LinearRecord<'static>>()
        + size_of::<MeteredWriter<'static>>()
        + size_of::<GroupReport>()
        + size_of::<Ledger>() * 3
        + size_of::<[u64; 32]>()
        + size_of::<[usize; 32]>()
        + size_of::<[u8; 60]>()
        <= GROUP_SCRATCH_BYTES
);
fn group_stop(s: Stop) -> GroupStop {
    match s {
        Stop::Deadline => GroupStop::Deadline,
        Stop::WorkLimit | Stop::TicketExhausted => GroupStop::CheckWorkLimit,
        Stop::OutputLimit => GroupStop::GroupBytes,
        Stop::Observer => GroupStop::PersistenceFailure,
        _ => GroupStop::InvalidState,
    }
}
fn group_charge(l: &mut Ledger, w: u64) -> Result<(), GroupStop> {
    let mut t = l.reserve(w, 0).map_err(group_stop)?;
    t.spend(w).map_err(group_stop)
}
fn group_validate(
    c: GroupControl,
    g: &GroupWorkspace,
    p: u64,
    l: &mut Ledger,
) -> Result<(), GroupStop> {
    if c.domain_len == 0 || c.domain_len > 64 || c.a >= p || c.b >= p {
        return Err(GroupStop::InvalidDomain);
    }
    group_charge(l, 512)?;
    if add(
        mul(4, mul(mul(c.a, c.a, p), c.a, p), p),
        mul(27, mul(c.b, c.b, p), p),
        p,
    ) == 0
    {
        return Err(GroupStop::InvalidDomain);
    }
    for i in 0..=c.domain_len {
        group_charge(l, 512)?;
        let q = if i == c.domain_len {
            c.target
        } else {
            match g.domain[i] {
                SmallPoint::Affine(q) => q,
                _ => return Err(GroupStop::InvalidDomain),
            }
        };
        if q.x >= p
            || q.y >= p
            || mul(q.y, q.y, p)
                != add(
                    add(mul(mul(q.x, q.x, p), q.x, p), mul(c.a, q.x, p), p),
                    c.b,
                    p,
                )
        {
            return Err(GroupStop::InvalidDomain);
        }
        if i < c.domain_len {
            let mut opposite = false;
            for j in 0..c.domain_len {
                group_charge(l, 64)?;
                if let SmallPoint::Affine(r) = g.domain[j] {
                    if j < i && q == r {
                        return Err(GroupStop::InvalidDomain);
                    }
                    opposite |= q.x == r.x && r.y == neg(q.y, p);
                }
            }
            if !opposite {
                return Err(GroupStop::InvalidDomain);
            }
        }
    }
    Ok(())
}
// Parent tuple admission already paid4096 per operation; this local ledger is
// exclusively supplied from that reservation, with no second parent debit.
fn group_add(
    field: &Fp64Arithmetic,
    a: u64,
    left: SmallPoint,
    right: SmallPoint,
    l: &mut Ledger,
) -> Result<SmallPoint, GroupStop> {
    let mut t = l.reserve(4096, 0).map_err(group_stop)?;
    t.spend(128).map_err(group_stop)?;
    let (x, y) = match (left, right) {
        (SmallPoint::Infinity, q) | (q, SmallPoint::Infinity) => return Ok(q),
        (SmallPoint::Affine(x), SmallPoint::Affine(y)) => (x, y),
    };
    if x.x == y.x && (x.y != y.y || x.y == 0) {
        return Ok(SmallPoint::Infinity);
    }
    t.spend(512).map_err(group_stop)?;
    let x1 = field.from_u64(x.x);
    let y1 = field.from_u64(x.y);
    let x2 = field.from_u64(y.x);
    let y2 = field.from_u64(y.y);
    let (num, den) = if x.x == y.x {
        let xx = field.sqr(x1);
        (
            field.add(field.add(field.add(xx, xx), xx), field.from_u64(a)),
            field.add(y1, y1),
        )
    } else {
        (field.sub(y2, y1), field.sub(x2, x1))
    };
    let mut exponent = field.p() - 2;
    let mut base = den;
    let mut inverse = field.one();
    while exponent != 0 {
        t.spend(48).map_err(group_stop)?;
        if exponent & 1 != 0 {
            inverse = field.mul(inverse, base);
        }
        exponent >>= 1;
        if exponent != 0 {
            base = field.sqr(base);
        }
    }
    t.spend(256).map_err(group_stop)?;
    if field.mul(den, inverse) != field.one() {
        return Err(GroupStop::InvalidDomain);
    }
    let slope = field.mul(num, inverse);
    let nx = field.sub(field.sub(field.sqr(slope), x1), x2);
    let ny = field.sub(field.mul(slope, field.sub(x1, nx)), y1);
    Ok(SmallPoint::Affine(AffinePoint {
        x: field.to_u64(nx),
        y: field.to_u64(ny),
    }))
}
// Conservative schedule based on actual original supports, paid scan itself.
fn group_eval_bound(input: &[InputPolynomial], v: usize, l: &mut Ledger) -> Result<u64, GroupStop> {
    if input.len() != v + 1 || v > 28 {
        return Err(GroupStop::InvalidState);
    }
    let mut bound = 128u64;
    let mut total = 0usize;
    for poly in input {
        if poly.terms.len() > 33 {
            return Err(GroupStop::InvalidState);
        }
        total += poly.terms.len();
        if total > 677 {
            return Err(GroupStop::InvalidState);
        }
        bound += 32;
        for term in &poly.terms {
            group_charge(l, 64 + 16 * v as u64)?;
            bound += 64 + 32 * v as u64;
            for &e in term.mono.exponents(v).ok_or(GroupStop::InvalidState)? {
                if e > 32 {
                    return Err(GroupStop::InvalidState);
                }
                if e != 0 {
                    bound += 64 + 48 * u64::from(u16::BITS - e.leading_zeros());
                }
            }
        }
    }
    Ok(bound)
}
fn group_eval(
    input: &[InputPolynomial],
    v: usize,
    p: u64,
    g: &mut GroupWorkspace,
    l: &mut Ledger,
) -> Result<bool, GroupStop> {
    group_charge(l, 128)?;
    let mut mismatch = false;
    for (i, poly) in input.iter().enumerate() {
        group_charge(l, 32)?;
        let mut sum = 0;
        for term in &poly.terms {
            group_charge(l, 64 + 32 * v as u64)?;
            if term.coefficient == 0 || term.coefficient >= p {
                return Err(GroupStop::InvalidState);
            }
            let mut value = term.coefficient;
            for (axis, &e) in term
                .mono
                .exponents(v)
                .ok_or(GroupStop::InvalidState)?
                .iter()
                .enumerate()
            {
                if e != 0 {
                    group_charge(l, 64 + 48 * u64::from(u16::BITS - e.leading_zeros()))?;
                    let mut k = e;
                    let mut base = g.coordinates[axis];
                    let mut power = 1;
                    while k != 0 {
                        if k & 1 != 0 {
                            power = ((u128::from(power) * u128::from(base)) % u128::from(p)) as u64;
                        }
                        k >>= 1;
                        if k != 0 {
                            base = ((u128::from(base) * u128::from(base)) % u128::from(p)) as u64;
                        }
                    }
                    value = ((u128::from(value) * u128::from(power)) % u128::from(p)) as u64;
                }
            }
            sum = ((u128::from(sum) + u128::from(value)) % u128::from(p)) as u64;
        }
        g.residues[i] = sum;
        mismatch |= sum != 0;
    }
    l.check().map_err(group_stop)?;
    Ok(!mismatch)
}
fn group_row_bound(
    m: usize,
    g: &mut GroupWorkspace,
    cap: usize,
    l: &mut Ledger,
) -> Result<usize, GroupStop> {
    for i in 0..15 {
        group_charge(l, 64)?;
        g.tuple[i] = usize::MAX;
        g.exceptional[i] = usize::MAX;
        g.prefix[i] = SmallPoint::Affine(AffinePoint {
            x: 0x1fff_ffff_ffff_ffff,
            y: 0x1fff_ffff_ffff_ffff,
        });
    }
    for x in &mut g.coordinates {
        group_charge(l, 32)?;
        *x = 0x1fff_ffff_ffff_ffff;
    }
    for x in &mut g.residues {
        group_charge(l, 32)?;
        *x = 0x1fff_ffff_ffff_ffff;
    }
    let row = GroupRow {
        schema: "crypto.ml-dataset.wide-prime-dynamic-group-row/v1",
        origin: GroupOrigin::ReportedBasisLinearCandidate,
        cursor: u128::MAX,
        indices: &g.tuple[..m],
        prefixes: &g.prefix[..m - 1],
        completed_additions: usize::MAX,
        exceptional_auxiliary_positions: &g.exceptional[..m - 2],
        coordinates: Some(&g.coordinates[..2 * m - 2]),
        residues: Some(&g.residues[..2 * m - 1]),
        total: Some(&g.prefix[0]),
        computed_match: false,
        candidate_coordinates_match: false,
        complete: false,
        tuples_reserved: u64::MAX,
        additions_reserved: u64::MAX,
        additions_performed: u64::MAX,
        checking_reserved_work: u64::MAX,
    };
    let mut count = MeteredWriter {
        ledger: l,
        output: None,
        count: 0,
        cap: cap.saturating_sub(1),
        stop: None,
    };
    serde_json::to_writer(&mut count, &row)
        .map_err(|_| group_stop(count.stop.unwrap_or(Stop::Serialization)))?;
    count.count.checked_add(1).ok_or(GroupStop::InvalidState)
}
fn append_group_row(
    row: &GroupRow<'_>,
    slot: &mut InputBuffer,
    bound: usize,
    l: &mut Ledger,
) -> Result<(), GroupStop> {
    let start = slot.bytes.len();
    let result = (|| {
        if slot.logical_cap.saturating_sub(start) < bound {
            return Err(GroupStop::GroupBytes);
        }
        let mut count = MeteredWriter {
            ledger: l,
            output: None,
            count: 0,
            cap: bound - 1,
            stop: None,
        };
        serde_json::to_writer(&mut count, row)
            .map_err(|_| group_stop(count.stop.unwrap_or(Stop::Serialization)))?;
        let expected = count.count;
        let mut writer = MeteredWriter {
            ledger: l,
            output: Some(&mut slot.bytes),
            count: 0,
            cap: bound - 1,
            stop: None,
        };
        serde_json::to_writer(&mut writer, row)
            .map_err(|_| group_stop(writer.stop.unwrap_or(Stop::Serialization)))?;
        if writer.count != expected {
            return Err(GroupStop::InvalidState);
        }
        group_charge(l, 40)?;
        slot.bytes.push(b'\n');
        Ok(())
    })();
    if result.is_err() {
        slot.bytes.truncate(start);
    }
    result
}
fn lift_index(
    g: &GroupWorkspace,
    leaf: usize,
    start: usize,
    d: usize,
    l: &mut Ledger,
) -> Result<Option<usize>, GroupStop> {
    let x = g.assignments[leaf].ok_or(GroupStop::InvalidCandidate)?;
    for i in start..d {
        group_charge(l, 64)?;
        if matches!(g.domain[i],SmallPoint::Affine(q) if q.x==x) {
            return Ok(Some(i));
        }
    }
    Ok(None)
}
fn start_cursor(
    g: &mut GroupWorkspace,
    m: usize,
    d: usize,
    origin: GroupOrigin,
    l: &mut Ledger,
) -> Result<(), GroupStop> {
    for leaf in 0..m {
        group_charge(l, 32)?;
        let i = if origin == GroupOrigin::ReportedBasisLinearCandidate {
            lift_index(g, leaf, 0, d, l)?.ok_or(GroupStop::InvalidCandidate)?
        } else {
            0
        };
        g.cursor[leaf] = i;
    }
    group_charge(l, 32 * m as u64)?;
    g.tuple[..m].copy_from_slice(&g.cursor[..m]);
    Ok(())
}
fn advance_cursor(
    g: &mut GroupWorkspace,
    m: usize,
    d: usize,
    origin: GroupOrigin,
    l: &mut Ledger,
) -> Result<bool, GroupStop> {
    group_charge(l, 32 * m as u64)?;
    g.cursor[..m].copy_from_slice(&g.tuple[..m]);
    for leaf in (0..m).rev() {
        group_charge(l, 64)?;
        let next = if origin == GroupOrigin::ReportedBasisLinearCandidate {
            lift_index(g, leaf, g.cursor[leaf] + 1, d, l)?
        } else {
            (g.cursor[leaf] + 1 < d).then_some(g.cursor[leaf] + 1)
        };
        if let Some(i) = next {
            g.cursor[leaf] = i;
            group_charge(l, 32 * m as u64)?;
            g.tuple[..m].copy_from_slice(&g.cursor[..m]);
            return Ok(true);
        }
        g.cursor[leaf] = if origin == GroupOrigin::ReportedBasisLinearCandidate {
            lift_index(g, leaf, 0, d, l)?.ok_or(GroupStop::InvalidCandidate)?
        } else {
            0
        };
    }
    Ok(false)
}
#[cfg(test)]
thread_local! {
    static GROUP_ADDITION_HOOK: std::cell::Cell<Option<fn(u64, usize, &mut Ledger)>> = const { std::cell::Cell::new(None) };
    static GROUP_CARRY_HOOK: std::cell::Cell<Option<fn(u64, &mut Ledger)>> = const { std::cell::Cell::new(None) };
}
fn run_group_internal<
    F: FnMut(&PhaseEvent, &mut TypedEventBuffer) -> Result<(), ()>,
    P: WitnessPublisher,
>(
    phase: &mut NativePhase,
    linear: Option<&LinearReport>,
    forward: &mut F,
    mut publisher: Option<&mut P>,
) -> GroupReport {
    let p = &mut phase.prepared;
    let mut report = GroupReport {
        stop: GroupStop::InvalidState,
        origin: GroupOrigin::IndependentGroupPrefix,
        cursor: 0,
        independent_visited: 0,
        native_lifts_visited: 0,
        independent_total: 0,
        completed_prefixes: 0,
        tuple_entered: false,
        computed_match: false,
        row_bound: 0,
        tuples_reserved: p.group_control.tuples,
        additions_reserved: p.group_control.additions_reserved,
        additions_performed: p.group_control.additions_performed,
        checking_reserved_work: p.checking_reserved_work,
        checking_performed_work: p.checking_performed_work,
        group_domain_complete: false,
        pdp_witness_verified: false,
        certificate_durably_persisted: false,
        independent_derivation_verified: false,
    };
    if publisher.is_some() && (!p.step4.prepared || p.step4.failed) {
        report.stop = GroupStop::InvalidState;
        return report;
    }
    if p.group_control.started {
        report.stop = GroupStop::AlreadyStarted;
        return report;
    }
    p.group_control.started = true;
    let mut checking = Ledger::new(p.checking_work, 0, p.original_deadline);
    let result = (|| {
        if p.observer.stop.is_some() {
            return Err(GroupStop::PersistenceFailure);
        }
        let m = p.m;
        let v = 2 * m - 2;
        let d = p.group_control.domain_len;
        group_validate(
            p.group_control,
            &p.workspace.group,
            p.setup.field.p(),
            &mut checking,
        )?;
        let mut total = 1u128;
        for _ in 0..m {
            group_charge(&mut checking, 32)?;
            total = total
                .checked_mul(d as u128)
                .ok_or(GroupStop::InvalidState)?;
        }
        report.independent_total = total;
        let eval = group_eval_bound(&p.input, v, &mut checking)?;
        report.row_bound = group_row_bound(
            m,
            &mut p.workspace.group,
            p.slots.buffers[5].logical_cap,
            &mut checking,
        )?;
        let tuple_work = (m as u64 - 1)
            .checked_mul(4096)
            .and_then(|x| x.checked_add(eval))
            .and_then(|x| x.checked_add((report.row_bound as u64).checked_mul(80)?))
            .and_then(|x| x.checked_add(GROUP_EVENT_WORK + 4096 + 40))
            .ok_or(GroupStop::InvalidState)?;
        let use_linear = linear
            .is_some_and(|r| r.candidate_verified_original && r.assignments == v)
            && p.linear_retained
            && phase.basis_retained;
        report.origin = if use_linear {
            GroupOrigin::ReportedBasisLinearCandidate
        } else {
            GroupOrigin::IndependentGroupPrefix
        };
        start_cursor(&mut p.workspace.group, m, d, report.origin, &mut checking)?;
        loop {
            report.tuple_entered = false;
            report.completed_prefixes = 0;
            report.computed_match = false;
            p.step4.last = LastProgress {
                initialized: true,
                origin: Some(report.origin),
                cursor: report.cursor,
                ..LastProgress::default()
            };
            if p.group_control.tuples >= p.group_control.tuple_limit {
                return Err(GroupStop::TupleLimit);
            }
            checking.check().map_err(group_stop)?;
            let reserve_add = m as u64 - 1;
            if p.group_control
                .addition_limit
                .saturating_sub(p.group_control.additions_reserved)
                < reserve_add
            {
                return Err(GroupStop::AdditionLimit);
            }
            if checking.work_limit.saturating_sub(checking.reserved_work) < tuple_work {
                return Err(GroupStop::CheckWorkLimit);
            }
            if p.slots.buffers[5]
                .logical_cap
                .saturating_sub(p.slots.buffers[5].bytes.len())
                < report.row_bound
            {
                return Err(GroupStop::GroupBytes);
            }
            if p.observer.acknowledged >= 4096
                || p.slots.buffers[3]
                    .logical_cap
                    .saturating_sub(p.slots.buffers[3].bytes.len())
                    < 1025
            {
                return Err(GroupStop::EventLimit);
            }
            {
                let _ticket = checking.reserve(tuple_work, 0).map_err(group_stop)?;
            }
            let mut local = Ledger::new(tuple_work, 0, p.original_deadline);
            p.group_control.additions_reserved += reserve_add;
            p.group_control.tuples += 1;
            report.tuple_entered = true;
            let mut exceptions = 0;
            let mut complete = false;
            let mut all_affine = false;
            let mut evaluated = false;
            let mut coords_match = false;
            let mut sum = SmallPoint::Infinity;
            let operation = (|| {
                group_charge(&mut local, 4096)?;
                for leaf in 0..m {
                    let index = p.workspace.group.tuple[leaf];
                    if index >= d {
                        return Err(GroupStop::InvalidState);
                    }
                    let point = p.workspace.group.domain[index];
                    let SmallPoint::Affine(q) = point else {
                        return Err(GroupStop::InvalidDomain);
                    };
                    p.workspace.group.coordinates[leaf] = q.x;
                    p.step4.last.coordinate_count = leaf + 1;
                    if leaf == 0 {
                        sum = point;
                    } else {
                        #[cfg(test)]
                        GROUP_ADDITION_HOOK.with(|hook| {
                            if let Some(f) = hook.get() {
                                f(p.group_control.tuples, leaf, &mut local);
                            }
                        });
                        sum = group_add(&p.setup.field, p.group_control.a, sum, point, &mut local)?;
                        p.group_control.additions_performed += 1;
                        p.workspace.group.prefix[leaf - 1] = sum;
                        report.completed_prefixes = leaf;
                        if leaf < m - 1 {
                            match sum {
                                SmallPoint::Infinity => {
                                    p.workspace.group.exceptional[exceptions] = leaf - 1;
                                    exceptions += 1;
                                    p.step4.last.exceptions = exceptions;
                                }
                                SmallPoint::Affine(q) => {
                                    p.workspace.group.coordinates[m + leaf - 1] = q.x
                                }
                            }
                        }
                    }
                }
                complete = true;
                p.step4.last.complete = true;
                all_affine = exceptions == 0;
                report.computed_match = sum == SmallPoint::Affine(p.group_control.target);
                if report.computed_match && p.step4.certificate.identity.is_none() {
                    p.workspace.group.first_certificate[..m]
                        .copy_from_slice(&p.workspace.group.tuple[..m]);
                    let compatible = all_affine
                        && (0..v).all(|i| {
                            p.workspace.group.assignments[i]
                                == Some(p.workspace.group.coordinates[i])
                        });
                    let origin = if report.origin == GroupOrigin::IndependentGroupPrefix {
                        report.origin
                    } else if compatible {
                        GroupOrigin::ReportedBasisLinearCandidate
                    } else {
                        GroupOrigin::ReportedBasisLeafLift
                    };
                    let mut id = FirstIdentity {
                        origin,
                        phase: if origin == GroupOrigin::IndependentGroupPrefix {
                            SearchPhase::IndependentPrefix
                        } else {
                            SearchPhase::NativeLifts
                        },
                        cursor: report.cursor,
                        indices: [0; 15],
                        exception_mask: 0,
                        coordinates_match: if origin == GroupOrigin::IndependentGroupPrefix {
                            None
                        } else {
                            Some(compatible)
                        },
                    };
                    id.indices[..m].copy_from_slice(&p.workspace.group.tuple[..m]);
                    for &i in &p.workspace.group.exceptional[..exceptions] {
                        id.exception_mask |= 1 << i;
                    }
                    report.origin = id.origin;
                    p.step4.last.origin = Some(id.origin);
                    p.step4.certificate.identity = Some(id);
                    p.step4.certificate.facts.computed = true;
                    p.step4.certificate.lifecycle = CertificateLifecycle::ComputedPending;
                }
                if all_affine {
                    p.step4.last.coordinate_count = v;
                    let zero = group_eval(
                        &p.input,
                        v,
                        p.setup.field.p(),
                        &mut p.workspace.group,
                        &mut local,
                    )?;
                    evaluated = true;
                    p.step4.last.residue_count = v + 1;
                    if zero && phase.report.stop == dynamic::Stop::UnitDerived {
                        return Err(GroupStop::NativeContradiction);
                    }
                    if report.computed_match && !zero {
                        return Err(GroupStop::OriginalEquationMismatch);
                    }
                    coords_match = report.origin == GroupOrigin::IndependentGroupPrefix
                        || (0..v).all(|i| {
                            p.workspace.group.assignments[i]
                                == Some(p.workspace.group.coordinates[i])
                        });
                }
                if report.computed_match
                    && report.origin == GroupOrigin::ReportedBasisLinearCandidate
                    && !coords_match
                {
                    report.origin = GroupOrigin::ReportedBasisLeafLift;
                    p.step4.last.origin = Some(report.origin);
                }
                Ok(())
            })();
            if complete {
                match report.origin {
                    GroupOrigin::IndependentGroupPrefix => report.independent_visited += 1,
                    GroupOrigin::ReportedBasisLinearCandidate
                    | GroupOrigin::ReportedBasisLeafLift => report.native_lifts_visited += 1,
                }
            }
            // Inline prefix/counters remain even when original expiry prevents
            // writing this final partial row. Earlier serialized rows survive.
            let row_result = (|| {
                let g = &p.workspace.group;
                let row = GroupRow {
                    schema: "crypto.ml-dataset.wide-prime-dynamic-group-row/v1",
                    origin: report.origin,
                    cursor: report.cursor,
                    indices: &g.tuple[..m],
                    prefixes: &g.prefix[..report.completed_prefixes],
                    completed_additions: report.completed_prefixes,
                    exceptional_auxiliary_positions: &g.exceptional[..exceptions],
                    coordinates: (complete && all_affine).then_some(&g.coordinates[..v]),
                    residues: evaluated.then_some(&g.residues[..v + 1]),
                    total: complete.then_some(&sum),
                    computed_match: report.computed_match,
                    candidate_coordinates_match: coords_match,
                    complete,
                    tuples_reserved: p.group_control.tuples,
                    additions_reserved: p.group_control.additions_reserved,
                    additions_performed: p.group_control.additions_performed,
                    checking_reserved_work: p.checking_reserved_work + checking.reserved_work,
                };
                let row_start = p.slots.buffers[5].bytes.len();
                append_group_row(&row, &mut p.slots.buffers[5], report.row_bound, &mut local)?;
                p.step4.last.row = Some(RowReference {
                    row_index: p.observer.group as u64,
                    offset: row_start as u64,
                    bytes: (p.slots.buffers[5].bytes.len() - row_start) as u64,
                });
                let event = PhaseEvent {
                    schema: "crypto.ml-dataset.wide-prime-dynamic-event/v1",
                    source: PhaseSource::Group,
                    global_sequence: p.observer.acknowledged,
                    local_sequence: p.observer.group,
                    stage: if complete { "tuple" } else { "partial_tuple" },
                    reserved_work: p.checking_reserved_work + checking.reserved_work,
                    reserved_bytes: p.ledger.reserved_bytes,
                    persistence: p.observer.persistence,
                };
                append_observation(
                    event,
                    &mut p.observer,
                    &mut p.slots.buffers[3],
                    &mut local,
                    &mut p.capture,
                    forward,
                )
                .map_err(|_| group_stop(p.observer.stop.unwrap_or(Stop::Observer)))?;
                p.step4.last.metadata_acknowledged = true;
                Ok(())
            })();
            checking.performed_work += local.performed_work;
            operation?;
            row_result?;
            if report.computed_match {
                if let Some(publisher) = publisher.as_deref_mut() {
                    complete_match(
                        p,
                        phase.report.stop == dynamic::Stop::UnitDerived,
                        &report,
                        exceptions,
                        publisher,
                        &mut checking,
                    )?;
                } else {
                    return Ok(GroupStop::PendingCertificate);
                }
            }
            // Leaf-lift is certificate provenance, not a third enumeration.
            if report.origin == GroupOrigin::ReportedBasisLeafLift {
                report.origin = GroupOrigin::ReportedBasisLinearCandidate;
            }
            #[cfg(test)]
            GROUP_CARRY_HOOK.with(|hook| {
                if let Some(f) = hook.get() {
                    f(p.group_control.tuples, &mut checking);
                }
            });
            if advance_cursor(&mut p.workspace.group, m, d, report.origin, &mut checking)? {
                report.cursor += 1;
                continue;
            }
            if report.origin == GroupOrigin::ReportedBasisLinearCandidate {
                start_cursor(
                    &mut p.workspace.group,
                    m,
                    d,
                    GroupOrigin::IndependentGroupPrefix,
                    &mut checking,
                )?;
                report.origin = GroupOrigin::IndependentGroupPrefix;
                report.cursor = 0;
            } else {
                return Ok(GroupStop::IndependentTraversalFinished);
            }
        }
    })();
    p.checking_work -= checking.reserved_work;
    p.checking_reserved_work += checking.reserved_work;
    p.checking_performed_work += checking.performed_work;
    report.stop = match result {
        Ok(stop) | Err(stop) => stop,
    };
    report.tuples_reserved = p.group_control.tuples;
    report.additions_reserved = p.group_control.additions_reserved;
    report.additions_performed = p.group_control.additions_performed;
    report.checking_reserved_work = p.checking_reserved_work;
    report.checking_performed_work = p.checking_performed_work;
    report.pdp_witness_verified = p.step4.certificate.facts.local_replay_complete;
    report.certificate_durably_persisted = p.step4.certificate.facts.durable_acknowledged;
    report.group_domain_complete =
        publisher.is_some() && report.stop == GroupStop::IndependentTraversalFinished;
    p.step4.last_report = Some(report);
    report
}

#[cfg(test)]
thread_local! {static REPLAY_ADDITION_HOOK:std::cell::Cell<Option<fn(usize,&mut Ledger)>>=const{std::cell::Cell::new(None)};}
// Step4 independently implemented canonical arithmetic. No Montgomery calls.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
enum SearchPhase {
    NativeLifts,
    IndependentPrefix,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
enum ReplayStage {
    Indices,
    OnCurve,
    Group,
    PrefixCompare,
    OriginalEquations,
    Provenance,
    Serialization,
    Publication,
    FinalStaging,
}
#[derive(Clone, Copy, Debug, Default, serde::Serialize)]
struct FirstFacts {
    computed: bool,
    local_replay_complete: bool,
    publication_attempted: bool,
    durable_acknowledged: bool,
    final_artifact_registered: bool,
}
#[derive(Clone, Copy, Debug, Default, serde::Serialize)]
struct ReplayProgress {
    additions_reserved: u64,
    additions_performed: u64,
    equation_checks_completed: u16,
    coordinate_checks_completed: u16,
    failed_stage: Option<ReplayStage>,
}
#[derive(Clone, Copy)]
struct FirstIdentity {
    origin: GroupOrigin,
    phase: SearchPhase,
    cursor: u128,
    indices: [usize; 15],
    exception_mask: u16,
    coordinates_match: Option<bool>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
enum CertificateLifecycle {
    Absent,
    ComputedPending,
    ReplayAdmitted,
    LocallyVerified,
    Published,
    Transferred,
    Failed,
}
struct CertificateState {
    identity: Option<FirstIdentity>,
    facts: FirstFacts,
    progress: ReplayProgress,
    lifecycle: CertificateLifecycle,
    receipt: Option<crate::ml_dataset::capture::WitnessReceipt>,
}
impl Default for CertificateState {
    fn default() -> Self {
        Self {
            identity: None,
            facts: FirstFacts::default(),
            progress: ReplayProgress::default(),
            lifecycle: CertificateLifecycle::Absent,
            receipt: None,
        }
    }
}
fn canonical_add(
    p: u64,
    a: u64,
    left: SmallPoint,
    right: SmallPoint,
    l: &mut Ledger,
) -> Result<SmallPoint, GroupStop> {
    let mut t = l.reserve(4096, 0).map_err(group_stop)?;
    t.spend(128).map_err(group_stop)?;
    let (SmallPoint::Affine(q), SmallPoint::Affine(r)) = (left, right) else {
        return Ok(if matches!(left, SmallPoint::Infinity) {
            right
        } else {
            left
        });
    };
    if q.x == r.x && (q.y != r.y || q.y == 0) {
        return Ok(SmallPoint::Infinity);
    }
    t.spend(512).map_err(group_stop)?;
    let modulus = u128::from(p);
    let mul = |x: u64, y: u64| ((u128::from(x) * u128::from(y)) % modulus) as u64;
    let sub = |x: u64, y: u64| ((u128::from(x) + modulus - u128::from(y)) % modulus) as u64;
    let (num, den) = if q.x == r.x {
        (
            ((3 * u128::from(mul(q.x, q.x)) + u128::from(a)) % modulus) as u64,
            ((2 * u128::from(q.y)) % modulus) as u64,
        )
    } else {
        (sub(r.y, q.y), sub(r.x, q.x))
    };
    let mut power = den;
    let mut inverse = 1u64;
    let mut exponent = p - 2;
    while exponent != 0 {
        t.spend(48).map_err(group_stop)?;
        if exponent & 1 != 0 {
            inverse = mul(inverse, power);
        }
        exponent >>= 1;
        if exponent != 0 {
            power = mul(power, power);
        }
    }
    t.spend(256).map_err(group_stop)?;
    if mul(den, inverse) != 1 {
        return Err(GroupStop::InvalidDomain);
    }
    let slope = mul(num, inverse);
    let x = sub(sub(mul(slope, slope), q.x), r.x);
    let y = sub(mul(slope, sub(q.x, x)), q.y);
    Ok(SmallPoint::Affine(AffinePoint { x, y }))
}
fn replay_on_curve(
    p: u64,
    a: u64,
    b: u64,
    q: AffinePoint,
    l: &mut Ledger,
) -> Result<(), GroupStop> {
    group_charge(l, 256)?;
    if !(3..=(1u64 << 61) - 1).contains(&p) || a >= p || b >= p || q.x >= p || q.y >= p {
        return Err(GroupStop::InvalidDomain);
    }
    let modulus = u128::from(p);
    let x = u128::from(q.x);
    let y = u128::from(q.y);
    if y * y % modulus != ((x * x % modulus) * x + u128::from(a) * x + u128::from(b)) % modulus {
        return Err(GroupStop::InvalidDomain);
    }
    Ok(())
}
// Stream original equations independently, without invoking group_eval or
// trusting the producer's residues. Coordinates are reconstructed from replay.
fn replay_equations(
    input: &[InputPolynomial],
    coordinates: &[u64],
    expected: &[u64],
    p: u64,
    l: &mut Ledger,
    progress: &mut ReplayProgress,
) -> Result<(), GroupStop> {
    group_charge(l, 128)?;
    for (i, poly) in input.iter().enumerate() {
        group_charge(l, 32)?;
        let mut residue = 0u64;
        for term in &poly.terms {
            group_charge(l, 64 + 32 * coordinates.len() as u64)?;
            if term.coefficient == 0 || term.coefficient >= p {
                return Err(GroupStop::InvalidState);
            }
            let mut monomial = term.coefficient;
            for (axis, &exponent) in term
                .mono
                .exponents(coordinates.len())
                .ok_or(GroupStop::InvalidState)?
                .iter()
                .enumerate()
            {
                if exponent == 0 {
                    continue;
                }
                if exponent > 32 {
                    return Err(GroupStop::InvalidState);
                }
                group_charge(l, 64 + 48 * u64::from(u16::BITS - exponent.leading_zeros()))?;
                let mut power = 1u64;
                let mut square = coordinates[axis];
                let mut bits = exponent;
                while bits != 0 {
                    if bits & 1 != 0 {
                        power = ((u128::from(power) * u128::from(square)) % u128::from(p)) as u64;
                    }
                    bits >>= 1;
                    if bits != 0 {
                        square = ((u128::from(square) * u128::from(square)) % u128::from(p)) as u64;
                    }
                }
                monomial = ((u128::from(monomial) * u128::from(power)) % u128::from(p)) as u64;
            }
            residue = ((u128::from(residue) + u128::from(monomial)) % u128::from(p)) as u64;
        }
        group_charge(l, 32)?;
        if residue != 0 || expected.get(i) != Some(&residue) {
            return Err(GroupStop::OriginalEquationMismatch);
        }
        progress.equation_checks_completed += 1;
    }
    Ok(())
}
fn replay_match(
    p: &mut PreparedOriginal,
    native_unit: bool,
    identity: FirstIdentity,
    checking: &mut Ledger,
    progress: &mut ReplayProgress,
) -> Result<(), GroupStop> {
    let m = p.m;
    let v = 2 * m - 2;
    let prime = p.setup.field.p();
    progress.failed_stage = Some(ReplayStage::Indices);
    if !(4..=15).contains(&m) || p.group_control.domain_len == 0 || p.group_control.domain_len > 64
    {
        return Err(GroupStop::InvalidState);
    }
    let eval = group_eval_bound(&p.input, v, checking)?;
    let work = 4096 * m as u64
        + 256 * (m as u64 + 1)
        + 128 * m as u64
        + 128 * (m as u64 - 1)
        + 64 * v as u64
        + eval
        + 32 * (v as u64 + 1);
    if p.group_control
        .addition_limit
        .saturating_sub(p.group_control.additions_reserved)
        < m as u64
    {
        return Err(GroupStop::AdditionLimit);
    }
    {
        let _ticket = checking.reserve(work, 0).map_err(group_stop)?;
    }
    p.group_control.additions_reserved += m as u64;
    progress.additions_reserved += m as u64;
    let mut local = Ledger::new(work, 0, p.original_deadline);
    let result = (|| {
        let mut coordinates = [0u64; 28];
        let mut sum = SmallPoint::Infinity;
        let mut mask = 0u16;
        progress.failed_stage = Some(ReplayStage::OnCurve);
        replay_on_curve(
            prime,
            p.group_control.a,
            p.group_control.b,
            p.group_control.target,
            &mut local,
        )?;
        for leaf in 0..m {
            progress.failed_stage = Some(ReplayStage::Indices);
            group_charge(&mut local, 128)?;
            let index = identity.indices[leaf];
            if index >= p.group_control.domain_len || index != p.workspace.group.tuple[leaf] {
                return Err(GroupStop::InvalidCandidate);
            }
            let SmallPoint::Affine(point) = p.workspace.group.domain[index] else {
                return Err(GroupStop::InvalidDomain);
            };
            progress.failed_stage = Some(ReplayStage::OnCurve);
            replay_on_curve(
                prime,
                p.group_control.a,
                p.group_control.b,
                point,
                &mut local,
            )?;
            coordinates[leaf] = point.x;
            progress.failed_stage = Some(ReplayStage::Group);
            #[cfg(test)]
            REPLAY_ADDITION_HOOK.with(|hook| {
                if let Some(f) = hook.get() {
                    f(leaf, &mut local);
                }
            });
            sum = canonical_add(
                prime,
                p.group_control.a,
                sum,
                SmallPoint::Affine(point),
                &mut local,
            )?;
            progress.additions_performed += 1;
            p.group_control.additions_performed += 1;
            if leaf != 0 {
                progress.failed_stage = Some(ReplayStage::PrefixCompare);
                group_charge(&mut local, 128)?;
                if p.workspace.group.prefix[leaf - 1] != sum {
                    return Err(GroupStop::InvalidCandidate);
                }
                if leaf < m - 1 {
                    match sum {
                        SmallPoint::Infinity => mask |= 1 << (leaf - 1),
                        SmallPoint::Affine(q) => coordinates[m + leaf - 1] = q.x,
                    }
                }
            }
        }
        if sum != SmallPoint::Affine(p.group_control.target) || mask != identity.exception_mask {
            return Err(GroupStop::InvalidCandidate);
        }
        progress.failed_stage = Some(ReplayStage::Provenance);
        if mask == 0 {
            let mut candidate_match = true;
            for (i, &coordinate) in coordinates[..v].iter().enumerate() {
                group_charge(&mut local, 64)?;
                if coordinate != p.workspace.group.coordinates[i] {
                    return Err(GroupStop::InvalidCandidate);
                }
                candidate_match &= p.workspace.group.assignments[i] == Some(coordinate);
                progress.coordinate_checks_completed += 1;
            }
            if identity
                .coordinates_match
                .is_some_and(|claimed| claimed != candidate_match)
            {
                return Err(GroupStop::InvalidCandidate);
            }
            progress.failed_stage = Some(ReplayStage::OriginalEquations);
            replay_equations(
                &p.input,
                &coordinates[..v],
                &p.workspace.group.residues[..v + 1],
                prime,
                &mut local,
                progress,
            )?;
            if native_unit {
                return Err(GroupStop::NativeContradiction);
            }
        } else if identity.coordinates_match == Some(true) {
            return Err(GroupStop::InvalidCandidate);
        }
        progress.failed_stage = None;
        Ok(())
    })();
    checking.performed_work += local.performed_work;
    result
}
struct CanonicalPointView(SmallPoint);
impl serde::Serialize for CanonicalPointView {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        match self.0 {
            SmallPoint::Infinity => s.serialize_none(),
            SmallPoint::Affine(q) => {
                let mut record = s.serialize_struct("Point", 2)?;
                record.serialize_field("x", &Decimal(q.x))?;
                record.serialize_field("y", &Decimal(q.y))?;
                record.end()
            }
        }
    }
}
struct SummandsView<'a> {
    domain: &'a [SmallPoint],
    indices: &'a [usize],
}
impl serde::Serialize for SummandsView<'_> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::{Error, SerializeSeq};
        let mut seq = s.serialize_seq(Some(self.indices.len()))?;
        for &i in self.indices {
            let q = *self
                .domain
                .get(i)
                .ok_or_else(|| S::Error::custom("invalid index"))?;
            seq.serialize_element(&CanonicalPointView(q))?;
        }
        seq.end()
    }
}
struct ExceptionView {
    mask: u16,
    m: usize,
}
impl serde::Serialize for ExceptionView {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        let mut seq = s.serialize_seq(Some(self.mask.count_ones() as usize))?;
        for i in 0..self.m - 2 {
            if self.mask & (1 << i) != 0 {
                seq.serialize_element(&i)?;
            }
        }
        seq.end()
    }
}
#[derive(serde::Serialize)]
struct DynamicWitness<'a> {
    schema: &'static str,
    summands: SummandsView<'a>,
    target: CanonicalPointView,
    sum: CanonicalPointView,
    replay_group_add_calls: u64,
    certificate_origin: GroupOrigin,
    search_phase: SearchPhase,
    #[serde(serialize_with = "serialize_u128_decimal")]
    source_cursor: u128,
    tuple_indices: &'a [usize],
    candidate_coordinates_match: Option<bool>,
    exceptional_auxiliary_positions: ExceptionView,
    independent_derivation_verified: bool,
}
fn witness_view<'a>(p: &'a PreparedOriginal, id: &'a FirstIdentity) -> DynamicWitness<'a> {
    DynamicWitness {
        schema: "restricted-wide-prime-dynamic-witness/v1",
        summands: SummandsView {
            domain: &p.workspace.group.domain[..p.group_control.domain_len],
            indices: &id.indices[..p.m],
        },
        target: CanonicalPointView(SmallPoint::Affine(p.group_control.target)),
        sum: CanonicalPointView(SmallPoint::Affine(p.group_control.target)),
        replay_group_add_calls: p.m as u64,
        certificate_origin: id.origin,
        search_phase: id.phase,
        source_cursor: id.cursor,
        tuple_indices: &id.indices[..p.m],
        candidate_coordinates_match: id.coordinates_match,
        exceptional_auxiliary_positions: ExceptionView {
            mask: id.exception_mask,
            m: p.m,
        },
        independent_derivation_verified: false,
    }
}

use crate::ml_dataset::capture::{
    WitnessBudget, WitnessCaptureWorkspace, WitnessCommit, WitnessError, WitnessReceipt,
};
use crate::ml_dataset::types::ArtifactPayload;

/// Implementations are trusted capture adapters, never untrusted mathematical
/// receipt suppliers. Production uses only the bound bridge below.
pub(super) trait WitnessPublisher {
    fn prepare(&mut self, budget: &mut WitnessBudget) -> Result<(), WitnessError>;
    fn confirmation_work(&self, bytes: usize) -> Result<u64, WitnessError>;
    fn publish(
        &mut self,
        bytes: &[u8],
        budget: &mut WitnessBudget,
        global: &mut usize,
    ) -> Result<WitnessReceipt, WitnessError>;
}
#[derive(Default)]
pub(super) struct BoundWitnessPublisher {
    workspace: Option<WitnessCaptureWorkspace>,
}
impl WitnessPublisher for BoundWitnessPublisher {
    fn confirmation_work(&self, bytes: usize) -> Result<u64, WitnessError> {
        crate::ml_dataset::capture::witness_confirmation_work(bytes)
    }

    fn prepare(&mut self, budget: &mut WitnessBudget) -> Result<(), WitnessError> {
        if self.workspace.is_some() {
            return Err(WitnessError::Failed);
        }
        self.workspace = Some(super::prepare_bound_witness(budget)?);
        Ok(())
    }
    fn publish(
        &mut self,
        bytes: &[u8],
        budget: &mut WitnessBudget,
        global: &mut usize,
    ) -> Result<WitnessReceipt, WitnessError> {
        super::publish_bound_witness(
            bytes,
            self.workspace.as_mut().ok_or(WitnessError::Failed)?,
            budget,
            global,
        )
    }
}
struct Step4State {
    certificate: CertificateState,
    budget: WitnessBudget,
    prepared: bool,
    failed: bool,
    final_artifacts: Vec<ArtifactPayload>,
    transferred: bool,
    new_payload: usize,
    last: LastProgress,
    last_report: Option<GroupReport>,
    publication_error: Option<WitnessError>,
    terminal_retained: bool,
    observer_performed: u64,
    linear_stop: Option<LinearStop>,
    final_staging_stop: Option<GroupStop>,
}
#[derive(Clone, Copy, Default)]
struct LastProgress {
    initialized: bool,
    origin: Option<GroupOrigin>,
    cursor: u128,
    complete: bool,
    coordinate_count: usize,
    residue_count: usize,
    exceptions: usize,
    row: Option<RowReference>,
    metadata_acknowledged: bool,
}
#[derive(Clone, Copy, serde::Serialize)]
struct RowReference {
    row_index: u64,
    offset: u64,
    bytes: u64,
}
impl Step4State {
    fn new(deadline: Instant) -> Self {
        Self {
            certificate: CertificateState::default(),
            budget: WitnessBudget::new(0, 0, deadline),
            prepared: false,
            failed: false,
            final_artifacts: Vec::new(),
            transferred: false,
            new_payload: 0,
            last: LastProgress::default(),
            last_report: None,
            publication_error: None,
            terminal_retained: false,
            observer_performed: 0,
            linear_stop: None,
            final_staging_stop: None,
        }
    }
}
const SLOT_REGISTRY: [(&str, &str, &str); 8] = [
    (
        "prime-f4-input.json",
        "complete-prime-field-polynomial-system",
        "json",
    ),
    (
        "prime-f4-wide-dynamic-setup.json",
        "restricted-wide-prime-dynamic-setup/v1",
        "json",
    ),
    (
        "prime-f4-wide-dynamic-native.json",
        "restricted-wide-prime-dynamic-native/v1",
        "json",
    ),
    (
        "prime-f4-wide-dynamic-events.jsonl",
        "restricted-wide-prime-dynamic-events/v1",
        "jsonl",
    ),
    (
        "prime-f4-wide-dynamic-linear.json",
        "restricted-wide-prime-dynamic-linear/v1",
        "json",
    ),
    (
        "prime-f4-wide-dynamic-group.json",
        "restricted-wide-prime-dynamic-group/v1",
        "jsonl",
    ),
    ("pdp-witness.json", "independently-verified-pdp", "json"),
    (
        "prime-f4-wide-dynamic-result.json",
        "restricted-wide-prime-dynamic-result/v1",
        "json",
    ),
];
fn admitted_string(s: &str, l: &mut Ledger) -> Result<String, Stop> {
    let work = 8 * s.len() as u64 + 32;
    let mut ticket = l.reserve(work, s.len() as u64 + 128)?;
    let mut out = String::new();
    out.try_reserve_exact(s.len())
        .map_err(|_| Stop::Allocation)?;
    ticket.spend(work)?;
    out.push_str(s);
    Ok(out)
}
fn witness_error(e: WitnessError) -> GroupStop {
    match e {
        WitnessError::Deadline => GroupStop::Deadline,
        WitnessError::Work => GroupStop::CheckWorkLimit,
        WitnessError::Storage | WitnessError::Allocation => GroupStop::GroupBytes,
        WitnessError::EventLimit => GroupStop::EventLimit,
        _ => GroupStop::PersistenceFailure,
    }
}
// Clock-independent receipt settlement; only the exclusively lent deltas are
// charged. Even failures close the supply before returning control to parent.
fn settle_witness(
    p: &mut PreparedOriginal,
    l: &mut Ledger,
    before: (u64, u64, u64),
) -> Result<(), GroupStop> {
    let budget = &mut p.step4.budget;
    budget.release_exclusive_supply();
    let work = budget
        .reserved_work
        .checked_sub(before.0)
        .ok_or(GroupStop::InvalidState)?;
    let done = budget
        .performed_work
        .checked_sub(before.1)
        .ok_or(GroupStop::InvalidState)?;
    let bytes = budget
        .reserved_bytes
        .checked_sub(before.2)
        .ok_or(GroupStop::InvalidState)?;
    if done > work
        || work > l.work_limit - l.reserved_work
        || bytes > p.ledger.bytes_limit - p.ledger.reserved_bytes
    {
        return Err(GroupStop::InvalidState);
    }
    l.reserved_work += work;
    l.performed_work += done;
    p.ledger.reserved_bytes += bytes;
    Ok(())
}
/// Mandatory before native when using step4; the dormant earlier-stage API
/// remains available for focused construction/native controls.
pub(super) fn prepare_step4<P: WitnessPublisher>(
    p: &mut PreparedOriginal,
    publisher: &mut P,
) -> Result<(), GroupStop> {
    if p.step4.prepared || p.step4.failed || p.group_control.started {
        return Err(GroupStop::InvalidState);
    }
    let mut checking = Ledger::new(
        p.checking_work,
        p.ledger.bytes_limit - p.ledger.reserved_bytes,
        p.original_deadline,
    );
    let result = (|| {
        p.step4.final_artifacts = requested_vec(9, &mut checking).map_err(group_stop)?;
        for (path, role, encoding) in SLOT_REGISTRY {
            p.step4.final_artifacts.push(ArtifactPayload {
                path: admitted_string(path, &mut checking).map_err(group_stop)?,
                role: admitted_string(role, &mut checking).map_err(group_stop)?,
                encoding: admitted_string(encoding, &mut checking).map_err(group_stop)?,
                bytes: Vec::new(),
            });
        }
        Ok(())
    })();
    p.ledger.reserved_bytes += checking.reserved_bytes;
    checking.bytes_limit = 0;
    checking.reserved_bytes = 0;
    let result = result.and_then(|()| {
        p.step4
            .budget
            .set_exclusive_supply(
                checking.work_limit - checking.reserved_work,
                p.ledger.bytes_limit - p.ledger.reserved_bytes,
            )
            .map_err(witness_error)?;
        let before = (
            p.step4.budget.reserved_work,
            p.step4.budget.performed_work,
            p.step4.budget.reserved_bytes,
        );
        let result = publisher
            .prepare(&mut p.step4.budget)
            .map_err(witness_error);
        settle_witness(p, &mut checking, before)?;
        result
    });
    p.checking_work -= checking.reserved_work;
    p.checking_reserved_work += checking.reserved_work;
    p.checking_performed_work += checking.performed_work;
    p.step4.prepared = result.is_ok();
    p.step4.failed = result.is_err();
    result
}
fn complete_match<P: WitnessPublisher>(
    p: &mut PreparedOriginal,
    native_unit: bool,
    report: &GroupReport,
    exceptions: usize,
    publisher: &mut P,
    checking: &mut Ledger,
) -> Result<(), GroupStop> {
    if !p.step4.prepared || p.step4.failed {
        return Err(GroupStop::InvalidState);
    }
    let first = !p.step4.certificate.facts.local_replay_complete;
    if first && p.observer.acknowledged >= 4096 {
        return Err(GroupStop::EventLimit);
    }
    let mut mask = 0u16;
    for &i in &p.workspace.group.exceptional[..exceptions] {
        mask |= 1 << i;
    }
    let phase = if report.origin == GroupOrigin::IndependentGroupPrefix {
        SearchPhase::IndependentPrefix
    } else {
        SearchPhase::NativeLifts
    };
    let mut identity = FirstIdentity {
        origin: report.origin,
        phase,
        cursor: report.cursor,
        indices: [0; 15],
        exception_mask: mask,
        coordinates_match: match report.origin {
            GroupOrigin::IndependentGroupPrefix => None,
            GroupOrigin::ReportedBasisLinearCandidate => Some(true),
            GroupOrigin::ReportedBasisLeafLift => Some(false),
        },
    };
    identity.indices[..p.m].copy_from_slice(&p.workspace.group.tuple[..p.m]);
    if first {
        p.workspace.group.first_certificate[..p.m].copy_from_slice(&identity.indices[..p.m]);
        p.step4.certificate.identity = Some(identity);
        p.step4.certificate.facts.computed = true;
        p.step4.certificate.lifecycle = CertificateLifecycle::ComputedPending;
    }
    let mut progress = ReplayProgress::default();
    if first {
        p.step4.certificate.lifecycle = CertificateLifecycle::ReplayAdmitted;
    }
    let replay = replay_match(p, native_unit, identity, checking, &mut progress);
    if first {
        p.step4.certificate.progress = progress;
    }
    if let Err(e) = replay {
        p.step4.failed = true;
        if first {
            p.step4.certificate.lifecycle = CertificateLifecycle::Failed;
        }
        return Err(e);
    }
    if !first {
        return Ok(());
    }
    p.step4.certificate.facts.local_replay_complete = true;
    p.step4.certificate.lifecycle = CertificateLifecycle::LocallyVerified;
    // Split immutable view fields from the separately owned admitted output.
    let witness = DynamicWitness {
        schema: "restricted-wide-prime-dynamic-witness/v1",
        summands: SummandsView {
            domain: &p.workspace.group.domain[..p.group_control.domain_len],
            indices: &identity.indices[..p.m],
        },
        target: CanonicalPointView(SmallPoint::Affine(p.group_control.target)),
        sum: CanonicalPointView(SmallPoint::Affine(p.group_control.target)),
        replay_group_add_calls: p.m as u64,
        certificate_origin: identity.origin,
        search_phase: identity.phase,
        source_cursor: identity.cursor,
        tuple_indices: &identity.indices[..p.m],
        candidate_coordinates_match: identity.coordinates_match,
        exceptional_auxiliary_positions: ExceptionView {
            mask: identity.exception_mask,
            m: p.m,
        },
        independent_derivation_verified: false,
    };
    if let Err(e) = encode_complete(&witness, &mut p.slots.buffers[6], checking) {
        p.step4.failed = true;
        p.step4.certificate.lifecycle = CertificateLifecycle::Failed;
        p.step4.certificate.progress.failed_stage = Some(ReplayStage::Serialization);
        return Err(group_stop(e));
    }
    p.step4.certificate.facts.publication_attempted = true;
    p.step4
        .budget
        .set_exclusive_supply(
            checking.work_limit - checking.reserved_work,
            p.ledger.bytes_limit - p.ledger.reserved_bytes,
        )
        .map_err(witness_error)?;
    let before = (
        p.step4.budget.reserved_work,
        p.step4.budget.performed_work,
        p.step4.budget.reserved_bytes,
    );
    let receipt = publisher.publish(
        &p.slots.buffers[6].bytes,
        &mut p.step4.budget,
        &mut p.observer.acknowledged,
    );
    if let Ok(ack) = receipt {
        p.step4.certificate.receipt = Some(ack);
        p.step4.certificate.facts.durable_acknowledged = true;
        p.step4.certificate.lifecycle = CertificateLifecycle::Published;
    }
    settle_witness(p, checking, before)?;
    match receipt {
        Ok(receipt) => {
            // Trusted bridge already committed and incremented global if new.
            // No post-ack clock/fallible allocation may revoke this fact.
            p.step4.certificate.receipt = Some(receipt);
            p.step4.certificate.facts.durable_acknowledged = true;
            p.step4.certificate.lifecycle = CertificateLifecycle::Published;
            if receipt.commit == WitnessCommit::NewCommit {
                p.step4.new_payload += 1;
                p.observer.durable += 1;
            }
            p.step4.final_artifacts[6].bytes = std::mem::take(&mut p.slots.buffers[6].bytes);
            p.step4.transferred = true;
            p.step4.certificate.lifecycle = CertificateLifecycle::Transferred;
            // Keep final readback inside the ORIGINAL parent remainder. Only
            // shrink this traversal ledger: the parent's unspent work remains
            // available to confirm_step4_witness after traversal settles.
            let held = publisher
                .confirmation_work(p.step4.final_artifacts[6].bytes.len())
                .map_err(witness_error)?;
            if held > checking.work_limit - checking.reserved_work {
                return Err(GroupStop::CheckWorkLimit);
            }
            checking.work_limit -= held;
            Ok(())
        }
        Err(error) => {
            p.step4.publication_error = Some(error);
            p.step4.failed = true;
            p.step4.certificate.lifecycle = CertificateLifecycle::Failed;
            p.step4.certificate.progress.failed_stage = Some(ReplayStage::Publication);
            Err(witness_error(error))
        }
    }
}

pub(super) fn run_group_prefixes<F: FnMut(&PhaseEvent, &mut TypedEventBuffer) -> Result<(), ()>>(
    phase: &mut NativePhase,
    linear: Option<&LinearReport>,
    forward: &mut F,
) -> GroupReport {
    run_group_internal::<F, BoundWitnessPublisher>(phase, linear, forward, None)
}
pub(super) fn run_group_step4<
    F: FnMut(&PhaseEvent, &mut TypedEventBuffer) -> Result<(), ()>,
    P: WitnessPublisher,
>(
    phase: &mut NativePhase,
    linear: Option<&LinearReport>,
    forward: &mut F,
    publisher: &mut P,
) -> GroupReport {
    run_group_internal(phase, linear, forward, Some(publisher))
}

struct HexDigest([u8; 32]);
impl serde::Serialize for HexDigest {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut bytes = [0u8; 64];
        for (i, &b) in self.0.iter().enumerate() {
            bytes[2 * i] = b"0123456789abcdef"[(b >> 4) as usize];
            bytes[2 * i + 1] = b"0123456789abcdef"[(b & 15) as usize];
        }
        s.serialize_str(std::str::from_utf8(&bytes).map_err(serde::ser::Error::custom)?)
    }
}
struct PointSlice<'a>(&'a [SmallPoint]);
impl serde::Serialize for PointSlice<'_> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        let mut seq = s.serialize_seq(Some(self.0.len()))?;
        for &p in self.0 {
            seq.serialize_element(&CanonicalPointView(p))?;
        }
        seq.end()
    }
}
#[derive(serde::Serialize)]
struct ReceiptView {
    commit: &'static str,
    payload_sha256: HexDigest,
    payload_bytes: u64,
    sequence: Option<u64>,
    provenance: &'static str,
}
fn receipt_view(r: WitnessReceipt) -> ReceiptView {
    ReceiptView {
        commit: match r.commit {
            WitnessCommit::NewCommit => "new_commit",
            WitnessCommit::AlreadyCommitted => "already_committed",
        },
        payload_sha256: HexDigest(r.payload_sha256),
        payload_bytes: r.payload_bytes,
        sequence: r.sequence,
        provenance: match r.provenance {
            crate::ml_dataset::capture::WitnessProvenance::Dedicated => "dedicated",
            crate::ml_dataset::capture::WitnessProvenance::LegacyAdopted => "legacy_adopted",
        },
    }
}
#[derive(serde::Serialize)]
struct IdentityView<'a> {
    origin: GroupOrigin,
    search_phase: SearchPhase,
    #[serde(serialize_with = "serialize_u128_decimal")]
    source_cursor: u128,
    tuple_indices: &'a [usize],
    exceptional_auxiliary_positions: ExceptionView,
    candidate_coordinates_match: Option<bool>,
}
#[derive(serde::Serialize)]
struct PublicationView {
    state: &'static str,
    receipt: Option<ReceiptView>,
}
#[derive(serde::Serialize)]
struct FirstRecord<'a> {
    lifecycle: CertificateLifecycle,
    identity: IdentityView<'a>,
    facts: FirstFacts,
    replay: ReplayProgress,
    publication: PublicationView,
    witness_owner: &'static str,
    witness_bytes: Option<u64>,
    witness_sha256: Option<HexDigest>,
}
#[derive(serde::Serialize)]
struct LastState<'a> {
    phase: SearchPhase,
    origin: GroupOrigin,
    #[serde(serialize_with = "serialize_u128_decimal")]
    cursor: u128,
    tuple_indices: &'a [usize],
    progress: &'static str,
    tuple_entered: bool,
    completed_prefixes: u8,
    prefix_points: PointSlice<'a>,
    exceptional_auxiliary_positions: &'a [usize],
    coordinate_count: u8,
    coordinates: &'a [u64],
    residue_count: u8,
    residues: &'a [u64],
    computed_match: bool,
    row: Option<RowReference>,
    metadata_acknowledged: bool,
}
#[derive(serde::Serialize)]
struct SlotEvidence {
    index: u8,
    state: &'static str,
    bytes: Option<u64>,
    digest_binding: &'static str,
    sha256: Option<HexDigest>,
}
#[derive(serde::Serialize)]
struct PoolView {
    name: &'static str,
    parent: &'static str,
    work_limit: u64,
    reserved_work: u64,
    performed_work: Option<u64>,
}
#[derive(serde::Serialize)]
struct ByteAccounting {
    limit: u64,
    reserved: u64,
}
#[derive(serde::Serialize)]
struct GroupAccounting {
    addition_limit: u64,
    additions_reserved: u64,
    additions_performed: u64,
    tuple_limit: u64,
    tuples_reserved: u64,
}
#[derive(serde::Serialize)]
struct TraversalAccounting {
    #[serde(serialize_with = "serialize_u128_decimal")]
    native_lifts_visited: u128,
    #[serde(serialize_with = "serialize_u128_decimal")]
    independent_visited: u128,
    #[serde(serialize_with = "serialize_u128_decimal")]
    independent_total: u128,
}
#[derive(serde::Serialize)]
struct EventAccounting {
    limit: u64,
    global_acknowledged: u64,
    setup: u64,
    native: u64,
    linear: u64,
    group: u64,
    new_payload: u64,
    attempted: u64,
    pending: u64,
}
#[derive(serde::Serialize)]
struct Accounting {
    counter_snapshot_stage: &'static str,
    pools: [PoolView; 6],
    requested_bytes: ByteAccounting,
    group: GroupAccounting,
    traversal: TraversalAccounting,
    events: EventAccounting,
}
#[derive(serde::Serialize)]
struct Claims {
    solver_completed: bool,
    group_domain_complete: bool,
    negative_verified: bool,
    pdp_witness_verified: bool,
    certificate_durably_persisted: bool,
    independent_derivation_verified: bool,
    operation_trajectory_verified: bool,
}
#[derive(serde::Serialize)]
struct DynamicResult<'a> {
    schema: &'static str,
    strategy: &'static str,
    stage: &'static str,
    disposition: &'static str,
    cause: &'static str,
    native_stop: Option<&'static str>,
    linear_stop: Option<LinearStop>,
    group_stop: Option<GroupStop>,
    search_phase: &'static str,
    accounting: Accounting,
    source_slots: [SlotEvidence; 7],
    last_state: Option<LastState<'a>>,
    first_certificate: Option<FirstRecord<'a>>,
    claims: Claims,
}
// A single exclusively supplied local ledger serves both serialization passes.
// Atomic slot promotion includes newline; failure never exposes partial JSON.
fn encode_complete<T: serde::Serialize>(
    record: &T,
    slot: &mut InputBuffer,
    l: &mut Ledger,
) -> Result<(), Stop> {
    slot.bytes.clear();
    let result = (|| {
        let cap = slot.logical_cap.checked_sub(1).ok_or(Stop::OutputLimit)?;
        let before = l.reserved_work;
        let mut count = MeteredWriter {
            ledger: l,
            output: None,
            count: 0,
            cap,
            stop: None,
        };
        if serde_json::to_writer(&mut count, record).is_err() {
            return Err(count.stop.unwrap_or(Stop::Serialization));
        }
        let expected = count.count;
        let cost = l.reserved_work - before;
        if l.work_limit - l.reserved_work < cost.checked_add(8).ok_or(Stop::ArithmeticOverflow)? {
            return Err(Stop::WorkLimit);
        }
        let write_before = l.reserved_work;
        let mut writer = MeteredWriter {
            ledger: l,
            output: Some(&mut slot.bytes),
            count: 0,
            cap,
            stop: None,
        };
        if serde_json::to_writer(&mut writer, record).is_err() {
            return Err(writer.stop.unwrap_or(Stop::Serialization));
        }
        if writer.count != expected || l.reserved_work - write_before != cost {
            return Err(Stop::Serialization);
        }
        {
            let mut t = l.reserve(8, 0)?;
            t.spend(8)?;
        }
        slot.bytes.push(b'\n');
        Ok(())
    })();
    if result.is_err() {
        slot.bytes.clear();
    }
    result
}
fn group_cause(stop: GroupStop) -> &'static str {
    match stop {
        GroupStop::IndependentTraversalFinished => "none",
        GroupStop::Deadline => "deadline",
        GroupStop::CheckWorkLimit => "work_limit",
        GroupStop::AdditionLimit => "addition_limit",
        GroupStop::TupleLimit => "tuple_limit",
        GroupStop::GroupBytes => "artifact_bytes",
        GroupStop::EventLimit => "event_limit",
        GroupStop::PersistenceFailure => "persistence_failure",
        GroupStop::NativeContradiction => "native_contradiction",
        _ => "group_stop",
    }
}
/// Terminal evidence is only a snapshot. Mathematical counters cannot assert
/// completion of later outer manifest registration or terminal serialization.
pub(super) fn retain_step4_terminal(phase: &mut NativePhase) -> Result<(), Stop> {
    let p = &mut phase.prepared;
    if p.step4.terminal_retained {
        return Err(Stop::InvalidInput);
    }
    let entered_group = p.step4.last_report.is_some();
    let report = p.step4.last_report.unwrap_or(GroupReport {
        stop: GroupStop::InvalidState,
        origin: GroupOrigin::IndependentGroupPrefix,
        cursor: 0,
        independent_visited: 0,
        native_lifts_visited: 0,
        independent_total: 0,
        completed_prefixes: 0,
        tuple_entered: false,
        computed_match: false,
        row_bound: 0,
        tuples_reserved: p.group_control.tuples,
        additions_reserved: p.group_control.additions_reserved,
        additions_performed: p.group_control.additions_performed,
        checking_reserved_work: p.checking_reserved_work,
        checking_performed_work: p.checking_performed_work,
        group_domain_complete: false,
        pdp_witness_verified: false,
        certificate_durably_persisted: false,
        independent_derivation_verified: false,
    });
    let supply = p.terminal.work_limit - p.terminal.reserved_work;
    {
        let _ticket = p.terminal.reserve(supply, 0)?;
    }
    let mut local = Ledger::new(supply, 0, p.original_deadline);
    let terminal_performed_before = p.terminal.performed_work;
    let result = (|| {
        // Charged fixed-view reads and scalar snapshot setup. Arrays remain
        // borrowed, their valid lengths were updated at completed operations.
        {
            let mut ticket = local.reserve(4096, 0)?;
            ticket.spend(4096)?;
        }
        let state = &p.step4;
        let cert = &state.certificate;
        let g = &p.workspace.group;
        let first_certificate = cert.identity.as_ref().map(|id| FirstRecord {
            lifecycle: cert.lifecycle,
            identity: IdentityView {
                origin: id.origin,
                search_phase: id.phase,
                source_cursor: id.cursor,
                tuple_indices: &id.indices[..p.m],
                exceptional_auxiliary_positions: ExceptionView {
                    mask: id.exception_mask,
                    m: p.m,
                },
                candidate_coordinates_match: id.coordinates_match,
            },
            facts: cert.facts,
            replay: cert.progress,
            publication: PublicationView {
                state: if cert.receipt.is_some() {
                    "acknowledged"
                } else if matches!(
                    state.publication_error,
                    Some(WitnessError::IndeterminateIo(_))
                ) {
                    "indeterminate"
                } else if cert.facts.publication_attempted {
                    "refused"
                } else {
                    "not_attempted"
                },
                receipt: cert.receipt.map(receipt_view),
            },
            witness_owner: if state.transferred {
                "final_artifact"
            } else {
                "slot6"
            },
            witness_bytes: if state.transferred {
                Some(state.final_artifacts[6].bytes.len() as u64)
            } else if !p.slots.buffers[6].bytes.is_empty() {
                Some(p.slots.buffers[6].bytes.len() as u64)
            } else {
                None
            },
            witness_sha256: cert.receipt.map(|r| HexDigest(r.payload_sha256)),
        });
        let last = state.last;
        let last_state = last.initialized.then(|| LastState {
            phase: if last.origin == Some(GroupOrigin::IndependentGroupPrefix) {
                SearchPhase::IndependentPrefix
            } else {
                SearchPhase::NativeLifts
            },
            origin: last.origin.unwrap_or(report.origin),
            cursor: last.cursor,
            tuple_indices: &g.tuple[..p.m],
            progress: if last.complete {
                "complete_tuple"
            } else if report.tuple_entered {
                "partial_tuple"
            } else {
                "before_tuple"
            },
            tuple_entered: report.tuple_entered,
            completed_prefixes: report.completed_prefixes as u8,
            prefix_points: PointSlice(&g.prefix[..report.completed_prefixes]),
            exceptional_auxiliary_positions: &g.exceptional[..last.exceptions],
            coordinate_count: last.coordinate_count as u8,
            coordinates: &g.coordinates[..last.coordinate_count],
            residue_count: last.residue_count as u8,
            residues: &g.residues[..last.residue_count],
            computed_match: report.computed_match,
            row: last.row,
            metadata_acknowledged: last.metadata_acknowledged,
        });
        let source_slots = std::array::from_fn(|i| {
            let transferred = i == 6 && state.transferred;
            let bytes = if transferred {
                &state.final_artifacts[6].bytes
            } else {
                &p.slots.buffers[i].bytes
            };
            let complete = !bytes.is_empty();
            let receipt = if i == 6 { cert.receipt } else { None };
            SlotEvidence {
                index: i as u8,
                state: if transferred {
                    "transferred"
                } else if complete {
                    "local_complete"
                } else {
                    "not_produced"
                },
                bytes: complete.then_some(bytes.len() as u64),
                digest_binding: if receipt.is_some() {
                    "capture_receipt"
                } else {
                    "outer_manifest"
                },
                sha256: receipt.map(|r| HexDigest(r.payload_sha256)),
            }
        });
        let settled = &phase.settlement;
        let observer = &phase.observer_settlement;
        let record = DynamicResult {
            schema: "restricted-wide-prime-dynamic-result/v1",
            strategy: "prime_f4_wide_dynamic",
            stage: if state.final_staging_stop.is_some() {
                "final_staging"
            } else if !entered_group {
                if state.linear_stop.is_some() {
                    "linear"
                } else {
                    "native"
                }
            } else if cert.progress.failed_stage == Some(ReplayStage::Publication) {
                "publication"
            } else if cert.progress.failed_stage.is_some() {
                "certificate_replay"
            } else {
                "group"
            },
            disposition: if state.failed || state.final_staging_stop.is_some() {
                "failed"
            } else if report.group_domain_complete {
                "complete"
            } else {
                "censored"
            },
            cause: if let Some(stop) = state.final_staging_stop {
                if matches!(
                    state.publication_error,
                    Some(WitnessError::IndeterminateIo(_))
                ) {
                    "indeterminate_io"
                } else {
                    group_cause(stop)
                }
            } else if !entered_group {
                if state.linear_stop.is_some() {
                    "linear_stop"
                } else {
                    "native_stop"
                }
            } else if matches!(
                state.publication_error,
                Some(WitnessError::IndeterminateIo(_))
            ) {
                "indeterminate_io"
            } else {
                group_cause(report.stop)
            },
            native_stop: Some(stop_name(phase.report.stop)),
            linear_stop: state.linear_stop,
            group_stop: entered_group.then_some(report.stop),
            search_phase: if !entered_group {
                "not_started"
            } else if report.group_domain_complete {
                "finished"
            } else if report.origin == GroupOrigin::IndependentGroupPrefix {
                "independent_prefix"
            } else {
                "native_lifts"
            },
            accounting: Accounting {
                counter_snapshot_stage: "before_terminal_retention",
                pools: [
                    PoolView {
                        name: "algebra",
                        parent: "root",
                        work_limit: p.ledger.work_limit,
                        reserved_work: p.ledger.reserved_work,
                        performed_work: Some(p.ledger.performed_work),
                    },
                    PoolView {
                        name: "checking",
                        parent: "root",
                        work_limit: p.checking_work + p.checking_reserved_work,
                        reserved_work: p.checking_reserved_work,
                        performed_work: Some(p.checking_performed_work),
                    },
                    PoolView {
                        name: "native",
                        parent: "algebra",
                        work_limit: settled.supplied.work,
                        reserved_work: settled.consumed.work,
                        performed_work: None,
                    },
                    PoolView {
                        name: "observer",
                        parent: "algebra",
                        work_limit: observer.supplied.work,
                        reserved_work: observer.consumed.work,
                        performed_work: Some(state.observer_performed),
                    },
                    PoolView {
                        name: "capture",
                        parent: "checking",
                        work_limit: state.budget.reserved_work,
                        reserved_work: state.budget.reserved_work,
                        performed_work: Some(state.budget.performed_work),
                    },
                    PoolView {
                        name: "terminal",
                        parent: "algebra",
                        work_limit: p.terminal.work_limit,
                        reserved_work: p.terminal.reserved_work,
                        performed_work: Some(terminal_performed_before),
                    },
                ],
                requested_bytes: ByteAccounting {
                    limit: p.ledger.bytes_limit,
                    reserved: p.ledger.reserved_bytes,
                },
                group: GroupAccounting {
                    addition_limit: p.group_control.addition_limit,
                    additions_reserved: p.group_control.additions_reserved,
                    additions_performed: p.group_control.additions_performed,
                    tuple_limit: p.group_control.tuple_limit,
                    tuples_reserved: p.group_control.tuples,
                },
                traversal: TraversalAccounting {
                    native_lifts_visited: report.native_lifts_visited,
                    independent_visited: report.independent_visited,
                    independent_total: report.independent_total,
                },
                events: EventAccounting {
                    limit: 4096,
                    global_acknowledged: p.observer.acknowledged as u64,
                    setup: p.observer.setup as u64,
                    native: p.observer.native as u64,
                    linear: p.observer.linear as u64,
                    group: p.observer.group as u64,
                    new_payload: state.new_payload as u64,
                    attempted: p.observer.attempted as u64,
                    pending: u64::from(p.observer.pending.is_some()),
                },
            },
            source_slots,
            last_state,
            first_certificate,
            claims: Claims {
                solver_completed: report.group_domain_complete && !state.failed,
                group_domain_complete: report.group_domain_complete,
                negative_verified: report.group_domain_complete && !cert.facts.computed,
                pdp_witness_verified: cert.facts.local_replay_complete,
                certificate_durably_persisted: cert.facts.durable_acknowledged,
                independent_derivation_verified: false,
                operation_trajectory_verified: false,
            },
        };
        // Borrow views from disjoint fields; slot7 is never a source reference.
        let mut output = InputBuffer {
            bytes: std::mem::take(&mut p.slots.buffers[7].bytes),
            logical_cap: 65536,
        };
        let result = encode_complete(&record, &mut output, &mut local);
        p.slots.buffers[7].bytes = output.bytes;
        result
    })();
    p.terminal.performed_work += local.performed_work;
    p.ledger.performed_work += local.performed_work;
    p.step4.terminal_retained = result.is_ok();
    result
}
/// Consumes already admitted owners; does not publish or imply registration.
/// Unacknowledged slot6 is deliberately omitted from the promoted witness role.
pub(super) fn take_step4_artifacts(p: &mut PreparedOriginal) -> Result<Vec<ArtifactPayload>, Stop> {
    if !p.step4.prepared || p.step4.final_artifacts.len() != 8 {
        return Err(Stop::InvalidInput);
    }
    for i in 0..8 {
        if i != 6 {
            p.step4.final_artifacts[i].bytes = std::mem::take(&mut p.slots.buffers[i].bytes);
        }
    }
    let mut out = std::mem::take(&mut p.step4.final_artifacts);
    out.retain(|a| !a.bytes.is_empty());
    Ok(out)
}

/// Idempotent final staging uses the same byte owner and persistent budget.
/// Call before terminal sealing and before moving final owners to the parent.
pub(super) fn confirm_step4_witness<P: WitnessPublisher>(
    p: &mut PreparedOriginal,
    publisher: &mut P,
) -> Result<(), GroupStop> {
    if !p.step4.certificate.facts.durable_acknowledged {
        return Ok(());
    }
    if !p.step4.transferred || p.step4.final_artifacts.len() != 8 {
        return Err(GroupStop::InvalidState);
    }
    let mut checking = Ledger::new(p.checking_work, 0, p.original_deadline);
    let result = (|| {
        p.step4
            .budget
            .set_exclusive_supply(
                checking.work_limit,
                p.ledger.bytes_limit - p.ledger.reserved_bytes,
            )
            .map_err(witness_error)?;
        let before = (
            p.step4.budget.reserved_work,
            p.step4.budget.performed_work,
            p.step4.budget.reserved_bytes,
        );
        let global_before = p.observer.acknowledged;
        let receipt = publisher.publish(
            &p.step4.final_artifacts[6].bytes,
            &mut p.step4.budget,
            &mut p.observer.acknowledged,
        );
        settle_witness(p, &mut checking, before)?;
        let receipt = receipt.map_err(|e| {
            p.step4.publication_error = Some(e);
            witness_error(e)
        })?;
        let first = p.step4.certificate.receipt.ok_or(GroupStop::InvalidState)?;
        if receipt.commit != WitnessCommit::AlreadyCommitted
            || p.observer.acknowledged != global_before
            || receipt.payload_sha256 != first.payload_sha256
            || receipt.payload_bytes != first.payload_bytes
            || receipt.sequence != first.sequence
            || receipt.provenance != first.provenance
        {
            return Err(GroupStop::PersistenceFailure);
        }
        Ok(())
    })();
    p.checking_work -= checking.reserved_work;
    p.checking_reserved_work += checking.reserved_work;
    p.checking_performed_work += checking.performed_work;
    if result.is_err() {
        p.step4.failed = true;
        p.step4.final_staging_stop = result.as_ref().err().copied();
    }
    result
}

/// Read-only early-failure custody fallback. No new owner/metadata allocation,
/// no fabricated terminal/group execution and no promoted unpublished witness.
/// Callback storage/hashing remains the existing outer custody boundary.
fn visit_slots<F: FnMut(&'static str, &'static str, &'static str, &[u8]) -> Result<(), ()>>(
    slots: &Slots,
    mut forward: F,
) -> Result<(), ()> {
    for (i, &(path, role, encoding)) in SLOT_REGISTRY.iter().enumerate() {
        if i == 6 || slots.buffers[i].bytes.is_empty() {
            continue;
        }
        forward(path, role, encoding, &slots.buffers[i].bytes)?;
    }
    Ok(())
}
pub(super) fn visit_prepare_failure_artifacts<
    F: FnMut(&'static str, &'static str, &'static str, &[u8]) -> Result<(), ()>,
>(
    failure: &PrepareFailure,
    forward: F,
) -> Result<(), ()> {
    if let Some(slots) = &failure.retained_slots {
        visit_slots(slots, forward)
    } else {
        Ok(())
    }
}
pub(super) fn visit_step4_retained_artifacts<
    F: FnMut(&'static str, &'static str, &'static str, &[u8]) -> Result<(), ()>,
>(
    p: &PreparedOriginal,
    mut forward: F,
) -> Result<(), ()> {
    visit_slots(&p.slots, &mut forward)?;
    if p.step4.certificate.facts.durable_acknowledged {
        let bytes = if p.step4.transferred {
            &p.step4.final_artifacts.get(6).ok_or(())?.bytes
        } else {
            &p.slots.buffers[6].bytes
        };
        if bytes.is_empty() {
            return Err(());
        }
        let (path, role, encoding) = SLOT_REGISTRY[6];
        forward(path, role, encoding, bytes)?;
    }
    Ok(())
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
    fn phase_limits() -> AttemptLimits {
        AttemptLimits {
            work_limit: 16384,
            memory_bytes: 256 * 1024 * 1024,
            trace_bytes: 128 * 1024 * 1024,
            degree_bound: 12,
            original_deadline: Instant::now() + Duration::from_secs(30),
        }
    }
    fn prepared_small() -> PreparedOriginal {
        prepare_original(
            101,
            Parameters {
                m: 4,
                a: 1,
                b: 2,
                target_x: 1,
                codes: &[1],
            },
            AffinePoint { x: 1, y: 2 },
            &[AffinePoint { x: 1, y: 2 }, AffinePoint { x: 1, y: 99 }],
            phase_limits(),
            EventPersistence::LocalOnly,
            &mut |_, _| panic!("local-only must not claim or invoke durable capture"),
        )
        .unwrap()
    }
    #[test]
    fn native_original_chain_runs_and_settles_once_without_promoting_derivation() {
        let prepared = prepared_small();
        let before_work = prepared.ledger.reserved_work;
        let before_bytes = prepared.ledger.reserved_bytes;
        let phase = invoke_original(prepared, &mut |_, _| panic!("local-only")).unwrap();
        assert!(phase.report.native_backend_invoked);
        assert_eq!(
            phase.report.acknowledged_events,
            phase.prepared.observer.native
        );
        assert_eq!(phase.prepared.observer.durable, 0);
        assert!(!phase.independent_derivation_verified);
        assert!(
            phase.prepared.ledger.reserved_work
                >= before_work
                    + phase.settlement.consumed.work
                    + phase.observer_settlement.consumed.work
        );
        assert_eq!(
            phase.prepared.ledger.reserved_bytes,
            before_bytes + phase.settlement.consumed.bytes
        );
        assert!(phase.prepared.workspace.linear_left.capacity() >= 8192);
        assert!(phase.prepared.workspace.linear_right.capacity() >= 8192);
        assert!(phase.prepared.slots.buffers[6].bytes().is_empty());
        assert!(phase.prepared.slots.buffers[6].logical_cap > 0);
        assert_eq!(phase.prepared.checking_work, 16_777_216 / 4);
        assert!(phase.prepared.ledger.performed_work <= phase.prepared.ledger.reserved_work);
    }
    #[test]
    fn escrow_settlement_survives_deadline_and_rejects_double_or_overclaimed_receipts() {
        let mut ledger = Ledger::new(100, 100, Instant::now());
        let supply = Supply {
            work: 50,
            bytes: 50,
        };
        let mut escrow = Escrow::new(supply);
        settle(
            &mut ledger,
            &mut escrow,
            Supply {
                work: 20,
                bytes: 30,
            },
            10,
        )
        .unwrap();
        assert_eq!((ledger.reserved_work, ledger.reserved_bytes), (20, 30));
        assert_eq!(
            settle(
                &mut ledger,
                &mut escrow,
                Supply {
                    work: 20,
                    bytes: 30
                },
                10
            )
            .unwrap_err(),
            Stop::InvalidInput
        );
        let mut bad = Escrow::new(supply);
        assert_eq!(
            settle(&mut ledger, &mut bad, Supply { work: 51, bytes: 0 }, 0).unwrap_err(),
            Stop::InvalidInput
        );
        assert_eq!((ledger.reserved_work, ledger.reserved_bytes), (20, 30));
        let mut second = Escrow::new(remaining(&ledger));
        settle(
            &mut ledger,
            &mut second,
            Supply {
                work: 80,
                bytes: 70,
            },
            0,
        )
        .unwrap();
        assert_eq!((ledger.reserved_work, ledger.reserved_bytes), (100, 100));
    }
    #[test]
    fn native_deadline_settles_before_original_deadline_metadata_fallback() {
        let mut prepared = prepared_small();
        prepared.phase_deadline = Instant::now();
        prepared.ledger.deadline = prepared.phase_deadline;
        let phase = invoke_original(prepared, &mut |_, _| Ok(())).unwrap();
        assert_eq!(phase.report.stop, dynamic::Stop::Deadline);
        assert!(!phase.report.native_backend_invoked);
        assert!(!phase.basis_retained);
        assert_eq!(phase.retention_stop, Some(Stop::Deadline));
        assert_eq!(phase.diagnostic_stop, None);
        let value: serde_json::Value =
            serde_json::from_slice(phase.prepared.slots.buffers[2].bytes()).unwrap();
        assert_eq!(value["metadata"]["stop"], "deadline");
        assert_eq!(value["metadata"]["basis_retained"], false);
        assert_eq!(value["metadata"]["independent_derivation_verified"], false);
        assert!(value["basis"].is_null());
    }
    #[test]
    fn native_preparation_storage_or_input_refusal_has_no_native_result() {
        let mut limits = phase_limits();
        limits.memory_bytes = 1024;
        let result = prepare_original(
            101,
            Parameters {
                m: 4,
                a: 1,
                b: 2,
                target_x: 1,
                codes: &[1],
            },
            AffinePoint { x: 1, y: 2 },
            &[AffinePoint { x: 1, y: 2 }],
            limits,
            EventPersistence::LocalOnly,
            &mut |_, _| Ok(()),
        );
        assert!(matches!(
            result,
            Err(PrepareFailure {
                stop: Stop::StorageLimit,
                ..
            })
        ));
        let result = prepare_original(
            101,
            Parameters {
                m: 4,
                a: 1,
                b: 2,
                target_x: 1,
                codes: &[1],
            },
            AffinePoint { x: 2, y: 2 },
            &[AffinePoint { x: 1, y: 2 }],
            phase_limits(),
            EventPersistence::LocalOnly,
            &mut |_, _| Ok(()),
        );
        let failure = match result {
            Err(f) => f,
            Ok(_) => panic!("mismatched target admitted"),
        };
        assert_eq!(failure.stop, Stop::InvalidInput);
        assert!(failure.ledger.reserved_work >= 131072);
        assert!(!failure.retained_slots.unwrap().buffers[3]
            .bytes()
            .is_empty());
    }
    #[test]
    fn observer_acknowledgement_survives_post_capture_expiry_and_refusal_rolls_back() {
        let mut outer = ledger();
        let mut slot = InputBuffer::new(4096, &mut outer).unwrap();
        let mut capture = TypedEventBuffer::try_new(CAPTURE_CAP, CAPTURE_CAP + 128).unwrap();
        let mut state = ObserverState {
            acknowledged: 0,
            attempted: 0,
            setup: 0,
            native: 0,
            linear: 0,
            group: 0,
            durable: 0,
            pending: None,
            stop: None,
            last_native: Supply { work: 0, bytes: 0 },
            terminal_acknowledged: false,
            persistence: EventPersistence::BoundCapture,
        };
        let event = PhaseEvent {
            schema: "crypto.ml-dataset.wide-prime-dynamic-event/v1",
            source: PhaseSource::Native,
            global_sequence: 0,
            local_sequence: 0,
            stage: "begin",
            reserved_work: 16,
            reserved_bytes: 256,
            persistence: EventPersistence::BoundCapture,
        };
        let deadline = Instant::now() + Duration::from_millis(10);
        let mut budget = Ledger::new(1_000_000, 0, deadline);
        append_observation(
            event,
            &mut state,
            &mut slot,
            &mut budget,
            &mut capture,
            &mut |_, _| {
                while Instant::now() < deadline {
                    std::hint::spin_loop();
                }
                Ok(())
            },
        )
        .unwrap();
        assert_eq!((state.acknowledged, state.durable), (1, 1));
        assert_eq!(budget.check(), Err(Stop::Deadline));
        let kept = slot.bytes().len();
        let mut budget = Ledger::new(1_000_000, 0, Instant::now() + Duration::from_secs(1));
        let next = PhaseEvent {
            global_sequence: 1,
            local_sequence: 1,
            ..event
        };
        assert!(append_observation(
            next,
            &mut state,
            &mut slot,
            &mut budget,
            &mut capture,
            &mut |_, _| Err(())
        )
        .is_err());
        assert_eq!(slot.bytes().len(), kept);
        assert_eq!((state.acknowledged, state.durable), (1, 1));
        assert_eq!(state.stop, Some(Stop::Observer));
        assert!(state.pending.is_some());
    }
    #[test]
    fn shared_observer_count_includes_setup_and_never_resets_for_native() {
        let mut prepared = prepared_small();
        assert_eq!(prepared.observer.setup, 2);
        prepared.observer.acknowledged = 4096;
        let phase = invoke_original(prepared, &mut |_, _| Ok(())).unwrap();
        assert_eq!(phase.report.stop, dynamic::Stop::EventLimit);
        assert_eq!(phase.report.acknowledged_events, 0);
        assert_eq!(phase.prepared.observer.acknowledged, 4096);
    }
    #[test]
    fn final_native_receipt_rejects_snapshot_regression_and_terminal_disagreement() {
        let mut phase = invoke_original(prepared_small(), &mut |_, _| Ok(())).unwrap();
        validate_final_snapshot(&phase.report, &phase.prepared.observer).unwrap();
        let original_work = phase.report.reserved.work;
        let original_bytes = phase.report.reserved.bytes;
        assert!(original_work > 0 && original_bytes > 0);
        // Deliberately modified receipt/state controls, not alleged engine bugs.
        phase.prepared.observer.last_native = Supply {
            work: original_work,
            bytes: original_bytes,
        };
        phase.prepared.observer.terminal_acknowledged = true;
        validate_final_snapshot(&phase.report, &phase.prepared.observer).unwrap();
        phase.report.reserved.work -= 1;
        assert_eq!(
            validate_final_snapshot(&phase.report, &phase.prepared.observer),
            Err(Stop::InvalidInput)
        );
        phase.report.reserved.work = original_work;
        phase.report.reserved.bytes += 1;
        assert_eq!(
            validate_final_snapshot(&phase.report, &phase.prepared.observer),
            Err(Stop::InvalidInput)
        );
        // No-terminal deadline/observer censoring may consume more than its
        // last acknowledged snapshot; it must not be forced to exact equality.
        phase.prepared.observer.terminal_acknowledged = false;
        phase.report.stop = dynamic::Stop::Deadline;
        validate_final_snapshot(&phase.report, &phase.prepared.observer).unwrap();
        phase.report.reserved.bytes = original_bytes - 1;
        assert_eq!(
            validate_final_snapshot(&phase.report, &phase.prepared.observer),
            Err(Stop::InvalidInput)
        );
        phase.report.reserved.bytes = original_bytes;
        phase.report.acknowledged_events += 1;
        assert_eq!(
            validate_final_snapshot(&phase.report, &phase.prepared.observer),
            Err(Stop::InvalidInput)
        );
    }
    /// Explicit long panel: all432 cases are attempted; ignored/unrun is NOT coverage.
    #[test]
    #[ignore = "explicit original-chain432-case software gate; invoke separately with --ignored --exact --nocapture"]
    #[allow(clippy::result_large_err)] // Exercise the allocation-free partial-state API unchanged.
    fn dynamic_original_chain_all_arity_twist_432_case_panel() {
        use num_traits::ToPrimitive;
        let source = std::env::var("ML_DYNAMIC_PANEL_SOURCE_COMMIT")
            .expect("explicit panel requires harness-bound ML_DYNAMIC_PANEL_SOURCE_COMMIT");
        assert!(
            source.len() == 40
                && source
                    .bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        );
        // Metadata-only executable identity; the parent additionally retains
        // immutable executable/source receipts and guards the file hash.
        let executable = std::env::current_exe().unwrap();
        let binary = super::super::super::digest(&std::fs::read(&executable).unwrap());
        let identity = |n: u32, twist: bool, m: usize, k: usize, target_kind: usize| {
            serde_json::json!({
                "schema":"crypto.ml-dataset.dynamic-native-software-panel-case/v1",
                "source_commit":source,"binary_sha256":binary,
                "seed":42,"curve_seed":7,"generator_index":1,"key_index":1,
                "n":n,"twist":twist,"m":m,"signed_domain_size":2*k,"distinct_x_count":k,
                "target_class":match target_kind{0=>"positive",1=>"negative",_=>"exceptional"},
                "domain_kind":"fixed_custom_software_control_not_cli_default",
                "original_allowances":{"work_limit":16384,"memory_bytes":256*1024*1024u64,"trace_bytes":128*1024*1024u64,"degree_bound":12,"process_seconds":30,"phase_seconds":5},
                "input_sha256":serde_json::Value::Null,"native_supplied_limits":serde_json::Value::Null,
                "scientific_dataset_cell":false,"independent_derivation_verified":false
            })
        };
        let emit = |row: &serde_json::Value| {
            eprintln!(
                "DYNAMIC_NATIVE_PANEL {}",
                serde_json::to_string(row).unwrap()
            )
        };
        let slots = |slots: &Slots| {
            serde_json::json!(slots
                .buffers
                .iter()
                .enumerate()
                .map(|(i, b)| serde_json::json!({
                    "slot_index":i,"logical_cap":b.logical_cap,"bytes":b.bytes().len(),
                    "sha256":super::super::super::digest(b.bytes()),
                    "raw_utf8":std::str::from_utf8(b.bytes()).unwrap()
                }))
                .collect::<Vec<_>>())
        };
        let mut failures = Vec::new();
        let mut attempted = 0;
        for n in [47, 53, 61] {
            for twist in [false, true] {
                for m in 4..=15 {
                    let cfg = super::super::CaseConfig {
                        family: "prime".into(),
                        n,
                        m,
                        twist,
                        curve_seed: 7,
                        generator_index: 1,
                        key_index: 1,
                        seed: 42,
                        factor_base_size: 8,
                        degree_bound: 12,
                        work_limit: 16384,
                        solver: "f4".into(),
                    };
                    let context = (|| {
                        let mut case = super::super::curves::generate(&cfg)?;
                        case.private_witness = serde_json::Value::Null;
                        let group = super::super::Group::read(&case)?;
                        let generator = group.point(&case.public_instance["generator"])?;
                        Ok::<_, String>((group, generator))
                    })();
                    let (group, generator) = match context {
                        Ok(context) => context,
                        Err(error) => {
                            for k in [1, 2] {
                                for target_kind in 0..3 {
                                    attempted += 1;
                                    let mut row = identity(n, twist, m, k, target_kind);
                                    row["status"] = serde_json::json!("fixed_context_failure");
                                    row["reason"] = serde_json::json!(error);
                                    emit(&row);
                                    failures.push(format!("context n{n} twist{twist} m{m} K{k} target{target_kind}: {error}"));
                                }
                            }
                            continue;
                        }
                    };
                    let super::super::Group::Prime(curve) = &group else {
                        panic!("prime recipe produced a different family")
                    };
                    let affine = |point: &super::super::Element| match point {
                        super::super::Element::Prime(crate::ecc::Point::Affine { x, y }) => {
                            Some(AffinePoint {
                                x: x.value.to_u64().unwrap(),
                                y: y.value.to_u64().unwrap(),
                            })
                        }
                        _ => None,
                    };
                    for k in [1usize, 2] {
                        for target_kind in 0..3 {
                            attempted += 1;
                            let mut row = identity(n, twist, m, k, target_kind);
                            let bound = 2 * k * m + 1;
                            let preconditions = (1..=bound)
                                .all(|i| group.mul(&generator, i as u64) != group.zero());
                            let p2 = group.mul(&generator, 2);
                            let mut signed = vec![generator.clone(), group.neg(&generator)];
                            if k == 2 {
                                signed.push(p2.clone());
                                signed.push(group.neg(&p2));
                            }
                            let signed: Option<Vec<_>> = signed.iter().map(&affine).collect();
                            let Some(signed) = signed else {
                                row["status"] = serde_json::json!("precondition_failure");
                                row["reason"] = serde_json::json!("nonaffine signed domain");
                                emit(&row);
                                failures.push(format!(
                                    "domain n{n} twist{twist} m{m} K{k} target{target_kind}"
                                ));
                                continue;
                            };
                            let mut codes: Vec<_> = signed.iter().map(|p| p.x).collect();
                            codes.sort_unstable();
                            codes.dedup();
                            if !preconditions || codes.len() != k {
                                row["status"] = serde_json::json!("precondition_failure");
                                row["reason"]=serde_json::json!("fixed group-order/domain cardinality precondition failed; no resampling");
                                emit(&row);
                                failures.push(format!(
                                    "preconditions n{n} twist{twist} m{m} K{k} target{target_kind}"
                                ));
                                continue;
                            }
                            let multiplier = match target_kind {
                                0 => k * m,
                                1 => k * m + 1,
                                _ => k * (m - 2),
                            };
                            let Some(target) = affine(&group.mul(&generator, multiplier as u64))
                            else {
                                row["status"] = serde_json::json!("precondition_failure");
                                row["reason"] = serde_json::json!("nonaffine fixed target");
                                emit(&row);
                                failures.push(format!(
                                    "target n{n} twist{twist} m{m} K{k} target{target_kind}"
                                ));
                                continue;
                            };
                            let params = Parameters {
                                m,
                                a: curve.a.to_u64().unwrap(),
                                b: curve.b.to_u64().unwrap(),
                                target_x: target.x,
                                codes: &codes,
                            };
                            row["curve"] = serde_json::json!({"p":curve.p.to_string(),"a":curve.a.to_string(),"b":curve.b.to_string(),"target_x":target.x.to_string(),"target_y":target.y.to_string()});
                            let prepared = prepare_original(
                                curve.p.to_u64().unwrap(),
                                params,
                                target,
                                &signed,
                                phase_limits(),
                                EventPersistence::LocalOnly,
                                &mut |_, _| Ok(()),
                            );
                            let phase =
                                prepared.and_then(|p| invoke_original(p, &mut |_, _| Ok(())));
                            match phase {
                                Err(error) => {
                                    row["status"] = serde_json::json!("phase_failure");
                                    row["stop"] = serde_json::json!(error.stop);
                                    row["reserved_work"] =
                                        serde_json::json!(error.ledger.reserved_work);
                                    row["reserved_bytes"] =
                                        serde_json::json!(error.ledger.reserved_bytes);
                                    row["setup_failure"] = serde_json::json!(error
                                        .setup_failure
                                        .map(|f| format!("{f:?}")));
                                    if let Some(retained) = &error.retained_slots {
                                        if !retained.buffers[0].bytes().is_empty() {
                                            row["input_sha256"] =
                                                serde_json::json!(super::super::super::digest(
                                                    retained.buffers[0].bytes()
                                                ));
                                        }
                                        row["retained_slots"] = slots(retained);
                                    }
                                    emit(&row);
                                    failures.push(format!("phase n{n} twist{twist} m{m} K{k} target{target_kind}: {:?}",error.stop));
                                }
                                Ok(phase) => {
                                    let useful = phase.report.matrix_rows_reduced > 0
                                        && phase.report.pivot_updates > 0;
                                    let limits = dynamic::Limits {
                                        work: phase.settlement.supplied.work,
                                        bytes: phase.settlement.supplied.bytes,
                                        events: 4096 - phase.prepared.observer.setup,
                                        max_pair_degree: 12,
                                        ..dynamic::Limits::default()
                                    };
                                    row["status"] = serde_json::json!("native_returned");
                                    row["native_supplied_limits"] =
                                        serde_json::to_value(NativeLimitRecord::from(&limits))
                                            .unwrap();
                                    row["input_sha256"] =
                                        serde_json::json!(super::super::super::digest(
                                            phase.prepared.slots.buffers[0].bytes()
                                        ));
                                    row["native"] = serde_json::to_value(NativeMetadata::new(
                                        m,
                                        &phase.report,
                                        &limits,
                                        phase.basis_retained,
                                        phase.retention_stop,
                                    ))
                                    .unwrap();
                                    row["useful_native_work"] = serde_json::json!(useful);
                                    row["diagnostic_stop"] =
                                        serde_json::json!(phase.diagnostic_stop);
                                    row["native_settlement"] =
                                        serde_json::to_value(phase.settlement).unwrap();
                                    row["observer_settlement"] =
                                        serde_json::to_value(phase.observer_settlement).unwrap();
                                    row["reserved_work"] =
                                        serde_json::json!(phase.prepared.ledger.reserved_work);
                                    row["reserved_bytes"] =
                                        serde_json::json!(phase.prepared.ledger.reserved_bytes);
                                    row["retained_slots"] = slots(&phase.prepared.slots);
                                    emit(&row);
                                    if !phase.report.native_backend_invoked || !useful {
                                        failures.push(format!("useful native gate n{n} twist{twist} m{m} K{k} target{target_kind}"));
                                    }
                                    if phase.independent_derivation_verified {
                                        failures.push("forbidden derivation promotion".into());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        eprintln!(
            "DYNAMIC_NATIVE_PANEL_SUMMARY {}",
            serde_json::json!({"source_commit":source,"binary_sha256":binary,"attempted":attempted,"required":432,"failures":failures})
        );
        assert_eq!(attempted, 432);
        assert!(
            failures.is_empty(),
            "all432 fixed cases retained; {} failed gates",
            failures.len()
        );
    }
    fn linear_test_phase() -> NativePhase {
        // P=(6,18) on y²=x³+7 over F101: 2P=(98,9), 3P=(99,91),
        // 4P=(84,12). The original equations are genuine, not x_i-c_i inputs.
        let prepared = prepare_original(
            101,
            Parameters {
                m: 4,
                a: 0,
                b: 7,
                target_x: 84,
                codes: &[6],
            },
            AffinePoint { x: 84, y: 12 },
            &[AffinePoint { x: 6, y: 18 }, AffinePoint { x: 6, y: 83 }],
            phase_limits(),
            EventPersistence::LocalOnly,
            &mut |_, _| Ok(()),
        )
        .unwrap();
        let mut phase = invoke_original(prepared, &mut |_, _| Ok(())).unwrap();
        // The following tests inject reported bases deliberately. They prove
        // checked candidate semantics, never that native derived these bases.
        phase.report.stop = dynamic::Stop::WorkLimit;
        phase.report.native_statistics_complete = false;
        phase.basis_retained = true;
        phase.report.basis = linear_test_basis();
        phase
    }
    fn linear_test_poly(terms: &[([u16; 6], u64)]) -> InputPolynomial {
        let mut terms: Vec<_> = terms
            .iter()
            .map(|(e, c)| CanonicalTerm {
                mono: Mono28::new(e, 6).unwrap(),
                coefficient: *c,
            })
            .collect();
        terms.sort_by(|a, b| cmp(b, a, 6));
        InputPolynomial { terms }
    }
    fn linear_test_basis() -> Vec<InputPolynomial> {
        [6u64, 6, 6, 6, 98, 99]
            .into_iter()
            .enumerate()
            .map(|(i, value)| {
                let mut powers = [0u16; 6];
                powers[i] = 1;
                linear_test_poly(&[(powers, 3), ([0; 6], neg(mul(3, value, 101), 101))])
            })
            .collect()
    }
    #[test]
    fn linear_full_rank_nonmonic_partial_report_checks_original_chain_without_group_claim() {
        let mut phase = linear_test_phase();
        let report = extract_reported_linear(&mut phase);
        assert_eq!(report.stop, LinearStop::ReportedBasisCandidate);
        assert_eq!(report.assignments, 6);
        assert_eq!(report.completed_passes, 7);
        assert_eq!(report.original_equations_checked, 7);
        assert!(report.candidate_verified_original);
        assert!(!report.independent_derivation_verified && !report.pdp_witness_verified);
        assert!(!phase.report.native_statistics_complete);
        assert_eq!(
            &phase.prepared.workspace.group.coordinates[..6],
            &[6, 6, 6, 6, 98, 99]
        );
        assert!(phase.prepared.workspace.group.residues[..7]
            .iter()
            .all(|&r| r == 0));
        for (i, transition) in phase.prepared.workspace.group.transitions[..6]
            .iter()
            .enumerate()
        {
            let t = transition.unwrap();
            assert_eq!(t.variable, i);
            assert_eq!(t.basis_id, i);
            assert_eq!(t.a, 3);
        }
    }
    #[test]
    fn linear_substitution_merges_cancellation_and_uses_lowest_basis_pivot() {
        let mut phase = linear_test_phase();
        // x0²-6*x0 cancels after x0=6, while x0*x1-3*x1-18 becomes 3*x1-18.
        phase.report.basis.insert(
            0,
            linear_test_poly(&[([2, 0, 0, 0, 0, 0], 1), ([1, 0, 0, 0, 0, 0], 95)]),
        );
        phase.report.basis[2] = linear_test_poly(&[
            ([1, 1, 0, 0, 0, 0], 1),
            ([0, 1, 0, 0, 0, 0], 98),
            ([0; 6], 83),
        ]);
        phase
            .report
            .basis
            .push(linear_test_poly(&[([1, 0, 0, 0, 0, 0], 1), ([0; 6], 95)]));
        let report = extract_reported_linear(&mut phase);
        assert_eq!(report.stop, LinearStop::ReportedBasisCandidate);
        assert_eq!(phase.prepared.workspace.group.sources[0], 1);
        assert_eq!(phase.prepared.workspace.group.sources[1], 2);
    }
    #[test]
    fn linear_contradiction_domain_missing_auxiliary_and_nonlinear_are_distinct() {
        let mut contradiction = linear_test_phase();
        contradiction
            .report
            .basis
            .push(linear_test_poly(&[([0; 6], 1)]));
        let report = extract_reported_linear(&mut contradiction);
        assert_eq!(report.stop, LinearStop::Contradiction);
        assert_eq!(report.assignments, 0);
        let mut outside = linear_test_phase();
        outside.report.basis[0] = linear_test_poly(&[([1, 0, 0, 0, 0, 0], 1), ([0; 6], 94)]);
        assert_eq!(
            extract_reported_linear(&mut outside).stop,
            LinearStop::OutsideDomain
        );
        let mut missing = linear_test_phase();
        missing.report.basis.pop();
        let report = extract_reported_linear(&mut missing);
        assert_eq!(report.stop, LinearStop::NonlinearOrUnderdetermined);
        assert_eq!(report.assignments, 5);
        assert!(!report.candidate_verified_original);
        let mut nonlinear = linear_test_phase();
        nonlinear.report.basis[0] = linear_test_poly(&[([2, 0, 0, 0, 0, 0], 1), ([0; 6], 65)]);
        let report = extract_reported_linear(&mut nonlinear);
        assert_eq!(report.stop, LinearStop::NonlinearOrUnderdetermined);
        assert!(!report.candidate_verified_original);
    }
    #[test]
    fn linear_reported_assignment_must_satisfy_originals_and_retained_basis_gate() {
        let mut phase = linear_test_phase();
        phase.report.basis[5] = linear_test_poly(&[([0, 0, 0, 0, 0, 1], 1), ([0; 6], 1)]);
        let report = extract_reported_linear(&mut phase);
        assert_eq!(report.stop, LinearStop::OriginalEquationMismatch);
        assert_eq!(report.original_equations_checked, 7);
        assert!(!report.candidate_verified_original);
        let mut absent = linear_test_phase();
        absent.basis_retained = false;
        assert_eq!(
            extract_reported_linear(&mut absent).stop,
            LinearStop::NoRetainedBasis
        );
        let mut unit = linear_test_phase();
        unit.report.stop = dynamic::Stop::UnitDerived;
        assert_eq!(
            extract_reported_linear(&mut unit).stop,
            LinearStop::NativeUnit
        );
    }
    #[test]
    fn linear_uses_all28_u16_exponents_without_truncation_and_term_limit() {
        let setup = field(101);
        let mut budget = ledger();
        budget.bytes_limit = 64 * 1024 * 1024;
        let mut workspace = Workspace::new(&[], &mut budget).unwrap();
        let assignments = std::array::from_fn(|i| Some(if i == 27 { 2 } else { 1 }));
        let source = InputPolynomial {
            terms: vec![CanonicalTerm {
                mono: Mono28::new(&[u16::MAX; 28], 28).unwrap(),
                coefficient: 7,
            }],
        };
        linear_remainder(
            &source,
            28,
            &setup.field,
            &assignments,
            &mut workspace.linear_left,
            &mut workspace.linear_right,
            &mut budget,
        )
        .unwrap();
        let mut expected = 7u64;
        for _ in 0..u16::MAX {
            expected = mul(expected, 2, 101);
        }
        assert_eq!(workspace.linear_left.len(), 1);
        assert_eq!(workspace.linear_left[0].coefficient, expected);
        assert_eq!(workspace.linear_left[0].mono.degree(), 0);
        let too_large = InputPolynomial {
            terms: vec![source.terms[0]; 8193],
        };
        assert_eq!(
            linear_remainder(
                &too_large,
                28,
                &setup.field,
                &assignments,
                &mut workspace.linear_left,
                &mut workspace.linear_right,
                &mut budget
            ),
            Err(LinearStop::TermLimit)
        );
    }
    #[test]
    fn linear_length_preflight_finds_late_oversize_before_processing() {
        let mut phase = linear_test_phase();
        let term = phase.report.basis[0].terms[0];
        phase.report.basis.push(InputPolynomial {
            terms: vec![term; 8193],
        });
        let scan = 32 + 16 * phase.report.basis.len() as u64;
        let before = phase.prepared.ledger.reserved_work;
        phase.prepared.ledger.work_limit = before + scan;
        let report = extract_reported_linear(&mut phase);
        assert_eq!(report.stop, LinearStop::TermLimit);
        assert_eq!(report.assignments, 0);
        assert_eq!(phase.prepared.ledger.reserved_work, before + scan);
        let mut phase = linear_test_phase();
        phase.report.basis.push(InputPolynomial {
            terms: vec![term; 8193],
        });
        phase.prepared.ledger.work_limit = phase.prepared.ledger.reserved_work + scan - 1;
        assert_eq!(
            extract_reported_linear(&mut phase).stop,
            LinearStop::WorkLimit
        );
    }
    #[test]
    fn linear_exact_and_one_less_algebra_and_checking_budget_admission() {
        let mut measured = linear_test_phase();
        let before = measured.prepared.ledger.reserved_work;
        assert_eq!(
            extract_reported_linear(&mut measured).stop,
            LinearStop::ReportedBasisCandidate
        );
        let algebra = measured.prepared.ledger.reserved_work - before;
        let checking = measured.prepared.checking_reserved_work;
        for (extra, check, expected) in [
            (algebra, checking, LinearStop::ReportedBasisCandidate),
            (algebra - 1, checking, LinearStop::WorkLimit),
            (algebra, checking - 1, LinearStop::WorkLimit),
        ] {
            let mut phase = linear_test_phase();
            phase.prepared.ledger.work_limit = phase.prepared.ledger.reserved_work + extra;
            phase.prepared.checking_work = check;
            let report = extract_reported_linear(&mut phase);
            assert_eq!(report.stop, expected);
            assert!(phase.prepared.checking_reserved_work <= check);
            assert!(phase.prepared.ledger.reserved_work <= phase.prepared.ledger.work_limit);
            if expected != LinearStop::ReportedBasisCandidate {
                assert!(!report.candidate_verified_original);
            }
        }
    }
    #[test]
    fn linear_expired_phase_and_partial_pass_do_not_commit_unfinished_assignment() {
        let mut phase = linear_test_phase();
        phase.prepared.ledger.deadline = Instant::now();
        let report = extract_reported_linear(&mut phase);
        assert_eq!(report.stop, LinearStop::Deadline);
        assert_eq!(report.assignments, 0);
        let mut phase = linear_test_phase();
        phase.prepared.ledger.work_limit = phase.prepared.ledger.reserved_work + 4096;
        let report = extract_reported_linear(&mut phase);
        assert_eq!(report.stop, LinearStop::WorkLimit);
        assert_eq!(report.assignments, 0);
        assert!(phase
            .prepared
            .workspace
            .group
            .assignments
            .iter()
            .all(Option::is_none));
    }
    fn group_test_phase(target: AffinePoint) -> NativePhase {
        let prepared = prepare_original(
            101,
            Parameters {
                m: 4,
                a: 0,
                b: 7,
                target_x: target.x,
                codes: &[6],
            },
            target,
            &[AffinePoint { x: 6, y: 18 }, AffinePoint { x: 6, y: 83 }],
            phase_limits(),
            EventPersistence::LocalOnly,
            &mut |_, _| Ok(()),
        )
        .unwrap();
        invoke_original(prepared, &mut |_, _| Ok(())).unwrap()
    }
    #[test]
    fn group_first_match_stops_pending_without_replay_or_publication() {
        let mut phase = group_test_phase(AffinePoint { x: 84, y: 12 });
        let native = phase.prepared.observer.native;
        let report = run_group_prefixes(&mut phase, None, &mut |_, _| panic!("local only"));
        assert_eq!(report.stop, GroupStop::PendingCertificate);
        assert_eq!(report.origin, GroupOrigin::IndependentGroupPrefix);
        assert_eq!(report.tuples_reserved, 1);
        assert_eq!(report.additions_reserved, 3);
        assert_eq!(report.additions_performed, 3);
        assert_eq!(report.independent_visited, 1);
        assert_eq!(report.cursor, 0);
        assert!(report.computed_match);
        assert!(
            !report.pdp_witness_verified
                && !report.certificate_durably_persisted
                && !report.group_domain_complete
        );
        assert!(phase.prepared.slots.buffers[6].bytes.is_empty());
        assert_eq!(phase.prepared.observer.native, native);
        assert_eq!(phase.prepared.observer.group, 1);
        assert_eq!(
            run_group_prefixes(&mut phase, None, &mut |_, _| Ok(())).stop,
            GroupStop::AlreadyStarted
        );
        let row: serde_json::Value =
            serde_json::from_slice(&phase.prepared.slots.buffers[5].bytes).unwrap();
        assert_eq!(row["indices"], serde_json::json!([0, 0, 0, 0]));
        assert_eq!(row["coordinates"], serde_json::json!([6, 6, 6, 6, 98, 99]));
        assert_eq!(row["residues"], serde_json::json!([0, 0, 0, 0, 0, 0, 0]));
    }
    #[test]
    fn group_native_lift_requires_retained_linear_and_preserves_origin() {
        // Injected reported basis isolates lift semantics, not native derivation.
        let mut phase = linear_test_phase();
        let linear = extract_reported_linear(&mut phase);
        retain_linear(&mut phase, &linear).unwrap();
        let raw: serde_json::Value =
            serde_json::from_slice(&phase.prepared.slots.buffers[4].bytes).unwrap();
        assert_eq!(raw["assignments"], serde_json::json!([6, 6, 6, 6, 98, 99]));
        assert_eq!(
            raw["unresolved"],
            serde_json::json!([null, null, null, null, null, null])
        );
        let report = run_group_prefixes(&mut phase, Some(&linear), &mut |_, _| Ok(()));
        assert_eq!(report.stop, GroupStop::PendingCertificate);
        assert_eq!(report.origin, GroupOrigin::ReportedBasisLinearCandidate);
        assert_eq!(report.native_lifts_visited, 1);
        assert_eq!(report.independent_visited, 0);
        assert_eq!(report.tuples_reserved, 1);
        assert!(!report.independent_derivation_verified);
    }
    #[test]
    fn group_complete_small_negative_keeps_durable_completeness_false() {
        // The singleton signed domain contains its own negative, a point of order2.
        let target = AffinePoint { x: 3, y: 38 };
        let prepared = prepare_original(
            101,
            Parameters {
                m: 4,
                a: 1,
                b: 0,
                target_x: 3,
                codes: &[0],
            },
            target,
            &[AffinePoint { x: 0, y: 0 }],
            phase_limits(),
            EventPersistence::LocalOnly,
            &mut |_, _| Ok(()),
        )
        .unwrap();
        let mut phase = invoke_original(prepared, &mut |_, _| Ok(())).unwrap();
        let report = run_group_prefixes(&mut phase, None, &mut |_, _| Ok(()));
        assert_eq!(report.stop, GroupStop::IndependentTraversalFinished);
        assert_eq!(report.independent_total, 1);
        assert_eq!(report.independent_visited, 1);
        assert!(
            !report.computed_match && !report.group_domain_complete && !report.pdp_witness_verified
        );
        let row: serde_json::Value =
            serde_json::from_slice(&phase.prepared.slots.buffers[5].bytes).unwrap();
        assert_eq!(
            row["exceptional_auxiliary_positions"],
            serde_json::json!([0])
        );
        assert!(row["coordinates"].is_null() && row["residues"].is_null());
        assert_eq!(row["prefixes"][1]["kind"], "affine");
        assert_eq!(row["total"]["kind"], "infinity");
    }
    #[test]
    fn group_ordered_prefix_preserves_exceptions_and_nonmatching_residues() {
        let mut phase = group_test_phase(AffinePoint { x: 63, y: 33 }); //5P, outside signed4P range.
        let report = run_group_prefixes(&mut phase, None, &mut |_, _| Ok(()));
        assert!(matches!(
            report.stop,
            GroupStop::CheckWorkLimit | GroupStop::IndependentTraversalFinished
        ));
        let rows: Vec<serde_json::Value> = phase.prepared.slots.buffers[5]
            .bytes
            .split(|&b| b == b'\n')
            .filter(|b| !b.is_empty())
            .map(|b| serde_json::from_slice(b).unwrap())
            .collect();
        assert!(
            rows.len() >= 5,
            "fixed budget must retain exceptional cursor4"
        );
        assert_eq!(rows[1]["indices"], serde_json::json!([0, 0, 0, 1]));
        assert_eq!(rows[4]["indices"], serde_json::json!([0, 1, 0, 0]));
        assert_eq!(
            rows[4]["exceptional_auxiliary_positions"],
            serde_json::json!([0])
        );
        assert_eq!(rows[4]["total"]["kind"], "affine");
        assert!(rows[4]["coordinates"].is_null());
        assert!(rows[0]["residues"]
            .as_array()
            .unwrap()
            .iter()
            .any(|r| r.as_u64() != Some(0)));
    }
    #[test]
    fn group_add_differential_true_curve_and_twist_no_wrapped_point_in_producer() {
        use crate::ecc::{FieldElement, Point};
        use num_bigint::BigUint;
        use num_traits::ToPrimitive;
        let setup = field(101);
        let convert = |q: SmallPoint| match q {
            SmallPoint::Infinity => Point::Infinity,
            SmallPoint::Affine(q) => Point::Affine {
                x: FieldElement::new(BigUint::from(q.x), BigUint::from(101u64)),
                y: FieldElement::new(BigUint::from(q.y), BigUint::from(101u64)),
            },
        };
        let back = |q: Point| match q {
            Point::Infinity => SmallPoint::Infinity,
            Point::Affine { x, y } => SmallPoint::Affine(AffinePoint {
                x: x.value.to_u64().unwrap(),
                y: y.value.to_u64().unwrap(),
            }),
        };
        // 2 is a nonsquare in F101; b56=7*2^3 is the actual quadratic twist.
        for b in [7, 56] {
            let mut points = vec![SmallPoint::Infinity];
            for x in 0..101 {
                for y in 0..101 {
                    if mul(y, y, 101) == add(mul(mul(x, x, 101), x, 101), b, 101) {
                        points.push(SmallPoint::Affine(AffinePoint { x, y }));
                        if points.len() == 9 {
                            break;
                        }
                    }
                }
                if points.len() == 9 {
                    break;
                }
            }
            let a = FieldElement::new(BigUint::from(0u64), BigUint::from(101u64));
            for &left in &points {
                for &right in &points {
                    let mut l = ledger();
                    let got = group_add(&setup.field, 0, left, right, &mut l).unwrap();
                    assert_eq!(got, back(convert(left).add(&convert(right), &a)));
                    assert_eq!(l.reserved_work, 4096);
                }
            }
        }
        let zero = SmallPoint::Affine(AffinePoint { x: 0, y: 0 });
        assert_eq!(
            group_add(&setup.field, 1, zero, zero, &mut ledger()).unwrap(),
            SmallPoint::Infinity
        );
    }
    #[test]
    fn group_row_bound_counting_is_paid_and_exact_one_less_is_rejected() {
        let mut phase = group_test_phase(AffinePoint { x: 84, y: 12 });
        let mut l = ledger();
        let cap = phase.prepared.slots.buffers[5].logical_cap;
        let bound = group_row_bound(4, &mut phase.prepared.workspace.group, cap, &mut l).unwrap();
        let work = l.reserved_work;
        assert!(work > bound as u64 * 8);
        for (w, ok) in [(work, true), (work - 1, false)] {
            let mut budget = Ledger::new(w, 0, Instant::now() + Duration::from_secs(30));
            let result = group_row_bound(4, &mut phase.prepared.workspace.group, cap, &mut budget);
            assert_eq!(result.is_ok(), ok);
            if !ok {
                assert_eq!(result, Err(GroupStop::CheckWorkLimit));
            }
        }
        let mut previous = 0;
        for m in 4..=15 {
            let b = group_row_bound(m, &mut phase.prepared.workspace.group, cap, &mut ledger())
                .unwrap();
            assert!(b > previous);
            previous = b;
        }
    }
    #[test]
    fn group_exact_one_less_tuple_work_additions_and_bytes_are_admitted_before_entry() {
        let mut baseline = group_test_phase(AffinePoint { x: 84, y: 12 });
        let report = run_group_prefixes(&mut baseline, None, &mut |_, _| Ok(()));
        assert_eq!(report.stop, GroupStop::PendingCertificate);
        let work = report.checking_reserved_work;
        for (w, expected) in [
            (work, GroupStop::PendingCertificate),
            (work - 1, GroupStop::CheckWorkLimit),
        ] {
            let mut phase = group_test_phase(AffinePoint { x: 84, y: 12 });
            phase.prepared.checking_work = w;
            let report = run_group_prefixes(&mut phase, None, &mut |_, _| Ok(()));
            assert_eq!(report.stop, expected);
            if expected != GroupStop::PendingCertificate {
                assert_eq!(report.tuples_reserved, 0);
            }
        }
        for (g, expected) in [
            (3, GroupStop::PendingCertificate),
            (2, GroupStop::AdditionLimit),
        ] {
            let mut phase = group_test_phase(AffinePoint { x: 84, y: 12 });
            phase.prepared.group_control.addition_limit = g;
            assert_eq!(
                run_group_prefixes(&mut phase, None, &mut |_, _| Ok(())).stop,
                expected
            );
        }
        let mut phase = group_test_phase(AffinePoint { x: 84, y: 12 });
        let cap = phase.prepared.slots.buffers[5].logical_cap;
        phase.prepared.slots.buffers[5]
            .bytes
            .resize(cap - report.row_bound + 1, b' ');
        let failed = run_group_prefixes(&mut phase, None, &mut |_, _| Ok(()));
        assert_eq!(failed.stop, GroupStop::GroupBytes);
        assert_eq!(failed.tuples_reserved, 0);
        let mut phase = group_test_phase(AffinePoint { x: 84, y: 12 });
        phase.prepared.slots.buffers[5].logical_cap = report.row_bound;
        assert_eq!(
            run_group_prefixes(&mut phase, None, &mut |_, _| Ok(())).stop,
            GroupStop::PendingCertificate
        );
    }
    #[test]
    fn group_shared_event_limit_deadline_and_callback_refusal_preserve_pending_match() {
        let mut phase = group_test_phase(AffinePoint { x: 84, y: 12 });
        phase.prepared.observer.acknowledged = 4096;
        let report = run_group_prefixes(&mut phase, None, &mut |_, _| Ok(()));
        assert_eq!(report.stop, GroupStop::EventLimit);
        assert_eq!(report.tuples_reserved, 0);
        let mut phase = group_test_phase(AffinePoint { x: 84, y: 12 });
        phase.prepared.original_deadline = Instant::now();
        let report = run_group_prefixes(&mut phase, None, &mut |_, _| Ok(()));
        assert_eq!(report.stop, GroupStop::Deadline);
        assert_eq!(report.additions_performed, 0);
        let mut phase = group_test_phase(AffinePoint { x: 84, y: 12 });
        phase.prepared.observer.persistence = EventPersistence::BoundCapture;
        let before = phase.prepared.observer.acknowledged;
        let report = run_group_prefixes(&mut phase, None, &mut |_, _| Err(()));
        assert_eq!(report.stop, GroupStop::PersistenceFailure);
        assert!(report.computed_match && !report.certificate_durably_persisted);
        assert_eq!(phase.prepared.observer.acknowledged, before);
        assert!(!phase.prepared.slots.buffers[5].bytes.is_empty());
    }
    #[test]
    fn group_near61_actual_curve_and_quadratic_twist_match_independent_point_oracle() {
        use crate::ecc::{FieldElement, Point};
        use num_bigint::BigUint;
        use num_traits::ToPrimitive;
        let p = 2_305_843_009_213_693_951u64;
        let bp = BigUint::from(p);
        let a = p - 1;
        let setup = field(p);
        let sqrt_exponent = BigUint::from((p + 1) / 4);
        assert_eq!(
            BigUint::from(p - 1).modpow(&BigUint::from((p - 1) / 2), &bp),
            BigUint::from(p - 1)
        );
        // d=-1 is nonsquare, hence twist a'=d²a=a and b'=d³b=-b.
        // Fixed x0..15 software fixture; no adaptive resampling if it lacks points.
        for b in [1, p - 1] {
            let mut points = vec![SmallPoint::Infinity];
            for x in 0u64..16 {
                let bx = BigUint::from(x);
                let rhs = ((&bx * &bx * &bx) + BigUint::from(a) * &bx + BigUint::from(b)) % &bp;
                let y = rhs.modpow(&sqrt_exponent, &bp);
                if (&y * &y) % &bp == rhs {
                    let y = y.to_u64().unwrap();
                    points.push(SmallPoint::Affine(AffinePoint { x, y }));
                    if y != 0 {
                        points.push(SmallPoint::Affine(AffinePoint { x, y: p - y }));
                    }
                }
            }
            assert!(
                points.len() >= 3,
                "fixed actual curve/twist must contain affine controls"
            );
            let coefficient = FieldElement::new(BigUint::from(a), bp.clone());
            let convert = |q: SmallPoint| match q {
                SmallPoint::Infinity => Point::Infinity,
                SmallPoint::Affine(q) => Point::Affine {
                    x: FieldElement::new(BigUint::from(q.x), bp.clone()),
                    y: FieldElement::new(BigUint::from(q.y), bp.clone()),
                },
            };
            for &left in &points {
                for &right in &points {
                    let reference = convert(left).add(&convert(right), &coefficient);
                    let expected = match reference {
                        Point::Infinity => SmallPoint::Infinity,
                        Point::Affine { x, y } => SmallPoint::Affine(AffinePoint {
                            x: x.value.to_u64().unwrap(),
                            y: y.value.to_u64().unwrap(),
                        }),
                    };
                    let mut l = ledger();
                    let actual = group_add(&setup.field, a, left, right, &mut l).unwrap();
                    assert_eq!(
                        canonical_add(p, a, left, right, &mut ledger()).unwrap(),
                        actual
                    );
                    assert_eq!(
                        actual, expected,
                        "p={p} a={a} b={b} left={left:?} right={right:?}"
                    );
                    assert_eq!(l.reserved_work, 4096);
                    assert!(l.performed_work <= 4096);
                }
            }
        }
    }
    struct GroupHookReset;
    impl Drop for GroupHookReset {
        fn drop(&mut self) {
            GROUP_ADDITION_HOOK.with(|h| h.set(None));
            GROUP_CARRY_HOOK.with(|h| h.set(None));
        }
    }
    #[test]
    fn group_controlled_midprefix_expiry_preserves_prior_row_and_reserved_performed_counts() {
        let target = AffinePoint { x: 63, y: 33 };
        let mut baseline = group_test_phase(target);
        baseline.prepared.group_control.tuple_limit = 1;
        assert_eq!(
            run_group_prefixes(&mut baseline, None, &mut |_, _| Ok(())).stop,
            GroupStop::TupleLimit
        );
        fn expire(tuple: u64, leaf: usize, l: &mut Ledger) {
            if tuple == 2 && leaf == 2 {
                l.deadline = Instant::now();
            }
        }
        let mut phase = group_test_phase(target);
        let _reset = GroupHookReset;
        GROUP_ADDITION_HOOK.with(|h| h.set(Some(expire)));
        let report = run_group_prefixes(&mut phase, None, &mut |_, _| Ok(()));
        assert_eq!(report.stop, GroupStop::Deadline);
        assert_eq!(report.cursor, 1);
        assert!(report.tuple_entered);
        assert_eq!(report.independent_visited, 1);
        assert_eq!(report.completed_prefixes, 1);
        assert_eq!(report.tuples_reserved, 2);
        assert_eq!(report.additions_reserved, 6);
        assert_eq!(report.additions_performed, 4);
        assert!(report.checking_performed_work < report.checking_reserved_work);
        assert_eq!(&phase.prepared.workspace.group.tuple[..4], &[0, 0, 0, 1]);
        assert_eq!(
            phase.prepared.workspace.group.prefix[0],
            SmallPoint::Affine(AffinePoint { x: 98, y: 9 })
        );
        assert_eq!(
            phase.prepared.slots.buffers[5].bytes,
            baseline.prepared.slots.buffers[5].bytes
        );
        assert_eq!(phase.prepared.observer.group, 1);
        assert!(!report.computed_match && !report.pdp_witness_verified);
    }
    #[test]
    fn group_midcarry_work_refusal_is_transactional_for_tuple_prefix_and_jsonl() {
        let target = AffinePoint { x: 63, y: 33 };
        let mut baseline = group_test_phase(target);
        baseline.prepared.group_control.tuple_limit = 2;
        assert_eq!(
            run_group_prefixes(&mut baseline, None, &mut |_, _| Ok(())).stop,
            GroupStop::TupleLimit
        );
        fn refuse(tuple: u64, l: &mut Ledger) {
            if tuple == 2 {
                l.work_limit = l.reserved_work + 32 * 4 + 64;
            }
        }
        let mut phase = group_test_phase(target);
        let _reset = GroupHookReset;
        GROUP_CARRY_HOOK.with(|h| h.set(Some(refuse)));
        let report = run_group_prefixes(&mut phase, None, &mut |_, _| Ok(()));
        assert_eq!(report.stop, GroupStop::CheckWorkLimit);
        assert_eq!(report.cursor, 1);
        assert_eq!(report.completed_prefixes, 3);
        assert_eq!(report.independent_visited, 2);
        assert_eq!(report.additions_reserved, 6);
        assert_eq!(report.additions_performed, 6);
        assert_eq!(&phase.prepared.workspace.group.tuple[..4], &[0, 0, 0, 1]);
        assert_eq!(
            phase.prepared.workspace.group.prefix[2],
            SmallPoint::Affine(AffinePoint { x: 98, y: 9 })
        );
        assert_eq!(
            phase.prepared.slots.buffers[5].bytes,
            baseline.prepared.slots.buffers[5].bytes
        );
        // Failure partway through a start/reset also leaves the original tuple intact.
        let before = phase.prepared.workspace.group.tuple;
        let mut l = Ledger::new(32 * 2, 0, Instant::now() + Duration::from_secs(30));
        assert_eq!(
            start_cursor(
                &mut phase.prepared.workspace.group,
                4,
                2,
                GroupOrigin::IndependentGroupPrefix,
                &mut l
            ),
            Err(GroupStop::CheckWorkLimit)
        );
        assert_eq!(phase.prepared.workspace.group.tuple, before);
    }
    #[test]
    fn group_preserves_earliest_leaf_lift_match_with_different_auxiliary_assignment() {
        // Valid reported affine root from (++-+), but earlier (+++-) has same
        // target and different final auxiliary. It remains earliest group witness.
        let mut phase = group_test_phase(AffinePoint { x: 98, y: 9 });
        phase.report.stop = dynamic::Stop::WorkLimit;
        phase.report.basis = linear_test_basis();
        phase.basis_retained = true;
        phase.report.basis[5] = linear_test_poly(&[([0, 0, 0, 0, 0, 1], 1), ([0; 6], 95)]);
        let linear = extract_reported_linear(&mut phase);
        assert!(linear.candidate_verified_original);
        retain_linear(&mut phase, &linear).unwrap();
        let report = run_group_prefixes(&mut phase, Some(&linear), &mut |_, _| Ok(()));
        assert_eq!(report.stop, GroupStop::PendingCertificate);
        assert_eq!(report.origin, GroupOrigin::ReportedBasisLeafLift);
        assert_eq!(report.cursor, 1);
        assert_eq!(report.native_lifts_visited, 2);
        assert_eq!(report.independent_visited, 0);
        assert_eq!(
            &phase.prepared.workspace.group.first_certificate[..4],
            &[0, 0, 0, 1]
        );
        let rows: Vec<serde_json::Value> = phase.prepared.slots.buffers[5]
            .bytes
            .split(|&b| b == b'\n')
            .filter(|b| !b.is_empty())
            .map(|b| serde_json::from_slice(b).unwrap())
            .collect();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[1]["origin"], "reported_basis_leaf_lift");
        assert_eq!(rows[1]["computed_match"], true);
        assert_eq!(rows[1]["candidate_coordinates_match"], false);
        assert_eq!(rows[1]["coordinates"][5], 99);
        let retained: serde_json::Value =
            serde_json::from_slice(&phase.prepared.slots.buffers[4].bytes).unwrap();
        assert_eq!(retained["assignments"][5], 6);
        assert!(
            !report.pdp_witness_verified
                && !report.certificate_durably_persisted
                && !report.independent_derivation_verified
        );
    }
    #[test]
    fn group_row_exact_golden_bounds_all_arities_origins_options_and_boolean_widths() {
        // Independent compact-JSON schema byte counts, including newline.
        const GOLDEN64: [usize; 12] = [
            1194, 1382, 1570, 1758, 1946, 2134, 2322, 2510, 2698, 2886, 3074, 3262,
        ];
        const GOLDEN32: [usize; 12] = [
            1124, 1292, 1460, 1628, 1796, 1964, 2132, 2300, 2468, 2636, 2804, 2972,
        ];
        let mut phase = group_test_phase(AffinePoint { x: 84, y: 12 });
        let cap = phase.prepared.slots.buffers[5].logical_cap;
        for m in 4..=15 {
            let bound = group_row_bound(m, &mut phase.prepared.workspace.group, cap, &mut ledger())
                .unwrap();
            assert_eq!(
                bound,
                if usize::BITS == 64 {
                    GOLDEN64[m - 4]
                } else {
                    GOLDEN32[m - 4]
                }
            );
            let g = &mut phase.prepared.workspace.group;
            let mut maximum = 0;
            for origin in [
                GroupOrigin::ReportedBasisLinearCandidate,
                GroupOrigin::ReportedBasisLeafLift,
                GroupOrigin::IndependentGroupPrefix,
            ] {
                for flags in 0..128 {
                    let point = if flags & 64 == 0 {
                        SmallPoint::Affine(AffinePoint {
                            x: 0x1fff_ffff_ffff_ffff,
                            y: 0x1fff_ffff_ffff_ffff,
                        })
                    } else {
                        SmallPoint::Infinity
                    };
                    g.prefix.fill(point);
                    let row = GroupRow {
                        schema: "crypto.ml-dataset.wide-prime-dynamic-group-row/v1",
                        origin,
                        cursor: u128::MAX,
                        indices: &g.tuple[..m],
                        prefixes: &g.prefix[..m - 1],
                        completed_additions: usize::MAX,
                        exceptional_auxiliary_positions: &g.exceptional[..m - 2],
                        coordinates: (flags & 8 != 0).then_some(&g.coordinates[..2 * m - 2]),
                        residues: (flags & 16 != 0).then_some(&g.residues[..2 * m - 1]),
                        total: (flags & 32 != 0).then_some(&point),
                        computed_match: flags & 1 != 0,
                        candidate_coordinates_match: flags & 2 != 0,
                        complete: flags & 4 != 0,
                        tuples_reserved: u64::MAX,
                        additions_reserved: u64::MAX,
                        additions_performed: u64::MAX,
                        checking_reserved_work: u64::MAX,
                    };
                    let bytes = serde_json::to_vec(&row).unwrap();
                    maximum = maximum.max(bytes.len() + 1);
                    assert!(bytes.len() < bound, "m={m} origin={origin:?} flags={flags}");
                }
            }
            assert_eq!(
                maximum, bound,
                "the bound must be attained by one serializer width case"
            );
        }
    }
    #[test]
    fn group_u128_wire_fields_roundtrip_as_decimal_strings_without_numeric_value_conversion() {
        #[derive(serde::Deserialize)]
        struct ReportWire {
            cursor: String,
            independent_visited: String,
            native_lifts_visited: String,
            independent_total: String,
        }
        #[derive(serde::Deserialize)]
        struct RowWire {
            cursor: String,
        }
        let mut phase = group_test_phase(AffinePoint { x: 84, y: 12 });
        let mut report = run_group_prefixes(&mut phase, None, &mut |_, _| Ok(()));
        report.cursor = u128::MAX;
        report.independent_visited = u128::from(u64::MAX) + 1;
        report.native_lifts_visited = 0;
        report.independent_total = 1u128 << 90;
        let bytes = serde_json::to_vec(&report).unwrap();
        let parsed: ReportWire = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(parsed.cursor, "340282366920938463463374607431768211455");
        assert_eq!(parsed.independent_visited, "18446744073709551616");
        assert_eq!(parsed.native_lifts_visited, "0");
        assert_eq!(parsed.independent_total, "1237940039285380274899124224");
        assert_eq!(parsed.cursor.parse::<u128>().unwrap(), report.cursor);
        assert_eq!(
            parsed.independent_visited.parse::<u128>().unwrap(),
            report.independent_visited
        );
        assert_eq!(
            parsed.independent_total.parse::<u128>().unwrap(),
            report.independent_total
        );
        let g = &phase.prepared.workspace.group;
        let row = GroupRow {
            schema: "crypto.ml-dataset.wide-prime-dynamic-group-row/v1",
            origin: GroupOrigin::IndependentGroupPrefix,
            cursor: u128::MAX,
            indices: &g.tuple[..4],
            prefixes: &g.prefix[..3],
            completed_additions: 3,
            exceptional_auxiliary_positions: &[],
            coordinates: None,
            residues: None,
            total: None,
            computed_match: false,
            candidate_coordinates_match: false,
            complete: true,
            tuples_reserved: 1,
            additions_reserved: 3,
            additions_performed: 3,
            checking_reserved_work: 0,
        };
        let parsed: RowWire = serde_json::from_slice(&serde_json::to_vec(&row).unwrap()).unwrap();
        assert_eq!(parsed.cursor, "340282366920938463463374607431768211455");
        assert_eq!(parsed.cursor.parse::<u128>().unwrap(), u128::MAX);
    }
    #[derive(Default)]
    struct FakeWitness {
        bytes: Vec<u8>,
        calls: usize,
        refuse: bool,
    }
    impl WitnessPublisher for FakeWitness {
        fn confirmation_work(&self, _: usize) -> Result<u64, WitnessError> {
            Ok(0)
        }

        fn prepare(&mut self, _: &mut WitnessBudget) -> Result<(), WitnessError> {
            Ok(())
        }
        fn publish(
            &mut self,
            bytes: &[u8],
            _: &mut WitnessBudget,
            global: &mut usize,
        ) -> Result<WitnessReceipt, WitnessError> {
            self.calls += 1;
            if self.refuse {
                return Err(WitnessError::Failed);
            }
            let new = self.bytes.is_empty();
            if new {
                self.bytes.extend_from_slice(bytes);
                *global += 1;
            } else if self.bytes != bytes {
                return Err(WitnessError::Integrity);
            }
            Ok(WitnessReceipt {
                commit: if new {
                    WitnessCommit::NewCommit
                } else {
                    WitnessCommit::AlreadyCommitted
                },
                payload_sha256: crate::hash::sha256(bytes),
                payload_bytes: bytes.len() as u64,
                sequence: Some(7),
                provenance: crate::ml_dataset::capture::WitnessProvenance::Dedicated,
            })
        }
    }
    fn step4_fixture(target: AffinePoint, publisher: &mut FakeWitness) -> NativePhase {
        let mut p = prepare_original(
            101,
            Parameters {
                m: 4,
                a: 0,
                b: 7,
                target_x: target.x,
                codes: &[6],
            },
            target,
            &[AffinePoint { x: 6, y: 18 }, AffinePoint { x: 6, y: 83 }],
            phase_limits(),
            EventPersistence::LocalOnly,
            &mut |_, _| Ok(()),
        )
        .unwrap();
        prepare_step4(&mut p, publisher).unwrap();
        invoke_original(p, &mut |_, _| Ok(())).unwrap()
    }
    #[test]
    fn step4_fresh_canonical_group_arithmetic_and_exact_admission() {
        for (prime, a, b, points) in [
            (
                101,
                0,
                7,
                vec![
                    AffinePoint { x: 6, y: 18 },
                    AffinePoint { x: 6, y: 83 },
                    AffinePoint { x: 98, y: 9 },
                ],
            ),
            (
                101,
                0,
                1,
                vec![
                    AffinePoint { x: 0, y: 1 },
                    AffinePoint { x: 0, y: 100 },
                    AffinePoint { x: 2, y: 3 },
                ],
            ),
        ] {
            let f = field(prime);
            for &q in &points {
                for &r in &points {
                    let mut checked = ledger();
                    replay_on_curve(prime, a, b, q, &mut checked).unwrap();
                    let expected = group_add(
                        &f.field,
                        a,
                        SmallPoint::Affine(q),
                        SmallPoint::Affine(r),
                        &mut ledger(),
                    )
                    .unwrap();
                    let mut exact = Ledger::new(4096, 0, Instant::now() + Duration::from_secs(10));
                    assert_eq!(
                        canonical_add(
                            prime,
                            a,
                            SmallPoint::Affine(q),
                            SmallPoint::Affine(r),
                            &mut exact
                        ),
                        Ok(expected)
                    );
                    assert_eq!(exact.reserved_work, 4096);
                    let mut short = Ledger::new(4095, 0, Instant::now() + Duration::from_secs(10));
                    assert_eq!(
                        canonical_add(
                            prime,
                            a,
                            SmallPoint::Affine(q),
                            SmallPoint::Affine(r),
                            &mut short
                        ),
                        Err(GroupStop::CheckWorkLimit)
                    );
                    assert_eq!(short.reserved_work, 0);
                }
            }
        }
    }
    #[test]
    fn step4_first_witness_bytes_survive_later_matches_and_ninth_owner() {
        let mut publisher = FakeWitness::default();
        let mut phase = step4_fixture(AffinePoint { x: 98, y: 9 }, &mut publisher);
        phase.prepared.group_control.tuple_limit = 5;
        let native = phase.prepared.observer.native;
        let report = run_group_step4(&mut phase, None, &mut |_, _| Ok(()), &mut publisher);
        assert_eq!(report.stop, GroupStop::TupleLimit);
        assert!(report.pdp_witness_verified && report.certificate_durably_persisted);
        assert_eq!(publisher.calls, 1);
        assert_eq!(
            &phase.prepared.step4.certificate.identity.unwrap().indices[..4],
            &[0, 0, 0, 1]
        );
        assert_eq!(
            phase.prepared.step4.final_artifacts[6].bytes,
            publisher.bytes
        );
        assert!(phase.prepared.slots.buffers[6].bytes.is_empty());
        let global = phase.prepared.observer.acknowledged;
        confirm_step4_witness(&mut phase.prepared, &mut publisher).unwrap();
        assert_eq!(publisher.calls, 2);
        assert_eq!(phase.prepared.observer.acknowledged, global);
        assert_eq!(phase.prepared.observer.native, native);
        retain_step4_terminal(&mut phase).unwrap();
        let record: serde_json::Value =
            serde_json::from_slice(&phase.prepared.slots.buffers[7].bytes).unwrap();
        assert_eq!(record["source_slots"].as_array().unwrap().len(), 7);
        assert_eq!(record["source_slots"][6]["state"], "transferred");
        assert_eq!(
            record["source_slots"][0]["digest_binding"],
            "outer_manifest"
        );
        assert!(record["source_slots"][0]["sha256"].is_null());
        assert_eq!(
            record["first_certificate"]["facts"]["final_artifact_registered"],
            false
        );
        let pointer = phase.prepared.step4.final_artifacts.as_ptr();
        let capacity = phase.prepared.step4.final_artifacts.capacity();
        let mut artifacts = take_step4_artifacts(&mut phase.prepared).unwrap();
        assert_eq!(artifacts.as_ptr(), pointer);
        assert!(capacity >= 9);
        artifacts.push(ArtifactPayload {
            path: "solver-domain.json".into(),
            role: "solver-domain".into(),
            encoding: "json".into(),
            bytes: b"{}".to_vec(),
        });
        assert_eq!(artifacts.as_ptr(), pointer);
        assert_eq!(artifacts.capacity(), capacity);
    }
    #[test]
    fn step4_publication_refusal_preserves_local_math_and_stops() {
        let mut publisher = FakeWitness {
            refuse: true,
            ..FakeWitness::default()
        };
        let mut phase = step4_fixture(AffinePoint { x: 84, y: 12 }, &mut publisher);
        let report = run_group_step4(&mut phase, None, &mut |_, _| Ok(()), &mut publisher);
        assert_eq!(report.stop, GroupStop::PersistenceFailure);
        assert_eq!(report.tuples_reserved, 1);
        let facts = phase.prepared.step4.certificate.facts;
        assert!(
            facts.computed
                && facts.local_replay_complete
                && facts.publication_attempted
                && !facts.durable_acknowledged
        );
        assert!(!phase.prepared.step4.transferred);
        assert_eq!(publisher.calls, 1);
        let artifacts = take_step4_artifacts(&mut phase.prepared).unwrap();
        assert!(!artifacts.iter().any(|a| a.path == "pdp-witness.json"));
    }
    #[test]
    fn step4_replay_rejects_tampered_prefix_and_preserves_reservations() {
        let mut phase = group_test_phase(AffinePoint { x: 84, y: 12 });
        let report = run_group_prefixes(&mut phase, None, &mut |_, _| Ok(()));
        assert!(report.computed_match);
        let id = phase.prepared.step4.certificate.identity.unwrap();
        phase.prepared.workspace.group.prefix[0] = SmallPoint::Infinity;
        let mut progress = ReplayProgress::default();
        let mut checking = ledger();
        let before = phase.prepared.group_control.additions_reserved;
        assert_eq!(
            replay_match(&mut phase.prepared, false, id, &mut checking, &mut progress),
            Err(GroupStop::InvalidCandidate)
        );
        assert_eq!(phase.prepared.group_control.additions_reserved, before + 4);
        assert_eq!(progress.additions_performed, 2);
        assert_eq!(progress.failed_stage, Some(ReplayStage::PrefixCompare));
        assert!(checking.reserved_work > checking.performed_work);
    }
    #[test]
    fn step4_replay_original_equations_and_domain_are_independently_checked() {
        let mut phase = group_test_phase(AffinePoint { x: 84, y: 12 });
        run_group_prefixes(&mut phase, None, &mut |_, _| Ok(()));
        let id = phase.prepared.step4.certificate.identity.unwrap();
        phase.prepared.workspace.group.residues[0] = 1;
        assert_eq!(
            replay_match(
                &mut phase.prepared,
                false,
                id,
                &mut ledger(),
                &mut ReplayProgress::default()
            ),
            Err(GroupStop::OriginalEquationMismatch)
        );
        phase.prepared.workspace.group.domain[0] = SmallPoint::Affine(AffinePoint { x: 6, y: 19 });
        assert_eq!(
            replay_match(
                &mut phase.prepared,
                false,
                id,
                &mut ledger(),
                &mut ReplayProgress::default()
            ),
            Err(GroupStop::InvalidDomain)
        );
    }
    #[test]
    fn step4_terminal_single_ticket_failure_clears_slot_and_keeps_witness() {
        let mut publisher = FakeWitness::default();
        let mut phase = step4_fixture(AffinePoint { x: 84, y: 12 }, &mut publisher);
        phase.prepared.group_control.tuple_limit = 1;
        run_group_step4(&mut phase, None, &mut |_, _| Ok(()), &mut publisher);
        let bytes = publisher.bytes.clone();
        phase.prepared.terminal.work_limit = phase.prepared.terminal.reserved_work + 4097;
        assert!(retain_step4_terminal(&mut phase).is_err());
        assert_eq!(
            phase.prepared.terminal.reserved_work,
            phase.prepared.terminal.work_limit
        );
        assert!(phase.prepared.slots.buffers[7].bytes.is_empty());
        assert!(phase.prepared.step4.certificate.facts.durable_acknowledged);
        assert_eq!(phase.prepared.step4.final_artifacts[6].bytes, bytes);
    }
    #[test]
    fn step4_complete_serializer_exact_work_bytes_and_no_partial_promotion() {
        let record = serde_json::json!({"cursor":"340282366920938463463374607431768211455","verified":false});
        let mut measured = ledger();
        let mut slot = InputBuffer::new(1024, &mut ledger()).unwrap();
        encode_complete(&record, &mut slot, &mut measured).unwrap();
        let work = measured.reserved_work;
        let length = slot.bytes.len();
        for (w, cap, ok) in [
            (work, length, true),
            (work - 1, length, false),
            (work, length - 1, false),
        ] {
            let mut budget = Ledger::new(w, 0, Instant::now() + Duration::from_secs(10));
            let mut output = InputBuffer::new(cap, &mut ledger()).unwrap();
            assert_eq!(
                encode_complete(&record, &mut output, &mut budget).is_ok(),
                ok
            );
            if !ok {
                assert!(output.bytes.is_empty());
            }
        }
    }
    #[test]
    fn step4_replay_deadline_and_exact_work_leave_monotone_receipts() {
        fn prepared() -> NativePhase {
            let mut p = group_test_phase(AffinePoint { x: 84, y: 12 });
            run_group_prefixes(&mut p, None, &mut |_, _| Ok(()));
            p
        }
        let mut phase = prepared();
        let id = phase.prepared.step4.certificate.identity.unwrap();
        let mut measured = ledger();
        replay_match(
            &mut phase.prepared,
            false,
            id,
            &mut measured,
            &mut ReplayProgress::default(),
        )
        .unwrap();
        let work = measured.reserved_work;
        for (limit, ok) in [(work, true), (work - 1, false)] {
            let mut phase = prepared();
            let mut check = Ledger::new(limit, 0, Instant::now() + Duration::from_secs(10));
            assert_eq!(
                replay_match(
                    &mut phase.prepared,
                    false,
                    id,
                    &mut check,
                    &mut ReplayProgress::default()
                )
                .is_ok(),
                ok
            );
        }
        fn expire(leaf: usize, l: &mut Ledger) {
            if leaf == 2 {
                l.deadline = Instant::now() - Duration::from_secs(1);
            }
        }
        let mut phase = prepared();
        let rows = phase.prepared.slots.buffers[5].bytes.clone();
        let mut progress = ReplayProgress::default();
        let mut check = ledger();
        REPLAY_ADDITION_HOOK.with(|h| h.set(Some(expire)));
        let result = replay_match(&mut phase.prepared, false, id, &mut check, &mut progress);
        REPLAY_ADDITION_HOOK.with(|h| h.set(None));
        assert_eq!(result, Err(GroupStop::Deadline));
        assert_eq!(progress.additions_reserved, 4);
        assert_eq!(progress.additions_performed, 2);
        assert!(check.reserved_work > check.performed_work);
        assert_eq!(phase.prepared.slots.buffers[5].bytes, rows);
    }
    #[test]
    fn step4_exceptional_group_witness_and_unit_affine_contradiction() {
        let mut phase = group_test_phase(AffinePoint { x: 98, y: 9 });
        let g = &mut phase.prepared.workspace.group;
        g.tuple[..4].copy_from_slice(&[0, 1, 0, 0]);
        g.prefix[..3].copy_from_slice(&[
            SmallPoint::Infinity,
            SmallPoint::Affine(AffinePoint { x: 6, y: 18 }),
            SmallPoint::Affine(AffinePoint { x: 98, y: 9 }),
        ]);
        let mut id = FirstIdentity {
            origin: GroupOrigin::IndependentGroupPrefix,
            phase: SearchPhase::IndependentPrefix,
            cursor: 4,
            indices: [0; 15],
            exception_mask: 1,
            coordinates_match: None,
        };
        id.indices[..4].copy_from_slice(&[0, 1, 0, 0]);
        let mut progress = ReplayProgress::default();
        replay_match(&mut phase.prepared, true, id, &mut ledger(), &mut progress).unwrap();
        assert_eq!(progress.equation_checks_completed, 0);
        assert_eq!(progress.additions_performed, 4);
        let mut affine = group_test_phase(AffinePoint { x: 84, y: 12 });
        run_group_prefixes(&mut affine, None, &mut |_, _| Ok(()));
        let id = affine.prepared.step4.certificate.identity.unwrap();
        assert_eq!(
            replay_match(
                &mut affine.prepared,
                true,
                id,
                &mut ledger(),
                &mut ReplayProgress::default()
            ),
            Err(GroupStop::NativeContradiction)
        );
    }
    #[test]
    fn step4_witness_serialization_all_arities_strict_cursor_and_options() {
        let domain = [SmallPoint::Affine(AffinePoint {
            x: (1 << 61) - 1,
            y: (1 << 61) - 1,
        }); 64];
        let indices = [63usize; 15];
        for m in 4..=15 {
            for origin in [
                GroupOrigin::ReportedBasisLinearCandidate,
                GroupOrigin::ReportedBasisLeafLift,
                GroupOrigin::IndependentGroupPrefix,
            ] {
                for relation in [None, Some(false), Some(true)] {
                    let record = DynamicWitness {
                        schema: "restricted-wide-prime-dynamic-witness/v1",
                        summands: SummandsView {
                            domain: &domain,
                            indices: &indices[..m],
                        },
                        target: CanonicalPointView(domain[0]),
                        sum: CanonicalPointView(domain[0]),
                        replay_group_add_calls: m as u64,
                        certificate_origin: origin,
                        search_phase: SearchPhase::IndependentPrefix,
                        source_cursor: u128::MAX,
                        tuple_indices: &indices[..m],
                        candidate_coordinates_match: relation,
                        exceptional_auxiliary_positions: ExceptionView {
                            mask: (1 << (m - 2)) - 1,
                            m,
                        },
                        independent_derivation_verified: false,
                    };
                    let bytes = serde_json::to_vec(&record).unwrap();
                    assert!(bytes.len() < 4096);
                    #[derive(serde::Deserialize)]
                    struct CursorOnly {
                        source_cursor: String,
                    }
                    let decoded: CursorOnly = serde_json::from_slice(&bytes).unwrap();
                    assert_eq!(decoded.source_cursor, u128::MAX.to_string());
                    let mut slot = InputBuffer::new(bytes.len() + 1, &mut ledger()).unwrap();
                    encode_complete(&record, &mut slot, &mut ledger()).unwrap();
                    assert_eq!(&slot.bytes[..bytes.len()], bytes);
                    assert_eq!(slot.bytes.last(), Some(&b'\n'));
                }
            }
        }
    }
    #[test]
    fn step4_terminal_max_arity_initialized_arrays_and_nullable_native_work() {
        let mut publisher = FakeWitness::default();
        let mut phase = step4_fixture(AffinePoint { x: 84, y: 12 }, &mut publisher);
        phase.prepared.group_control.tuple_limit = 1;
        run_group_step4(&mut phase, None, &mut |_, _| Ok(()), &mut publisher);
        // Serializer-only fixed maxima: not an executed m15 mathematical run.
        let p = &mut phase.prepared;
        p.m = 15;
        p.workspace.group.tuple.fill(63);
        p.workspace
            .group
            .prefix
            .fill(SmallPoint::Affine(AffinePoint {
                x: (1 << 61) - 1,
                y: (1 << 61) - 1,
            }));
        p.workspace.group.coordinates.fill((1 << 61) - 1);
        p.workspace.group.residues.fill((1 << 61) - 1);
        for i in 0..13 {
            p.workspace.group.exceptional[i] = i;
        }
        let id = p.step4.certificate.identity.as_mut().unwrap();
        id.indices.fill(63);
        id.cursor = u128::MAX;
        id.exception_mask = (1 << 13) - 1;
        p.step4.last = LastProgress {
            initialized: true,
            origin: Some(GroupOrigin::ReportedBasisLinearCandidate),
            cursor: u128::MAX,
            complete: true,
            coordinate_count: 28,
            residue_count: 29,
            exceptions: 13,
            row: Some(RowReference {
                row_index: u64::MAX,
                offset: u64::MAX,
                bytes: u64::MAX,
            }),
            metadata_acknowledged: true,
        };
        let report = p.step4.last_report.as_mut().unwrap();
        report.completed_prefixes = 14;
        report.cursor = u128::MAX;
        report.native_lifts_visited = u128::MAX;
        report.independent_visited = u128::MAX;
        report.independent_total = u128::MAX;
        retain_step4_terminal(&mut phase).unwrap();
        let bytes = &phase.prepared.slots.buffers[7].bytes;
        assert!(bytes.len() <= 65536);
        let value: serde_json::Value = serde_json::from_slice(bytes).unwrap();
        assert_eq!(
            value["last_state"]["prefix_points"]
                .as_array()
                .unwrap()
                .len(),
            14
        );
        assert_eq!(
            value["last_state"]["coordinates"].as_array().unwrap().len(),
            28
        );
        assert_eq!(
            value["last_state"]["residues"].as_array().unwrap().len(),
            29
        );
        assert!(value["accounting"]["pools"][2]["performed_work"].is_null());
        assert_eq!(value["accounting"]["pools"][0]["parent"], "root");
    }
    #[test]
    fn step4_staged_legacy_exhaustion_is_not_completion() {
        let p = prepare_original(
            101,
            Parameters {
                m: 4,
                a: 1,
                b: 0,
                target_x: 3,
                codes: &[0],
            },
            AffinePoint { x: 3, y: 38 },
            &[AffinePoint { x: 0, y: 0 }],
            phase_limits(),
            EventPersistence::LocalOnly,
            &mut |_, _| Ok(()),
        )
        .unwrap();
        let mut phase = invoke_original(p, &mut |_, _| Ok(())).unwrap();
        let report = run_group_prefixes(&mut phase, None, &mut |_, _| Ok(()));
        assert_eq!(report.stop, GroupStop::IndependentTraversalFinished);
        assert!(!report.group_domain_complete);
    }
    #[test]
    fn step4_final_confirmation_failure_does_not_erase_ack_or_claim_complete() {
        let mut publisher = FakeWitness::default();
        let mut phase = step4_fixture(AffinePoint { x: 84, y: 12 }, &mut publisher);
        phase.prepared.group_control.tuple_limit = 1;
        let report = run_group_step4(&mut phase, None, &mut |_, _| Ok(()), &mut publisher);
        assert!(report.certificate_durably_persisted);
        // Inject only terminal completed disposition to test precedence. This
        // does not claim the bounded execution actually exhausted its domain.
        let terminal = phase.prepared.step4.last_report.as_mut().unwrap();
        terminal.stop = GroupStop::IndependentTraversalFinished;
        terminal.group_domain_complete = true;
        publisher.refuse = true;
        assert_eq!(
            confirm_step4_witness(&mut phase.prepared, &mut publisher),
            Err(GroupStop::PersistenceFailure)
        );
        retain_step4_terminal(&mut phase).unwrap();
        let v: serde_json::Value =
            serde_json::from_slice(&phase.prepared.slots.buffers[7].bytes).unwrap();
        assert_eq!(v["stage"], "final_staging");
        assert_eq!(v["disposition"], "failed");
        assert_eq!(v["cause"], "persistence_failure");
        assert_eq!(v["claims"]["solver_completed"], false);
        assert_eq!(v["claims"]["group_domain_complete"], true);
        assert_eq!(
            v["first_certificate"]["facts"]["durable_acknowledged"],
            true
        );
    }
    #[test]
    fn step4_pre_group_terminal_and_early_failure_visitors_do_not_execute_group() {
        let mut phase = group_test_phase(AffinePoint { x: 84, y: 12 });
        phase.basis_retained = false;
        extract_reported_linear(&mut phase);
        assert!(!phase.prepared.group_control.started);
        retain_step4_terminal(&mut phase).unwrap();
        let v: serde_json::Value =
            serde_json::from_slice(&phase.prepared.slots.buffers[7].bytes).unwrap();
        assert_eq!(v["stage"], "linear");
        assert_eq!(v["search_phase"], "not_started");
        assert!(v["group_stop"].is_null() && v["last_state"].is_null());
        assert_eq!(phase.prepared.group_control.tuples, 0);
        let mut visited = 0;
        visit_step4_retained_artifacts(&phase.prepared, |path, _, _, bytes| {
            assert!(!bytes.is_empty());
            assert_ne!(path, "pdp-witness.json");
            visited += 1;
            Ok(())
        })
        .unwrap();
        assert!(visited >= 2);
        let failure = PrepareFailure {
            stop: Stop::Deadline,
            ledger: phase.prepared.ledger,
            setup_failure: None,
            retained_slots: Some(phase.prepared.slots),
        };
        let mut recovered = 0;
        visit_prepare_failure_artifacts(&failure, |_, _, _, _| {
            recovered += 1;
            Ok(())
        })
        .unwrap();
        assert_eq!(recovered, visited);
        assert_eq!(
            visit_prepare_failure_artifacts(&failure, |_, _, _, _| Err(())),
            Err(())
        );
    }
    #[test]
    fn step4_partial_owner_preparation_preserves_original_slot_custody() {
        let mut p = prepare_original(
            101,
            Parameters {
                m: 4,
                a: 0,
                b: 7,
                target_x: 84,
                codes: &[6],
            },
            AffinePoint { x: 84, y: 12 },
            &[AffinePoint { x: 6, y: 18 }, AffinePoint { x: 6, y: 83 }],
            phase_limits(),
            EventPersistence::LocalOnly,
            &mut |_, _| Ok(()),
        )
        .unwrap();
        let input = p.slots.buffers[0].bytes.clone();
        p.checking_work = 1;
        assert!(prepare_step4(&mut p, &mut FakeWitness::default()).is_err());
        assert!(!p.group_control.started);
        let mut found = false;
        visit_step4_retained_artifacts(&p, |path, _, _, bytes| {
            if path == "prime-f4-input.json" {
                assert_eq!(bytes, input);
                found = true;
            }
            Ok(())
        })
        .unwrap();
        assert!(found);
    }
    #[test]
    fn step4_terminal_wide_typed_golden() {
        let indices = [63usize; 15];
        let points = [SmallPoint::Affine(AffinePoint {
            x: (1 << 61) - 1,
            y: (1 << 61) - 1,
        }); 14];
        let coordinates = [(1u64 << 61) - 1; 28];
        let residues = [(1u64 << 61) - 1; 29];
        let exceptions: [usize; 13] = std::array::from_fn(|i| i);
        let record = DynamicResult {
            schema: "restricted-wide-prime-dynamic-result/v1",
            strategy: "prime_f4_wide_dynamic",
            stage: "certificate_replay",
            disposition: "censored",
            cause: "serialization_failure",
            native_stop: Some("exponent_overflow"),
            linear_stop: Some(LinearStop::NonlinearOrUnderdetermined),
            group_stop: Some(GroupStop::IndependentTraversalFinished),
            search_phase: "independent_prefix",
            accounting: Accounting {
                counter_snapshot_stage: "before_terminal_retention",
                pools: std::array::from_fn(|i| PoolView {
                    name: [
                        "algebra", "checking", "native", "observer", "capture", "terminal",
                    ][i],
                    parent: ["root", "root", "algebra", "algebra", "checking", "algebra"][i],
                    work_limit: u64::MAX,
                    reserved_work: u64::MAX,
                    performed_work: if i == 2 { None } else { Some(u64::MAX) },
                }),
                requested_bytes: ByteAccounting {
                    limit: u64::MAX,
                    reserved: u64::MAX,
                },
                group: GroupAccounting {
                    addition_limit: u64::MAX,
                    additions_reserved: u64::MAX,
                    additions_performed: u64::MAX,
                    tuple_limit: u64::MAX,
                    tuples_reserved: u64::MAX,
                },
                traversal: TraversalAccounting {
                    native_lifts_visited: u128::MAX,
                    independent_visited: u128::MAX,
                    independent_total: u128::MAX,
                },
                events: EventAccounting {
                    limit: u64::MAX,
                    global_acknowledged: u64::MAX,
                    setup: u64::MAX,
                    native: u64::MAX,
                    linear: u64::MAX,
                    group: u64::MAX,
                    new_payload: u64::MAX,
                    attempted: u64::MAX,
                    pending: u64::MAX,
                },
            },
            source_slots: std::array::from_fn(|i| SlotEvidence {
                index: i as u8,
                state: if i == 6 {
                    "transferred"
                } else {
                    "local_complete"
                },
                bytes: Some(u64::MAX),
                digest_binding: if i == 6 {
                    "capture_receipt"
                } else {
                    "outer_manifest"
                },
                sha256: if i == 6 {
                    Some(HexDigest([255; 32]))
                } else {
                    None
                },
            }),
            last_state: Some(LastState {
                phase: SearchPhase::IndependentPrefix,
                origin: GroupOrigin::ReportedBasisLinearCandidate,
                cursor: u128::MAX,
                tuple_indices: &indices,
                progress: "complete_tuple",
                tuple_entered: false,
                completed_prefixes: 14,
                prefix_points: PointSlice(&points),
                exceptional_auxiliary_positions: &exceptions,
                coordinate_count: 28,
                coordinates: &coordinates,
                residue_count: 29,
                residues: &residues,
                computed_match: false,
                row: Some(RowReference {
                    row_index: u64::MAX,
                    offset: u64::MAX,
                    bytes: u64::MAX,
                }),
                metadata_acknowledged: false,
            }),
            first_certificate: Some(FirstRecord {
                lifecycle: CertificateLifecycle::ComputedPending,
                identity: IdentityView {
                    origin: GroupOrigin::ReportedBasisLinearCandidate,
                    search_phase: SearchPhase::IndependentPrefix,
                    source_cursor: u128::MAX,
                    tuple_indices: &indices,
                    exceptional_auxiliary_positions: ExceptionView {
                        mask: (1 << 13) - 1,
                        m: 15,
                    },
                    candidate_coordinates_match: Some(false),
                },
                facts: FirstFacts::default(),
                replay: ReplayProgress {
                    additions_reserved: u64::MAX,
                    additions_performed: u64::MAX,
                    equation_checks_completed: u16::MAX,
                    coordinate_checks_completed: u16::MAX,
                    failed_stage: Some(ReplayStage::OriginalEquations),
                },
                publication: PublicationView {
                    state: "acknowledged",
                    receipt: Some(ReceiptView {
                        commit: "already_committed",
                        payload_sha256: HexDigest([255; 32]),
                        payload_bytes: u64::MAX,
                        sequence: Some(u64::MAX),
                        provenance: "dedicated",
                    }),
                },
                witness_owner: "final_artifact",
                witness_bytes: Some(u64::MAX),
                witness_sha256: Some(HexDigest([255; 32])),
            }),
            claims: Claims {
                solver_completed: false,
                group_domain_complete: false,
                negative_verified: false,
                pdp_witness_verified: false,
                certificate_durably_persisted: false,
                independent_derivation_verified: false,
                operation_trajectory_verified: false,
            },
        };
        // Wide combinations deliberately exceed semantic caps; this is only a
        // finite serialization bound, never a purported valid execution.
        let bytes = serde_json::to_vec(&record).unwrap();
        assert_eq!(bytes.len() + 1, 6789);
        assert!(bytes.len() < 65536);
        let mut slot = InputBuffer::new(65536, &mut ledger()).unwrap();
        let mut meter = ledger();
        encode_complete(&record, &mut slot, &mut meter).unwrap();
        assert_eq!(&slot.bytes[..bytes.len()], bytes);
        assert_eq!(slot.bytes.last(), Some(&b'\n'));
        // The byte bound does not authorize extra terminal work.
        let mut short = Ledger::new(
            meter.reserved_work - 1,
            0,
            Instant::now() + Duration::from_secs(10),
        );
        assert_eq!(
            encode_complete(&record, &mut slot, &mut short),
            Err(Stop::WorkLimit)
        );
        assert!(slot.bytes.is_empty());
    }
    #[test]
    fn step4_actual_pre_group_producer_bytes_independently_replay() {
        use super::super::prime_f4_wide_dynamic_replay as replay;
        use sha2::{Digest, Sha256};
        let mut phase = group_test_phase(AffinePoint { x: 84, y: 12 });
        retain_step4_terminal(&mut phase).unwrap();
        let domain = [replay::Point { x: 6, y: 18 }, replay::Point { x: 6, y: 83 }];
        let expected = replay::Expected {
            p: 101,
            a: 0,
            b: 7,
            m: 4,
            target: replay::Point { x: 84, y: 12 },
            domain: &domain,
            codes: &[6],
            work_limit: 16384,
            trace_bytes: 128 * 1024 * 1024,
            memory_bytes: 256 * 1024 * 1024,
        };
        let limits = || replay::ReplayLimits {
            work: 10_000_000_000,
            allocation_bytes: 128 * 1024 * 1024,
            deadline: Instant::now() + Duration::from_secs(20),
        };
        let verify = |raw: &[Vec<u8>; 8]| {
            let slots = std::array::from_fn(|i| {
                let bytes = &raw[i];
                (!bytes.is_empty()).then(|| replay::Artifact {
                    path: SLOT_REGISTRY[i].0,
                    role: SLOT_REGISTRY[i].1,
                    encoding: SLOT_REGISTRY[i].2,
                    bytes,
                    sha256: Sha256::digest(bytes).into(),
                })
            });
            replay::verify(
                &expected,
                &replay::Bundle {
                    slots,
                    committed_witness: None,
                },
                limits(),
            )
        };
        let raw: [Vec<u8>; 8] =
            std::array::from_fn(|i| phase.prepared.slots.buffers[i].bytes.clone());
        let result = verify(&raw).expect("actual pre-group producer output must replay");
        assert!(result.original_input_verified);
        assert!(!result.independent_group_domain_complete && !result.first_witness_verified);
        for field in [
            "independent_total",
            "independent_visited",
            "native_lifts_visited",
        ] {
            let mut altered = raw.clone();
            let mut terminal: serde_json::Value = serde_json::from_slice(&altered[7]).unwrap();
            terminal["accounting"]["traversal"][field] = serde_json::json!("1");
            altered[7] = serde_json::to_vec(&terminal).unwrap();
            assert!(
                verify(&altered).is_err(),
                "rehash cannot authorize changed {field}"
            );
        }
    }
    #[test]
    fn step4_actual_positive_producer_bytes_independently_replay() {
        use super::super::prime_f4_wide_dynamic_replay as replay;
        use sha2::{Digest, Sha256};
        // Mock publication tests wire compatibility; actual filesystem custody
        // is covered by the dedicated capture tests, not this mock receipt.
        let mut publisher = FakeWitness::default();
        let mut phase = step4_fixture(AffinePoint { x: 84, y: 12 }, &mut publisher);
        let report = run_group_step4(&mut phase, None, &mut |_, _| Ok(()), &mut publisher);
        assert!(report.pdp_witness_verified && report.certificate_durably_persisted);
        confirm_step4_witness(&mut phase.prepared, &mut publisher).unwrap();
        retain_step4_terminal(&mut phase).unwrap();
        let raw: [Vec<u8>; 8] = std::array::from_fn(|i| {
            if i == 6 {
                phase.prepared.step4.final_artifacts[6].bytes.clone()
            } else {
                phase.prepared.slots.buffers[i].bytes.clone()
            }
        });
        let domain = [replay::Point { x: 6, y: 18 }, replay::Point { x: 6, y: 83 }];
        let expected = replay::Expected {
            p: 101,
            a: 0,
            b: 7,
            m: 4,
            target: replay::Point { x: 84, y: 12 },
            domain: &domain,
            codes: &[6],
            work_limit: 16384,
            trace_bytes: 128 * 1024 * 1024,
            memory_bytes: 256 * 1024 * 1024,
        };
        let verify = |raw: &[Vec<u8>; 8], sequence: u64| {
            let slots = std::array::from_fn(|i| {
                let bytes = &raw[i];
                (!bytes.is_empty()).then(|| replay::Artifact {
                    path: SLOT_REGISTRY[i].0,
                    role: SLOT_REGISTRY[i].1,
                    encoding: SLOT_REGISTRY[i].2,
                    bytes,
                    sha256: Sha256::digest(bytes).into(),
                })
            });
            let committed = replay::CapturedWitness {
                bytes: &publisher.bytes,
                sha256: Sha256::digest(&publisher.bytes).into(),
                original_sequence: sequence,
            };
            replay::verify(
                &expected,
                &replay::Bundle {
                    slots,
                    committed_witness: Some(committed),
                },
                replay::ReplayLimits {
                    work: 10_000_000_000,
                    allocation_bytes: 128 * 1024 * 1024,
                    deadline: Instant::now() + Duration::from_secs(20),
                },
            )
        };
        let verified = verify(&raw, 7).expect("actual positive producer output must replay");
        assert!(verified.first_witness_verified && verified.witness_capture_committed);
        assert!(
            !verified.independent_derivation_verified && !verified.operation_trajectory_verified
        );
        assert!(verify(&raw, 8).is_err());
        let mut altered = raw.clone();
        let mut terminal: serde_json::Value = serde_json::from_slice(&altered[7]).unwrap();
        terminal["accounting"]["group"]["additions_performed"] = serde_json::json!(0);
        altered[7] = serde_json::to_vec(&terminal).unwrap();
        assert!(verify(&altered, 7).is_err());
    }
    #[test]
    fn step4_real_bound_capture_restore_and_independent_witness_replay() {
        use super::super::prime_f4_wide_dynamic_replay as replay;
        use super::super::{EventSink, LiveSinkGuard, LIVE_SINK};
        use crate::ml_dataset::capture;
        use sha2::{Digest, Sha256};
        for exceptional in [false, true] {
            let (m, a, b, target, points, codes) = if exceptional {
                (
                    5,
                    1,
                    0,
                    AffinePoint { x: 0, y: 0 },
                    vec![AffinePoint { x: 0, y: 0 }],
                    vec![0],
                )
            } else {
                (
                    4,
                    0,
                    7,
                    AffinePoint { x: 84, y: 12 },
                    vec![AffinePoint { x: 6, y: 18 }, AffinePoint { x: 6, y: 83 }],
                    vec![6],
                )
            };
            let nonce = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let root = std::env::temp_dir().join(format!(
                "dynamic-bound-restore-{}-{nonce}",
                std::process::id()
            ));
            let cell = root.join("cell");
            std::fs::create_dir_all(&cell).unwrap();
            let binding = capture::AttemptBinding {
                schema: "crypto.ml-dataset.attempt-binding/v1".into(),
                run_sha256: "1".repeat(64),
                plan_sha256: "2".repeat(64),
                protocol_sha256: "3".repeat(64),
                config_sha256: "4".repeat(64),
                source_commit: "5".repeat(40),
                binary_sha256: "6".repeat(64),
                cell_index: 0,
                attempt_sha256: "7".repeat(64),
            };
            let limits = capture::capture_limits(128).unwrap();
            let sink = capture::CaptureEmitter::create(
                &cell.join("live"),
                binding.clone(),
                limits.clone(),
            )
            .unwrap();
            assert!(LIVE_SINK.with(|slot| slot.borrow().is_none()));
            LIVE_SINK.with(|slot| *slot.borrow_mut() = Some(EventSink::Bound(sink)));
            let guard = LiveSinkGuard;
            let mut forward = |event: &PhaseEvent, buffer: &mut TypedEventBuffer| {
                super::super::live_typed_event(
                    capture::TypedEventKind::WidePrimeDynamic,
                    event,
                    buffer,
                )
                .map_err(|_| ())
            };
            let mut prepared = prepare_original(
                101,
                Parameters {
                    m,
                    a,
                    b,
                    target_x: target.x,
                    codes: &codes,
                },
                target,
                &points,
                phase_limits(),
                EventPersistence::BoundCapture,
                &mut forward,
            )
            .unwrap();
            let mut publisher = BoundWitnessPublisher::default();
            prepare_step4(&mut prepared, &mut publisher).unwrap();
            let mut phase = invoke_original(prepared, &mut forward).unwrap();
            let linear = extract_reported_linear(&mut phase);
            retain_linear(&mut phase, &linear).unwrap();
            let report = run_group_step4(&mut phase, Some(&linear), &mut forward, &mut publisher);
            assert!(
                report.pdp_witness_verified && report.certificate_durably_persisted,
                "real bound publication did not complete: {report:?}"
            );
            let before_confirmation = phase.prepared.checking_work;
            let confirmation = confirm_step4_witness(&mut phase.prepared, &mut publisher);
            assert!(confirmation.is_ok(), "confirmation={confirmation:?}; remaining before={before_confirmation}; report={report:?}");
            retain_step4_terminal(&mut phase).unwrap();
            visit_step4_retained_artifacts(&phase.prepared, |path, role, encoding, bytes| {
                if path == "pdp-witness.json" {
                    return Ok(());
                }
                super::super::live_borrowed_payload(path, role, encoding, bytes).map_err(|_| ())
            })
            .unwrap();
            LIVE_SINK.with(|slot| match slot.borrow_mut().as_mut() {
                Some(EventSink::Bound(sink)) => sink.finish().unwrap(),
                _ => panic!("bound sink lost"),
            });
            drop(guard);
            let verified_live = capture::verify_live_committed_prefix(&cell, &binding, &limits)
                .unwrap()
                .unwrap();
            let live_witness: Vec<_> = verified_live
                .payloads
                .iter()
                .filter(|(_, a)| a.path == "payloads/pdp-witness.json")
                .collect();
            assert_eq!(live_witness.len(), 1);
            let initial = capture::CaptureCursor::initial(&binding).unwrap();
            let snapshot = root.join("restored");
            let delta =
                capture::capture_committed_delta(&cell, &snapshot, &binding, &initial, &limits)
                    .unwrap()
                    .unwrap();
            capture::verify_captured_delta(&snapshot, &delta, &initial).unwrap();
            let mut sequences = Vec::new();
            for segment in &delta.segments {
                let bytes = std::fs::read(snapshot.join(&segment.artifact.path)).unwrap();
                for line in bytes.split(|&c| c == b'\n').filter(|line| !line.is_empty()) {
                    let event: capture::EventRecord = serde_json::from_slice(line).unwrap();
                    if event.event["kind"] == "capture_payload_committed"
                        && event.event["artifact"]["path"] == "payloads/pdp-witness.json"
                    {
                        sequences.push(event.sequence);
                    }
                }
            }
            assert_eq!(
                sequences.len(),
                1,
                "confirmation must not republish the witness"
            );
            assert_eq!(sequences[0], live_witness[0].0);
            let descriptors: [Option<&crate::ml_dataset::types::Artifact>; 8] =
                std::array::from_fn(|i| {
                    let path = format!("payloads/{}", SLOT_REGISTRY[i].0);
                    let matches: Vec<_> =
                        delta.artifacts.iter().filter(|a| a.path == path).collect();
                    assert!(matches.len() <= 1);
                    matches.first().copied()
                });
            let raw: [Vec<u8>; 8] = std::array::from_fn(|i| {
                descriptors[i]
                    .map_or_else(Vec::new, |a| std::fs::read(snapshot.join(&a.path)).unwrap())
            });
            let slots = std::array::from_fn(|i| {
                descriptors[i].map(|a| replay::Artifact {
                    path: a.path.strip_prefix("payloads/").unwrap(),
                    role: &a.role,
                    encoding: &a.encoding,
                    bytes: &raw[i],
                    sha256: Sha256::digest(&raw[i]).into(),
                })
            });
            let replay_domain: Vec<_> = points
                .iter()
                .map(|q| replay::Point { x: q.x, y: q.y })
                .collect();
            let expected = replay::Expected {
                p: 101,
                a,
                b,
                m,
                target: replay::Point {
                    x: target.x,
                    y: target.y,
                },
                domain: &replay_domain,
                codes: &codes,
                work_limit: 16384,
                trace_bytes: 128 * 1024 * 1024,
                memory_bytes: 256 * 1024 * 1024,
            };
            let verified = replay::verify(
                &expected,
                &replay::Bundle {
                    slots,
                    committed_witness: Some(replay::CapturedWitness {
                        bytes: &raw[6],
                        sha256: Sha256::digest(&raw[6]).into(),
                        original_sequence: sequences[0],
                    }),
                },
                replay::ReplayLimits {
                    work: 10_000_000_000,
                    allocation_bytes: 128 * 1024 * 1024,
                    deadline: Instant::now() + Duration::from_secs(20),
                },
            )
            .unwrap();
            assert!(verified.first_witness_verified && verified.witness_capture_committed);
            let mut file_artifacts = Vec::new();
            for a in descriptors.iter().flatten() {
                let mut canonical = (*a).clone();
                canonical.path = canonical
                    .path
                    .strip_prefix("payloads/")
                    .unwrap()
                    .to_string();
                std::fs::write(
                    cell.join(&canonical.path),
                    std::fs::read(snapshot.join(&a.path)).unwrap(),
                )
                .unwrap();
                file_artifacts.push(canonical);
            }
            let check_files = |artifacts: &[crate::ml_dataset::types::Artifact]| {
                replay::verify_files(
                    &cell,
                    &expected,
                    artifacts,
                    &binding,
                    &limits,
                    replay::ReplayLimits {
                        work: 10_000_000_000,
                        allocation_bytes: 128 * 1024 * 1024,
                        deadline: Instant::now() + Duration::from_secs(20),
                    },
                )
            };
            let file_verified = check_files(&file_artifacts).unwrap();
            assert!(
                file_verified.first_witness_verified && file_verified.witness_capture_committed
            );
            let outer = serde_json::json!({
                "status": if file_verified.solver_completed_verified { "verified" } else { "partial" },
                "solver_completed": file_verified.solver_completed_verified,
                "pdp_witness_verified": true,
                "certificate_durably_persisted": true,
                "group_domain_complete": file_verified.independent_group_domain_complete,
                "exhaustive_no_decomposition": file_verified.negative_verified,
                "native_backend_invoked": file_verified.native_backend_invoked_reported,
                "native_statistics_complete": file_verified.native_statistics_complete_reported
            });
            super::super::verify_dynamic_claims(&outer, &file_verified).unwrap();
            for key in [
                "scalar_recovered_without_witness",
                "native_basis_derivation_verified",
                "execution_trace_complete_verified",
                "total_operation_count_verified",
                "independent_derivation_verified",
                "operation_trajectory_verified",
                "exact_instruction_count_verified",
                "full_execution_trace_verified",
                "fall_degree_verified",
                "extraction_complete",
                "exhaustive_no_decomposition",
            ] {
                let mut forged = outer.clone();
                forged[key] = serde_json::json!(true);
                assert!(
                    super::super::verify_dynamic_claims(&forged, &file_verified).is_err(),
                    "{key}"
                );
            }
            let mut forged = outer.clone();
            forged["pdp_witness_verified"] = serde_json::json!(1);
            assert!(super::super::verify_dynamic_claims(&forged, &file_verified).is_err());
            forged = outer.clone();
            forged["native_backend_invoked"] =
                serde_json::json!(!file_verified.native_backend_invoked_reported);
            assert!(super::super::verify_dynamic_claims(&forged, &file_verified).is_err());
            forged = outer.clone();
            forged["solver_completed"] = serde_json::json!(true);
            forged["live_persistence_errors"] = serde_json::json!(["retention failed"]);
            assert!(super::super::verify_dynamic_claims(&forged, &file_verified).is_err());
            super::super::verify_dynamic_claims(
                &serde_json::json!({"status":"timeout",
                "solver_completed":false,"pdp_witness_verified":false}),
                &file_verified,
            )
            .unwrap();
            if !exceptional {
                assert!(!file_verified.independent_group_domain_complete);
                assert!(!file_verified.solver_completed_verified);
                assert!(!file_verified.negative_verified);
                let terminal_index = file_artifacts
                    .iter()
                    .position(|a| a.path == "prime-f4-wide-dynamic-result.json")
                    .unwrap();
                let mut forged = file_artifacts.clone();
                let mut terminal: serde_json::Value = serde_json::from_slice(&raw[7]).unwrap();
                terminal["disposition"] = serde_json::json!("complete");
                terminal["claims"]["solver_completed"] = serde_json::json!(true);
                let bytes = serde_json::to_vec(&terminal).unwrap();
                std::fs::write(cell.join(&forged[terminal_index].path), &bytes).unwrap();
                forged[terminal_index].bytes = bytes.len() as u64;
                forged[terminal_index].sha256 = crate::ml_dataset::digest(&bytes);
                assert!(
                    check_files(&forged).is_err(),
                    "positive prefix cannot be promoted to completed search"
                );
                std::fs::write(cell.join(&forged[terminal_index].path), &raw[7]).unwrap();
            }
            let mut duplicate = file_artifacts.clone();
            duplicate.push(duplicate[0].clone());
            assert!(check_files(&duplicate).is_err());
            let mut moved = file_artifacts.clone();
            moved[0].path = "moved-input.json".into();
            assert!(check_files(&moved).is_err());
            let witness_index = file_artifacts
                .iter()
                .position(|a| a.path == "pdp-witness.json")
                .unwrap();
            let mut altered = file_artifacts.clone();
            let mut altered_bytes = raw[6].clone();
            altered_bytes.push(b' ');
            std::fs::write(cell.join("pdp-witness.json"), &altered_bytes).unwrap();
            altered[witness_index].bytes = altered_bytes.len() as u64;
            altered[witness_index].sha256 = crate::ml_dataset::digest(&altered_bytes);
            assert_eq!(
                check_files(&altered).unwrap_err(),
                "dynamic witness differs from committed publication"
            );
            std::fs::write(cell.join("pdp-witness.json"), &raw[6]).unwrap();
            assert!(
                !verified.independent_derivation_verified
                    && !verified.operation_trajectory_verified
            );
            std::fs::write(snapshot.join("payloads/pdp-witness.json"), b"corrupt").unwrap();
            assert!(capture::verify_captured_delta(&snapshot, &delta, &initial).is_err());
            std::fs::remove_dir_all(root).unwrap();
        }
    }
}
