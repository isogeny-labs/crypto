//! Versioned producer checkpoint. Consistency verification is not independent
//! measurement of instructions; old records without a checkpoint are incomplete.
use serde::{Deserialize, Serialize};
pub(super) const SCHEMA: &str = "ml-dataset.sat-owner-ledger/v1";
pub(super) const MAX_WIRE_BYTES: u64 = 1536;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(super) enum Phase {
    Setup,
    Construction,
    Constructed,
    Matrix,
    MatrixComplete,
    Extraction,
    Complete,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(super) enum Stop {
    InvalidLimits,
    Overflow,
    Deadline,
    Phase,
    Nonmonotone,
    Work,
    Bytes,
    Trace,
    Additions,
    Observer,
    NativeFailure,
    Allocation,
    Serialization,
    Events,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Checkpoint {
    pub schema: String,
    pub original_binding: [u8; 32],
    pub pattern: u32,
    pub checkpoint_index: u64,
    pub previous_checkpoint_sequence: u64,
    pub previous_checkpoint_sha256: String,
    pub covered_sequence: u64,
    pub covered_sha256: String,
    /// SHA256 of canonical serialized original LaunchContext, not a fresh clock.
    pub launch_sha256: String,
    pub owner_ordinal: u64,
    /// work,memory,trace,addition caps from the one original Budget.
    pub limits: [u64; 4],
    /// native_work,native_bytes,checking_work,checking_performed_work,
    /// adapter_bytes,trace_bytes,additions. Cumulative, never phase deltas.
    pub totals: [u64; 7],
    pub phase: Phase,
    pub phase_native: [u64; 2],
    pub phase_limits: [u64; 2],
    pub first_stop: Option<Stop>,
}
/// Actual capture receipt, not self-reported checkpoint fields.
pub(super) struct Receipt<'a> {
    pub sequence: u64,
    pub hash: &'a str,
}
pub(super) struct Bound<'a> {
    pub original_binding: [u8; 32],
    pub pattern: u32,
    pub launch_sha256: &'a str,
    pub limits: [u64; 4],
    pub owner_ordinal: u64,
    pub covered: Receipt<'a>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Error {
    Identity,
    Range,
    Cap,
    Rollback,
    Stopped,
}
pub(super) struct Prior<'a> {
    pub checkpoint: &'a Checkpoint,
    pub receipt: Receipt<'a>,
}
fn digest(s: &str) -> bool {
    s.len() == 64
        && s.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
fn phase(p: Phase) -> u8 {
    match p {
        Phase::Setup => 0,
        Phase::Construction => 1,
        Phase::Constructed => 2,
        Phase::Matrix => 3,
        Phase::MatrixComplete => 4,
        Phase::Extraction => 5,
        Phase::Complete => 6,
    }
}
/// Call only after the capture visitor authenticates the exact raw event and
/// the wrapper independently authenticates bound original launch and marker.
pub(super) fn verify(
    c: &Checkpoint,
    b: &Bound<'_>,
    actual: Receipt<'_>,
    previous: Option<Prior<'_>>,
) -> Result<(), Error> {
    if c.schema != SCHEMA
        || c.original_binding != b.original_binding
        || c.pattern != b.pattern
        || c.launch_sha256 != b.launch_sha256
        || c.owner_ordinal != b.owner_ordinal
        || c.limits != b.limits
        || !digest(&c.launch_sha256)
        || !digest(&c.covered_sha256)
        || !digest(&c.previous_checkpoint_sha256)
        || !digest(actual.hash)
    {
        return Err(Error::Identity);
    }
    if c.covered_sequence != b.covered.sequence
        || c.covered_sha256 != b.covered.hash
        || actual.sequence != c.covered_sequence.checked_add(1).ok_or(Error::Range)?
    {
        return Err(Error::Range);
    }
    let t = c.totals;
    let l = c.limits;
    if t[0].checked_add(t[2]).filter(|n| *n <= l[0]).is_none()
        || t[1].checked_add(t[4]).filter(|n| *n <= l[1]).is_none()
        || t[5] > l[2]
        || t[6] > l[3]
        || t[3] > t[2]
        || c.phase_native[0] > t[0]
        || c.phase_native[1] > t[1]
        || c.phase_limits[0] > l[0]
        || c.phase_limits[1] > l[1]
        || c.phase_native[0] > c.phase_limits[0]
        || c.phase_native[1] > c.phase_limits[1]
    {
        return Err(Error::Cap);
    }
    match previous {
        None => {
            if c.checkpoint_index != 0
                || c.previous_checkpoint_sequence != 0
                || !c.previous_checkpoint_sha256.bytes().all(|b| b == b'0')
            {
                return Err(Error::Range);
            }
        }
        Some(p) => {
            let old = p.checkpoint;
            if c.checkpoint_index != old.checkpoint_index.checked_add(1).ok_or(Error::Range)?
                || c.previous_checkpoint_sequence != p.receipt.sequence
                || c.previous_checkpoint_sha256 != p.receipt.hash
                || old.original_binding != c.original_binding
                || old.launch_sha256 != c.launch_sha256
                || old.limits != c.limits
                || old.owner_ordinal != c.owner_ordinal
            {
                return Err(Error::Identity);
            }
            if c.totals.iter().zip(old.totals).any(|(new, old)| *new < old)
                || phase(c.phase) < phase(old.phase)
                || (c.phase == old.phase
                    && c.phase_native
                        .iter()
                        .zip(old.phase_native)
                        .any(|(new, old)| *new < old))
            {
                return Err(Error::Rollback);
            }
            if old.first_stop.is_some() && c.first_stop != old.first_stop {
                return Err(Error::Stopped);
            }
        }
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    fn checkpoint() -> Checkpoint {
        Checkpoint {
            schema: SCHEMA.into(),
            original_binding: [7; 32],
            pattern: 0,
            checkpoint_index: 0,
            previous_checkpoint_sequence: 0,
            previous_checkpoint_sha256: "0".repeat(64),
            covered_sequence: 3,
            covered_sha256: "a".repeat(64),
            launch_sha256: "b".repeat(64),
            owner_ordinal: 0,
            limits: [1000; 4],
            totals: [20, 20, 100, 90, 30, 40, 2],
            phase: Phase::Setup,
            phase_native: [20, 20],
            phase_limits: [1000, 1000],
            first_stop: None,
        }
    }
    #[test]
    fn joined_monotone_and_no_inferred_legacy_success() {
        let c = checkpoint();
        let b = Bound {
            original_binding: c.original_binding,
            pattern: 0,
            launch_sha256: &c.launch_sha256,
            limits: c.limits,
            owner_ordinal: 0,
            covered: Receipt {
                sequence: 3,
                hash: &c.covered_sha256,
            },
        };
        verify(
            &c,
            &b,
            Receipt {
                sequence: 4,
                hash: &"c".repeat(64),
            },
            None,
        )
        .unwrap();
        for index in 0..7 {
            let mut changed = c.clone();
            changed.totals[index] = u64::MAX;
            assert!(verify(
                &changed,
                &b,
                Receipt {
                    sequence: 4,
                    hash: &"c".repeat(64)
                },
                None
            )
            .is_err());
        }
        let mut second = c.clone();
        second.checkpoint_index = 1;
        second.previous_checkpoint_sequence = 4;
        second.previous_checkpoint_sha256 = "c".repeat(64);
        second.covered_sequence = 6;
        second.covered_sha256 = "d".repeat(64);
        let b2 = Bound {
            covered: Receipt {
                sequence: 6,
                hash: &second.covered_sha256,
            },
            ..b
        };
        verify(
            &second,
            &b2,
            Receipt {
                sequence: 7,
                hash: &"e".repeat(64),
            },
            Some(Prior {
                checkpoint: &c,
                receipt: Receipt {
                    sequence: 4,
                    hash: &second.previous_checkpoint_sha256,
                },
            }),
        )
        .unwrap();
        let mut bad = second.clone();
        bad.totals[2] -= 1;
        assert_eq!(
            verify(
                &bad,
                &b2,
                Receipt {
                    sequence: 7,
                    hash: &"e".repeat(64)
                },
                Some(Prior {
                    checkpoint: &c,
                    receipt: Receipt {
                        sequence: 4,
                        hash: &second.previous_checkpoint_sha256
                    }
                })
            ),
            Err(Error::Rollback)
        );
    }
    #[test]
    fn actual_maximum_capture_envelope_fits_prepayment() {
        use crate::ml_dataset::capture;
        let mut c = checkpoint();
        c.original_binding = [255; 32];
        c.pattern = u32::MAX;
        c.checkpoint_index = u64::MAX;
        c.previous_checkpoint_sequence = u64::MAX;
        c.covered_sequence = u64::MAX;
        c.owner_ordinal = u64::MAX;
        c.limits = [u64::MAX; 4];
        c.totals = [u64::MAX; 7];
        c.phase = Phase::MatrixComplete;
        c.phase_native = [u64::MAX; 2];
        c.phase_limits = [u64::MAX; 2];
        c.first_stop = Some(Stop::Serialization);
        let wire =
            capture::maximum_sequence_typed_wire(capture::TypedEventKind::BinarySatInitial, &c);
        assert!(
            wire.len() as u64 <= MAX_WIRE_BYTES,
            "actual maximum envelope exceeds prepaid cap: {}",
            wire.len()
        );
    }
}
