//! One conservative allocation-volume ledger for the complete invocation.
use anyhow::{ensure, Context, Result};

pub(crate) const LIMIT: usize = 256 * 1024 * 1024;
pub(crate) const ANCHOR_MAX: u64 = 65536;
pub(crate) const METADATA_MAX: u64 = 4 * 1024 * 1024;
pub(crate) const ARTIFACT_MAX: u64 = 64 * 1024 * 1024;
pub(crate) const SEGMENT_MAX: u64 = 16 * 1024 * 1024;
pub(crate) const LINE_MAX: u64 = 1024 * 1024;
pub(crate) const SNAPSHOT_ARTIFACTS: usize = 8192;
pub(crate) const MAX_ITEMS: u64 = 100_000;
pub(crate) const MAX_STREAM: u64 = 4 * 1024 * 1024 * 1024;
pub(crate) const CHAIN_MAX: u64 = 256;
pub(crate) const OBJECTS_MAX: u64 = 65536;
pub(crate) const LOGICAL_BYTES_MAX: u64 = 8 * 1024 * 1024 * 1024;
pub(crate) const CONTEXT_BYTES_MAX: u64 = 16 * 1024 * 1024;
pub(crate) const REPORT_MAX: usize = 1024 * 1024;
const FIXED: usize = 131072 + 2 * REPORT_MAX + 32 * 1024 * 1024;

pub(crate) struct Budget {
    limit: usize,
    used: usize,
    pub(crate) logical_objects: u64,
    pub(crate) logical_bytes: u64,
    pub(crate) read_bytes: u64,
    pub(crate) written_bytes: u64,
    pub(crate) disk_reserved: u64,
    pub(crate) phase: crate::phase::PhaseBudget,
}
impl Budget {
    /// Fixed envelopes are reserved before any caller-owned paths/decoders.
    pub(crate) fn new() -> Self {
        Self {
            limit: LIMIT,
            used: FIXED,
            logical_objects: 0,
            logical_bytes: 0,
            read_bytes: 0,
            written_bytes: 0,
            disk_reserved: 0,
            phase: crate::phase::PhaseBudget::new(32 * 1024 * 1024),
        }
    }
    #[cfg(test)]
    pub(crate) fn with_limit(limit: usize) -> Self {
        Self {
            limit,
            used: 0,
            ..Self::new()
        }
    }
    pub(crate) fn charge(&mut self, bytes: usize, what: &str) -> Result<()> {
        let next = self
            .used
            .checked_add(bytes)
            .context("consumer_resource_limit: partial allocation overflow")?;
        ensure!(
            next <= self.limit,
            "consumer_resource_limit: partial {what}"
        );
        self.used = next;
        Ok(())
    }
    pub(crate) fn raw(&mut self, bytes: u64) -> Result<()> {
        let bytes = usize::try_from(bytes)?;
        self.charge(
            bytes
                .checked_add(1)
                .and_then(|n| n.checked_mul(4))
                .context("consumer_resource_limit: partial raw reservation")?,
            "raw and decoder scratch",
        )
    }
    pub(crate) fn read(&mut self, bytes: u64) -> Result<()> {
        self.read_bytes = self
            .read_bytes
            .checked_add(bytes)
            .context("consumer_resource_limit: actual read overflow")?;
        Ok(())
    }
    pub(crate) fn written(&mut self, bytes: u64) -> Result<()> {
        let next = self
            .written_bytes
            .checked_add(bytes)
            .context("consumer_resource_limit: actual write overflow")?;
        ensure!(
            next <= self.disk_reserved,
            "consumer_resource_limit: destination reservation"
        );
        self.written_bytes = next;
        Ok(())
    }
    pub(crate) fn admit_object(&mut self, bytes: u64) -> Result<()> {
        let objects = self
            .logical_objects
            .checked_add(1)
            .context("consumer_resource_limit: logical object count overflow")?;
        let total = self
            .logical_bytes
            .checked_add(bytes)
            .context("consumer_resource_limit: logical bytes overflow")?;
        ensure!(
            objects <= OBJECTS_MAX && total <= LOGICAL_BYTES_MAX,
            "consumer_resource_limit: cumulative logical objects/bytes"
        );
        self.logical_objects = objects;
        self.logical_bytes = total;
        Ok(())
    }
    pub(crate) fn reserve_disk(&mut self, bytes: u64) -> Result<()> {
        self.disk_reserved = self
            .disk_reserved
            .checked_add(bytes)
            .context("consumer_resource_limit: destination size overflow")?;
        ensure!(
            self.disk_reserved <= LOGICAL_BYTES_MAX + 16 * 1024 * 1024,
            "consumer_resource_limit: destination bound"
        );
        Ok(())
    }
    pub(crate) fn used(&self) -> usize {
        self.used
    }
    pub(crate) fn reserve_vec<T>(
        &mut self,
        values: &mut Vec<T>,
        extra: usize,
        what: &str,
    ) -> Result<()> {
        let required = values
            .len()
            .checked_add(extra)
            .context("consumer_resource_limit: collection length")?;
        if required > values.capacity() {
            let capacity = required.max(
                values
                    .capacity()
                    .checked_mul(2)
                    .context("consumer_resource_limit: collection capacity")?,
            );
            // A reallocation may briefly coexist with its previous allocation.
            // Charge the whole new logical capacity, not just its increment;
            // earlier reservations remain in this cumulative-volume ledger.
            self.charge(
                capacity
                    .checked_mul(std::mem::size_of::<T>())
                    .context("consumer_resource_limit: collection capacity")?,
                what,
            )?;
            values.try_reserve_exact(capacity - values.len())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cumulative_admission_refuses_without_advancing() {
        let mut b = Budget::with_limit(8);
        b.charge(8, "boundary").unwrap();
        assert!(b.charge(1, "excess").is_err());
        assert_eq!(b.used(), 8);
        b.logical_objects = OBJECTS_MAX;
        assert!(b.admit_object(0).is_err());
        assert_eq!(b.logical_objects, OBJECTS_MAX);
        b.logical_objects = 0;
        b.logical_bytes = LOGICAL_BYTES_MAX;
        assert!(b.admit_object(1).is_err());
        assert_eq!(b.logical_objects, 0);
    }
    #[test]
    fn raw_and_growth_refuse_before_allocation() {
        let mut b = Budget::with_limit(7);
        assert!(b.raw(1).is_err());
        let mut values = Vec::<u64>::new();
        assert!(b.reserve_vec(&mut values, 1, "test").is_err());
        assert_eq!(values.capacity(), 0);
        assert!(b.raw(u64::MAX).is_err());
        assert_eq!(b.used(), 0);
    }
}
