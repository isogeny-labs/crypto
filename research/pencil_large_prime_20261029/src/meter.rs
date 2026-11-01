//! Exact semantic event counts, not instructions, allocation or total CPU cost.
//! Per-thread state includes failed attempts and constructor primality work.
use std::cell::Cell;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "reporting", derive(serde::Serialize))]
pub struct Snapshot {
    /// Atomic deterministic walk-label draws; mixing/reduction internals excluded.
    pub walk_label_calls: u128,
    pub scalar_adds: u128,
    pub scalar_muls: u128,
    pub scalar_negations: u128,
    pub scalar_inversions: u128,
    pub modular_reductions: u128,
    pub sparse_accumulations: u128,
    pub payload_terms: u128,
    pub parent_hops: u128,
    pub partition_terms: u128,
    pub primality_checks: u128,
    pub collection_entry_visits: u128,
    pub collection_elements_copied: u128,
    pub collection_elements_initialized: u128,
    pub collection_comparisons: u128,
    pub map_calls: u128,
    pub map_key_comparisons: u128,
    pub map_entry_visits: u128,
    pub map_cloned_entries: u128,
}
thread_local! { static COUNTS: Cell<Snapshot> = Cell::new(Snapshot::default()); }
pub(crate) fn charge(f: impl FnOnce(&mut Snapshot)) {
    COUNTS.with(|c| {
        let mut value = c.get();
        f(&mut value);
        c.set(value);
    });
}
pub fn snapshot() -> Snapshot {
    COUNTS.with(Cell::get)
}
pub fn reset() {
    COUNTS.with(|c| c.set(Snapshot::default()));
}
impl std::ops::Sub for Snapshot {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            walk_label_calls: self.walk_label_calls - rhs.walk_label_calls,
            scalar_adds: self.scalar_adds - rhs.scalar_adds,
            scalar_muls: self.scalar_muls - rhs.scalar_muls,
            scalar_negations: self.scalar_negations - rhs.scalar_negations,
            scalar_inversions: self.scalar_inversions - rhs.scalar_inversions,
            modular_reductions: self.modular_reductions - rhs.modular_reductions,
            sparse_accumulations: self.sparse_accumulations - rhs.sparse_accumulations,
            payload_terms: self.payload_terms - rhs.payload_terms,
            parent_hops: self.parent_hops - rhs.parent_hops,
            partition_terms: self.partition_terms - rhs.partition_terms,
            primality_checks: self.primality_checks - rhs.primality_checks,
            collection_entry_visits: self.collection_entry_visits - rhs.collection_entry_visits,
            collection_elements_copied: self.collection_elements_copied
                - rhs.collection_elements_copied,
            collection_elements_initialized: self.collection_elements_initialized
                - rhs.collection_elements_initialized,
            collection_comparisons: self.collection_comparisons - rhs.collection_comparisons,
            map_calls: self.map_calls - rhs.map_calls,
            map_key_comparisons: self.map_key_comparisons - rhs.map_key_comparisons,
            map_entry_visits: self.map_entry_visits - rhs.map_entry_visits,
            map_cloned_entries: self.map_cloned_entries - rhs.map_cloned_entries,
        }
    }
}
impl std::ops::AddAssign for Snapshot {
    fn add_assign(&mut self, rhs: Self) {
        self.walk_label_calls += rhs.walk_label_calls;
        self.scalar_adds += rhs.scalar_adds;
        self.scalar_muls += rhs.scalar_muls;
        self.scalar_negations += rhs.scalar_negations;
        self.scalar_inversions += rhs.scalar_inversions;
        self.modular_reductions += rhs.modular_reductions;
        self.sparse_accumulations += rhs.sparse_accumulations;
        self.payload_terms += rhs.payload_terms;
        self.parent_hops += rhs.parent_hops;
        self.partition_terms += rhs.partition_terms;
        self.primality_checks += rhs.primality_checks;
        self.collection_entry_visits += rhs.collection_entry_visits;
        self.collection_elements_copied += rhs.collection_elements_copied;
        self.collection_elements_initialized += rhs.collection_elements_initialized;
        self.collection_comparisons += rhs.collection_comparisons;
        self.map_calls += rhs.map_calls;
        self.map_key_comparisons += rhs.map_key_comparisons;
        self.map_entry_visits += rhs.map_entry_visits;
        self.map_cloned_entries += rhs.map_cloned_entries;
    }
}

