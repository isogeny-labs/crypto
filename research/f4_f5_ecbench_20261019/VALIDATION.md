# Requirement and validation receipt

The measured source is **9bed104b0a**. Results from the earlier two source
contracts are retained separately. All six dedicated jobs completed with exit0. The final source is analyzed
separately from both retained intermediate versions.

| Requested work | Status | Evidence and remaining requirement |
|---|---|---|
| Improve F4/F5 matrix construction | Implemented and correctness verified | Direct parity packing, cached active layouts, specialized column lookup, explicit adaptive degree policies; exact ordinary-row comparison and complete-root checks in `matrix_runtime_` tests. All32 paired F4/F5 cells pass the native improvement gate. |
| Improve recursive root solving | Implemented and correctness verified | Bounded quadratic Gray tail after algebraic reduction; root order/caps, original-system verification, degree/width/budget/custom-oracle fallbacks tested. All32 paired F4/F5 cells pass the native improvement gate. |
| Compare F4/F5 with F6, general polynomial solvers, generalized roots and both Semaev backends | Verified complete on admitted n17/n23 fixtures | Frozen18-arm/72-cell panel,32 paired changes, fresh holdouts, A/A controls and four-worker matrix/full-table controls. All72 cells have five admitted observations; all32 paired changes are faster; six four-worker controls are neutral;40 rejected attempts remain. The original n19 matrix-admission gap stays open. |
| Full ecbench end-to-end runtime | Verified execution and native comparisons complete; timing coverage partial | Both original declared curve sizes;11 arms each,8 public targets,5 measured rounds plus1 warmup, complete cold solves and whole-process clocks. Both528-record sessions verify; all440 measured replays per curve reproduce. Four F4/F5 gates admit lower solve times outside A/A noise; n17 F5 uses39 L2 pairs, other three use40. Some short table/rho clocks remain below L2. |
| Finish the available native generalized solver | Bounded native implementation verified | Standalone SDK13 passing tests; typed/scalar/wide Boolean paths and generic polynomial controls included in the panel. General hard implicit-domain and8x qualification remains the separate open problem. |
| Full-price or external-fastest/scaling claim | Not established | Unpriced native work remains; no matched external-runtime panel, other-hardware or four-size scaling qualification. These are not inferred from native wall times. |

## Local checks of the final source

- `cargo test --release --locked --lib`: **4414 passed,0 failed,118 ignored**;
  full output and exit in `records/final-adaptive-library.*`. The recorded
  command enables the previously qualified single-worker policies. This is
  correctness validation, not a local native timing claim.
- All17 `ecbench` integration tests pass; the expanded policy/generalized
  integration executes48 verified recoveries and48 exact replays across
  Koblitz and generic binary curves: `records/final-adaptive-integration.*`.
- The touched `f4_f5_boolean_perf` example compiles and its test harness
  passes (it declares zero unit tests): `records/final-touched-example.*`.
- Standalone generalized SDK13 tests and strict all-target Clippy pass:
  `records/final-adaptive-sdk.*`.
- Strict root Clippy fails with the unchanged PMAC
  `mismatched_bit_width_type` lint at `src/symmetric/modes/pmac.rs:112`.
  `records/final-adaptive-clippy.*` preserves exit101. The separate scoped
  diagnostic allows only that existing lint and passes; it does not waive
  the strict failure (`final-adaptive-clippy-scoped.*`).
- Both local adaptive screens verify16 records, including8 warmups, and
  replay8 measured records. The pre-version screen uses its exact frozen
  source; the versioned screen uses current policy identities. L0 screen
  elapsed times are not performance evidence.
- The full legacy n17 session verifies528 records and reproduces440
  measured runs on the current binary with zero problems. Its88 warmups
  are verified but excluded by native `--replay-all` semantics:
  `records/legacy-policy-current-full-replay.audit.json`.
- Every runtime source in `candidate-source.sha256` matches the final
  source after the temporary frozen-screen replay restores the current
  module: `records/final-source-verification.txt`.

Failed commands, the unsupported full-table test fixture, initial local
lock refusal, interrupted `/tmp`-capacity validation and the full-pipeline
regression are retained in `EXECUTION.md`, raw trees and their logs. No
performance admission threshold or sealed record was changed.

The owner merge preserves measured runtime bytes. Cover assessment verification
passes with normal subprocess access; the refreshed experiment's locked
metadata resolves. With repository-pinned Cairn9.1.0, backlog integration
passes13/15 including the owner's changed concurrency test. Two tests fail
on the unchanged baseline quintic card's unresolved {artifact} placeholder.
Logs are retained in `records/owner-merge-backlog-pinned.*`; this is not a
passing full-backlog claim.

## Final evidence and generated views

The deterministic archive contains20123 raw files in3008848 bytes. Its
SHA256 sidecar verifies; extracted-tree repacking is byte-identical. All11
Rust-prepared summaries reproduce byte for byte. Every one of the11 archived
sealed sessions also passes a current-binary integrity audit with replay0;
these integrity checks supplement the original full replay certificates.

All65 pages of the14 final PDFs were opened and visually inspected, including
the final9-page native report and4-page results report. Their hashes are in
`records/inspected-final-pdfs.sha256`. Every benchmark copy comes from the
same renderer, with SVG and150dpi PNG graphs.

Leaderboard JSON/Markdown/HTML and the browser are regenerated and pass their
currentness checks. Every existing leaderboard value compares equal to the
pre-continuation snapshot; only the separately labeled latest component
section is added. Both workflow path filters cover all220 builder inputs.
The browser indexes all11 archived sessions. Its6 archive/input tests and
2 price-admission tests pass; all4 workflow-parser tests pass; browser
JavaScript parses; the Rust analysis files pass rustfmt checks.

Current-head CI remains an external publication/merge gate. The last observed
source head6f0b8518f has40 queued checks and no passing-suite completion; it
is not merge-ready. A new evidence commit requires fresh current-head CI.
No pending check is counted as a pass.
