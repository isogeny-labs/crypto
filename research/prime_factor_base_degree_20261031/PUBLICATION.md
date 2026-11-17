# PR publication validation

2026-10-31. The GitHub connector became available after the original local
verification records were committed. Its connection has repository write
access; the separate command-line token still fails authentication. The
canonical repository is now `isogeny-labs/crypto` (the old owner URL redirects).

The PR includes both local source rounds, 50a51d94 and 309331b2, with their
committed source and PDF bytes unchanged. It is assembled on current main,
preserving concurrent session-log and problem-index entries. Uploaded source
blobs are checked against their local Git blob hashes.

Fresh pre-publication checks:

- Boolean standalone release suite: 7 passed, zero failed.
- Prime standalone release suite: 8 passed, zero failed.
- Lean 4.19.0 monomial-coverage proof and all four axiom reports: passed.
- Earlier strict Clippy/formatting, report inspection and size checks remain
  recorded in each study's VERIFICATION.md; no code changed afterward.
- Full `cargo test --release --lib` was attempted. First hydrate missing local
  path dependencies from the sparse checkout, then rerun under a 300-second
  outer cap. The rerun emitted missing `include_str!`/`include_bytes!` fixture
  diagnostics and ended at the cap (exit124). It did not complete the library
  suite. Missing fixtures include archived ecbench sessions and quartic records;
  no source reference or assertion was removed to hide them.

The root crate source is unchanged by this PR. Full-library validation is
incomplete, and exact-head CI/review remain pending. This is a research-design,
certificate-tool and scoped-proof PR. It does not claim a new factor-base
degree win or any performance improvement. Research execution remains gated
on its separate checkpoint/backend requirements.
