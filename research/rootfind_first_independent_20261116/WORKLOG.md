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
