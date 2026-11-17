# Build-lock refresh — 2026-10-19

The study's original `Cargo.lock` is preserved byte for byte as
[`historical-locks/20261002-Cargo.lock`](historical-locks/20261002-Cargo.lock),
SHA-256 `2e92fdc6725e68a3085c274d0fa86693a96c2a6214d6bed3754e25b4e82b9f25`.
It was read from crypto PR #2146 revision
`92a5f4f159eed9f794f728bdfdd1a67117e8a190`.

The working lock is now resolved against the current root crate, using that
revision's root `Cargo.lock` as the dependency-version baseline. The root crate
added SQLite and the in-repository volcano CLI after the study was prepared;
the original study lock therefore failed `--locked` before any tests ran.
The refreshed lock retains the current root package versions and adds this
standalone study package. The protocol, worker, campaign, and original lock
bytes are preserved. This is a build repair; no discovery or holdout campaign
was executed by this review, and historical measurements are not reassigned to
the new dependency set. A future authorized run records the working lock and
exact checkout in its own retained output directory.

Validation: the standalone locked release suite passed with the six environment
settings already fixed by protocol.json and the workflow (11 passed, two ignored
development timing tests). An initial review command omitted those settings and
failed the direct-unpack assertion; the assertion and workflow remain unchanged.
The main library suite previously passed 4,403 tests with 118 ignored on the
identical Rust source tree.
