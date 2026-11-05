//! Explicit alternative encoding: affine coordinate domains over the original
//! checked map. No selector fallback, raw-encoding substitution, or dataset seal.
use super::{sparse, Admission, CheckedMap, Error, Limits, Polynomial, Quote};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::io::Write;

pub const SCHEMA: &str = "checked-codebook-affine-domain/v1";
pub const ORDER: &str =
    "original-and-xor-unit-impossible;factor-block-major-nonpivot-bit-ascending/v1";
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Failure {
    Cap,
    Overflow,
    Allocation,
    InvalidIdentity,
    UnsupportedNonAffine,
    Owner(Error),
    Original(sparse::Failure),
    Serialization,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    Validate,
    Recognize,
    Plan,
    Original,
    Domain,
    Seal,
    Complete,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Route {
    Empty,
    Affine,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Identity {
    pub schema: &'static str,
    pub order: &'static str,
    pub route: Route,
    pub original_map_sha256: [u8; 32],
    pub ordered_generators_sha256: [u8; 32],
    pub original_binding: [u8; 32],
    pub variables: u32,
    pub generators: usize,
    pub factor_blocks: usize,
    pub n: u32,
    pub rank: u32,
    pub origin: u64,
    // The original map digest binds every positive global block ID and code.
    // Pivots use increasing local bit order; output indices are global wire-1.
    pub pivot_rows: [u64; 61],
}
/// Both success and refusal retain the actual source, paid buffers and counters.
/// Identity appears only after complete successful construction and final check.
pub struct Attempt {
    source: CheckedMap,
    identity: Option<Identity>,
    stage: Stage,
    failure: Option<Failure>,
    charge: Quote,
    codes: Vec<u64>,
    basis: [u64; 61],
    origin: u64,
    rank: u32,
    block: usize,
    coordinate: usize,
    original: Option<sparse::Attempt>,
    equations: Vec<Polynomial>,
    partial: Polynomial,
    scratch: Vec<u32>,
    source_hasher: Sha256,
    equations_hasher: Sha256,
}
impl Attempt {
    pub fn status(&self) -> Result<(), Failure> {
        self.failure.map_or(Ok(()), Err)
    }
    pub fn source(&self) -> &CheckedMap {
        &self.source
    }
    pub fn identity(&self) -> Option<&Identity> {
        self.identity.as_ref()
    }
    pub fn variables(&self) -> u32 {
        self.source.variables()
    }
    pub fn equations(&self) -> &[Polynomial] {
        &self.equations
    }
    pub fn charged(&self) -> Quote {
        self.charge
    }
    pub fn stage(&self) -> Stage {
        self.stage
    }
    pub fn position(&self) -> (usize, usize) {
        (self.block, self.coordinate)
    }
    pub fn retained(&self) -> (&[u64], &[u64; 61], &Polynomial, &[u32]) {
        (&self.codes, &self.basis, &self.partial, &self.scratch)
    }
    pub fn original_attempt(&self) -> Option<&sparse::Attempt> {
        self.original.as_ref()
    }
}
struct Meter<'a, T: Admission> {
    owner: &'a mut T,
    limits: Limits,
    charge: Quote,
    first: Option<Failure>,
}
impl<T: Admission> Meter<'_, T> {
    fn fail<R>(&mut self, f: Failure) -> Result<R, Failure> {
        Err(*self.first.get_or_insert(f))
    }
    fn check(&mut self) -> Result<(), Failure> {
        if let Some(f) = self.first {
            return Err(f);
        }
        if let Err(e) = self.owner.check() {
            return self.fail(Failure::Owner(e));
        }
        Ok(())
    }
    fn pay(&mut self, work: u64, bytes: u64) -> Result<(), Failure> {
        self.check()?;
        let Some(w) = self.charge.work.checked_add(work) else {
            return self.fail(Failure::Overflow);
        };
        let Some(b) = self.charge.bytes.checked_add(bytes) else {
            return self.fail(Failure::Overflow);
        };
        if w > self.limits.work || b > self.limits.bytes {
            return self.fail(Failure::Cap);
        }
        if let Err(e) = self.owner.reserve(Quote { work, bytes }) {
            return self.fail(Failure::Owner(e));
        }
        self.charge = Quote { work: w, bytes: b };
        self.check()
    }
}
fn bytes<T>(n: usize) -> Result<u64, Failure> {
    n.checked_mul(std::mem::size_of::<T>())
        .and_then(|v| v.checked_add(64))
        .and_then(|v| u64::try_from(v).ok())
        .ok_or(Failure::Overflow)
}
struct OriginalOwner<'a, 'b, T: Admission>(&'a mut Meter<'b, T>);
impl<T: Admission> sparse::Owner for OriginalOwner<'_, '_, T> {
    fn reserve(&mut self, q: sparse::Charge) -> Result<(), ()> {
        self.0.pay(q.work, q.bytes).map_err(|_| ())
    }
    fn check(&mut self) -> Result<(), ()> {
        self.0.check().map_err(|_| ())
    }
}
struct HashWriter<'a, 'b, T: Admission> {
    meter: &'a mut Meter<'b, T>,
    hash: &'a mut Sha256,
}
impl<T: Admission> Write for HashWriter<'_, '_, T> {
    fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
        let work = (b.len() as u64)
            .checked_mul(16)
            .and_then(|v| v.checked_add(8))
            .ok_or(std::io::ErrorKind::InvalidData)?;
        self.meter
            .pay(work, 0)
            .map_err(|_| std::io::ErrorKind::Other)?;
        self.hash.update(b);
        Ok(b.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
fn hash<T: Serialize, O: Admission>(
    value: &T,
    digest: &mut Sha256,
    meter: &mut Meter<'_, O>,
) -> Result<[u8; 32], Failure> {
    serde_json::to_writer(
        HashWriter {
            meter: &mut *meter,
            hash: &mut *digest,
        },
        value,
    )
    .map_err(|_| meter.first.unwrap_or(Failure::Serialization))?;
    meter.check()?;
    Ok(digest.clone().finalize().into())
}
fn append_term<T: Admission>(
    out: &mut Attempt,
    variable: Option<u32>,
    meter: &mut Meter<'_, T>,
) -> Result<(), Failure> {
    let degree = usize::from(variable.is_some());
    if degree > meter.limits.degree || out.partial.len() >= meter.limits.terms {
        return meter.fail(Failure::Cap);
    }
    meter.pay(8, bytes::<u32>(degree)?)?;
    out.scratch
        .try_reserve_exact(degree)
        .map_err(|_| Failure::Allocation)?;
    if let Some(v) = variable {
        out.scratch.push(v)
    }
    // The partial row's full capacity was paid before any terms were created.
    out.partial.push(std::mem::take(&mut out.scratch));
    Ok(())
}
/// Takes ownership even if its first frame reservation fails. No retry/reset API.
pub fn construct(source: CheckedMap, limits: Limits, owner: &mut impl Admission) -> Attempt {
    let mut out = Attempt {
        source,
        identity: None,
        stage: Stage::Validate,
        failure: None,
        charge: Quote { work: 0, bytes: 0 },
        codes: Vec::new(),
        basis: [0; 61],
        origin: 0,
        rank: 0,
        block: 0,
        coordinate: 0,
        original: None,
        equations: Vec::new(),
        partial: Vec::new(),
        scratch: Vec::new(),
        source_hasher: Sha256::new(),
        equations_hasher: Sha256::new(),
    };
    let mut meter = Meter {
        owner,
        limits,
        charge: Quote { work: 0, bytes: 0 },
        first: None,
    };
    let result = run(&mut out, &mut meter);
    out.charge = meter.charge;
    if let Err(f) = result {
        out.failure = Some(meter.first.unwrap_or(f));
        out.identity = None
    }
    out
}
fn run<T: Admission>(out: &mut Attempt, meter: &mut Meter<'_, T>) -> Result<(), Failure> {
    meter.pay(64, bytes::<Attempt>(1)?)?;
    let n = out.source.0.identity.n as usize;
    let m = out.source.0.identity.m;
    if !(1..=61).contains(&n)
        || m < 2
        || m > 15
        || out.source.variables() > meter.limits.variables
        || out.source.0.blocks.len() < m
    {
        return meter.fail(Failure::InvalidIdentity);
    }
    let count = out.source.0.identity.codes.len();
    meter.pay(count as u64, bytes::<u64>(count)?)?;
    out.codes
        .try_reserve_exact(count)
        .map_err(|_| Failure::Allocation)?;
    for words in &out.source.0.identity.codes {
        meter.pay(8, 0)?;
        if words.len() > 2 {
            return meter.fail(Failure::InvalidIdentity);
        }
        let code = words.first().copied().unwrap_or(0) as u64
            | ((words.get(1).copied().unwrap_or(0) as u64) << 32);
        if code >= (1u64 << n) || out.codes.last().is_some_and(|p| *p >= code) {
            return meter.fail(Failure::InvalidIdentity);
        }
        out.codes.push(code);
    }
    // Authentication already establishes map semantics; assert indexing shape
    // defensively and use actual block IDs, never local arithmetic offsets.
    for b in &out.source.0.blocks[..m] {
        meter.pay((n * n) as u64, 0)?;
        if b.len() != n
            || b.iter().any(|v| *v == 0 || *v > out.source.variables())
            || (0..n).any(|i| b[..i].contains(&b[i]))
        {
            return meter.fail(Failure::InvalidIdentity);
        }
    }
    out.stage = Stage::Recognize;
    if let Some(origin) = out.codes.first().copied() {
        out.origin = origin;
        for code in out.codes.iter().copied().skip(1) {
            meter.pay((n * n * 2) as u64, 0)?;
            let mut v = code ^ origin;
            for p in 0..n {
                if v & (1u64 << p) != 0 && out.basis[p] != 0 {
                    v ^= out.basis[p]
                }
            }
            if v == 0 {
                continue;
            }
            let pivot = v.trailing_zeros() as usize;
            for row in &mut out.basis[..n] {
                if *row & (1u64 << pivot) != 0 {
                    *row ^= v
                }
            }
            out.basis[pivot] = v;
            out.rank += 1;
        }
        if 1u64.checked_shl(out.rank).ok_or(Failure::Overflow)? != count as u64 {
            return meter.fail(Failure::UnsupportedNonAffine);
        }
    }
    out.stage = Stage::Plan;
    let domain_rows = if out.codes.is_empty() {
        1
    } else {
        m.checked_mul(n - out.rank as usize)
            .ok_or(Failure::Overflow)?
    };
    let map = &out.source.0;
    let original_rows = map
        .ands
        .len()
        .checked_add(map.xors.len())
        .and_then(|x| x.checked_add(map.units.len()))
        .and_then(|x| x.checked_add(usize::from(map.impossible)))
        .ok_or(Failure::Overflow)?;
    let total = original_rows
        .checked_add(domain_rows)
        .ok_or(Failure::Overflow)?;
    if total > meter.limits.generators {
        return meter.fail(Failure::Cap);
    }
    out.stage = Stage::Original;
    let mut attempt = sparse::expand(
        sparse::Source {
            variables: map.variables,
            ands: &map.ands,
            xors: &map.xors,
            units: &map.units,
            impossible: map.impossible,
            domain: &[],
        },
        meter.limits,
        &mut OriginalOwner(meter),
    );
    let status = attempt.result;
    out.equations = std::mem::take(&mut attempt.equations);
    out.original = Some(attempt);
    status.map_err(|f| meter.first.unwrap_or(Failure::Original(f)))?;
    meter.pay(total as u64, bytes::<Polynomial>(total)?)?;
    out.equations
        .try_reserve_exact(domain_rows)
        .map_err(|_| Failure::Allocation)?;
    out.stage = Stage::Domain;
    if out.codes.is_empty() {
        meter.pay(1, bytes::<Vec<u32>>(1)?)?;
        out.partial
            .try_reserve_exact(1)
            .map_err(|_| Failure::Allocation)?;
        append_term(out, None, meter)?;
        out.equations.push(std::mem::take(&mut out.partial));
    } else {
        for block in 0..m {
            out.block = block;
            for free in 0..n {
                if out.basis[free] != 0 {
                    continue;
                }
                out.coordinate = free;
                let terms = 2 + out.rank as usize;
                if terms > meter.limits.terms {
                    return meter.fail(Failure::Cap);
                }
                // Full linear-row allocation and worst-case comparison envelope.
                meter.pay((terms * terms * 4) as u64, bytes::<Vec<u32>>(terms)?)?;
                out.partial
                    .try_reserve_exact(terms)
                    .map_err(|_| Failure::Allocation)?;
                let wire = out.source.0.blocks[block][free];
                append_term(out, Some(wire - 1), meter)?;
                let mut rhs = ((out.origin >> free) & 1) != 0;
                for pivot in 0..n {
                    if out.basis[pivot] & (1u64 << free) != 0 {
                        let wire = out.source.0.blocks[block][pivot];
                        append_term(out, Some(wire - 1), meter)?;
                        rhs ^= ((out.origin >> pivot) & 1) != 0;
                    }
                }
                if rhs {
                    append_term(out, None, meter)?
                }
                out.partial.sort_unstable();
                out.equations.push(std::mem::take(&mut out.partial));
                meter.check()?;
            }
        }
    }
    out.stage = Stage::Seal;
    // Tagged streams have exact deterministic ordering; JSON is never parsed to
    // mint this identity. Partial hasher state remains retained on owner refusal.
    meter.pay(128, 0)?;
    out.source_hasher.update(b"checked-map-serde-json/v1\0");
    let source_hash = hash(&out.source.0, &mut out.source_hasher, meter)?;
    out.equations_hasher.update(SCHEMA.as_bytes());
    out.equations_hasher.update(ORDER.as_bytes());
    let equations_hash = hash(&out.equations, &mut out.equations_hasher, meter)?;
    meter.check()?;
    out.identity = Some(Identity {
        schema: SCHEMA,
        order: ORDER,
        route: if out.codes.is_empty() {
            Route::Empty
        } else {
            Route::Affine
        },
        original_map_sha256: source_hash,
        ordered_generators_sha256: equations_hash,
        original_binding: out.source.original_binding(),
        variables: out.source.variables(),
        generators: out.equations.len(),
        factor_blocks: m,
        n: n as u32,
        rank: out.rank,
        origin: out.origin,
        pivot_rows: out.basis,
    });
    out.stage = Stage::Complete;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::super::super::{BigUint, F2mElement, IrreduciblePoly};
    use super::super::super::{build_original, Original};
    use super::*;
    #[derive(Default)]
    struct Ledger {
        work: u64,
        bytes: u64,
        refuse: bool,
    }
    impl Admission for Ledger {
        fn reserve(&mut self, q: Quote) -> Result<(), Error> {
            if self.refuse {
                return Err(Error::Admission);
            }
            let work = self.work.checked_add(q.work).ok_or(Error::Admission)?;
            let bytes = self.bytes.checked_add(q.bytes).ok_or(Error::Admission)?;
            if work > 16_777_216 || bytes > 256 * 1024 * 1024 {
                return Err(Error::Admission);
            }
            self.work = work;
            self.bytes = bytes;
            Ok(())
        }
        fn check(&mut self) -> Result<(), Error> {
            if self.refuse {
                Err(Error::Admission)
            } else {
                Ok(())
            }
        }
    }
    fn limits() -> Limits {
        Limits {
            variables: 4096,
            generators: 4096,
            terms: 4096,
            degree: 64,
            work: 4_000_000,
            bytes: 8_000_000,
        }
    }
    fn element(x: u64, n: u32) -> F2mElement {
        F2mElement::from_bit_positions(
            &(0..n).filter(|b| x & (1u64 << b) != 0).collect::<Vec<_>>(),
            n,
        )
    }
    fn genuine(n: u32, twist: bool, codes: &[u64], owner: &mut Ledger) -> CheckedMap {
        let irr = IrreduciblePoly {
            degree: n,
            low_terms: if n == 23 { vec![0, 5] } else { vec![0, 1] },
        };
        let a = element(if n == 23 { 1887600 } else { 1 }, n);
        let b = element(if n == 23 { 570608 } else { 2 }, n);
        let target = element(if n == 23 { 7715066 } else { 1 }, n);
        let codes = codes.iter().copied().map(BigUint::from).collect::<Vec<_>>();
        let original = Original {
            binding: [23; 32],
            a: &a,
            b: &b,
            irr: &irr,
            twist,
            m: 3,
            target: Some(&target),
            pattern: 1,
            codes: &codes,
        };
        build_original(
            &original,
            super::super::super::Limits {
                variables: 4096,
                codes: 16,
                domain_clauses: 4096,
                work: 4_000_000,
                bytes: 8_000_000,
            },
            owner,
        )
        .unwrap()
    }
    fn eval(p: &Polynomial, bits: &[bool]) -> bool {
        p.iter()
            .fold(false, |a, t| a ^ t.iter().all(|v| bits[*v as usize]))
    }
    #[test]
    fn actual_n23_affine_map_both_twists_keeps_original_limits_and_canonical_counts() {
        for twist in [false, true] {
            let mut owner = Ledger::default();
            let map = genuine(23, twist, &[4833900, 6976212], &mut owner);
            let before = (owner.work, owner.bytes);
            let binding = map.original_binding();
            let out = construct(map, limits(), &mut owner);
            assert_eq!(out.status(), Ok(()));
            assert_eq!(out.stage(), Stage::Complete);
            let id = out.identity().unwrap();
            assert_eq!(id.schema, SCHEMA);
            assert_eq!(id.rank, 1);
            assert_eq!(id.variables, 115);
            assert_eq!(id.generators, 158);
            assert_eq!(id.original_binding, binding);
            assert_eq!(
                out.charged(),
                Quote {
                    work: owner.work - before.0,
                    bytes: owner.bytes - before.1
                }
            );
            assert!(out.charged().work <= 4_000_000 && out.charged().bytes <= 8_000_000);
            assert!(out
                .equations()
                .iter()
                .all(|p| p.windows(2).all(|w| w[0] < w[1])));
        }
    }
    #[test]
    fn affine_domain_rows_match_exact_small_codebooks_on_each_global_block() {
        for codes in [
            vec![],
            vec![3],
            vec![1, 6],
            vec![0, 3, 5, 6],
            (0..8).collect(),
        ] {
            for twist in [false, true] {
                let mut owner = Ledger::default();
                let map = genuine(3, twist, &codes, &mut owner);
                let original_count = map.0.ands.len()
                    + map.0.xors.len()
                    + map.0.units.len()
                    + usize::from(map.0.impossible);
                let out = construct(map, limits(), &mut owner);
                assert_eq!(out.status(), Ok(()));
                let domain = &out.equations()[original_count..];
                if codes.is_empty() {
                    assert_eq!(domain, &[vec![Vec::<u32>::new()]]);
                    continue;
                }
                let per_block = 3 - out.identity().unwrap().rank as usize;
                for block in 0..3 {
                    for x in 0..8 {
                        let mut bits = vec![false; out.variables() as usize];
                        for (j, wire) in out.source.0.blocks[block].iter().enumerate() {
                            bits[*wire as usize - 1] = x & (1 << j) != 0
                        }
                        let accepted = domain[block * per_block..(block + 1) * per_block]
                            .iter()
                            .all(|p| !eval(p, &bits));
                        assert_eq!(accepted, codes.contains(&x));
                    }
                }
            }
        }
    }
    #[test]
    fn nonaffine_refusal_retains_checked_source_basis_and_paid_recognition() {
        let mut owner = Ledger::default();
        let map = genuine(3, false, &[0, 1, 3], &mut owner);
        let out = construct(map, limits(), &mut owner);
        assert_eq!(out.status(), Err(Failure::UnsupportedNonAffine));
        assert_eq!(out.stage(), Stage::Recognize);
        assert!(out.identity().is_none());
        assert!(out.equations().is_empty());
        assert_eq!(out.retained().0, &[0, 1, 3]);
        assert!(out.charged().work > 0);
        assert_eq!(out.source().original_binding(), [23; 32]);
    }
    #[test]
    fn initial_owner_and_local_cap_refusals_keep_source_without_success_identity() {
        for owner_refusal in [false, true] {
            let mut owner = Ledger::default();
            let map = genuine(3, false, &[0, 1], &mut owner);
            let before = (owner.work, owner.bytes);
            let mut cap = limits();
            if owner_refusal {
                owner.refuse = true
            } else {
                cap.bytes = 0
            }
            let out = construct(map, cap, &mut owner);
            assert_eq!(
                out.status(),
                Err(if owner_refusal {
                    Failure::Owner(Error::Admission)
                } else {
                    Failure::Cap
                })
            );
            assert_eq!(out.charged(), Quote { work: 0, bytes: 0 });
            assert_eq!((owner.work, owner.bytes), before);
            assert_eq!(out.source().original_binding(), [23; 32]);
            assert!(out.identity().is_none());
            assert!(out.equations().is_empty());
        }
    }
    #[test]
    fn ordered_commitments_reconstruct_and_bind_actual_source_identity() {
        let mut owner = Ledger::default();
        let a = construct(genuine(3, false, &[0, 1], &mut owner), limits(), &mut owner);
        let b = construct(genuine(3, false, &[0, 1], &mut owner), limits(), &mut owner);
        let c = construct(genuine(3, true, &[0, 1], &mut owner), limits(), &mut owner);
        assert_eq!(a.status(), Ok(()));
        assert_eq!(b.status(), Ok(()));
        assert_eq!(c.status(), Ok(()));
        assert_eq!(a.identity(), b.identity());
        assert_eq!(a.equations(), b.equations());
        assert_ne!(
            a.identity().unwrap().original_map_sha256,
            c.identity().unwrap().original_map_sha256
        );
    }
}
