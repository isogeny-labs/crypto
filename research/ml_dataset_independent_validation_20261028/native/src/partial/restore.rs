//! Restore orchestration and a fresh audit using the same cumulative ledgers.
use super::{
    budget::*,
    capture::{self, CaptureReader, CaptureState},
    chain::{self, Preflight, Snapshot},
    io::{self, Root},
    report,
    schema::*,
    source::{LocalSource, ObjectReader},
};
use anyhow::{ensure, Context, Result};
use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet},
    io::Read,
    path::Path,
};

pub(crate) const MARKER: &str = "PARTIAL_RESTORE_INCOMPLETE";
pub(crate) const ANCHOR_PATH: &str = "partial-consumer/anchor.json";
pub(crate) const REPORT_PATH: &str = "partial-consumer/report.json";

pub(crate) fn pinned_anchor(
    path: &Path,
    expected: &str,
    budget: &mut Budget,
) -> Result<(Anchor, Vec<u8>)> {
    ensure!(
        crate::common::valid_hex(expected, 64),
        "partial caller anchor pin grammar"
    );
    let (parent, leaf) = Root::file_parent(path)?;
    let raw = parent.anchor_bytes(&leaf, budget)?;
    ensure!(
        crate::common::digest(&raw) == expected,
        "partial caller anchor pin mismatch"
    );
    let anchor = io::typed(&raw, budget)?;
    chain::validate_anchor(&anchor, budget)?;
    Ok((anchor, raw))
}
pub(crate) fn restore(
    source: &mut dyn ObjectReader,
    anchor: Anchor,
    raw: &[u8],
    expected: &str,
    out: &Path,
    budget: &mut Budget,
) -> Result<Value> {
    let target = Root::new_destination(out)?;
    budget.reserve_disk(16 * 1024 * 1024)?; // anchor, bounded reports and finite control scratch
    target.write_bytes(MARKER, b"incomplete\n", budget)?;
    target.write_bytes(ANCHOR_PATH, raw, budget)?;
    let snapshots = chain::load(source, &anchor, Some(&target), budget)?;
    let admitted = chain::preflight(&snapshots, budget)?;
    budget.reserve_disk(admitted.artifact_bytes)?;
    target.space(budget.disk_reserved.saturating_sub(budget.written_bytes))?;
    for (i, member) in admitted.members.iter().enumerate() {
        budget.charge(4096, "artifact staging/control paths")?;
        let stage = format!(".partial-stage-{i:08}");
        let final_path = chain::artifact_path(&anchor, member.artifact, budget)?;
        let mut input = source.open(&member.object, &final_path, budget)?;
        let mut output = target.create_file(&stage)?;
        io::copy_exact(
            &mut input,
            &mut output,
            member.artifact.bytes,
            &member.artifact.sha256,
            budget,
        )?;
        drop(output);
        target.rename_new(&stage, &final_path)?;
    }
    // Every ordinal's semantic context and capture prefix is validated before
    // repeating the entire audit from retained original metadata and fresh FDs.
    let _ = semantics(&target, &anchor, &snapshots, budget)?;
    let report = audit_root(
        &target,
        expected,
        true,
        true,
        source.actual_remote(),
        budget,
    )?;
    let raw = report::serialized(&report)?;
    target.write_bytes(".partial-report.tmp", &raw, budget)?;
    target.rename_new(".partial-report.tmp", REPORT_PATH)?;
    target.remove_file(MARKER)?;
    Ok(report)
}

