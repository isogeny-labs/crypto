# Library gates repaired; shared current-main CI failures remain

- **Status:** `open`
- **Kind:** repaired local library/lint regressions; remaining shared artifact, frozen-lock and workflow input failures
- **Affected work:** [Rust extension and hardening](../docs/rust-extension-hardening.md), [IC integration validation](ic-live-trace-main-compile.md)
- **Next action owner:** #2101 authoring session for final-head checks; research registry, backlog, leaderboard and frozen-replay maintainers for shared CI repairs
- **Last updated:** 2026-10-17

## Problem

The four library failures and two maintained-target lint findings originally
blocking PR #2101 are now repaired and validated locally: 4,393 release library
tests pass, 117 existing tests remain ignored, all 53 selected consumers pass,
and full strict Clippy passes on Rust 1.98.0 and 1.99.1. All workspace targets
type-check on 1.99.1. Required final-head CI remains a separate condition.

The published c78a36db check suite also exposes shared baseline failures in
frozen replay dependency locks, sparse checkout inputs, curve registration,
leaderboard freshness and a backlog card. Those failures prevent merging;
neither local test success nor recording this problem satisfies those gates.

## Evidence and uncertainty

The initial observations below are retained with their original scope. The
follow-up repair and additional current-main CI evidence are recorded after
them; old failure logs are not replaced with passing output.

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
- **Confirmed in follow-up:** the committed traits artifact was schema `/1`,
  while the native producer and types require `/2`. Rebuilding from the same
  six input files supplies the actual levels and source hashes, retaining 586
  unresolved levels explicitly. No missing-field defaults were introduced.
- **Confirmed in follow-up:** the calibrated genus-8 operation price is 2620.9
  multiplications, not the former 640. The c2pnb304w1 total is 160.259 formula
  bits versus source rho at 160.226, hence `Boundary` with ratio -0.033. A
  controlled regression restores the old +2.001 margin by changing that one
  price in a temporary model only. Production model parameters are unchanged.
- **Remaining implementation/data gap:** coherent frozen-replay dependency
  environments, complete CI checkout inputs, reviewed curve registrations and
  aliases, current leaderboard artifacts, and a defined backlog verifier.
- **What this does not establish:** a failure of the new cryptographic helpers,
  a mathematical obstruction, or permission to weaken a verification gate.

## What would unblock it

- [x] Reconcile the committed trait artifact with the strict schema through
  the native evidence workflow, retaining unknown/unproved facts explicitly.
  Do not fabricate `levels` certificates or silently default away proof data.
- [x] Review the `c2pnb304w1` route and cost-model calculation, correct the
  underlying inconsistency, and justify the intended classification.
- [x] Reconcile the two maintained-target lint findings
  with their source/evidence rules, then pass `scripts/clippy-maintained.sh`
  on CI's pinned toolchain without suppressing checks.
- [x] Pass the four formerly failing tests and
  `cargo +1.98.0 test --locked --release --lib` on the integrated source.
- [ ] Repair the shared CI failures below through their owning evidence and
  build workflows, without changing frozen facts or dropping validation.
- [ ] Pass required/applicable CI and review gates on the actual final PR head.

### Shared CI evidence at c78a36db

