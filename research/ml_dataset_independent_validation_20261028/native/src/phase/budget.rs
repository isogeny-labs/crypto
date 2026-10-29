//! Conservative component allocation volume; never recycled within one cell.
use anyhow::{ensure, Context, Result};

pub(super) const LIMIT: usize = 32 * 1024 * 1024;
pub(crate) struct Budget {
    limit: usize,
    used: usize,
}
impl Budget {
    pub(crate) fn new(limit: usize) -> Self {
        Self { limit, used: 0 }
    }
    pub(crate) fn charge(&mut self, bytes: usize, what: &str) -> Result<()> {
        let next = self
            .used
            .checked_add(bytes)
            .context("consumer_resource_limit: phase storage overflow")?;
        ensure!(next <= self.limit, "consumer_resource_limit: phase {what}");
        self.used = next;
        Ok(())
    }
    pub(crate) fn used(&self) -> usize {
        self.used
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn refuses_cumulative_allocation_before_advancing() {
        let mut b = Budget::new(15);
        b.charge(8, "first").unwrap();
        assert!(b
            .charge(8, "second")
            .unwrap_err()
            .to_string()
            .starts_with("consumer_resource_limit"));
        assert_eq!(b.used(), 8);
        b.charge(7, "boundary").unwrap();
        assert!(b.charge(usize::MAX, "overflow").is_err());
    }
}
