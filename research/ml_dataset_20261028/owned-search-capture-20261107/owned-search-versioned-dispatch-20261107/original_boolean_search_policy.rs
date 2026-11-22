//! Finite owner entry for the explicit v5 matrix-continuation request.
//! The existing affine worker's finite ceilings are a conservative envelope,
//! not an assertion that this matrix continuation policy was already approved.
use super::*;
use crate::ml_dataset::types::CaseConfig;

/// Explicit finite policy bound by the v5 request. No v1-v4 channel implies it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::ml_dataset::solvers) enum ProposedPolicy {
    ConservativeAffineEnvelopeV1,
}

// Exact envelope from affine_boolean_row_worker.rs::caps. New scheduler count
// dimensions are restricted to its 4096-candidate ceiling; batch rows retain
// its window=1 ceiling. These restrictions never replenish the original owner.
const COUNT_MAX: usize = 4096;
const DEGREE_MAX: usize = 64;
const WORK_MAX: u64 = 4_000_000;
const BYTES_MAX: u64 = 8_000_000;
const WIRE_MAX: usize = 65_536;
const CHECKER_CAPS: [u64; 9] = [
    65_536, 262_144, 262_144, 262_144, 262_144, 65_536, 1024, 1_048_576, 262_144,
];

fn eligible(config: &CaseConfig, policy: ProposedPolicy) -> bool {
    super::super::owned_search_worker::selection(config).is_some_and(|(_,chosen)|chosen==policy)
}
/// Called by the consuming entry only after its existing setup reservation and
/// actual sink/attempt checks, before it moves the original fields. The caller
/// performs a final Budget::check after this bounded, allocation-free factory.
pub(super) fn for_owner(
    owner: &OriginalOwner<'_>,
    policy: ProposedPolicy,
) -> Result<Caps, Stop> {
    if !eligible(owner.original.config, policy)
        || super::super::owned_search_worker::selection(owner.original.config).map(|(pattern,_)|pattern)!=owner.pattern {
        return Err(Stop::InvalidLimits);
    }
    from_budget(&owner.budget, owner.original.config.degree_bound)
}

