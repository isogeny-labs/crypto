# Current-main research fixtures, classification and lint prevent green gates

- **Status:** `open`
- **Kind:** fixture/schema mismatch, unresolved classification regression and unrelated maintained-target lint findings
- **Affected work:** [Rust extension and hardening](../docs/rust-extension-hardening.md), [IC integration validation](ic-live-trace-main-compile.md)
- **Next action owner:** unassigned; curve-traits, Jacobian-transfer and research-driver maintainers
- **Last updated:** 2026-10-17

## Problem

After repairing main's missing library wiring, the integrated release library
gate fails in unchanged research modules: three committed-fixture deserialization
tests and one Jacobian route-classification expectation. The full maintained
lint script also rejects two unchanged research targets. These failures prevent
passing integrated-head gates and merging PR #2101.

## Evidence and uncertainty

- **Observed:** main `c6d64436` and trace head `459eceff` cannot compile before
  the wiring repair documented in the linked problem. On that head plus the
  repair/extension work, the release suite reaches 4,506 tests and reports the
  failures below. Final counts and later reruns are recorded in the extension
  validation record; this initial run predates the shared GF(2^128) refactor.
- **Observed:** `curve_traits::query` tests
  `neighbours_of_ecc2k_130_share_its_cm_field_and_descent`,
  `the_koblitz_family_is_one_group_across_sizes` and
  `unknown_values_never_match` parse committed `docs/curves/traits.json` and
  fail with `missing field levels`, line 66 column 7. The current
  `EndomorphismRecord` requires that field. The source and artifact are
  unchanged by the extension/hardening work.
- **Observed:** `jacobian_transfer::tests::trace_of_a_decides_whether_the_descent_exists`
  reaches `jacobian_transfer/mod.rs:1189` with `Boundary`, expecting
  `FormulaMarginal` for `c2pnb304w1` under the default cost model. That source,
  registry data and cost model are unchanged here. A direct isolated replay
  reproduces it; the three fixture failures share the independently replayed
  deserialization error.
- **Observed:** CI's `RUSTUP_TOOLCHAIN=1.98.0 bash scripts/clippy-maintained.sh`
  flags `drop_non_drop` at
  `research/full_volcano_graphwide_ic_20261017/execution/driver.rs:206`
  (the normal and test forms of `volcano-ic-driver`) and
  `manual_is_multiple_of` at `examples/n83_precomputed_rho.rs:249`.
  The full Rust 1.99 script also reports the latter. Both files are unchanged;
  strict Clippy for all changed library/IC/consumer targets passes on both
  toolchains. No lint exception or source rewrite was used to hide these findings.
- **Derived or proved:** these errors are in paths outside the modified
  arithmetic and plug-in APIs. An untouched-main executable cannot serve as
  a control until its compile failures are repaired; this is not a claim that
  unmodified main's whole suite passed or ran.
- **Suspected explanation:** the committed traits artifact predates the
  required schema. The classification mismatch's intended resolution is not
  established by this task; changing an expected value would not justify it.
- **Missing capability:** current-schema, evidence-backed committed trait data
  and a reviewed explanation/repair for the route-classification mismatch.
- **What this does not establish:** a failure of the new cryptographic helpers,
  a mathematical obstruction, or permission to weaken a verification gate.

## What would unblock it

- [ ] Reconcile the committed trait artifact with the strict schema through
  the native evidence workflow, retaining unknown/unproved facts explicitly.
  Do not fabricate `levels` certificates or silently default away proof data.
- [ ] Review the `c2pnb304w1` route and cost-model calculation, correct the
  underlying inconsistency, and justify the intended classification.
- [ ] Have the research-target maintainers reconcile the two lint findings
  with their source/evidence rules, then pass `scripts/clippy-maintained.sh`
  on CI's pinned toolchain without suppressing checks.
- [ ] Pass the four isolated tests and `cargo +1.98.0 test --locked --release --lib`
  on the integrated PR head, followed by required CI/review checks.

## Next discriminating step

- **Action:** schema-maintainer review of the first committed record and
  route-maintainer review of the actual/expected `c2pnb304w1` classification;
  research-target maintainer review of the full lint diagnostics.
- **Inputs and controls:** unchanged committed traits/registry data; isolated
  failing tests and the full release suite, retaining their raw failure logs.
- **Execution limits and stop condition:** local regression diagnostics only.
  No research campaign, artifact regeneration or paid job was launched to
  change these unrelated expectations in the extension task.
- **Outcomes:** a justified repair permits a fresh integrated gate; missing
  evidence or continued mismatches leave the PR open.
- **Recheck trigger:** a reviewed repair of either source/artifact mismatch.

## Resolution and history

2026-10-17: opened from integrated Rust regression checks; isolated errors
reproduced without editing their sources or assertions. Unresolved.

Final network-enabled release gate: 4,387 passed, 4 failed, 117 ignored
(4,508 total), exit 101. All 49 selected release IC/extension/example tests
pass. All binaries type-check, CI-pinned formatting passes, and changed-target
strict Clippy passes on Rust 1.98 and 1.99; the full maintained-target lint
findings above remain. The exact diagnostics and source hashes are retained in
[the validation record](../docs/validation/rust-extensions-20261017/README.md).
