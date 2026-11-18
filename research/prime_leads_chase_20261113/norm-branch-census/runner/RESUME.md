# Resume: exact GitHub-runner norm-branch census

## Goal

Execute the already-frozen exact norm-branch coefficient census on a separate
GitHub-hosted x86-64 backend after the EKS admission path failed, without
silently treating the changed backend as an EKS v1 result. Determine only
whether the frozen linear-message rank gate says `ADVANCE_B2`,
`KILL_MEASURED_FAMILY`, or `INCONCLUSIVE`.

## Lead / hypothesis

The public source-bound Laplace/product-kernel coefficient tensor may have a
growing balanced-separator rank defect absent from support-matched random
coefficients. The fixed `d=5` rank-one counterexample establishes only a local
defect. The frozen held-out `k=12,14` universal half-rank, slope-gap, and
persistent-block conditions decide whether a separate SAT/proof-size B2 study
may be designed.

## Spec

The mathematical protocol is
`research/prime_leads_chase_20261113/norm-branch-census/protocol.json`, SHA-256
`8c3a7e5cbf0647e077c8fd5b1a1b8f19c82f31adab1e4b63eb3ffddd9ed8c0d6`.
The separate execution protocol is
`research/prime_leads_chase_20261113/norm-branch-census/runner/protocol.json`.
Its companion digest and `SOURCE_CLOSURE.sha256` bind the runner, workflow,
canonical source, checker, lockfile, formal identity inputs, and both protocols.

## State

The canonical typed IR, exact producer, independent checker, tiny semantic and
mutation controls, family-completeness checks, and frozen decision function are
implemented. The EKS v1 execution is incomplete and remains so. At checkpoint
creation no held-out runner artifact has been generated, no result is known,
and no SAT stage is authorized.

## Next steps

1. Require exact `HEAD == SOURCE_REVISION == GITHUB_SHA` and verify both
   protocol digests plus `SOURCE_CLOSURE.sha256`.
2. Build and test with Rust 1.99.1 and the committed lockfile; require static
   x86-64 musl producer/checker binaries and retain `build-info.json`.
3. Upload this recovery state with `scripts/checkpoint.sh gate` through the
   workflow's OIDC credentials; preserve `.resume/.last-push` as
   `norm-branch-checkpoint.txt`.
4. With AWS credentials removed, run
   `timeout --kill-after=10s 1800s runner/run.sh ...` under a 4 GiB virtual-
   memory cap.
5. Upload all outputs even on failure. On success, independently download the
   workflow artifact, verify `SHA256SUMS`, and rerun the checker from the exact
   pinned source before reporting.

## Runs in flight

None at checkpoint creation. The workflow appends its run ID, attempt, exact
source revision, and URL below before uploading this brief.

## Stop and success conditions

Stop before computation on any source, digest, closure, checkpoint, budget, or
main-ref mismatch. Stop and retain the attempt on timeout, OOM, output cap,
nonzero process exit, schema mismatch, control failure, checker disagreement,
or replay mismatch. The total ceiling is USD 2. One original attempt and at
most one identical, separately retained retry are permitted, only after a
definite GitHub infrastructure failure. A valid completion requires 40 frozen
cells, two byte-identical passing checker receipts, matching frozen verdicts,
and a verified payload manifest. Wall time is never evidence.

No SAT or proof-size experiment runs unless the valid verdict is exactly
`ADVANCE_B2`; even that verdict is not an ECDLP speedup claim.
