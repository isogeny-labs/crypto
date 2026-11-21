# IC compilation restored; integration gates tracked in the successor record

- **Status:** `superseded` by [current-main research regression gates](current-main-research-regressions.md)
- **Kind:** implementation gap in the integration baseline
- **Affected work:** [IC live relation output](../docs/ic/README.md#reports-and-resource-accounting)
- **Next action owner:** unassigned; recent quartic/ecbench integration maintainers
- **Last updated:** 2026-10-17

## Problem

The original compile obstruction below is repaired by the Rust extension update
in PR #2101. Library and all-binary type checks now pass; release integration
still has the unrelated regression/lint gates recorded in the successor. The
original requirement for passing integrated checks remains unmet.

The live relation/verbosity change cannot complete validation on main
`c6d64436` because the unchanged library already fails to compile outside
the trace paths. The original requirement remains live, selectable relation
output and factor-base plug-in documentation, with the ordinary solver
results, query streams, counters and JSON reports preserved.

## Evidence and uncertainty

- **Observed:** `cargo check --locked --lib --bin ic` on that integration
  baseline reports 25 errors. `ecbench/workload.rs` defines `Instance::Fp4`
  twice and refers to an absent `CurveSpec::PrimePublicFixture`;
  `ecbench::fbm` and `OnbWeightBase` imports are missing;
  `gaudry_quartic.rs` uses an undefined `sparsest` and
  `echelon_by_lead_with`. Remaining type/match failures follow those paths.
  These files were not changed by the live-trace implementation.
- **Control:** the earlier verbose-output PR #1880 head
  `14bf033e2ee85d05b465f6b99e685243d6b8a6fb`, without the new patch,
  passes `cargo check --locked --lib --bin ic` locally on Rust 1.99.1.
  Applying this PR's patch to that revision permits separate trace
  validation; that is not integrated-main validation.
- **Derived or proved:** no mathematical claim is involved.
- **Suspected explanation:** incomplete integration of recent independent
  changes; the exact responsible merge has not been established here.
- **Missing capability:** a compiling integrated main revision.
- **What this evidence does not establish:** passing checks on the earlier
  control cannot establish that the final current-main combination builds.

## What would unblock it

- [x] Restore the intended quartic types, imports, dispatcher cases and
  elimination entry points on main without dropping supported methods.
- [ ] Apply the live-trace change and pass
  `cargo test --release --lib --bin ic --test ic_framework --test ic_progress`
  on that exact integrated revision, plus applicable lint/review gates.

These conditions establish integration and regression validation, not a
performance improvement or a security result. Trace I/O has no speed claim.

## Next discriminating step

- **Action:** restore the missing integration wiring in the affected modules
  and rerun the exact checks above.
- **Inputs and controls:** main `c6d64436`; unchanged and patched #1880 head;
  deterministic synthetic fixtures in the existing IC test suites.
- **Execution limits and stop condition:** local compilation and regression
  tests only; no cloud campaign or mathematical search.
- **Outcomes:** a compiling base permits integrated validation; persistent
  unrelated errors keep this record open; trace-specific failures must be
  fixed before the feature is considered validated.
- **Recheck trigger:** a repair to the affected main modules.

## Resolution and history

Compilation restored; the complete integrated-validation condition remains
unresolved in the successor record. Creating or merging the feature PR does
not satisfy that condition. Historical control evidence is retained below.

2026-10-17: with the feature patch on the #1880 control revision, the release
library suite passed 4,297 tests (117 ignored). The IC binary unit tests
passed 10/10, CLI integration 25/25, and pipeline progress 5/5. The initial
verbosity-filter fixture covered every target and therefore had no misses;
it was replaced by a saved dimension-4 base containing both hits and misses,
and the CLI tests passed on rerun. The manual preview emits 9 successful
relations at `-v`/`-vv` and all 64 attempts at `-vvv`, with identical recovered
scalar and counted work. These are correctness and UI checks, not a speed
measurement, and do not resolve the integrated-main build gate.

Strict Clippy for the library, IC binary and both integration targets also
passes on CI's pinned Rust 1.98.0. Rust 1.99's new
`mismatched_bit_width_type` lint instead stops in unchanged
`src/symmetric/modes/pmac.rs:112`; no lint was suppressed for this feature.

2026-10-17 Rust extension update: restore the distinct D4 / translation quartic
instances, constructors and dispatcher cases, missing FBM / ONB imports and
elimination wrapper, plus the IC `prime` module and ecbench's FBM import. The
all-binary type check passes. On the integrated code, the release library gate
reports 4,387 passed, 4 failed and 117 ignored; all 49 selected IC, extension
and embedding-example tests pass. The four failures and full maintained-lint
findings are documented with raw logs in the successor and
[validation record](../docs/validation/rust-extensions-20261017/README.md).
The PMAC bit-width expression is also corrected as part of shared, branchless
field-offset arithmetic; changed-target strict Clippy now passes on Rust 1.98
and 1.99. The crate remains unaudited; these are scoped correctness checks.

2026-10-17 requested regression repairs: the complete local release library
gate now passes 4,393 tests (117 existing ignored), all 53 selected consumers
pass, full maintained strict Clippy passes on Rust 1.98.0 and 1.99.1, and
workspace all-target checks pass. Current trait/transfer catalogs and the
calibrated classification expectation are reconciled without weakening
schemas or changing cost-model inputs. The successor remains open for the
shared research CI and final-head review gates; see the
[follow-up evidence](../docs/validation/rust-regression-repairs-20261017/README.md).
