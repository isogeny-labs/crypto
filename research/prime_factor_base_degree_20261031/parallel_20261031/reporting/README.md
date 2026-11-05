# Native descriptive reporting after data

This is a **post-data descriptive audit**, not a replacement for the frozen
analysis, a new preregistered decisive hypothesis, a new CAS run, or a retry.
It changes no instrument, protocol, original result, raw basis, trace or v1
extraction cap. The parent requested this additional accounting after observing
the completed toy panel and the original larger-field verification caps.

## Reproduce

```sh
cargo test --release --locked --offline --bin secant-descriptive-report
cargo clippy --locked --offline --bin secant-descriptive-report -- -D warnings
cargo build --release --locked --offline --bin secant-descriptive-report
target/release/secant-descriptive-report REPO_ROOT AUDIT_DIR F4_ROOT \
  FRESH_REPORT.json REPLAY_V2_ROOT LADDER_ROOT
```

Audit, replay and ladder arguments may be `-` where documented by the CLI.
The run outputs a full JSON report and `FRESH_REPORT.json.compact.json`.
The latter is explicitly capped at 64 KiB. Existing destinations are refused.
There is no process-launch or solver execution path in this program.

`v1.rs`, exposed as `secant-descriptive-report-v1`, preserves the initial
four-argument reporting source that produced the toy report cited by
`audit/REPORT_EVIDENCE.md`. Its source SHA-256 is
`6a87814ff1609096e83b11b8636adb4f05b60f0dd6ce403fc41074f9d6ad0b74`.
The interim compact-v1-status reporter source is separately retained in the
raw root's `reporting-source-v1-status/`; its combined source hash is
`49dbb770de051a1ee04f7b56b0c50a2500ac9ff03987eb3db35637ba8533bcbd`.
The current report records the hash of its own source, helper modules, Cargo
manifest and lockfile. No historical report is overwritten.

The emitted all-v2 root-proof report is reproducible from
`snapshots/v2-root-proof/`. The combined main/helper/Cargo/lock source hash is
`70deb380d44e996e9c588e433a501118db9d04df9d8b6efc9d90ed746ed50f96`,
matching the report's recorded source hash. Its compact output is 43,708
bytes, SHA-256
`414f2bbee7f8e39a9d7358f06fa81ba8fd79dab2a14186a938bb9dfc37a65c87`.
The original raw-root v2 source-copy directory was a working copy;
this owned-directory snapshot is the byte-verified source preimage.

Current development checks pass four artificial tests in the current target
and four preserved v1 tests, current-target strict Clippy and formatting.
The initial current-target Clippy error (`manual_is_multiple_of`) is retained
in `development-checks-initial.log`; the current source uses the standard
method. The current optional-v2 frontend also conservatively leaves missing
or failed companions unverified. Every companion in the emitted final report
was present and verified, so these later frontend changes alter no reported
values. Existing emitted reports and frozen sources remain untouched.

## What is checked

- Frozen instrument source-file hashes from `source-lock.json`.
- Audit cell/reference manifest hashes and lengths, exact reported/reference
  root equality, all four sign branches, recorded assignment totals, empty
  discrepancy lists, independent recorded-relation group replay and analytic
  56/8/32/8 aggregate invariants. The 707,472-point search is not rerun.
- F4 panel result hashes and each recorded input JSON, tokens, msolve input,
  solver JSON and stderr hash, including original failed/capped records.
- Exact equality of recorded root/reference lists on v1 verified cases,
  metadata/point inventories and fresh standard group replay of relations.
- Each v2 companion file hash and its exact original-result/input/stdout/stderr
  linkage; all named two-inclusion proof gates; preserved v1 metrics; trace
  handled-degree definition; and fresh group replay of its reference relations.
- Ladder system/reference/attempt-file hashes and exact attempt-count equality.

The v2 proof is supplied by the separately reviewed offline replayer; this
reporter checks its bindings, declared obligations and recorded relations. It
does not claim to be another full polynomial normal-form implementation.
No ideal/multiplicity equality or operational extraction success follows.

## Full report schema

`secant-post-data-descriptive-report/v1` includes:

- `instrument_source_audit`, `toy_audit`, `construction` and raw manifest hashes.
- `f4_descriptive.groups`: strict **original v1 statuses**, retaining failed
  branches and unknown target-max degrees.
- `f4_descriptive.groups_with_v2_root_set_verification`: separately labeled
  saved-basis proof route, with missing/failed companions left unverified.
- `f4_descriptive.original_failed_or_capped_records` and the separately counted
  `original_failed_or_capped_records_proved_by_v2`. Caps remain operational caps.
- Per curve/design-d/arm/control: case and complete-target handled-degree
  histograms, input/selected-degree histograms, all-branch pair/row sums,
  maximum matrix columns/rows, natural target coverage and relation counts.
- Original panel summary hashes recursively bind every result; each recorded
  raw file hash is rechecked. Unlisted result files are surfaced, not dropped.

The degree aggregation is the target maximum over all four verified secant
branches. Pair/row counters sum all branches and targets, including actual
failed/capped attempt records where counters exist. A missing counter makes
the corresponding total null. Raw case degree histograms retain unknowns.
Target coverage has the completed-target denominator and does not silently
stand for an uncompleted population. `d` is recorded for ladder cells; the toy
matched size gives design d=2 and is explicitly labeled as derived metadata.

Field additions/multiplications, complete field cost per usable relation and
clock speedup remain **null**. Selected-pair/row proxies do not fill them.
The reporter does not price reference enumeration as relation discovery.

## Compact figure/dashboard schema

`secant-descriptive-compact/v1`, at most 65,536 bytes:

```text
original_v1_records / original_v1_verified / original_v1_failed_or_capped
v2_verified_saved_basis_cases / v1_caps_resolved_by_separate_v2_proof
operational_v1_cap_status_changed = false
cells[]
  cell = {p,a,b,d}; curve_slug
  construction = {attempts,forced_points,operations,status} or null for toy
  v1_recorded_status[]
  saved_basis_v2_verified[]
    arm; control; target_count; complete_targets; verified_cases
    unknown_or_failed_cases; handled_degree_histogram
    pairs_sum_all_branches; rows_sum_all_branches; peak_columns; peak_rows
    sat_targets; ordered_relations; complete_field_cost_per_relation=null
```

Each histogram maps a degree string to a target count; `unknown` is retained.
Visualization code may read these already computed values and lay them out
without recomputing research results. The full report and original hashes
remain the source for auditing the compact view. Report both v1 caps and v2
proof coverage when using the second table; do not relabel it a v1 extractor win.
