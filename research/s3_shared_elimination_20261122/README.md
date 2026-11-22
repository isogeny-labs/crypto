# Exact floating-point matrices and shared S3 elimination

This study implements bounded exact modular matrix arithmetic and reuses the
target-independent part of a finite-degree Macaulay matrix. It is the separate
cross-target caching study deferred by [PR #4920](https://github.com/isogeny-labs/crypto/pull/4920),
tracked in [PR #4922](https://github.com/isogeny-labs/crypto/pull/4922).

- [REPORT.md](REPORT.md): construction, correctness scope, prior work, and evidence.
- [PROTOCOL.md](PROTOCOL.md): prospective matched performance design and admission dependencies.
- [projection.svg](projection.svg): the shared and fresh paths and their accounting boundaries.
- [report.pdf](report.pdf): rendered construction and evidence report.
- [evidence/development-receipt.json](evidence/development-receipt.json): commands,
  source bindings, successful development checks, and retained failed attempts.
- [REVIEW.md](REVIEW.md): same-team source review, including the defect missed
  by inspection and caught by execution.
- [registration/README.md](registration/README.md): unsigned PCC statement
  `PCC-20261122-2fb226`, committed-receipt binding, and append-only metadata.
- [qualifier/README.md](qualifier/README.md) and
  [integration_qualifier/README.md](integration_qualifier/README.md): native
  source-bound development builds, with explicit exclusions.
- [workload-plan.json](workload-plan.json): machine-readable prospective scope;
  execution is not admitted or ready.
- [../../src/cryptanalysis/exact_modular_matrix.rs](../../src/cryptanalysis/exact_modular_matrix.rs):
  integer, FMA, and bounded split16 kernels.
- [../../src/cryptanalysis/shared_macaulay.rs](../../src/cryptanalysis/shared_macaulay.rs):
  metadata-only fresh construction and shared coefficient projection.
- [../../examples/shared_s3_matrix.rs](../../examples/shared_s3_matrix.rs):
  S3 integration fixtures and executable development checks.

The implementation uses floating-point instructions to perform exact integer
arithmetic. It never uses a tolerance to decide a finite-field zero or pivot.
The current split16 backend is a portable native loop, with no BLAS or GPU call.

Performance is an open deliverable. No admitted timing, complete ECDLP cost,
or speed ratio is reported by this change. Local correctness checks and their
producer receipts are not signed PCC verification.

The component suite reports 16 passing cases at both development optimization
level 0 and explicit optimization level 3. The corrected integration suite
reports five passing cases; the delivered `--check` command also completes.
The earlier integration failure and both full-library compiler SIGKILLs are
preserved. These are local producer observations, not signed verification.
[pcc:unverified]
