//! Owned finite unit-span rows over independently audited affine-domain equations.
//! Does not activate dataset dispatch or reuse raw-domain row identities.
use super::super::{sparse, Admission, CheckedMap, Error, Quote};
use super::{audit, Attempt, Identity, ORDER, SCHEMA};
use serde::{Deserialize, Serialize};
use std::time::Instant;
#[path = "boolean_identity_bqi.rs"]
mod bqi;
#[path = "matrix_rows.rs"]
mod kernel;
pub use bqi::{Audit, Limits as CheckerLimits};
pub use kernel::{Failure as MatrixFailure, Limits};
static UNIT: [sparse::Monomial; 1] = [Vec::new()];
pub const ROW_SCHEMA: &str = "checked-affine-unit-row/v1";
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Failure {
    Incomplete,
    Owner,
    Deadline,
    Overflow,
    Identity,
    Matrix(kernel::Failure),
    Checker,
}
/// Untrusted fixed-size receipt fields. Parsing these does not mint an audited
/// source. Numeric versions name exactly SCHEMA/ORDER; no string allocation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EncodingBinding {
    pub encoding_version: u32,
    pub order_version: u32,
    pub route: u8,
    pub original_map_sha256: [u8; 32],
    pub ordered_generators_sha256: [u8; 32],
    pub original_binding: [u8; 32],
    pub variables: u32,
    pub generators: u64,
    pub factor_blocks: u32,
    pub n: u32,
    pub rank: u32,
    pub origin: u64,
    pub pivot_rows_low: [u64; 32],
    pub pivot_rows_high: [u64; 29],
}
impl EncodingBinding {
    pub fn from_identity(id: &Identity) -> Self {
        let mut low = [0; 32];
        let mut high = [0; 29];
        low.copy_from_slice(&id.pivot_rows[..32]);
        high.copy_from_slice(&id.pivot_rows[32..]);
        Self {
            encoding_version: 1,
            order_version: 1,
            route: match id.route {
                super::Route::Empty => 0,
                super::Route::Affine => 1,
            },
            original_map_sha256: id.original_map_sha256,
            ordered_generators_sha256: id.ordered_generators_sha256,
            original_binding: id.original_binding,
            variables: id.variables,
            generators: id.generators as u64,
            factor_blocks: id.factor_blocks as u32,
            n: id.n,
            rank: id.rank,
            origin: id.origin,
            pivot_rows_low: low,
            pivot_rows_high: high,
        }
    }
    pub fn matches_identity(&self, id: &Identity) -> bool {
        id.schema == SCHEMA && id.order == ORDER && *self == Self::from_identity(id)
    }
}
struct Bridge<'a, T: Admission> {
    owner: &'a mut T,
    last: Option<Error>,
}
impl<T: Admission> sparse::Owner for Bridge<'_, T> {
    fn reserve(&mut self, q: sparse::Charge) -> Result<(), ()> {
        self.owner
            .reserve(Quote {
                work: q.work,
                bytes: q.bytes,
            })
            .map_err(|e| {
                self.last = Some(e);
            })
    }
    fn check(&mut self) -> Result<(), ()> {
        self.owner.check().map_err(|e| {
            self.last = Some(e);
        })
    }
}
/// No constructor from an unaudited polynomial slice or parsed binding exists.
pub struct OwnedRows {
    source: audit::Audited,
    attempt: Option<kernel::Attempt>,
    failure: Option<Failure>,
    owner_failure: Option<Error>,
}
impl OwnedRows {
    pub fn source(&self) -> &audit::Audited {
        &self.source
    }
    pub fn failure(&self) -> Option<Failure> {
        self.failure
    }
    pub fn owner_failure(&self) -> Option<Error> {
        self.owner_failure
    }
    pub fn rows(&self) -> usize {
        self.attempt.as_ref().map_or(0, |v| v.basis.len())
    }
    pub fn retained_attempt(&self) -> Option<&kernel::Attempt> {
        self.attempt.as_ref()
    }
    pub fn encoding(&self) -> Option<EncodingBinding> {
        self.source
            .source()
            .identity()
            .map(EncodingBinding::from_identity)
    }
}
pub fn reduce_unit(
    source: audit::Audited,
    limits: Limits,
    owner: &mut impl Admission,
) -> OwnedRows {
    let mut out = OwnedRows {
        source,
        attempt: None,
        failure: None,
        owner_failure: None,
    };
    if out.source.status().is_err() {
        out.failure = Some(Failure::Incomplete);
        return out;
    }
    let mut bridge = Bridge { owner, last: None };
    let source = out.source.source();
    let attempt = kernel::reduce(
        source.variables(),
        source.equations(),
        &UNIT,
        limits,
        &mut bridge,
    );
    out.failure = attempt.result.err().map(Failure::Matrix);
    out.attempt = Some(attempt);
    out.owner_failure = bridge.last;
    out
}
pub struct OwnedCertificate {
    rows: OwnedRows,
    wire: Option<kernel::Encoded>,
    audit: Option<Audit>,
    failure: Option<Failure>,
    owner_failure: Option<Error>,
}
impl OwnedCertificate {
    pub fn rows(&self) -> &OwnedRows {
        &self.rows
    }
    pub fn raw(&self) -> Option<&[u8]> {
        self.wire.as_ref().map(|w| w.raw.as_slice())
    }
    pub fn audit(&self) -> Option<&Audit> {
        self.audit.as_ref()
    }
    pub fn failure(&self) -> Option<Failure> {
        self.failure
    }
    pub fn owner_failure(&self) -> Option<Error> {
        self.owner_failure
    }
    pub fn encoding(&self) -> Option<EncodingBinding> {
        self.rows.encoding()
    }
}
fn source_bound_check(
    source: &Attempt,
    raw: &[u8],
    limits: CheckerLimits,
    deadline: Instant,
    owner: &mut impl Admission,
) -> (Option<Audit>, Option<Failure>, Option<Error>) {
    if Instant::now() >= deadline {
        return (None, Some(Failure::Deadline), None);
    }
    let Some(work) = limits.caps.iter().try_fold(0u64, |n, v| n.checked_add(*v)) else {
        return (None, Some(Failure::Overflow), None);
    };
    if let Err(e) = owner.check() {
        return (None, Some(Failure::Owner), Some(e));
    }
    if let Err(e) = owner.reserve(Quote {
        work,
        bytes: limits.caps[bqi::Resource::AllocationBytes as usize],
    }) {
        return (None, Some(Failure::Owner), Some(e));
    }
    if let Err(e) = owner.check() {
        return (None, Some(Failure::Owner), Some(e));
    }
    let audit = bqi::verify_expected(
        raw,
        source.variables(),
        source.equations(),
        limits,
        deadline,
    );
    let mut failure = if audit.result.is_err() {
        Some(Failure::Checker)
    } else {
        None
    };
    if Instant::now() >= deadline {
        failure.get_or_insert(Failure::Deadline);
    }
    let owner_failure = owner.check().err();
    if owner_failure.is_some() {
        failure.get_or_insert(Failure::Owner);
    }
    (Some(audit), failure, owner_failure)
}
fn certify_inner(
    rows: OwnedRows,
    row: usize,
    limits: Limits,
    checker: CheckerLimits,
    deadline: Instant,
    owner: &mut impl Admission,
) -> OwnedCertificate {
    let mut out = OwnedCertificate {
        rows,
        wire: None,
        audit: None,
        failure: None,
        owner_failure: None,
    };
    if let Some(f) = out.rows.failure {
        out.failure = Some(f);
        out.owner_failure = out.rows.owner_failure;
        return out;
    }
    if Instant::now() >= deadline {
        out.failure = Some(Failure::Deadline);
        return out;
    }
    let source = out.rows.source.source();
    let mut bridge = Bridge { owner, last: None };
    let encoded = kernel::encode_retained(
        source.variables(),
        source.equations(),
        &UNIT,
        out.rows.attempt.as_ref().unwrap(),
        row,
        limits,
        &mut bridge,
    );
    out.failure = encoded.result.err().map(Failure::Matrix);
    out.wire = Some(encoded);
    out.owner_failure = bridge.last;
    if out.failure.is_none() {
        let (audit, failure, error) = source_bound_check(
            source,
            &out.wire.as_ref().unwrap().raw,
            checker,
            deadline,
            bridge.owner,
        );
        out.audit = audit;
        out.failure = failure;
        out.owner_failure = error;
    }
    out
}
enum ReplaySource {
    Checked(CheckedMap),
    Constructed(Attempt),
    Audited(audit::Audited),
    Moving,
}
pub struct OwnedReplay {
    source: ReplaySource,
    expected: EncodingBinding,
    raw: Vec<u8>,
    audit: Option<Audit>,
    failure: Option<Failure>,
    owner_failure: Option<Error>,
}
impl OwnedReplay {
    pub fn source(&self) -> Option<&audit::Audited> {
        if let ReplaySource::Audited(v) = &self.source {
            Some(v)
        } else {
            None
        }
    }
    pub fn checked_map(&self) -> &CheckedMap {
        match &self.source {
            ReplaySource::Checked(v) => v,
            ReplaySource::Constructed(v) => v.source(),
            ReplaySource::Audited(v) => v.source().source(),
            ReplaySource::Moving => unreachable!("private transient move"),
        }
    }
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
    pub fn audit(&self) -> Option<&Audit> {
        self.audit.as_ref()
    }
    pub fn failure(&self) -> Option<Failure> {
        self.failure
    }
    pub fn owner_failure(&self) -> Option<Error> {
        self.owner_failure
    }
    pub fn expected(&self) -> &EncodingBinding {
        &self.expected
    }
}
/// Replay performs deterministic constructor reconstruction AND the separate
/// membership/rank/original-prefix audit. It is not native chronology replay.
/// Caller must independently authenticate this CheckedMap from original inputs.
fn replay_inner(
    map: CheckedMap,
    expansion: super::Limits,
    equivalence: audit::Limits,
    raw: Vec<u8>,
    expected: EncodingBinding,
    checker: CheckerLimits,
    deadline: Instant,
    owner: &mut impl Admission,
) -> OwnedReplay {
    let mut out = OwnedReplay {
        source: ReplaySource::Checked(map),
        expected,
        raw,
        audit: None,
        failure: None,
        owner_failure: None,
    };
    if Instant::now() >= deadline {
        out.failure = Some(Failure::Deadline);
        return out;
    }
    if let Err(e) = owner.check() {
        out.failure = Some(Failure::Owner);
        out.owner_failure = Some(e);
        return out;
    }
    let ReplaySource::Checked(map) = std::mem::replace(&mut out.source, ReplaySource::Moving)
    else {
        unreachable!()
    };
    let constructed = super::construct(map, expansion, owner);
    let failed = constructed.status().is_err();
    out.source = ReplaySource::Constructed(constructed);
    if failed {
        out.failure = Some(Failure::Incomplete);
        return out;
    }
    if Instant::now() >= deadline {
        out.failure = Some(Failure::Deadline);
        return out;
    }
    let ReplaySource::Constructed(source) =
        std::mem::replace(&mut out.source, ReplaySource::Moving)
    else {
        unreachable!()
    };
    let audited = audit::audit(source, equivalence, owner);
    let failed = audited.status().is_err();
    out.source = ReplaySource::Audited(audited);
    if failed {
        out.failure = Some(Failure::Incomplete);
        return out;
    }
    if let Err(e) = owner.reserve(Quote {
        work: 1024,
        bytes: 0,
    }) {
        out.failure = Some(Failure::Owner);
        out.owner_failure = Some(e);
        return out;
    }
    let source = out.source().unwrap().source();
    if !out.expected.matches_identity(source.identity().unwrap()) {
        out.failure = Some(Failure::Identity);
        return out;
    }
    let (audit, failure, error) = source_bound_check(source, &out.raw, checker, deadline, owner);
    out.audit = audit;
    out.failure = failure;
    out.owner_failure = error;
    out
}

