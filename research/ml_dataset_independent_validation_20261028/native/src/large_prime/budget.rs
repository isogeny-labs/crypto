use anyhow::{ensure, Context, Result};

pub const MEMORY: usize = 512 * 1024 * 1024;
#[derive(Default)]
pub struct Budget {
    pub logical: u64,
    pub curve: u64,
    pub dense: usize,
    pub retained: usize,
}
impl Budget {
    pub fn work(&mut self, n: usize) -> Result<()> {
        self.logical = self
            .logical
            .checked_add(n as u64)
            .context("consumer_resource_limit: logical overflow")?;
        ensure!(
            self.logical <= 128_000_000,
            "consumer_resource_limit: logical work"
        );
        Ok(())
    }
    pub fn curves(&mut self, n: u64) -> Result<()> {
        self.curve = self
            .curve
            .checked_add(n)
            .context("consumer_resource_limit: curve overflow")?;
        ensure!(
            self.curve <= 1_048_576,
            "consumer_resource_limit: curve operations"
        );
        Ok(())
    }
    pub fn scratch(&self, n: usize) -> Result<()> {
        ensure!(
            n <= MEMORY.saturating_sub(self.retained),
            "consumer_resource_limit: decoded memory"
        );
        Ok(())
    }
    pub fn matrix(&mut self, rows: usize, columns: usize) -> Result<()> {
        let cells = rows
            .checked_mul(columns)
            .context("consumer_resource_limit: matrix overflow")?;
        self.dense = self
            .dense
            .checked_add(cells)
            .context("consumer_resource_limit: dense overflow")?;
        ensure!(
            self.dense <= 4_194_304,
            "consumer_resource_limit: dense coefficient cells"
        );
        self.scratch(
            cells
                .checked_mul(64)
                .and_then(|n| rows.checked_mul(128).and_then(|r| n.checked_add(r)))
                .and_then(|n| columns.checked_mul(64).and_then(|c| n.checked_add(c)))
                .context("consumer_resource_limit: matrix bytes")?,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_matrix_still_reserves_column_vectors_before_allocation() {
        let mut budget = Budget {
            retained: MEMORY - 63,
            ..Budget::default()
        };
        assert!(budget.matrix(0, 1).is_err());
        budget.retained = MEMORY - 64;
        assert!(budget.matrix(0, 1).is_ok());
    }
}
