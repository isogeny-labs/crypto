//! Monotonic semantic collection ledger; arithmetic is counted separately.
//! Snapshot differences may nest. Never reset a process-global measurement.
use std::cell::Cell;
#[path = "three_leads_constructor_primality.rs"]
mod constructor_primality;
use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Snapshot {
    pub entry_visits: u128,
    pub metadata_value_nodes_copied: u128,
    pub metadata_string_bytes_copied: u128,
    pub metadata_key_bytes_copied: u128,
    pub metadata_family_bytes_copied: u128,

    pub scalar_constructor_prime_calls: u128,
    pub scalar_constructor_modular_muls: u128,
    pub scalar_constructor_modular_squares: u128,
    pub scalar_constructor_standalone_reductions: u128,
    pub constructor_prime_calls: u128,
    pub constructor_modular_muls: u128,
    pub constructor_modular_squares: u128,
    pub constructor_standalone_reductions: u128,

    pub elements_copied: u128,
    pub elements_initialized: u128,
    pub key_comparisons: u128,
}
thread_local! {static EVENTS: Cell<Snapshot> = const { Cell::new(Snapshot::ZERO) };}
impl Snapshot {
    const ZERO: Self = Self {
        entry_visits: 0,
        metadata_value_nodes_copied: 0,
        metadata_string_bytes_copied: 0,
        metadata_key_bytes_copied: 0,
        metadata_family_bytes_copied: 0,
        scalar_constructor_prime_calls: 0,
        scalar_constructor_modular_muls: 0,
        scalar_constructor_modular_squares: 0,
        scalar_constructor_standalone_reductions: 0,
        constructor_prime_calls: 0,
        constructor_modular_muls: 0,
        constructor_modular_squares: 0,
        constructor_standalone_reductions: 0,
        elements_copied: 0,
        elements_initialized: 0,
        key_comparisons: 0,
    };
    pub fn checked_since(self, before: Self) -> Option<Self> {
        Some(Self {
            metadata_value_nodes_copied: self
                .metadata_value_nodes_copied
                .checked_sub(before.metadata_value_nodes_copied)?,
            metadata_string_bytes_copied: self
                .metadata_string_bytes_copied
                .checked_sub(before.metadata_string_bytes_copied)?,
            metadata_key_bytes_copied: self
                .metadata_key_bytes_copied
                .checked_sub(before.metadata_key_bytes_copied)?,
            metadata_family_bytes_copied: self
                .metadata_family_bytes_copied
                .checked_sub(before.metadata_family_bytes_copied)?,
            scalar_constructor_prime_calls: self
                .scalar_constructor_prime_calls
                .checked_sub(before.scalar_constructor_prime_calls)?,
            scalar_constructor_modular_muls: self
                .scalar_constructor_modular_muls
                .checked_sub(before.scalar_constructor_modular_muls)?,
            scalar_constructor_modular_squares: self
                .scalar_constructor_modular_squares
                .checked_sub(before.scalar_constructor_modular_squares)?,
            scalar_constructor_standalone_reductions: self
                .scalar_constructor_standalone_reductions
                .checked_sub(before.scalar_constructor_standalone_reductions)?,
            constructor_prime_calls: self
                .constructor_prime_calls
                .checked_sub(before.constructor_prime_calls)?,
            constructor_modular_muls: self
                .constructor_modular_muls
                .checked_sub(before.constructor_modular_muls)?,
            constructor_modular_squares: self
                .constructor_modular_squares
                .checked_sub(before.constructor_modular_squares)?,
            constructor_standalone_reductions: self
                .constructor_standalone_reductions
                .checked_sub(before.constructor_standalone_reductions)?,
            entry_visits: self.entry_visits.checked_sub(before.entry_visits)?,
            elements_copied: self.elements_copied.checked_sub(before.elements_copied)?,
            elements_initialized: self
                .elements_initialized
                .checked_sub(before.elements_initialized)?,
            key_comparisons: self.key_comparisons.checked_sub(before.key_comparisons)?,
        })
    }
}
pub fn snapshot() -> Snapshot {
    EVENTS.with(Cell::get)
}
pub fn record(visits: usize, copied: usize, initialized: usize, comparisons: usize) {
    EVENTS.with(|slot| {
        let mut s = slot.get();
        s.entry_visits = s
            .entry_visits
            .checked_add(visits as u128)
            .expect("geometry visits overflow");
        s.elements_copied = s
            .elements_copied
            .checked_add(copied as u128)
            .expect("geometry copies overflow");
        s.elements_initialized = s
            .elements_initialized
            .checked_add(initialized as u128)
            .expect("geometry initialization overflow");
        s.key_comparisons = s
            .key_comparisons
            .checked_add(comparisons as u128)
            .expect("geometry comparisons overflow");
        slot.set(s);
    });
}
pub fn copied(a: &[u64]) -> Vec<u64> {
    record(0, a.len(), 0, 0);
    a.to_vec()
}
pub fn initialized(a: Vec<u64>) -> Vec<u64> {
    record(0, 0, a.len(), 0);
    a
}
pub fn last(a: &[u64]) -> Option<&u64> {
    let value = a.last();
    record(usize::from(value.is_some()), 0, 0, 0);
    value
}
pub fn last_is(a: &[u64], x: u64) -> bool {
    let value = last(a);
    record(0, 0, 0, 1);
    value == Some(&x)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn snapshot_is_monotonic_and_checked() {
        let a = snapshot();
        record(1, 2, 3, 4);
        let b = snapshot();
        assert_eq!(
            b.checked_since(a),
            Some(Snapshot {
                metadata_value_nodes_copied: 0,
                metadata_string_bytes_copied: 0,
                metadata_key_bytes_copied: 0,
                metadata_family_bytes_copied: 0,
                scalar_constructor_prime_calls: 0,
                scalar_constructor_modular_muls: 0,
                scalar_constructor_modular_squares: 0,
                scalar_constructor_standalone_reductions: 0,
                constructor_prime_calls: 0,
                constructor_modular_muls: 0,
                constructor_modular_squares: 0,
                constructor_standalone_reductions: 0,
                entry_visits: 1,
                elements_copied: 2,
                elements_initialized: 3,
                key_comparisons: 4
            })
        );
        assert_eq!(a.checked_since(b), None);
    }
    #[test]
    fn empty_last_counts_predicate_but_no_entry() {
        let a = snapshot();
        assert!(!last_is(&[], 0));
        assert_eq!(snapshot().checked_since(a).unwrap().entry_visits, 0);
        assert_eq!(snapshot().checked_since(a).unwrap().key_comparisons, 1);
        assert!(last_is(&[0], 0));
        assert_eq!(snapshot().checked_since(a).unwrap().key_comparisons, 2);
    }
}

