# Exact floating-point matrices and shared S3 elimination

This study implements bounded exact modular matrix arithmetic and reuses the
target-independent part of a finite-degree Macaulay matrix. It is the separate
cross-target caching study deferred by [PR #4920](https://github.com/isogeny-labs/crypto/pull/4920),
tracked in [PR #4922](https://github.com/isogeny-labs/crypto/pull/4922).

The current principal correctness command is `--check-prime31-recovery-arms`, using
Fp with p=2147483647 and subgroup arithmetic modulo 2147446831. It exercises
Original and independently prepared Fresh/Shared recovery with the existing
Rust F4, signed lifting and relation matrix implementations. See
[REPORT.md, section 8](REPORT.md), the [matched recovery receipt](matched_recovery/development-receipt.json)
and its [source review](matched_recovery/REVIEW.md). The earlier
[Prime31 receipt](prime31/development-receipt.json) is preserved. [pcc:unverified]

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

The initial component suite reports 16 passing cases at both development optimization
level 0 and explicit optimization level 3. The corrected integration suite
reports five passing cases; the delivered `--check` command also completes.
The earlier integration failure and both full-library compiler SIGKILLs are
preserved. These are local producer observations, not signed verification.
[pcc:unverified]

The later Prime31 follow-up reports twelve passing integration cases and nine
identical finite-chart solutions in original and fresh/shared F4 arms. All
three preconditioner backends recover k=7 for the fixed public Q using three
F4 relations and the G anchor, followed by the group check. This structured
target/base is a correctness fixture, not a uniform-target benchmark; F4 and
relation Gaussian elimination themselves remain integer code. [pcc:unverified]

The matched continuation reports seventeen passing example tests. All seven
Original/Fresh/Shared arms recover both signs of the Prime31 public target;
all 18 nonidentity F17 targets recover across seven arms. The full Prime31 CLI
retains 28 planned outcomes, including the deliberately incomplete cap and
deadline controls. Phase records retain unpriced setup work and counter
exclusions. No timing samples or complete-cost ratio were collected.
[pcc:unverified]