impl Snapshot {
    /// Refuse counter regression rather than wrapping a missing phase.
    pub fn checked_difference(self, rhs: Self) -> Option<Self> {
        Some(Self {
            walk_label_calls: self.walk_label_calls.checked_sub(rhs.walk_label_calls)?,
            scalar_adds: self.scalar_adds.checked_sub(rhs.scalar_adds)?,
            scalar_muls: self.scalar_muls.checked_sub(rhs.scalar_muls)?,
            scalar_negations: self.scalar_negations.checked_sub(rhs.scalar_negations)?,
            scalar_inversions: self.scalar_inversions.checked_sub(rhs.scalar_inversions)?,
            modular_reductions: self
                .modular_reductions
                .checked_sub(rhs.modular_reductions)?,
            sparse_accumulations: self
                .sparse_accumulations
                .checked_sub(rhs.sparse_accumulations)?,
            payload_terms: self.payload_terms.checked_sub(rhs.payload_terms)?,
            parent_hops: self.parent_hops.checked_sub(rhs.parent_hops)?,
            partition_terms: self.partition_terms.checked_sub(rhs.partition_terms)?,
            primality_checks: self.primality_checks.checked_sub(rhs.primality_checks)?,
            collection_entry_visits: self
                .collection_entry_visits
                .checked_sub(rhs.collection_entry_visits)?,
            collection_elements_copied: self
                .collection_elements_copied
                .checked_sub(rhs.collection_elements_copied)?,
            collection_elements_initialized: self
                .collection_elements_initialized
                .checked_sub(rhs.collection_elements_initialized)?,
            collection_comparisons: self
                .collection_comparisons
                .checked_sub(rhs.collection_comparisons)?,
            map_calls: self.map_calls.checked_sub(rhs.map_calls)?,
            map_key_comparisons: self
                .map_key_comparisons
                .checked_sub(rhs.map_key_comparisons)?,
            map_entry_visits: self.map_entry_visits.checked_sub(rhs.map_entry_visits)?,
            map_cloned_entries: self
                .map_cloned_entries
                .checked_sub(rhs.map_cloned_entries)?,
        })
    }
}

#[cfg(test)]
mod checked_delta_tests {
    use super::*;
    #[test]
    fn exact_delta_and_regressed_counter() {
        let a = Snapshot {
            map_entry_visits: 5,
            scalar_muls: 8,
            ..Snapshot::default()
        };
        let b = Snapshot {
            map_entry_visits: 2,
            scalar_muls: 8,
            ..Snapshot::default()
        };
        assert_eq!(
            a.checked_difference(b),
            Some(Snapshot {
                map_entry_visits: 3,
                ..Snapshot::default()
            })
        );
        assert_eq!(b.checked_difference(a), None);
        assert_eq!(
            a.checked_difference(Snapshot {
                scalar_muls: 9,
                ..Snapshot::default()
            }),
            None
        );
    }
}

/// Explicit collection primitives for the native geometry adapter. A visit is a
/// consumed collection entry; range-loop indices are not collection visits.
/// Copies count clone/copied source elements. Initialization counts fresh pushed
/// or filled elements, excluding elements already counted as copies. Elements
/// have their Rust collection element type (a tuple is one element). No allocator
/// internals, capacity reservation, API-call surcharge, or implicit CPU claim.
pub fn collection(visits: usize, copied: usize, initialized: usize, comparisons: usize) {
    charge(|c| {
        c.collection_entry_visits += visits as u128;
        c.collection_elements_copied += copied as u128;
        c.collection_elements_initialized += initialized as u128;
        c.collection_comparisons += comparisons as u128;
    });
}

/// One observed call, priced as an inherited atomic RNG/label event. Its integer
/// mixing and terminal reduction are not separately charged in this model.
pub fn walk_label_call() {
    charge(|c| c.walk_label_calls += 1);
}