pub fn slice_eq(a: &[u64], b: &[u64]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b).all(|(a, b)| {
        record(2, 0, 0, 1);
        a == b
    })
}
pub fn constructor_prime(p: u64) -> bool {
    let observation = constructor_primality::observe(p);
    EVENTS.with(|slot| {
        let mut s = slot.get();
        s.constructor_prime_calls = s
            .constructor_prime_calls
            .checked_add(1)
            .expect("constructor count overflow");
        s.constructor_modular_muls = s
            .constructor_modular_muls
            .checked_add(observation.counts.modular_muls as u128)
            .expect("constructor count overflow");
        s.constructor_modular_squares = s
            .constructor_modular_squares
            .checked_add(observation.counts.modular_squares as u128)
            .expect("constructor count overflow");
        s.constructor_standalone_reductions = s
            .constructor_standalone_reductions
            .checked_add(observation.counts.standalone_modular_reductions as u128)
            .expect("constructor count overflow");
        slot.set(s);
    });
    observation.prime
}

/// Census of the existing serde Value clone, not additional algorithm work.
/// The observer traversal itself contributes wall overhead, never visit events.
pub fn metadata_clone(value: &serde_json::Value, family: &str) {
    fn census(v: &serde_json::Value, out: &mut (u128, u128, u128)) {
        out.0 = out.0.checked_add(1).expect("metadata nodes overflow");
        match v {
            serde_json::Value::String(s) => {
                out.1 = out
                    .1
                    .checked_add(s.len() as u128)
                    .expect("metadata bytes overflow")
            }
            serde_json::Value::Array(a) => {
                for v in a {
                    census(v, out)
                }
            }
            serde_json::Value::Object(m) => {
                for (k, v) in m {
                    out.2 = out
                        .2
                        .checked_add(k.len() as u128)
                        .expect("metadata key bytes overflow");
                    census(v, out)
                }
            }
            _ => {}
        }
    }
    let mut counts = (0, 0, 0);
    census(value, &mut counts);
    EVENTS.with(|slot| {
        let mut s = slot.get();
        s.metadata_value_nodes_copied = s
            .metadata_value_nodes_copied
            .checked_add(counts.0)
            .expect("metadata nodes overflow");
        s.metadata_string_bytes_copied = s
            .metadata_string_bytes_copied
            .checked_add(counts.1)
            .expect("metadata bytes overflow");
        s.metadata_key_bytes_copied = s
            .metadata_key_bytes_copied
            .checked_add(counts.2)
            .expect("metadata key bytes overflow");
        s.metadata_family_bytes_copied = s
            .metadata_family_bytes_copied
            .checked_add(family.len() as u128)
            .expect("metadata family bytes overflow");
        slot.set(s);
    });
}
#[cfg(test)]
mod metadata_tests {
    use super::*;
    #[test]
    fn census_does_not_charge_observer_as_collection_work() {
        let before = snapshot();
        metadata_clone(&serde_json::json!({"ab":[1,"xy",null]}), "scholten");
        let delta = snapshot().checked_since(before).unwrap();
        assert_eq!(delta.metadata_value_nodes_copied, 5);
        assert_eq!(delta.metadata_string_bytes_copied, 2);
        assert_eq!(delta.metadata_key_bytes_copied, 2);
        assert_eq!(delta.metadata_family_bytes_copied, 8);
        assert_eq!(
            (
                delta.entry_visits,
                delta.elements_copied,
                delta.elements_initialized,
                delta.key_comparisons
            ),
            (0, 0, 0, 0)
        );
    }
}