pub(crate) fn audit_inner(
    path: &Path,
    expected: &str,
    incomplete: bool,
    restored: bool,
    actual_remote: bool,
    budget: &mut Budget,
) -> Result<Value> {
    ensure!(
        crate::common::valid_hex(expected, 64),
        "partial caller anchor pin grammar"
    );
    let root = Root::open(path)?;
    audit_root(&root, expected, incomplete, restored, actual_remote, budget)
}
pub(crate) fn audit_root(
    root: &Root,
    expected: &str,
    incomplete: bool,
    restored: bool,
    actual_remote: bool,
    budget: &mut Budget,
) -> Result<Value> {
    let raw = root.anchor_bytes(ANCHOR_PATH, budget)?;
    ensure!(
        crate::common::digest(&raw) == expected,
        "partial restored anchor pin mismatch"
    );
    let anchor: Anchor = io::typed(&raw, budget)?;
    chain::validate_anchor(&anchor, budget)?;
    let mut source = LocalSource::restored(root)?;
    let chain = chain::load(&mut source, &anchor, None, budget)?;
    let admitted = chain::preflight(&chain, budget)?;
    for member in &admitted.members {
        let location = chain::artifact_path(&anchor, member.artifact, budget)?;
        let mut input = source.open(&member.object, &location, budget)?;
        verify_exact(
            &mut input,
            member.artifact.bytes,
            &member.artifact.sha256,
            budget,
        )?;
    }
    let (capture, phase) = semantics(root, &anchor, &chain, budget)?;
    inventory(root, &anchor, &chain, &admitted, incomplete, budget)?;
    report::build(
        &anchor,
        expected,
        &chain,
        &admitted,
        report::Outcome {
            capture,
            phase,
            restored,
            actual_remote,
        },
        budget,
    )
}

fn verify_exact(reader: &mut dyn Read, size: u64, sha: &str, budget: &mut Budget) -> Result<()> {
    use sha2::{Digest, Sha256};
    budget.charge(256, "artifact audit digest formatting")?;
    let mut hash = Sha256::new();
    let mut remaining = size;
    let mut buf = [0; 65536];
    while remaining > 0 {
        let length = usize::try_from(remaining.min(buf.len() as u64))?;
        let count = reader.read(&mut buf[..length])?;
        budget.read(count as u64)?;
        ensure!(count > 0, "partial restored artifact truncated");
        hash.update(&buf[..count]);
        remaining -= count as u64;
    }
    let mut extra = [0];
    let count = reader.read(&mut extra)?;
    budget.read(count as u64)?;
    ensure!(
        count == 0 && format!("{:x}", hash.finalize()) == sha,
        "partial restored consumed artifact identity"
    );
    Ok(())
}

