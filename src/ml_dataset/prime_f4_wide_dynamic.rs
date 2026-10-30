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
#[derive(Clone, Copy)]
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
    assignments: [Option<u64>; 28],
    sources: [usize; 28],
    transitions: [usize; 29],
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
            assignments: [None; 28],
            sources: [0; 28],
            transitions: [0; 29],
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
    source: &'static str,
    global_sequence: usize,
    local_sequence: usize,
    stage: &'static str,
    reserved_work: u64,
    reserved_bytes: u64,
    persistence: EventPersistence,
}
struct ObserverState {
    acknowledged: usize,
    setup: usize,
    native: usize,
    durable: usize,
    pending: Option<PhaseEvent>,
    stop: Option<Stop>,
    last_native: Supply,
    persistence: EventPersistence,
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
        if event.source == "setup" {
            state.setup += 1;
        } else {
            state.native += 1;
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
    terminal: Ledger,
    observer: ObserverState,
    phase_deadline: Instant,
    original_deadline: Instant,
    m: usize,
    degree_bound: u32,
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
            + size_of::<[usize; 32]>()) as u64;
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
            setup: 0,
            native: 0,
            durable: 0,
            pending: None,
            stop: None,
            last_native: Supply { work: 0, bytes: 0 },
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
                    source: "setup",
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
            terminal,
            observer,
            phase_deadline,
            original_deadline: limits.original_deadline,
            m: parameters.m,
            degree_bound: limits.degree_bound.min(12),
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
            if event.sequence != prepared.observer.native
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
                source: "native",
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
            Ok(())
        },
    );
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
        if report.acknowledged_events != prepared.observer.native {
            return Err(Stop::InvalidInput);
        }
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
            setup: 0,
            native: 0,
            durable: 0,
            pending: None,
            stop: None,
            last_native: Supply { work: 0, bytes: 0 },
            persistence: EventPersistence::BoundCapture,
        };
        let event = PhaseEvent {
            schema: "crypto.ml-dataset.wide-prime-dynamic-event/v1",
            source: "native",
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
    /// Explicit long panel: all432 cases are attempted; ignored/unrun is NOT coverage.
    #[test]
    #[ignore = "explicit original-chain432-case software gate; invoke separately with --ignored --exact --nocapture"]
    fn dynamic_original_chain_all_arity_twist_432_case_panel() {
        use num_traits::ToPrimitive;
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
                    let mut case = match super::super::curves::generate(&cfg) {
                        Ok(case) => case,
                        Err(error) => {
                            attempted += 6;
                            failures.push(format!("all six fixed targets failed generation n{n} twist{twist} m{m}: {error}"));
                            continue;
                        }
                    };
                    case.private_witness = serde_json::Value::Null;
                    let group = super::super::Group::read(&case).unwrap();
                    let generator = group.point(&case.public_instance["generator"]).unwrap();
                    let super::super::Group::Prime(curve) = &group else {
                        panic!("wrong curve family")
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
                                failures.push(format!("n{n} twist{twist} m{m} K{k} target{target_kind}: nonaffine domain"));
                                continue;
                            };
                            let mut codes: Vec<_> = signed.iter().map(|p| p.x).collect();
                            codes.sort_unstable();
                            codes.dedup();
                            if !preconditions || codes.len() != k {
                                failures.push(format!("n{n} twist{twist} m{m} K{k} target{target_kind}: failed fixed preconditions"));
                                continue;
                            }
                            let multiplier = match target_kind {
                                0 => k * m,
                                1 => k * m + 1,
                                _ => k * (m - 2),
                            };
                            let Some(target) = affine(&group.mul(&generator, multiplier as u64))
                            else {
                                failures.push(format!(
                                    "nonaffine fixed target n{n} twist{twist} m{m} K{k}"
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
                                    eprintln!("native_case n={n} twist={twist} m={m} K={k} target={target_kind} error={error:?}");
                                    failures.push(format!("preparation/integration n{n} twist{twist} m{m} K{k} target{target_kind}: {:?}",error.stop));
                                }
                                Ok(phase) => {
                                    let useful = phase.report.matrix_rows_reduced > 0
                                        && phase.report.pivot_updates > 0;
                                    eprintln!("native_case n={n} twist={twist} m={m} K={k} target={target_kind} stop={:?} invoked={} rows={} pivots={} useful={} retained={} retention={:?} work={} bytes={} input_sha256={}",phase.report.stop,phase.report.native_backend_invoked,phase.report.matrix_rows_reduced,phase.report.pivot_updates,useful,phase.basis_retained,phase.retention_stop,phase.prepared.ledger.reserved_work,phase.prepared.ledger.reserved_bytes,super::super::super::digest(phase.prepared.slots.buffers[0].bytes()));
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
        assert_eq!(attempted, 432);
        assert!(
            failures.is_empty(),
            "all432 fixed cells retained; {} failed gates: {failures:?}",
            failures.len()
        );
    }
}
