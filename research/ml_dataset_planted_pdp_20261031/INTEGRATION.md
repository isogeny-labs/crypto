# Native integration handoff for Conductor T-887

This is a reviewable source-change handoff, not a merged runtime patch. At
2026-10-31 16:43 UTC, T-887 still has an active exclusive reservation on
`src/ml_dataset/mod.rs` and no linked pull request. No source file under that
reservation was edited for this handoff. The companion [contract](CONTRACT.md)
defines the full acceptance gates and experiment freeze.

## Exact planner change

On the current main module, `PlanArgs` starts at line 227, `DatasetPlan` at
298, `DatasetPlan::cell` at 405, and the `DatasetCommand::Plan` construction
at 939. T-887 should integrate the following changes against its then-current
head, preserving its own edits.

```rust
// PlanArgs: omission means the legacy v1 serializer and constructor.
#[arg(long)]
pub target_populations: Option<String>,

// DatasetPlan: append after split_rule, so a v1 empty value is omitted from
// serialized bytes and binding_sha256 remains byte-identical.
#[serde(default, skip_serializing_if = "Vec::is_empty")]
pub target_populations: Vec<String>,
```

The plan command parses the option as a comma-separated, nonempty ordered
list, sets schema `crypto.ml-dataset.plan/v2` when present and otherwise
`crypto.ml-dataset.plan/v1`, and leaves `target_populations=[]` when omitted.
`validate` accepts exactly these two states:

```rust
match self.schema.as_str() {
    "crypto.ml-dataset.plan/v1" if self.target_populations.is_empty() => {}
    "crypto.ml-dataset.plan/v2"
        if !self.target_populations.is_empty()
            && self.target_populations.iter().all(|s|
                ["natural-v1", "planted-pdp-v1"].contains(&s.as_str()))
            && self.target_populations.iter().collect::<BTreeSet<_>>().len()
                == self.target_populations.len() => {}
    _ => return Err("invalid target population axis or plan schema".into()),
}
```

`cell_count` multiplies by `max(1,target_populations.len())` with the existing
checked overflow. In `cell`, take the new population digit immediately after
`key_index` and before `generator_index`; for v1, do not consume a new digit
and set `natural-v1`. Add `target_population: String` to `CaseConfig` with a
version-aware migration layer: direct `serde(default)` alone is insufficient
because old worker config digests and equality checks are frozen. The v1
decoder should supply `natural-v1` after authenticating old bytes; v2 must
require the field. Existing `DatasetPlan` literals in native tests and other
modules must be updated to empty population vectors without altering fixture
JSON. The new population axis is explicit in plan, cell, report and export
denominators; v1 cell indexes and RNG streams must remain unchanged.

The planner change is not safe to merge by itself: until `curves::generate`
and restricted capture support the planted branch, reject v2 planted cells at
preflight with an explicit unsupported status. Never let them fall through to
the natural constructor. Before enabling generation, implement the exact
known-log base and signed target construction in the contract, an independent
replay checker, restricted record role, and strict export exclusion in
`curves.rs`, `capture.rs`/`storage.rs`, and `export.rs` as appropriate.

## Source-level merge gate

1. Test serialization of existing `plan/v1` fixture bytes and
   `DatasetPlan::binding_sha256` before/after. Test v1 `cell(i)` equality for
   every index of a small frozen plan, including twist, generator and key.
2. Test CLI `dataset plan` without the option produces v1; with both values
   produces v2 and doubles the denominator; unknown/duplicate/empty values
   fail before storage. Test the exact index order at population boundaries.
3. Test v2 preflight rejects planted cells until the construction/replay and
   exporter are integrated. Once enabled, test both populations with the same
   axes and all outcomes charged, including construction failures.
4. Test a mock solver sees no private witness or planted-construction record;
   tampered signs, indices, base logs, scalar and target fail exact replay.
5. Test all model-input files and feature IDs stay byte-identical when only
   restricted planted metadata changes; enforce a recursive forbidden-key
   scan, not merely a top-level allowlist.

T-887 has no active GitHub PR and the Conductor CLI provides no comment or
message operation. The pending integration route is this Git-reviewed handoff
plus a fresh source reservation after T-887 releases `mod.rs`; it is not an
implicit handoff or permission to edit that reserved file.