struct Phases<'a> {
    root: Root,
    reads: &'a mut u64,
}
impl crate::phase::ContextReader for Phases<'_> {
    fn read_context(
        &mut self,
        a: &crate::phase::ContextArtifact,
        cap: u64,
        ledger: &mut crate::phase::PhaseBudget,
    ) -> Result<Vec<u8>> {
        ensure!(
            a.bytes <= cap,
            "consumer_resource_limit: partial phase raw file"
        );
        let size = usize::try_from(a.bytes)?;
        ledger.charge(
            size.checked_add(1)
                .and_then(|n| n.checked_mul(4))
                .context("partial phase raw overflow")?,
            "raw and decoder scratch",
        )?;
        ledger.charge(
            a.path
                .len()
                .checked_mul(8)
                .and_then(|n| n.checked_add(1024))
                .context("partial phase path reservation")?,
            "opened phase path",
        )?;
        let mut file = self.root.open_file(&a.path)?;
        ensure!(
            file.metadata()?.len() == a.bytes,
            "partial phase descriptor size changed"
        );
        let mut raw = Vec::new();
        raw.try_reserve_exact(size)?;
        raw.resize(size, 0);
        let mut offset = 0;
        while offset < size {
            let count = file.read(&mut raw[offset..])?;
            *self.reads = self
                .reads
                .checked_add(count as u64)
                .context("partial phase read count overflow")?;
            ensure!(count > 0, "partial phase context truncated");
            offset += count;
        }
        let mut extra = [0];
        let count = file.read(&mut extra)?;
        *self.reads = self
            .reads
            .checked_add(count as u64)
            .context("partial phase read count overflow")?;
        ensure!(
            count == 0 && crate::common::digest(&raw) == a.sha256,
            "partial phase consumed identity mismatch"
        );
        Ok(raw)
    }
}
struct Captured {
    root: Root,
}
impl CaptureReader for Captured {
    fn open(&mut self, a: &Artifact) -> Result<Box<dyn Read>> {
        let file = self.root.open_file(&a.path)?;
        ensure!(
            file.metadata()?.len() == a.bytes,
            "partial captured descriptor size changed"
        );
        Ok(Box::new(file))
    }
}
fn semantics(
    root: &Root,
    anchor: &Anchor,
    chain: &[Snapshot],
    budget: &mut Budget,
) -> Result<(
    Option<CaptureState>,
    Option<crate::phase::CheckedPhaseAudit>,
)> {
    budget.charge(16384, "semantic readers and finite context paths")?;
    let base = chain::prefix(anchor, budget)?;
    let mut capture_reader = Captured {
        root: root.child(&format!("{base}/artifacts"))?,
    };
    let phase_root = root.child(&format!("{base}/artifacts/context"))?;
    let mut context = BTreeMap::new();
    let mut captured = None;
    let mut phase = None;
    let mut authoritative = None;
    for snapshot in chain {
        for a in &snapshot.manifest.context {
            let path = a
                .path
                .strip_prefix("context/")
                .context("partial context namespace")?;
            let amount = path
                .len()
                .checked_add(a.role.len())
                .and_then(|n| n.checked_add(a.encoding.len()))
                .and_then(|n| n.checked_add(a.sha256.len()))
                .and_then(|n| n.checked_mul(8))
                .and_then(|n| n.checked_add(1024))
                .context("partial typed context reservation")?;
            budget.charge(amount, "typed context identities and keys")?;
            let identity = crate::phase::ContextArtifact {
                path: path.to_owned(),
                role: a.role.clone(),
                encoding: a.encoding.clone(),
                sha256: a.sha256.clone(),
                bytes: a.bytes,
            };
            ensure!(
                context.insert(path.to_owned(), identity).is_none(),
                "partial semantic repeated context"
            );
        }
        let mut phase_reader = Phases {
            root: phase_root.try_clone()?,
            reads: &mut budget.read_bytes,
        };
        if authoritative.is_none() {
            authoritative = Some(crate::phase::reconstruct_admitted(
                &mut phase_reader,
                &context,
                snapshot.manifest.binding.cell_index,
                &mut budget.phase,
            )?);
        }
        let bound = authoritative.as_ref().expect("constructed above");
        ensure!(
            bound == &snapshot.manifest.binding,
            "partial actual authoritative binding disagreement"
        );
        if snapshot.manifest.schema == MANIFEST_V2 {
            phase = Some(crate::phase::audit_admitted(
                &mut phase_reader,
                &context,
                bound,
                &mut budget.phase,
            )?);
        }
        if let Some(delta) = &snapshot.manifest.delta {
            captured = Some(capture::replay(
                delta,
                captured.as_ref(),
                bound,
                &mut capture_reader,
                budget,
            )?);
        }
        chain::coverage(&snapshot.manifest.coverage, captured.as_ref())?;
    }
    Ok((captured, phase))
}
fn add_expected(
    expected: &mut BTreeSet<(String, bool)>,
    path: &str,
    budget: &mut Budget,
) -> Result<()> {
    let mut current = path;
    let mut directory = false;
    loop {
        budget.charge(
            current
                .len()
                .checked_mul(8)
                .and_then(|n| n.checked_add(1024))
                .context("expected namespace reservation")?,
            "expected namespace",
        )?;
        expected.insert((current.to_owned(), directory));
        if let Some((parent, _)) = current.rsplit_once('/') {
            current = parent;
            directory = true;
        } else {
            break;
        }
    }
    Ok(())
}
fn inventory(
    root: &Root,
    anchor: &Anchor,
    chain: &[Snapshot],
    admitted: &Preflight<'_>,
    incomplete: bool,
    budget: &mut Budget,
) -> Result<()> {
    let mut expected = BTreeSet::new();
    add_expected(&mut expected, ANCHOR_PATH, budget)?;
    add_expected(
        &mut expected,
        if incomplete { MARKER } else { REPORT_PATH },
        budget,
    )?;
    if !incomplete {
        let report = root.open_file(REPORT_PATH)?;
        ensure!(
            report.metadata()?.len() <= REPORT_MAX as u64,
            "consumer_resource_limit: saved report size"
        );
    }
    for snapshot in chain {
        for kind in ["catalog", "receipt", "manifest"] {
            add_expected(
                &mut expected,
                &chain::metadata_path(&snapshot.anchor, kind, budget)?,
                budget,
            )?;
        }
    }
    for member in &admitted.members {
        add_expected(
            &mut expected,
            &chain::artifact_path(anchor, member.artifact, budget)?,
            budget,
        )?;
    }
    ensure!(
        root.inventory(budget)? == expected,
        "partial exact restored namespace mismatch"
    );
    Ok(())
}
