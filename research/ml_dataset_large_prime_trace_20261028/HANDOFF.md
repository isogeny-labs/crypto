# Native bounded-filter implementation handoff

Update2026-10-28: the parent-owned full gate completed with5191passed,
0failed,123ignored,exit0. See VALIDATION.md and validation-52ded7.json for
source binding and freshly read-back canonical S3 evidence. The historical
implementation handoff below predates that completed gate and is retained.

Implemented after design commit `77160fa8657`, with unchanged base runtime
`d79f658f7a0a83e2f75f4cb6f0d1f7542d0e6b83`. Conductor T-748 owns only the
native filter registration, its child modules and this research directory.

`large_prime_filter::trace::filter_large_primes_with_trace` returns bounded
normalization, duplicate, singleton-peel, core, pivot, row-operation and source
combination events. The legacy function body is unchanged. New row states
commit only after sink acknowledgment. A failed Finish remains censored;
retained complete rows are individual identities, not a completed batch.

`large_prime_filter::trace::verify_filter_trace` independently normalizes and
combines coefficients, verifies legal recorded peeling and static-degree pivot
selection, raw pivot references, source-coefficient propagation, fill decisions,
partial/final reports, limits and byte/work acknowledgment. It does not execute
the native filter or call its row arithmetic/replay helper. Source identities
follow inductively from checked initial rows and every checked row operation.
It does not prove that the sink actually stored data outside the process.

Final focused release gate: **13 passed, 0 failed, 0 ignored**, comprising the
five existing tests and eight new tests. New coverage includes all 729 two-row
systems with one large/small/RHS column over F3, 200 bounded three-row systems
over F5/F7 against a separate dense projected-row-space reference, exact legacy
rows/reports, every event/sink boundary (including duplicates and rejected
fill), exact byte boundaries, every work reservation, input/state admission,
tampering and serialization. The upper-word case uses modulus `2^63-1`,
`u64::MAX` coefficients/source IDs/large columns and high small-column IDs with
manually known unit-pivot cancellation; it makes no primality claim.

Strict release Clippy passed with `--lib --tests -- -D warnings`. Scoped
Rustfmt and `git diff --check` passed. Commands:

```sh
PATH=/home/ec2-user/.cargo/bin:$PATH CARGO_TARGET_DIR=/var/tmp/crypto-store-scoped-head-target cargo +1.98 test --release --locked --lib cryptanalysis::large_prime_filter -- --test-threads=1
PATH=/home/ec2-user/.cargo/bin:$PATH CARGO_TARGET_DIR=/var/tmp/crypto-store-scoped-head-target cargo +1.98 clippy --release --locked --lib --tests -- -D warnings
```

Logs, the exact four-file compile manifest and an immutable test executable are
saved in `/home/ec2-user/ml-dataset-resume-20261028/large-prime-trace-validation`.
The parent owns the full-library gate and publication. These focused results
do not assert that the full library passed. The test build preceded the runtime
commit; all four compiled native source hashes were rechecked unchanged after
the build and are committed without code changes.

Limitations: capture is optional and batch-local, not a reusable incremental
filter. Counters are defined logical admission units, with explicit exclusions;
execution, total operation count and dependency-basis completeness flags stay
false. Input-admission rejection occurs before input hashing and proves only
the size limit. Traced singleton order is deterministic and may differ from the
legacy HashMap queue; the surviving core/results/reports must agree. The
duplicated native schedule requires compatibility tests on future changes.

No dataset v2 adapter, signed-Frobenius reducer capture, dense-rank operation
capture, scientific data, cloud job, new budget/deadline, PR or push was produced
by this increment. Full-library validation, canonical checkpoint receipt and
root review/publication remain parent delivery steps.

Errors retained: no native compile, test or lint failure occurred. The first
Conductor check used a malformed project slug and returned 404; the exact
migrated project UUID allowed the scopes, then T-748 was claimed. This CLI has
no heartbeat subcommand; the existing credential-safe heartbeat helper was
copied to an isolated task directory and maintained the lease. No existing
source checkout, lock, job or credential was removed or modified.
