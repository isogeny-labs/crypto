# Cache validation record

## Completed local checks

- The baseline revision passed `cargo test --release --lib`: 4,395 passed,
  zero failed, 118 pre-existing ignored. The complete output is
  `small-m4-final-release-lib.log`. This is the cache-off pre-change
  library, not a validation of the later cache implementation.
- The cache implementation passed both new release-library tests:
  `xor-cache-unit-tests.log`. They cover directed cache invalidation and
  400 mixed CNF/XOR truth-table comparisons, including exact models and
  work-counter equality.
- The completed prior public card passed the native receipt audit over
  its 132 files; the audit and its two rejection tests are recorded under
  `small-m4-v1/final/` and the `small-m4-audit-*.log` files.
- The report PDF was inspected, including all labels and the elapsed-time
  accounting boundary. The study and benchmark visual/PDF copies agree.

## Cancelled local duplicate build

`small-m4-final-example-tests.log` records an unfinished local example-test
build. It first waited for the candidate's release-library compilation.
After the two cache tests passed, its separate library build was still
running on the contended host. The session deliberately terminated only
its own compiler (PID 77104, child of Cargo PID 47242); the command exited
143. This invocation is excluded from passing checks. The same touched
examples are to be validated on the authorized Linux runner before launch
and push. Other agents' builds and research processes were left alone.

## Linux source boundary

The first Linux build at `628ec4b8d384e0b19c61fb0ec77e513a8c629a4e`
succeeded; `xor-cache-linux-build-1.log` is retained. Source arrived by
credential-free Git bundle, SHA-256
`0aa01d9a92e8223607ac8113c4ff55852b38181cff14efd6c562ffbb7c201014`.
Its source tree is `/var/tmp/n83-xor-cache-source`. The candidate's complete
library and example checks run there before the final model-digest worker
is built. Later validation receipts must state their exact revision.

The final worker revision is
`1de47a9d4186bff08abc84c224a111183e820106`. Its additional bundle SHA-256 is
`3ed3efd2ac24e3cc63d1baf9ee98f7129e7138016342c99420c26cd78b1db75f`.
The solver/library source and dependencies are unchanged from the first
Linux build; changes add the baseline report/auditor, command usage and
the complete SAT-model digest in the cache worker receipt. The final
worker, verifier and receipt-auditor tests were required at that
revision; their subsequent passing receipts are listed below.

## Completed Linux checks

- Candidate `cargo test --release --lib -- --test-threads=4` at `628ec4b8d`:
  **4,403 passed, zero failed, 118 ignored**, including both cache tests.
  `xor-cache-linux-release-lib.log` preserves the full output. The platform
  has additional Linux-specific tests compared with the local baseline.
- `xor-cache-linux-example-tests-1.log`: first worker 3 / replayer 2 passed.
- Final revision `1de47a9d`, `xor-cache-linux-build-2.log`: build passed.
  `xor-cache-linux-example-tests-2.log`: worker **3**, receipt audit **2**,
  general-field replayer **2** passed, all zero failures.
- `xor-cache-library-tree-identities.txt` pairs the identical `src` and
  Cargo.lock Git object IDs; `xor-cache-library-revision-diff.log` is empty
  for src, Cargo manifests/lock and native path dependencies. The complete
  library validation therefore covers the unchanged library used by the
  final worker. Example validation was rerun after the receipt change.

The broader catalog/PR checks remain separately blocked as recorded in
`CI_STATUS_20261020.md`; N83 validation does not waive them.

## Completed-card auditor and full-base continuation

The new native `n83_xor_cache_audit` passed both release tests after a
13m10s local library build (`xor-cache-audit-tests.log`). Direct native
dependency-linked builds also passed both tests during development
(`xor-cache-audit-direct-tests.log`, `xor-cache-audit-direct-tests-2.log`);
the later Cargo run supplies the normal example gate. The auditor rejects
changed models/search counters, broken visit conservation and promotion of
unfinished progress; wall-capped endpoints receive no paired ratio.

The explicit `full-m4` continuation was frozen in `FULL_M4_PROTOCOL.md`
before implementation. `full-m4-local-tests.log` records passing release
tests: worker **5**, general-field replay **2**, full-card fixtures **2**,
and completed-card audit **2**. The fixtures exercise all 26 container
calls, both native control comparisons, the prerequisite gate, a retained
wall cap and output-reuse rejection. They copy fixture receipts and do
not simulate cryptographic correctness. Full-base actual controls and
replays are a separate mandatory launch gate.

The continuation changes examples and study code only; the solver/library
tree and Cargo dependencies remain identical to the fully tested Linux
candidate. Its Linux binaries and final source hashes must be recorded
after the current remote card ends, before the next timed run starts.

`full-m4-local-clippy.log` preserves the first lint failure: a new control
predicate could be simplified, and an existing RSS conversion was an
identity map on macOS. Both were corrected without changing units or
policy. `full-m4-local-clippy-2.log` exits successfully; the local Clippy
version still warns about the repository's pre-existing unknown
`chunks_exact_to_as_chunks` allowance. No lint was disabled by this change.
`full-m4-local-tests-2.log` repeats all **5 + 2 + 2 + 2** example checks
successfully after the correction and the receipt auditor's directory
symlink / root-UNSAT handling review.
