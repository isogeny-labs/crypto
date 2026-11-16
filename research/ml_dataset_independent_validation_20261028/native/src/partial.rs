//! Independent artifact-only interrupted-attempt lineage validation.
mod budget;
mod capture;
mod chain;
mod io;
mod report;
mod restore;
mod schema;
mod source;
#[cfg(test)]
mod test_support;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_io;
#[cfg(test)]
mod tests_limits;
#[cfg(test)]
mod tests_negative;

use anyhow::Result;
use serde_json::Value;
use std::{io::Write, path::Path};

/// Restore a caller-selected local object mirror. The anchor's remote remains
/// an exact identity and is never followed by this command.
pub fn restore_local(anchor: &Path, expected: &str, objects: &Path, out: &Path) -> Result<Value> {
    let mut budget = budget::Budget::new();
    let (anchor, raw) = restore::pinned_anchor(anchor, expected, &mut budget)?;
    let mut source = source::LocalSource::mirror(objects)?;
    restore::restore(&mut source, anchor, &raw, expected, out, &mut budget)
}
/// Fresh read-only audit of original metadata and all supplied artifact bytes.
pub fn audit_restored(root: &Path, expected_anchor: &str) -> Result<Value> {
    let mut budget = budget::Budget::new();
    restore::audit_inner(root, expected_anchor, false, false, false, &mut budget)
}
/// Read-only canonical S3 HEAD/conditioned GET adapter; no listing or retries.
/// Deployment authorization is separate from parser/software validation.
pub fn fetch_remote(anchor: &Path, expected: &str, out: &Path) -> Result<Value> {
    let mut budget = budget::Budget::new();
    let (anchor, raw) = restore::pinned_anchor(anchor, expected, &mut budget)?;
    let mut source = source::RemoteSource::new(&anchor.remote, &mut budget)?;
    restore::restore(&mut source, anchor, &raw, expected, out, &mut budget)
}
/// Bound CLI serialization before emitting any success bytes. The invocation's
/// fixed report allowance covers this separate final 1 MiB buffer.
pub fn print_report(report: &Value) -> Result<()> {
    let bytes = report::serialized(report)?;
    std::io::stdout().lock().write_all(&bytes)?;
    Ok(())
}