// One caller deadline is enforced on every existing kernel/constructor admission,
// not just at phase boundaries. No deadline renewal or independent work allowance.
struct Timed<'a, T: Admission> {
    owner: &'a mut T,
    deadline: Instant,
    expired: bool,
    last: Option<Error>,
}
impl<T: Admission> Admission for Timed<'_, T> {
    fn check(&mut self) -> Result<(), Error> {
        if self.expired {
            return Err(Error::Admission);
        }
        if let Some(e) = self.last {
            return Err(e);
        }
        if Instant::now() >= self.deadline {
            self.expired = true;
            return Err(Error::Admission);
        }
        self.owner.check().map_err(|e| {
            self.last = Some(e);
            e
        })
    }
    fn reserve(&mut self, q: Quote) -> Result<(), Error> {
        self.check()?;
        self.owner.reserve(q).map_err(|e| {
            self.last = Some(e);
            e
        })?;
        self.check()
    }
}
pub fn certify_owned(
    rows: OwnedRows,
    row: usize,
    limits: Limits,
    checker: CheckerLimits,
    deadline: Instant,
    owner: &mut impl Admission,
) -> OwnedCertificate {
    let mut timed = Timed {
        owner,
        deadline,
        expired: false,
        last: None,
    };
    let mut result = certify_inner(rows, row, limits, checker, deadline, &mut timed);
    if timed.expired {
        result.failure.get_or_insert(Failure::Deadline);
    }
    result
}
pub fn replay_from_checked_map(
    map: CheckedMap,
    expansion: super::Limits,
    equivalence: audit::Limits,
    raw: Vec<u8>,
    expected: EncodingBinding,
    checker: CheckerLimits,
    deadline: Instant,
    owner: &mut impl Admission,
) -> OwnedReplay {
    let mut timed = Timed {
        owner,
        deadline,
        expired: false,
        last: None,
    };
    let mut result = replay_inner(
        map,
        expansion,
        equivalence,
        raw,
        expected,
        checker,
        deadline,
        &mut timed,
    );
    if timed.expired {
        result.failure.get_or_insert(Failure::Deadline);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::super::super::super::{build_original, Limits as MapLimits, Original};
    use super::*;
    use crate::binary_ecc::{F2mElement, IrreduciblePoly};
    use num_bigint::BigUint;
    struct Ledger {
        work: u64,
        bytes: u64,
        limit: u64,
        byte_limit: u64,
        deadline: Instant,
        deny: bool,
    }
    impl Ledger {
        fn producer() -> Self {
            Self {
                work: 0,
                bytes: 0,
                limit: 16_777_216,
                byte_limit: 256 * 1024 * 1024,
                deadline: Instant::now() + std::time::Duration::from_secs(60),
                deny: false,
            }
        }
        fn replay() -> Self {
            Self {
                limit: 1_000_000_000_000,
                byte_limit: 8_000_000_000,
                ..Self::producer()
            }
        }
    }
    impl Admission for Ledger {
        fn check(&mut self) -> Result<(), Error> {
            if self.deny || Instant::now() >= self.deadline {
                Err(Error::Admission)
            } else {
                Ok(())
            }
        }
        fn reserve(&mut self, q: Quote) -> Result<(), Error> {
            self.check()?;
            let w = self.work.checked_add(q.work).ok_or(Error::Admission)?;
            let b = self.bytes.checked_add(q.bytes).ok_or(Error::Admission)?;
            if w > self.limit || b > self.byte_limit {
                return Err(Error::Admission);
            }
            self.work = w;
            self.bytes = b;
            Ok(())
        }
    }
    fn expansion() -> super::super::Limits {
        super::super::Limits {
            variables: 4096,
            generators: 4096,
            terms: 4096,
            degree: 64,
            work: 4_000_000,
            bytes: 8_000_000,
        }
    }
    fn limits() -> Limits {
        Limits {
            variables: 4096,
            generators: 4096,
            window: 1,
            candidates: 4096,
            basis: 4096,
            terms: 4096,
            degree: 64,
            proof: 4096,
            wire: 65536,
            work: 4_000_000,
            bytes: 8_000_000,
        }
    }
    fn checker() -> CheckerLimits {
        CheckerLimits {
            variables: 4096,
            generators: 4096,
            polynomial_terms: 4096,
            proof_terms: 4096,
            accumulator_entries: 4096,
            caps: [
                65536, 262144, 262144, 262144, 262144, 65536, 1024, 1048576, 262144,
            ],
        }
    }
    fn element(x: u64) -> F2mElement {
        F2mElement::from_bit_positions(
            &(0..23).filter(|b| x & (1u64 << b) != 0).collect::<Vec<_>>(),
            23,
        )
    }
    fn map(twist: bool, owner: &mut Ledger) -> CheckedMap {
        let irr = IrreduciblePoly {
            degree: 23,
            low_terms: vec![0, 5],
        };
        let a = element(1887600);
        let b = element(570608);
        let target = element(7715066);
        let codes = vec![BigUint::from(4833900u64), BigUint::from(6976212u64)];
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
            MapLimits {
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
    fn source(twist: bool, owner: &mut Ledger) -> audit::Audited {
        let map = map(twist, owner);
        let attempt = super::super::construct(map, expansion(), owner);
        assert_eq!(attempt.status(), Ok(()));
        let result = audit::audit(attempt, expansion(), owner);
        assert_eq!(result.status(), Ok(()));
        result
    }
    fn certificate(twist: bool, owner: &mut Ledger) -> OwnedCertificate {
        let audited = source(twist, owner);
        let rows = reduce_unit(audited, limits(), owner);
        assert_eq!(rows.failure(), None);
        // A positive must actually combine generators, not merely echo an input row.
        let row = rows
            .retained_attempt()
            .unwrap()
            .basis
            .iter()
            .position(|r| r.proof.len() > 1 && !r.polynomial.is_empty())
            .expect("nontrivial actual affine-source row required");
        let result = certify_owned(rows, row, limits(), checker(), owner.deadline, owner);
        assert_eq!(result.failure(), None);
        result
    }
    #[test]
    fn actual_n23_both_twist_audited_rows_require_nontrivial_certificate_and_replay() {
        for twist in [false, true] {
            let mut producer = Ledger::producer();
            let certificate = certificate(twist, &mut producer);
            assert!(certificate.audit().unwrap().result.is_ok());
            assert!(!certificate.audit().unwrap().source_authenticated);
            assert!(!certificate.audit().unwrap().native_operation_witness);
            let binding = certificate.encoding().unwrap();
            assert_eq!(binding.generators, 158);
            assert_eq!(binding.variables, 115);
            let mut replay = Ledger::replay();
            let map = map(twist, &mut replay);
            let deadline = replay.deadline;
            let checked = replay_from_checked_map(
                map,
                expansion(),
                expansion(),
                certificate.raw().unwrap().to_vec(),
                binding,
                checker(),
                deadline,
                &mut replay,
            );
            assert_eq!(checked.failure(), None);
            assert!(checked.source().unwrap().status().is_ok());
            assert!(checked.audit().unwrap().result.is_ok());
        }
    }
    #[test]
    fn independent_audit_refusal_cannot_enter_row_reduction() {
        let mut owner = Ledger::producer();
        let map = map(false, &mut owner);
        let attempt = super::super::construct(map, expansion(), &mut owner);
        assert_eq!(attempt.status(), Ok(()));
        let mut cap = expansion();
        cap.work = 0;
        let failed = audit::audit(attempt, cap, &mut owner);
        assert!(failed.status().is_err());
        let before = (owner.work, owner.bytes);
        let rows = reduce_unit(failed, limits(), &mut owner);
        assert_eq!(rows.failure(), Some(Failure::Incomplete));
        assert!(rows.retained_attempt().is_none());
        assert_eq!((owner.work, owner.bytes), before);
        assert_eq!(rows.source().source().variables(), 115);
    }
    #[test]
    fn zero_wire_cap_keeps_actual_audited_rows_and_source_identity() {
        let mut owner = Ledger::producer();
        let audited = source(false, &mut owner);
        let identity = audited.source().identity().unwrap().clone();
        let rows = reduce_unit(audited, limits(), &mut owner);
        assert_eq!(rows.failure(), None);
        assert!(rows.rows() > 0);
        let mut cap = limits();
        cap.wire = 0;
        let deadline = owner.deadline;
        let out = certify_owned(rows, 0, cap, checker(), deadline, &mut owner);
        assert_eq!(out.failure(), Some(Failure::Matrix(MatrixFailure::Cap)));
        assert!(out.rows().rows() > 0);
        assert_eq!(out.rows().source().source().identity(), Some(&identity));
        assert!(out.audit().is_none());
        assert!(out.raw().is_some());
    }
    #[test]
    fn wrong_encoding_and_valid_foreign_wire_are_rejected_with_owned_inputs() {
        let mut producer = Ledger::producer();
        let cert = certificate(false, &mut producer);
        let binding = cert.encoding().unwrap();
        let mut wrong = binding;
        wrong.order_version = 2;
        let mut replay = Ledger::replay();
        let original = map(false, &mut replay);
        let deadline = replay.deadline;
        let denied = replay_from_checked_map(
            original,
            expansion(),
            expansion(),
            cert.raw().unwrap().to_vec(),
            wrong,
            checker(),
            deadline,
            &mut replay,
        );
        assert_eq!(denied.failure(), Some(Failure::Identity));
        assert!(denied.audit().is_none());
        assert!(!denied.raw().is_empty());
        assert!(denied.source().is_some());
        let foreign = b"BQI1 1 1 1 1 0 1 1 0 1 0 0\n".to_vec();
        assert!(bqi::verify_untrusted(&foreign, checker(), deadline)
            .result
            .is_ok());
        let mut replay = Ledger::replay();
        let original = map(false, &mut replay);
        let deadline = replay.deadline;
        let denied = replay_from_checked_map(
            original,
            expansion(),
            expansion(),
            foreign,
            binding,
            checker(),
            deadline,
            &mut replay,
        );
        assert_eq!(denied.failure(), Some(Failure::Checker));
        assert!(denied.audit().unwrap().result.is_err());
        assert!(denied.source().unwrap().status().is_ok());
    }
    #[test]
    fn replay_expired_deadline_preserves_checked_map_wire_and_original_owner_charges() {
        let mut producer = Ledger::producer();
        let cert = certificate(false, &mut producer);
        let mut replay = Ledger::replay();
        let original = map(false, &mut replay);
        let before = (replay.work, replay.bytes);
        let raw = cert.raw().unwrap().to_vec();
        let pointer = raw.as_ptr();
        let denied = replay_from_checked_map(
            original,
            expansion(),
            expansion(),
            raw,
            cert.encoding().unwrap(),
            checker(),
            Instant::now(),
            &mut replay,
        );
        assert_eq!(denied.failure(), Some(Failure::Deadline));
        assert_eq!(denied.raw().as_ptr(), pointer);
        assert_eq!(denied.checked_map().variables(), 115);
        assert!(denied.source().is_none());
        assert_eq!((replay.work, replay.bytes), before);
    }
    #[test]
    fn fixed_binding_closed_wire_roundtrip_is_metadata_only() {
        let mut owner = Ledger::producer();
        let audited = source(false, &mut owner);
        let identity = audited.source().identity().unwrap();
        let binding = EncodingBinding::from_identity(identity);
        let bytes = serde_json::to_vec(&binding).unwrap();
        let decoded: EncodingBinding = serde_json::from_slice(&bytes).unwrap();
        assert!(decoded.matches_identity(identity));
        let mut value = serde_json::to_value(binding).unwrap();
        value["extra"] = true.into();
        assert!(serde_json::from_value::<EncodingBinding>(value).is_err());
        let mut mutated = binding;
        mutated.pivot_rows_high[28] ^= 1;
        assert!(!mutated.matches_identity(identity));
    }
}
