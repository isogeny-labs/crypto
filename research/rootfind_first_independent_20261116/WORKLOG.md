# Work log

## 2026-11-16, protocol freeze

### What was done

- Inspected the clean stacked branch at `c2468c8f5d`, the block-tail
  result and production collector; fetched `origin/main` at
  `a91b8b66d0f1ffc175fdce984d1059144ce232f6`.
- Queried shared KB revision `9141e386254340a9bdba3036f2bb706a` and
  read the supplied rootfinding PDF and the repository's Koblitz relation
  note. Sources checked (KB): `source:883ab40565c1f261353efc03ba04473e`
  at `crypto:research/notes/ecc2k130/RESEARCH_KOBLITZ_INDEX_CALCULUS.md`
  led to the rank-aware stopping rule; the recent block-tail report is newer
  than KB refresh.
- Conductor T-6795 reserved this study directory and test path. Conductor
  T-6708 still reserves the production collector paths; T-6720 reserves
  today's shared session log.

### Decisions made

- Test first-independent scanning with fixed disjoint K19/K23 streams and
  full K13 enumeration. Freeze the operation proxy and answer checks before
  implementation and data.
- AWS STS returned account `590183823895`, not the required IsoLab account
  `104186811093`. The existing monthly budget receipt already exceeded its
  limit. Do not submit the frozen EKS panel from this identity.

### Open threads

- Implement the standalone harness, run once from committed source, derive
  the result and visual record, publish in the existing stacked PR.
- Reconcile today's shared session log when T-6720 releases it.

### Errors

- The current AWS identity is the wrong account for IsoLab admission.

## 2026-11-16, source implementation

### What was done

- Added `tests/rootfind_first_independent.rs`, adapting the source-bound
  Koblitz harness to maintain three independent rank bases. The candidate
  probes until its first rank-advancing triple or the full scan; all
  candidate triples are re-added in the group before insertion.
- Formatted the source with Rust 1.99.1 and checked the diff. No scientific
  run has occurred yet. [pcc:unverified]

### Decisions made

- Keep source separate from the production collector held by T-6708.
  Commit the source before the locked release harness run.

### Open threads

- Build and run the frozen harness, retain raw output, derive the comparison,
  make the visual report, and publish the PR update.

### Errors

- None in the source step.

## 2026-11-16, compile-only check

### What was done

- Added the library-only harness manifest and generated its Cargo lockfile.
- The first compile-only command `cargo +1.99.1` exited 101 because the
  Homebrew Cargo does not accept rustup's toolchain selector. A second
  compile-only command invoked Rustup Cargo but resolved Homebrew rustc
  1.93.1, which refused dependencies requiring 1.94.1. Both failures
  preceded the corrected build. [pcc:unverified]
- With the Rust 1.99.1 Cargo and rustc paths both explicit, `cargo test
  --release --no-run` exited 0 and produced a test executable. This was
  compilation only; the scientific test did not execute. The compiler
  printed pre-existing library warnings and a nonfatal `rust-objcopy`
  missing-`libLLVM.dylib` warning. [pcc:unverified]

### Decisions made

- Pin the harness manifest and lockfile before repeating the compile-only
  check with `--locked`. Preserve the compile result separately from any
  future experimental result.
- The branch's `.resume/RESUME.md` describes other work and contains merge
  markers; T-6342 holds `dir:.resume`. Do not run the experiment or write
  around that reservation. [pcc:unverified]

### Open threads

- Commit the harness manifest/lock, rerun compile-only with `--locked`,
  publish its exact build status, and update the stacked PR.
- When T-6342 releases `.resume`, write this branch's complete resume,
  push and pass its gate, then execute the frozen screen.

### Errors

- Toolchain selection needed explicit Cargo and rustc paths. The active
  resume-file reservation prevents a branch-specific checkpoint.

## 2026-11-16, locked build and admission decision

### What was done

- Committed the manifest and lockfile at `ea2fc503bf`, then repeated the
  compile-only release command with `--locked`. Exit code was zero; raw
  stderr, empty stdout and the executable digest are bound in
  `BUILD_STATUS.json`. No test or scientific trial executed. [pcc:unverified]
- Read-only `scripts/checkpoint.sh status` reported filled resume sections
  and a stale last push. Direct inspection found merge markers at lines 66,
  198 and 202 and descriptions of unrelated studies. [pcc:unverified]

### Decisions made

- Record a compile-only readiness result and an explicit run admission
  obstruction. Do not use the current shared resume file for this branch.

### Open threads

