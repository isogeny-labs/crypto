# Development checks

Producer observations, standing unverified (`PCC-20261106-7ccd3b`).

- Native helper/comparator suite: passes; exact original and latest development
  output in native-tests.txt and native-tests-final.txt. These are unit tests,
  not scientific measurements or independent replay.
- Historical depth-10 parser compatibility: passes on the existing L2-t10-r2,
  L2-t10-r4 and L2-t10-r8 files. No new solver execution or research trial.
- rustfmt on touched example/helper; cargo fmt on native manifest; worker shell
  syntax; git diff --check: pass locally.
- PCC ledger verify against origin/main: chain passes. Claim standing remains
  unverified; local verify reports not_covered (unsigned statement/attestation).
- Benchmark-record gate: passes after exact-file sparse hydration; no benchmark
  record or generated progress file is changed.
- Backlog validator: structurally passes with --no-git; committed-protocol check
  will be rerun after the preparation commit.
- Full release library suite and touched-example build: pending in the initial
  preparation commit; final outcomes must be appended before publication.

The administrative backlog validator is the unchanged src/bin/backlog.rs plus
its card/cairn modules. It was built in a temporary Cargo package against the
unchanged SHA-1/SHA-256 source modules (crate alias crypto_lib), with clap,
serde_json, hex and libc, while the full library builds. This packaging changes
no card validation rule. No pruned crypto solver is used for a scientific run.

The one-page report.pdf was rendered with Typst 0.15.1 and visually inspected.
The protocol source and both card copies are byte-identical; protocol.blob is
computed by git hash-object. No output verifier or image/admission receipt has
been invented; the blocked cards deliberately remain unready.
