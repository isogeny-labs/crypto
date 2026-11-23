# Resume: frozen norm-branch census

## Goal

Determine whether the public signed norm-branch coefficient tensor on frozen
prime-field curve instances has a growing exact flattening-rank defect large
enough to justify a SAT/proof-size stage. This is an exact rank experiment, not
a timing, attack, or security claim.

## Lead / hypothesis

The fixed `d=5` rank-one counterexample refutes a universal full-rank
conjecture but may be isolated. Advance only if every held-out prime/seed at
`k=12,14` passes the frozen global-rank, slope-gap, and persistent-block-defect
rules. Kill only the measured linear-message family under the frozen full-rank
rule. Otherwise report `INCONCLUSIVE`.

## Spec

The controlling files are `PROTOCOL.md`, `protocol.json`, and
`PROTOCOL.sha256` in this directory's parent. The protocol digest is
`8c3a7e5cbf0647e077c8fd5b1a1b8f19c82f31adab1e4b63eb3ffddd9ed8c0d6`.
The workflow must bind its exact 40-hex `SOURCE_REVISION`; checkpoint metadata
records the tested checkout's exact `HEAD`.

## State

The typed canonical IR, producer, independent checker, exact controls, family
completeness gate, Lean factorization, identity certificate, and immutable
Git-publication receipt passed before dispatch. The trusted workflow creates
this S3 checkpoint before image publication or EKS submission. No held-out
result exists at checkpoint time.

## Next steps

1. Resume the same GitHub workflow run and source revision; do not invent a new
   request after an ambiguous submission.
2. Require one command request with 2 CPUs, 4 GiB memory, 10 GiB disk, a
   1,800-second deadline, zero Kubernetes retries, and CPU Spot capacity.
3. Retain the launcher result, Job/Pod UIDs, S3 `SHA256SUMS`, raw artifact,
   checker receipt, build manifest, protocol, and workflow artifact.
4. Apply only the preregistered `ADVANCE_B2`, `KILL_MEASURED_FAMILY`, or
   `INCONCLUSIVE` decision rule, then publish the source-linked report and
   controls.

## Runs in flight

At checkpoint creation, no EKS request has been submitted. The workflow appends
its run ID, attempt, and tested source revision below before upload.

## Stop and success conditions

The total ceiling is USD 2. Stop before launch on checkpoint, source, protocol,
budget, capacity, or request mismatch. Stop after any ambiguous submission and
reconcile the original immutable request. At most one separately retained retry
is permitted, only after a definite infrastructure failure. Wall time is
non-scientific; only a checker-clean frozen verdict counts.
