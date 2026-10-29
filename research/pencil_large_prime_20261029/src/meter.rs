//! Exact semantic event counts, not instructions, allocation or total CPU cost.
//! Per-thread state includes failed attempts and constructor primality work.
use std::cell::Cell;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "producer", derive(serde::Serialize))]
pub struct Snapshot {
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
        }
    }
}
impl std::ops::AddAssign for Snapshot {
    fn add_assign(&mut self, rhs: Self) {
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
    }
}