- Commit and push the source and status, update the existing stacked PR,
  and reconcile the shared session log when T-6720 releases it.
- Recheck T-6342's scope release before the fixed scientific run. A clean
  branch-specific S3 checkpoint and passing gate are its exit criteria.

### Errors

- The strip warning is nonfatal. No further build error occurred.

## 2026-11-16, repository development suite

### What was done

- Ran the full locked release library suite from commit `58c7557e20` with
  four test threads. It exited 101 after 844.31 reported test seconds:
  6,416 passed, 16 failed, 135 ignored. Raw logs and digests are in
  `TEST_STATUS.json`. [pcc:unverified]
- Reran the ecbench APS timing test exactly; it passed 1/1. Reran the
  chained-S3 rank/solve test exactly; it failed again at the same `16 != 0`
  assertion. The remaining failures are twelve Linux `/proc` report/restore
  paths and two missing wide-prime dataset files. [pcc:unverified]
- Checked the branch diff from parent `c2468c8f5d`: it adds only this study
  directory and its standalone test, with zero `src/` changes.

### Decisions made

- Preserve the failed suite as a real validation gap. Do not alter unrelated
  production tests to make this source-only collector screen look green.
- The scientific trial remains gated by the branch-specific checkpoint, and
  therefore no relation-quality or performance observation exists yet.

### Open threads

- Publish the source-ready PR update with the suite's exact status. Re-run
  the study only after the resume reservation clears and the S3 gate passes.

### Errors

- One chained-S3 assertion remains reproducible on this checkout; the
  ecbench timing assertion differed between suite and exact rerun.

## 2026-11-16, checkpoint repair and fixed run

### What was done

- Rechecked Conductor: T-6342 reserved `.resume`, T-6708 reserved the
  production collector sources, and T-6720 reserved today's shared session
  log. Claimed the ready T-6342 through Conductor CLI. Preserved the exact
  14,220-byte incoming resume locally, retained both sides of its merge
  conflict, removed three marker lines, and placed this branch's complete
  handoff first. `scripts/checkpoint.sh gate` exited zero; S3 `RESUME.md`
  matched local bytes at SHA-256 `7126716a0f0e...`. [pcc:unverified]
- An attempted CLI re-claim of T-6795 omitted its write scope; releasing
  that attempt cancelled the task. No research artifact was lost. New
  Conductor T-6803 claimed this study directory before further edits.
- Executed the source-bound test once at 2026-11-16 13:42:02 UTC, exit 0,
  one passed. Raw stdout/stderr/meta and whole-process resources are kept.
  Pushed a second checkpoint after the result arrived. [pcc:unverified]

### Decisions made

- The first-independent operation prediction failed on all three fixed
  streams. K19 saved target queries but increased modeled additions.
  Keep the result at its fixed-stream operation scope and do not integrate
  the candidate policy. [pcc:unverified]
- Correct-account AWS budget readback remains above both monthly and daily
  limits, so the L2 F4/F5/F6 panel was not submitted.

### Open threads

- Audit the machine derivation, inspect the visual PDF, commit and publish
  the result in PR #4421. Keep the frozen panel for a future admitted window.
- The shared session log remains reserved by T-6720; this work log is the
  branch handoff until that scope releases.

### Errors

- The earlier incorrect default-profile AWS check was superseded by a
  correct-account readback. T-6795 was cancelled by the CLI release behavior;
  T-6803 replaced its study scope.

## 2026-11-16, result validation and visual report

### What was done

- Independently hashed the saved executable, raw stdout/stderr/meta and
  derived `RESULTS.json` against `RUN_RECORD.json`. Parsed the three raw
  JSON lines a second way and checked each full-rank arm, addition proxy and
  candidate-minus-first difference. All checks passed. [pcc:unverified]
- Generated the editable policy diagram and count chart, built the two-page
  Typst PDF, rendered and visually inspected both pages, and checked text
  extraction, PDF metadata and result labels. `pcc lint-prose` found zero
  issues in the new Markdown; `git diff --check` passed. [pcc:unverified]
- Checked the canonical IC leaderboard, progress timeline and rendered HTML.
  This operation-only fixed-stream result has no admitted timing or complete
  cost comparison to add there. [pcc:unverified]

### Decisions made

- Publish the measured loss and raw evidence in the existing stacked PR.
  Keep first-independent out of the production collector. [pcc:unverified]

### Open threads

- Commit, push and update PR #4421. Independent reviewer replay and the
  isolated complete-call F4/F5/F6 panel remain separate gates.

### Errors

- None in the validation step.