fn from_budget(budget: &Budget, requested_degree: u32) -> Result<Caps, Stop> {
    if let Some(stop) = budget.stopped {
        return Err(stop);
    }
    if budget.phase != super::super::super::Phase::Setup {
        return Err(Stop::Phase);
    }
    if requested_degree == 0 {
        return Err(Stop::InvalidLimits);
    }
    let work = budget
        .native_work_limit
        .checked_sub(budget.native.work)
        .and_then(|v| v.checked_sub(budget.checking_work))
        .ok_or(Stop::Overflow)?
        .min(WORK_MAX);
    let bytes = budget
        .memory_limit
        .checked_sub(budget.native.bytes)
        .and_then(|v| v.checked_sub(budget.adapter_bytes))
        .ok_or(Stop::Overflow)?
        .min(BYTES_MAX);
    if work == 0 {
        return Err(Stop::Work);
    }
    if bytes == 0 {
        return Err(Stop::Bytes);
    }
    let degree = usize::try_from(requested_degree)
        .map_err(|_| Stop::Overflow)?
        .min(DEGREE_MAX);
    let count = COUNT_MAX.min(usize::try_from(work).map_err(|_| Stop::Overflow)?);
    // Source construction has the original stricter quarter-memory ceiling.
    // The continuation adapter re-clamps against actual begin_native results
    // after construction; this early restriction is never an admission token.
    let construction_bytes = bytes.min(budget.memory_limit / 4);
    if construction_bytes == 0 {
        return Err(Stop::Bytes);
    }
    let mut checker_caps = CHECKER_CAPS;
    for (index, cap) in checker_caps.iter_mut().enumerate() {
        *cap = (*cap).min(if index == bqi::Resource::AllocationBytes as usize {
            bytes
        } else {
            work
        });
    }
    let wire = WIRE_MAX.min(usize::try_from(bytes).map_err(|_| Stop::Overflow)?);
    // The proof checker prepays its whole envelope on the same owner. Its sum
    // may still refuse against remaining shared work; no cap is raised to fit.
    let checker = bqi::Limits {
        variables: COUNT_MAX as u32,
        generators: count as u32,
        polynomial_terms: count as u32,
        proof_terms: count as u32,
        accumulator_entries: count,
        caps: checker_caps,
    };
    Ok(Caps {
        expansion: equations::Limits {
            variables: COUNT_MAX as u32,
            generators: count,
            terms: count,
            degree,
            work,
            bytes: construction_bytes,
        },
        continuation: continuation::Limits {
            partitions: count,
            work,
            bytes,
            search: SearchLimits {
                native: scheduler::Limits {
                    batch_rows: 1,
                    batch_ops: count,
                    batches: count,
                    variables: COUNT_MAX as u32,
                    generators: count,
                    basis: count,
                    tasks: count,
                    terms: count,
                    proof: count,
                    degree,
                    work,
                    bytes,
                },
                nodes: count,
            },
            audit: check::Limits {
                nodes: count,
                work,
                bytes,
                certificate: bridge::Limits {
                    variables: COUNT_MAX as u32,
                    generators: count,
                    terms: count,
                    degree,
                    proof: count,
                    wire,
                    work,
                    bytes,
                },
                checker,
            },
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn budget() -> Budget {
        let now = Instant::now();
        Budget::new(100_000, 64 * 1024 * 1024, 1024, now + Duration::from_secs(30), now)
            .unwrap()
    }

    #[test]
    fn inherited_envelope_caps_every_private_limit_without_changing_owner() {
        let b = budget();
        let before = (b.ownership_id, b.deadline, b.construction_deadline, b.phase,
            b.native, b.phase_native, b.checking_work, b.adapter_bytes, b.additions);
        let c = from_budget(&b, 100).unwrap();
        assert_eq!(c.expansion.work, WORK_MAX);
        assert_eq!(c.expansion.bytes, BYTES_MAX);
        assert_eq!(c.expansion.degree, DEGREE_MAX);
        assert_eq!(c.continuation.partitions, COUNT_MAX);
        assert_eq!(c.continuation.search.nodes, COUNT_MAX);
        assert_eq!(c.continuation.search.native.batch_rows, 1);
        assert_eq!(c.continuation.search.native.batch_ops, COUNT_MAX);
        assert_eq!(c.continuation.search.native.batches, COUNT_MAX);
        assert_eq!(c.continuation.audit.certificate.wire, WIRE_MAX);
        assert_eq!(c.continuation.audit.checker.caps, CHECKER_CAPS);
        assert_eq!(before, (b.ownership_id, b.deadline, b.construction_deadline, b.phase,
            b.native, b.phase_native, b.checking_work, b.adapter_bytes, b.additions));
    }

    #[test]
    fn remaining_original_allowance_and_requested_degree_only_reduce_caps() {
        let mut b = budget();
        b.checking_work = b.native_work_limit - 100;
        b.adapter_bytes = b.memory_limit - 200;
        let c = from_budget(&b, 2).unwrap();
        assert_eq!(c.expansion.work, 100);
        assert_eq!(c.expansion.bytes, 200);
        assert_eq!(c.expansion.degree, 2);
        assert_eq!(c.continuation.work, 100);
        assert_eq!(c.continuation.bytes, 200);
        assert_eq!(c.continuation.partitions, 100);
        assert_eq!(c.continuation.audit.certificate.wire, 200);
        for (index, cap) in c.continuation.audit.checker.caps.into_iter().enumerate() {
            assert!(cap <= CHECKER_CAPS[index]);
            assert!(cap <= if index == bqi::Resource::AllocationBytes as usize {200} else {100});
        }
    }

    #[test]
    fn original_quarter_memory_ceiling_is_preserved_before_construction() {
        let now = Instant::now();
        let b = Budget::new(100_000, 4000, 1024, now + Duration::from_secs(30), now)
            .unwrap();
        let c = from_budget(&b, 3).unwrap();
        assert_eq!(c.expansion.bytes, 1000);
        assert_eq!(c.continuation.bytes, 4000);
    }

    #[test]
    fn stopped_wrong_phase_and_empty_allowance_fail_without_mutation() {
        let mut b = budget();
        b.stopped = Some(Stop::Deadline);
        assert!(matches!(from_budget(&b, 3), Err(Stop::Deadline)));
        b.stopped = None;
        b.phase = super::super::super::super::Phase::Matrix;
        assert!(matches!(from_budget(&b, 3), Err(Stop::Phase)));
        b.phase = super::super::super::super::Phase::Setup;
        b.checking_work = b.native_work_limit;
        assert!(matches!(from_budget(&b, 3), Err(Stop::Work)));
        b.checking_work = 0;
        b.adapter_bytes = b.memory_limit;
        assert!(matches!(from_budget(&b, 3), Err(Stop::Bytes)));
        assert!(matches!(from_budget(&b, 0), Err(Stop::InvalidLimits)));
    }

}
