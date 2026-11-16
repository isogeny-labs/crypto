# Development validation (not experiment evidence)

Toolchain: rustc 1.99.1 (58cf8f9c3 2026-10-13), x86_64 Linux.
No experiment or wall-clock comparison was run. Commands use existing OpenSSL
libraries via OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-gnu and OPENSSL_INCLUDE_DIR=/usr/include.
Correctness builds use `--config 'profile.release.opt-level=0'`; these are
release-profile unoptimised checks and do not qualify performance figures.

## Completed checks

- `cargo test --release --lib --locked -j 1 --config 'profile.release.opt-level=0' trace_line`:
  **4 passed, 0 failed**. See [focused-tests.log](focused-tests.log).
- Both signs of every nonzero selected-line abscissa at N=8,12,16 lift and
  pass the general-arithmetic curve equation; standard-line elements satisfy
  the embedded subfield equation. These are finite correctness fixtures.
- Invalid dimensions, policies, bases, twist selection and scan caps fail closed.
- Both standard and selected four-summand 2LP fixtures recover a known scalar;
  every accepted raw relation and projected equation independently replays.
  Changed query coefficients, changed linear coefficients and truncated
  transcripts are rejected.
- The ecbench method adapter retains all accepted relations and leaves native
  construction work explicitly unpriced. No planted scalar is passed to solve.
- PR review fixes make replay fail closed on the authoritative
  `outcome.recovered` value and reconstruct the exact projected column table
  and small-column boundary from the declared `small_dimension`. The focused
  gates still pass; see [review-fixes.log](review-fixes.log).
- After merging current `main`, both focused gates still pass and the reviewed
  trace-line/replay bytes remain unchanged. The targeted zero-weight,
  hyperelliptic and rule-v8 rechecks also pass on the merged base once the
  tracked sparse fixtures are materialised. The exact merge audit and the
  remaining artifact-backed leaderboard publication blocker are recorded in
  [merge-validation.log](merge-validation.log).
- The blocked backlog card passes the repository's original native validator
  (`src/bin/backlog/card.rs`, built with the original SHA-1 source and existing
  serde_json/hex rlibs); see [backlog.txt](backlog.txt). This is card validation,
  not a queue publication or execution.
- `rustfmt --edition 2021 --check` on touched Rust sources and `git diff --check` pass.
- REPORT.pdf was rendered and visually inspected; it has one page and a source
  SVG status diagram. It reports implementation status rather than performance.

## Retained failed development attempt

[development-first.log](development-first.log) records the first tests: two
passed and the scalar fixture failed because a tiny subgroup gave a zero
planted target. Tests and prospective specs use max_cofactor=16 after that
correction, before any measurement. These development fixtures are not the
registered comparison sample. No favourable experimental row was dropped.

The initial optimised build was stopped before tests; sparse compile-time
fixtures and OpenSSL discovery were fixed without changing their source bytes.

## Measurement gates

[checkpoint.txt](checkpoint.txt) records the failed S3 checkpoint upload: no
existing profile or upload token. Conductor is unavailable. Iso Lab submission,
complete C1 pricing, final native protocol/schema freeze, actual sessions,
experiment.export/v1, independent host audit and review, external tool timings,
and VALIDATION.json remain missing. None is represented by a fabricated receipt.

The protocol's DESIGN.sha256 is a file-integrity manifest only, not a canonical
experiment-protocol freeze certificate. All complete pipeline costs stay unknown.

## Retained whole-library and lint history

The complete library suite on the original source head was invoked with the
same unoptimised release profile. It was stopped after failures were diagnosed
(exit 130), not reported as a passing or completed suite. Ten initially failing
fixture-dependent tests passed on exact rerun after materialising their tracked
sessions. The retained [fixture rechecks](fixture-rechecks.log) preserve three
then-current symptoms:

- `weighted_comparison_prices_verified_pairs_and_records_weight_identity`:
  assertion `zero.legacy_comparison_id.is_none()` at compare.rs:2448. This is
  the existing [zero-weight compatibility obstruction](../../../problems/ecbench-weighted-zero-legacy-id-contract.md), with a changed symptom on this base.
- Three existing `hyper_ic` tests reject the `branches` parameter of
  `ic.hyperelliptic`; this change does not edit that registration or engine.
- A rule-v8 test needs its additional tracked parameter fixture; no result is
  inferred from that absent file.

The strict Clippy check also fails at unchanged zero_dim_la.rs:1191
(`manual_range_contains`) and sat.rs:1343 (`manual_is_multiple_of`), as
[clippy.log](clippy.log) and [review-fixes.log](review-fixes.log) record. No lint or test was disabled or changed.
After the current-main merge, exact targeted rechecks pass for the zero-weight
test, all seven hyperelliptic tests, and the rule-v8 test after its six tracked
parameter directories are materialised; see [merge-validation.log](merge-validation.log).
The full suite was not rerun, so these targeted results are not represented as
a completed whole-library green gate. The historical failures remain preserved
rather than rewritten.

## Additional completed preflight

- `cargo test --release --example trace_line_replay --locked -j 1 --config 'profile.release.opt-level=0'`:
  **1 passed**, exercising serialized native records, general-arithmetic replay,
  corruption rejection, empty input and a file containing no matching method.
  See [replay-example.log](replay-example.log).
- `cargo build --release --bin ecbench --locked -j 1 --config 'profile.release.opt-level=0'` succeeds.
- All four prospective specs expand with `ecbench plan`: 448 planned executions
  each, 1792 total, zero measurement executions. Plans are retained as
  `plan-d2.txt` through `plan-d5.txt`. Their exact curves remain unregistered
  (`registered=false`, no EC1) and must be registered before measurement claims.
- The initial five-round design was changed to six before any measurements
  because the native planner correctly warned that five alternating rounds
  cannot balance arm order. No measurements informed this change.
