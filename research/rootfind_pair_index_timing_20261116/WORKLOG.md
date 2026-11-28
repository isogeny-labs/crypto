# Work log

## 2026-11-16, protocol freeze

### What was done

- Read the previous K13/K19/K23 first-witness and storage-parity records,
  the binary pair-table source and the program paper. KB revision
  `9141e386254340a9bdba3036f2bb706a` predates the current prototypes.
- Checked the live correct-account EKS budgets: monthly USD 2,094.914
  against 1,300 and daily USD 216.868 against 200. The frozen L2 panel is
  not admitted at those readbacks.
- Conductor T-6708 still reserves both production collector source files.
  T-6814 claims this separate study, test and resume paths. [pcc:unverified]

### Decisions made

- Measure a direct-build table component on actual Koblitz pair/probe
  streams, with hash A/A and every correctness check. This is a local L0
  selection screen for later production integration, not the F4/F5/F6
  complete-call panel. [pcc:unverified]

### Open threads

- Implement and commit the standalone harness, execute once with the
  checkpoint gate, retain raw rows, and publish the visual run report.

### Errors

- The frozen draft described a default `u32` hash reference. A source read
  before implementation showed that production uses `u64` keys, a custom
  `MixHasher`, and capacity `a*a+1`. Pre-source Amendment 1 corrects the
  control without changing inputs or timing boundaries. [pcc:unverified]

## 2026-11-16, source implementation

### What was done

- Added a standalone release harness that reconstructs the ordered Koblitz
  torsion pair streams, checks every duplicate as a group equation, and
  precomputes 64 public residual probe streams per cell before timing.
- Added hash, hash A/A, direct flat `u32`, paged and inline index arms. The
  hash control copies the production `MixHasher` and `a*a+1` reservation;
  every arm checks first witnesses and lookup checksum before timing.
  Formatted the source with installed rustfmt 1.8.0. [pcc:unverified]

### Decisions made

- Commit source and lockfile before the compile-only check and fixed run.

### Open threads

- Compile the source-bound release harness and repair any pre-run source
  error by a new commit, then pass the S3 checkpoint gate and execute once.

### Errors

- Rustup's 1.99.1 toolchain lacks rustfmt, so the installed rustfmt 1.8.0
  formatted the file. The first locked release compile-only command exited
  101 because the harness passed a `usize` dimension to an existing `u32`
  factor-base API. Complete output and UTC/exit are retained in `build.*`;
  a one-type source correction follows in a new commit. No timed result has
  been obtained. [pcc:unverified]

## 2026-11-16, corrected compile and run admission

### What was done

- Committed the one-type correction and first compile failure at
  `1b91f79a5c`. Repeated the locked release compile-only command with the
  same explicit Rust 1.99.1 Cargo/rustc and shared target. It exited zero
  in the UTC interval saved in `build2.meta`; full stderr/stdout are kept.
  The source-bound executable SHA-256 is `4f8e218be771...`. [pcc:unverified]
- Updated the branch-specific resume with protocol, amendments, source,
  binary, run command, stop conditions and S3 checkpoint steps.

### Decisions made

- Push a fresh checkpoint before the one frozen local diagnostic. The
  monthly and daily correct-account AWS budgets remain over their limits,
  so the L2 F4/F5/F6 panel is not submitted.

### Open threads

- Commit the corrected build receipt and resume, pass S3 gate, execute once,
  derive result and visual report, and open a stacked PR.

### Errors

- The Rust toolchain's `rust-objcopy` emitted its known missing
  `libLLVM.dylib` warning; the corrected compile returned zero.

## 2026-11-16, source-bound local timing run

### What was done

- `scripts/checkpoint.sh gate` exited zero and uploaded the branch-specific
  handoff to the S3 URI in `RUN_RECORD.json`. The downloaded `RESUME.md`
  matched local bytes at SHA-256 `2cd71da8aa01...`.
- Checked the executable SHA-256 immediately before the run. The one frozen
  test under the 600-second watchdog exited zero, passed 1/1, emitted three
  fixed-case records and all 1,515 planned measured rows. Saved its exact
  command interval, stdout, stderr and whole-process resource counters.
  The harness checked every duplicate pair as a group equation, all
  first-witness lookups and every arm checksum. [pcc:unverified]
- `analyze.py` asserted the identity/count/row matrix and derived
  `RESULTS.json` from immutable `run.stdout`. Its comparison is limited
  to fresh index insertion plus precomputed-key lookup under L0.

### Decisions made

- Archive the raw run and machine derivation before the derived visual
  report. The local component result cannot decide complete-call ordering
  against F4/F5/F6. [pcc:unverified]

### Open threads

- Commit and push run records, then render and inspect the editable graph,
  report and PDF, and open the ready stacked PR.

### Errors

- None in the bounded run. The 0.38-second `/usr/bin/time -l` wall value
  covers the entire harness and is not a per-arm interval.

## 2026-11-16, derived visual report

### What was done

- Confirmed raw run commit `11abb8a59a` was pushed before deriving this
  report. Rendered the chart from `RESULTS.json`, built the one-page PDF from
  editable Typst source, and inspected a Poppler-rendered page plus extracted
  text. Its caption names the run, nanosecond units and 101 repetitions per
  arm. [pcc:unverified]
- Updated the branch handoff to distinguish the completed local component
  screen from the still-unrun matched F4/F5/F6 panel.

### Decisions made

- Keep this result at descriptive L0 scope. No canonical leaderboard or
  complete-call performance graph changes without the admitted panel and
  independent verification. [pcc:unverified]

### Open threads

- Push the report and open the ready stacked PR. Integrate direct-build flat
  only after T-6708 releases production collector files; then measure its
  complete call under a fresh frozen contract.

### Errors

- No rendering or document-inspection error. The earlier source compile
  refusal remains archived and is not a timed trial.