impl Snapshot {
    pub fn checked_add(self, other: Self) -> Option<Self> {
        Some(Self {
            entry_visits: self.entry_visits.checked_add(other.entry_visits)?,
            metadata_value_nodes_copied: self
                .metadata_value_nodes_copied
                .checked_add(other.metadata_value_nodes_copied)?,
            metadata_string_bytes_copied: self
                .metadata_string_bytes_copied
                .checked_add(other.metadata_string_bytes_copied)?,
            metadata_key_bytes_copied: self
                .metadata_key_bytes_copied
                .checked_add(other.metadata_key_bytes_copied)?,
            metadata_family_bytes_copied: self
                .metadata_family_bytes_copied
                .checked_add(other.metadata_family_bytes_copied)?,
            scalar_constructor_prime_calls: self
                .scalar_constructor_prime_calls
                .checked_add(other.scalar_constructor_prime_calls)?,
            scalar_constructor_modular_muls: self
                .scalar_constructor_modular_muls
                .checked_add(other.scalar_constructor_modular_muls)?,
            scalar_constructor_modular_squares: self
                .scalar_constructor_modular_squares
                .checked_add(other.scalar_constructor_modular_squares)?,
            scalar_constructor_standalone_reductions: self
                .scalar_constructor_standalone_reductions
                .checked_add(other.scalar_constructor_standalone_reductions)?,
            constructor_prime_calls: self
                .constructor_prime_calls
                .checked_add(other.constructor_prime_calls)?,
            constructor_modular_muls: self
                .constructor_modular_muls
                .checked_add(other.constructor_modular_muls)?,
            constructor_modular_squares: self
                .constructor_modular_squares
                .checked_add(other.constructor_modular_squares)?,
            constructor_standalone_reductions: self
                .constructor_standalone_reductions
                .checked_add(other.constructor_standalone_reductions)?,
            elements_copied: self.elements_copied.checked_add(other.elements_copied)?,
            elements_initialized: self
                .elements_initialized
                .checked_add(other.elements_initialized)?,
            key_comparisons: self.key_comparisons.checked_add(other.key_comparisons)?,
        })
    }
}

/// Exact nonoverlapping engine ledger partition. Metadata/JSON observation work
/// is not added to mathematical work, and coverage is not inferred from sums.
pub fn partition(start: Snapshot, endpoints: &[(&str, Snapshot)]) -> Option<serde_json::Value> {
    let mut previous = start;
    let mut sum = Snapshot::ZERO;
    let mut segments = serde_json::Map::new();
    for &(name, end) in endpoints {
        let delta = end.checked_since(previous)?;
        sum = sum.checked_add(delta)?;
        if segments
            .insert(name.into(), serde_json::to_value(delta).ok()?)
            .is_some()
        {
            return None;
        }
        previous = end;
    }
    let total = previous.checked_since(start)?;
    if sum != total {
        return None;
    }
    Some(
        serde_json::json!({"schema":"engine-geometry-partition/v1-draft","total":total,"segments":segments,"partition_sum_verified":true}),
    )
}
#[cfg(test)]
mod partition_tests {
    use super::*;
    #[test]
    fn telescoping_and_duplicates() {
        let a = snapshot();
        record(1, 2, 3, 4);
        let b = snapshot();
        record(2, 3, 4, 5);
        let c = snapshot();
        let p = partition(a, &[("a", b), ("b", c)]).unwrap();
        assert_eq!(p["total"]["entry_visits"], 3);
        assert_eq!(partition(a, &[("a", b), ("a", c)]), None);
        assert_eq!(partition(c, &[("reverse", b)]), None);
    }
}

/// Actual scalar Ring constructor validation, distinct from base-field validation.
/// These are expanded noninverse modular operations; the atomic scalar model
/// prices M and S as scalar events, never as base-field M/S or inverse expansion.
pub fn scalar_constructor_prime(r: u64) -> bool {
    let observation = constructor_primality::observe(r);
    EVENTS.with(|slot| {
        let mut s = slot.get();
        s.scalar_constructor_prime_calls = s
            .scalar_constructor_prime_calls
            .checked_add(1)
            .expect("scalar constructor overflow");
        s.scalar_constructor_modular_muls = s
            .scalar_constructor_modular_muls
            .checked_add(observation.counts.modular_muls as u128)
            .expect("scalar constructor overflow");
        s.scalar_constructor_modular_squares = s
            .scalar_constructor_modular_squares
            .checked_add(observation.counts.modular_squares as u128)
            .expect("scalar constructor overflow");
        s.scalar_constructor_standalone_reductions = s
            .scalar_constructor_standalone_reductions
            .checked_add(observation.counts.standalone_modular_reductions as u128)
            .expect("scalar constructor overflow");
        slot.set(s);
    });
    observation.prime
}