- Four replay jobs fail before compilation because they copy archived locks
  over the current Cargo manifest: [four-policy](https://github.com/aburan28/crypto/actions/runs/39069469325/job/117503875583),
  [pair-table](https://github.com/aburan28/crypto/actions/runs/39069469372/job/117503874714),
  [table-floor](https://github.com/aburan28/crypto/actions/runs/39069469338/job/117503874669)
  and [unknown-scalar](https://github.com/aburan28/crypto/actions/runs/39069469342/job/117503874794).
  Cargo rejects the resulting graph with `--locked`. The Frobenius workflow
  also pins the archived lock's SHA-256. A repair must preserve that archived
  environment and explicitly validate any current-source compatibility build;
  replacing the pin, regenerating the frozen lock or removing `--locked` would
  not satisfy the original check.
- [Conductor native controls](https://github.com/aburan28/crypto/actions/runs/39069469274/job/117503874512)
  omit `tools/volcano-cli/Cargo.toml` from the sparse checkout. This is an input
  wiring failure, not evidence of an arithmetic obstruction. The later
  workflow contains fresh measured-child runs; none were launched locally.
- [Curve names](https://github.com/aburan28/crypto/actions/runs/39069469292/job/117503874461)
  reports 4,042 findings, including unregistered identifiers and alias-map
  drift. The same official text linter reports 4,042 on an isolated, unchanged
  main c6d64436 worktree. Registration needs the actual model/trace evidence
  and dependent alias updates, not an allowlist of unknown identifiers.
- [Leaderboard](https://github.com/aburan28/crypto/actions/runs/39069469343/job/117503874581)
  reports stale `docs/ic/leaderboard.json`, `docs/ic/LEADERBOARD.md` and
  `docs/ic-leaderboard.html`. Its source inputs and builder are unchanged here.
- [Backlog](https://github.com/aburan28/crypto/actions/runs/39069469328/job/117503874395)
  rejects `XP-20261017-quintic-support-frobenius`: `outputs.verify` uses
  `{artifact}` without a value. The [lab job](https://github.com/aburan28/crypto/actions/runs/39069469328/job/117503874183)
  has three related failures. A repair needs the intended artifact and real
  verifier semantics; inventing a success command or quarantining the card
  would not preserve its requirement.

The Cargo manifests/locks, affected workflow definitions, registry, alias map,
name linter, backlog inputs and leaderboard builder have no diff from main
c6d64436. Full decoded CI logs, the independent main naming control and
source hashes are retained in the
[follow-up validation record](../docs/validation/rust-regression-repairs-20261017/README.md).
Related historical delivery failures are recorded in
[the ECC arithmetic merge-gate problem](ecc-arithmetic-pr-merge-gates.md).

## Next discriminating step

- **Action:** check the new PR head, then have the owning workstreams supply
  the missing frozen-build compatibility, registrations, artifact freshness
  and verifier definitions. Preserve their source evidence and original gates.
- **Inputs and controls:** current source hashes, retained c78a36db CI logs,
  clean-main naming control, unchanged manifests/locks and research inputs.
- **Execution limits and stop condition:** local regression validation and
  existing catalog maintenance only. No new research campaign or paid job.
  Stop promotion on a changed frozen fact, missing proof or failed gate.
- **Outcomes:** completed passing final-head CI/review permits integration;
  shared failures remain explicit blockers rather than being patched around.
- **Recheck trigger:** final-head CI/review activity or an accepted upstream
  repair of a named shared gate.

## Resolution and history

2026-10-17: opened from integrated Rust regression checks; isolated errors
reproduced without editing their sources or assertions. Unresolved.

Final network-enabled release gate: 4,387 passed, 4 failed, 117 ignored
(4,508 total), exit 101. All 49 selected release IC/extension/example tests
pass. All binaries type-check, CI-pinned formatting passes, and changed-target
strict Clippy passes on Rust 1.98 and 1.99; the full maintained-target lint
findings above remain. The exact diagnostics and source hashes are retained in
[the validation record](../docs/validation/rust-extensions-20261017/README.md).

2026-10-17 follow-up, explicitly requested repairs: native catalog refreshes,
the justified classification regression and two lint repairs pass the full
release library gate (4,393 passed, 0 failed, 117 ignored), all 53 selected
consumers, full strict maintained-target Clippy on 1.98.0/1.99.1 and workspace
all-target checks. The earlier four failures and two lint findings are repaired.
The managed workspace restart interrupted an earlier attempt; the completed
reruns and surviving evidence are retained separately. This record remains
open for the shared final-head CI/review gates above. No entire-crate timing
resistance or research performance conclusion follows from these checks.
