use anyhow::{ensure, Context, Result};
use serde_json::Value;

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
                .checked_mul(16)
                .and_then(|n| n.checked_add(rows * 64))
                .context("consumer_resource_limit: matrix bytes")?,
        )
    }
}
/// Conservative decoded-storage admission, not allocator/RSS measurement.
pub fn decoded(value: &Value) -> Result<usize> {
    let mut n = 64usize;
    match value {
        Value::String(s) => {
            n = n
                .checked_add(s.capacity())
                .context("consumer_resource_limit: string")?
        }
        Value::Array(a) => {
            n = n
                .checked_add(
                    a.capacity()
                        .checked_mul(64)
                        .context("consumer_resource_limit: array")?,
                )
                .context("consumer_resource_limit: array")?;
            for v in a {
                n = n
                    .checked_add(decoded(v)?)
                    .context("consumer_resource_limit: array")?;
            }
        }
        Value::Object(o) => {
            for (k, v) in o {
                n = n
                    .checked_add(128 + k.capacity())
                    .and_then(|n| n.checked_add(decoded(v).ok()?))
                    .context("consumer_resource_limit: object")?;
            }
        }
        _ => {}
    }
    ensure!(n <= MEMORY, "consumer_resource_limit: decoded JSON");
    Ok(n)
}
