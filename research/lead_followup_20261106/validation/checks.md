# Development checks

Producer observations, standing unverified (`PCC-20261106-ded5b6`).

- Native helper/chart/radix suite: eleven tests pass. Current stdout is
  native-tests-chart.txt. Original eight-test outputs remain pinned in
  native-tests.txt and native-tests-final.txt; they are historical development
  outputs, not scientific measurements or independent replay.
- Touched example: all seven tests pass, including homogeneous polynomial
  evaluation, distinct seeded charts and safe-resume checks. Exact command/output
  is in example-correctness.txt (PCC-20261106-5ebf9a, unverified).
- The example correctness command uses release with package-specific overrides:
  `--config 'profile.release.package.crypto.opt-level=0'`
  `--config 'profile.release.package.crypto.codegen-units=256'`.
  Dependency profiles and sources stay unchanged. Default optimized qualification
  remains a future scientific-image prerequisite; no runtime comparison is made.
- Full-library suite: currently running the same explicit correctness profile.
  It entered the 6,231-test phase; five device tests failed on an off-disk tracked
  emulator source. After restoring gpu/certicom/, all six tests in that group
  passed against the same binary (device-fixtures-restored.txt). The original
  broad run still includes those failures and remains incomplete; no overall
  library pass is asserted.
  Its final outcome is pending and is not represented as a pass. The initial
  default release attempt failed with missing sparse-input diagnostics; all
  tracked inputs named by those diagnostics were restored. Subsequent default
  optimized compilation was stopped before tests; those noncompletions are retained.
- Historical depth-10 parser compatibility: passes on existing L2-t10-r2/r4/r8
  records. No new solver trial. The native comparator accepts terminal summaries
  and the pilot's `weights:null` unit-grading encoding.
- rustfmt/cargo fmt, worker shell syntax and source whitespace checks pass.
  Raw stdout retains its exact terminal blank lines; diff --check can report those
  output-only lines. Frozen proof inputs were not trimmed to suppress that notice.
- PCC ledger chain against origin/main passes. All new statement/attestation
  claims remain unverified/not_covered; none has an independent signature.
- Benchmark-record gate passes after exact-file sparse hydration. No record or
  generated progress file changes. The prepared panels have not run.
- Both cards pass the original backlog validator with their committed protocol blobs.

The administrative backlog validator is unchanged src/bin/backlog.rs plus its
card/cairn modules. It was built in a temporary Cargo package against unchanged
SHA-1/SHA-256 source modules (crate alias crypto_lib), with clap, serde_json, hex
and libc, while the full library builds. No card rule changed and no pruned crypto
solver is used for a scientific run.

The one-page PDF was rendered with Typst 0.15.1 and visually inspected. The study
protocol and both card copies are byte-identical; protocol.blob is computed by
git hash-object. No independent output verifier or image/admission receipt is
invented; the cards deliberately remain blocked.
