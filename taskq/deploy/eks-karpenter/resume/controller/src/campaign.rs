//! Conservative campaign admission charges, never measured worker-time costs.
//! Existing allocations are not refunded, including failed or ambiguous ones.
use crate::contract::{digest, hex};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

const CAP: usize = 256 * 1024;
const MAX_RESERVATIONS: usize = 256;
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Policy {
    pub schema: String,
    pub authorization_sha256: String,
    pub cumulative_ceiling_seconds: u64,
    pub absolute_deadline_unix_seconds: u64,
    pub latest_scientific_stop_unix_seconds: u64,
    pub allocation_unit: String,
    pub route_spec_sha256: Vec<String>,
    pub prior_charge_seconds: u64,
    pub reconciliation_receipt_sha256: String,
    pub owner_enforcement_review_sha256: String,
}
impl Policy {
    pub fn validate(&self) -> Result<(), String> {
        if self.schema != "isolab.worker-admission-policy/v1"
            || self.cumulative_ceiling_seconds != 25_200
            || self.absolute_deadline_unix_seconds != 1_793_398_120
            || self.latest_scientific_stop_unix_seconds > 1_793_396_320
            || self.latest_scientific_stop_unix_seconds == 0
            || self.allocation_unit != "conservative_worker_allocation_seconds"
            || self.authorization_sha256
                != "201df07743540ac247e2ae0ec79c82c7b284aa16dd07d468924c87cf2a4b7a2b"
            || !hex(&self.reconciliation_receipt_sha256, 64)
            || !hex(&self.owner_enforcement_review_sha256, 64)
            || self.prior_charge_seconds > self.cumulative_ceiling_seconds
            || self.route_spec_sha256.len() != 8
            || self.route_spec_sha256.iter().any(|s| !hex(s, 64))
            || self.route_spec_sha256.iter().collect::<BTreeSet<_>>().len() != 8
        {
            return Err("invalid explicit campaign policy or original authorization".into());
        }
        Ok(())
    }
    pub fn hash(&self) -> Result<String, String> {
        self.validate()?;
        Ok(digest(
            &serde_json::to_vec(self).map_err(|e| e.to_string())?,
        ))
    }
    pub fn name(&self) -> String {
        format!("worker-campaign-{}", self.authorization_sha256)
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Binding {
    pub policy: Policy,
    /// Existing campaign ConfigMap UID; replacement must never reset balances.
    pub ledger_uid: String,
    pub computation_ceiling_seconds: u64,
    pub fresh_ceiling_seconds: u64,
    /// Explicit reviewed tail allowance; not a measured shutdown duration.
    pub shutdown_reserve_seconds: u64,
}
impl Binding {
    pub fn validate(&self, spec_sha256: &str, request_deadline: u64) -> Result<(), String> {
        self.policy.validate()?;
        if self.ledger_uid.is_empty()
            || self.ledger_uid.len() > 128
            || !self
                .ledger_uid
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || c == b'-')
        {
            return Err("explicit existing campaign ledger UID required".into());
        }
        if !self
            .policy
            .route_spec_sha256
            .iter()
            .any(|h| h == spec_sha256)
            || request_deadline > self.policy.latest_scientific_stop_unix_seconds
            || self.shutdown_reserve_seconds < 60
            || [self.computation_ceiling_seconds, self.fresh_ceiling_seconds]
                .iter()
                .any(|n| {
                    *n <= self.shutdown_reserve_seconds
                        || *n > self.policy.cumulative_ceiling_seconds
                })
        {
            return Err("request outside campaign route/deadline/allocation envelope".into());
        }
        Ok(())
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Epoch {
    Computation,
    Fresh,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Reservation {
    pub request_sha256: String,
    pub attempt: u32,
    pub epoch: Epoch,
    pub policy_sha256: String,
    pub admitted_unix_seconds: u64,
    pub deadline_unix_seconds: u64,
    pub ceiling_seconds: u64,
    pub shutdown_reserve_seconds: u64,
}
impl Reservation {
    pub fn remaining_payload_seconds(&self, now: u64) -> Result<u64, String> {
        if now < self.admitted_unix_seconds {
            return Err("campaign clock moved backwards".into());
        }
        self.deadline_unix_seconds
            .checked_sub(now)
            .filter(|n| *n > 0)
            .ok_or_else(|| "campaign reservation payload cutoff reached; charge retained".into())
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ledger {
    pub schema: String,
    pub policy: Policy,
    pub reservations: Vec<Reservation>,
}
pub struct AdmissionWindow {
    pub request_deadline: u64,
    pub now: u64,
}
impl Ledger {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() > CAP {
            return Err("campaign ledger byte cap".into());
        }
        let ledger: Self = serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
        ledger.validate()?;
        Ok(ledger)
    }
    pub fn validate(&self) -> Result<(), String> {
        self.policy.validate()?;
        if self.schema != "isolab.worker-admission-ledger/v1"
            || self.reservations.len() > MAX_RESERVATIONS
        {
            return Err("invalid campaign ledger schema/count".into());
        }
        let mut keys = BTreeSet::new();
        let hash = self.policy.hash()?;
        for r in &self.reservations {
            if !hex(&r.request_sha256, 64)
                || r.policy_sha256 != hash
                || !keys.insert((
                    r.request_sha256.clone(),
                    r.attempt,
                    match r.epoch {
                        Epoch::Computation => 0,
                        Epoch::Fresh => 1,
                    },
                ))
                || r.shutdown_reserve_seconds < 60
                || r.ceiling_seconds <= r.shutdown_reserve_seconds
                || r.deadline_unix_seconds <= r.admitted_unix_seconds
                || r.deadline_unix_seconds > self.policy.latest_scientific_stop_unix_seconds
                || r.deadline_unix_seconds
                    .checked_sub(r.admitted_unix_seconds)
                    .and_then(|n| n.checked_add(r.shutdown_reserve_seconds))
                    .is_none_or(|n| n > r.ceiling_seconds)
            {
                return Err("invalid or duplicate campaign reservation".into());
            }
        }
        self.charged_seconds()?;
        Ok(())
    }
    pub fn charged_seconds(&self) -> Result<u64, String> {
        let total =
            self.reservations
                .iter()
                .try_fold(self.policy.prior_charge_seconds, |sum, r| {
                    sum.checked_add(r.ceiling_seconds)
                        .ok_or("campaign charge overflow")
                })?;
        if total > self.policy.cumulative_ceiling_seconds {
            return Err("campaign cumulative ceiling exhausted".into());
        }
        Ok(total)
    }
    pub fn reserve(
        &mut self,
        binding: &Binding,
        spec_sha: &str,
        request_sha: &str,
        attempt: u32,
        epoch: Epoch,
        window: AdmissionWindow,
    ) -> Result<Reservation, String> {
        let AdmissionWindow {
            request_deadline,
            now,
        } = window;
        self.validate()?;
        binding.validate(spec_sha, request_deadline)?;
        if self.policy != binding.policy || !hex(request_sha, 64) {
            return Err("campaign policy/request mismatch".into());
        }
        if now >= self.policy.absolute_deadline_unix_seconds || now >= request_deadline {
            return Err("campaign original/request deadline expired".into());
        }
        let ceiling = match epoch {
            Epoch::Computation => binding.computation_ceiling_seconds,
            Epoch::Fresh => binding.fresh_ceiling_seconds,
        };
        if let Some(old) = self
            .reservations
            .iter()
            .find(|r| r.request_sha256 == request_sha && r.attempt == attempt && r.epoch == epoch)
        {
            if old.ceiling_seconds != ceiling
                || old.shutdown_reserve_seconds != binding.shutdown_reserve_seconds
            {
                return Err("campaign reservation cannot be rebound".into());
            }
            old.remaining_payload_seconds(now)?;
            return Ok(old.clone());
        }
        if self.reservations.len() >= MAX_RESERVATIONS {
            return Err("campaign reservation count cap".into());
        }
        let deadline = now
            .checked_add(ceiling - binding.shutdown_reserve_seconds)
            .ok_or("campaign cutoff overflow")?
            .min(request_deadline);
        let next = Reservation {
            request_sha256: request_sha.into(),
            attempt,
            epoch,
            policy_sha256: self.policy.hash()?,
            admitted_unix_seconds: now,
            deadline_unix_seconds: deadline,
            ceiling_seconds: ceiling,
            shutdown_reserve_seconds: binding.shutdown_reserve_seconds,
        };
        // Validate a copy so a failed admission cannot mutate the caller's state.
        let mut changed = self.clone();
        changed.reservations.push(next.clone());
        changed.validate()?;
        *self = changed;
        Ok(next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> (Ledger, Binding) {
        let policy = Policy {
            schema: "isolab.worker-admission-policy/v1".into(),
            authorization_sha256:
                "201df07743540ac247e2ae0ec79c82c7b284aa16dd07d468924c87cf2a4b7a2b".into(),
            cumulative_ceiling_seconds: 25200,
            absolute_deadline_unix_seconds: 1793398120,
            latest_scientific_stop_unix_seconds: 1793396320,
            allocation_unit: "conservative_worker_allocation_seconds".into(),
            route_spec_sha256: (0..8).map(|n| format!("{n:064x}")).collect(),
            prior_charge_seconds: 100,
            reconciliation_receipt_sha256: "a".repeat(64),
            owner_enforcement_review_sha256: "b".repeat(64),
        };
        (
            Ledger {
                schema: "isolab.worker-admission-ledger/v1".into(),
                policy: policy.clone(),
                reservations: vec![],
            },
            Binding {
                policy,
                ledger_uid: "fixture-ledger-uid".into(),
                computation_ceiling_seconds: 1000,
                fresh_ceiling_seconds: 1000,
                shutdown_reserve_seconds: 60,
            },
        )
    }
    #[test]
    fn eight_routes_share_balance_and_restart_does_not_refund() {
        let (mut l, b) = fixture();
        let now = 1793390000;
        for i in 0..8 {
            let sha = format!("{i:064x}");
            l.reserve(
                &b,
                &sha,
                &sha,
                0,
                Epoch::Computation,
                AdmissionWindow {
                    request_deadline: 1793396000,
                    now,
                },
            )
            .unwrap();
        }
        assert_eq!(l.charged_seconds().unwrap(), 8100);
        let raw = serde_json::to_vec(&l).unwrap();
        let mut restored = Ledger::from_bytes(&raw).unwrap();
        let old = restored
            .reserve(
                &b,
                &format!("{:064x}", 0),
                &format!("{:064x}", 0),
                0,
                Epoch::Computation,
                AdmissionWindow {
                    request_deadline: 1793396000,
                    now: now + 1,
                },
            )
            .unwrap();
        assert_eq!(old.admitted_unix_seconds, now);
        assert_eq!(restored, l);
        assert!(restored
            .reserve(
                &b,
                &format!("{:064x}", 0),
                &format!("{:064x}", 0),
                0,
                Epoch::Computation,
                AdmissionWindow {
                    request_deadline: 1793396000,
                    now: now + 1000
                }
            )
            .is_err());
        assert_eq!(restored, l);
    }
    #[test]
    fn ambiguous_restore_and_fresh_are_separate_permanent_charges() {
        let (mut l, b) = fixture();
        let h = format!("{:064x}", 0);
        for (attempt, epoch) in [
            (0, Epoch::Computation),
            (1, Epoch::Computation),
            (1, Epoch::Fresh),
        ] {
            l.reserve(
                &b,
                &h,
                &h,
                attempt,
                epoch,
                AdmissionWindow {
                    request_deadline: 1793396000,
                    now: 1793390000,
                },
            )
            .unwrap();
        }
        assert_eq!(l.charged_seconds().unwrap(), 3100);
        let before = l.clone();
        let mut too_large = b.clone();
        too_large.computation_ceiling_seconds = 25000;
        assert!(l
            .reserve(
                &too_large,
                &h,
                &h,
                2,
                Epoch::Computation,
                AdmissionWindow {
                    request_deadline: 1793396000,
                    now: 1793390000
                }
            )
            .is_err());
        assert_eq!(l, before);
    }
    #[test]
    fn policy_deadline_and_forged_history_reject() {
        let (mut l, b) = fixture();
        let h = format!("{:064x}", 0);
        assert!(l
            .reserve(
                &b,
                &h,
                &h,
                0,
                Epoch::Fresh,
                AdmissionWindow {
                    request_deadline: 1793396000,
                    now: 1793396000
                }
            )
            .is_err());
        let mut bad = b.clone();
        bad.policy.cumulative_ceiling_seconds = 25201;
        assert!(bad.policy.validate().is_err());
        bad = b.clone();
        bad.policy.absolute_deadline_unix_seconds += 1;
        assert!(bad.policy.validate().is_err());
        l.reserve(
            &b,
            &h,
            &h,
            0,
            Epoch::Fresh,
            AdmissionWindow {
                request_deadline: 1793396000,
                now: 1793390000,
            },
        )
        .unwrap();
        l.reservations.push(l.reservations[0].clone());
        assert!(Ledger::from_bytes(&serde_json::to_vec(&l).unwrap()).is_err());
    }
}
