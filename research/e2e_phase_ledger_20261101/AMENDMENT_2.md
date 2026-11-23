# Amendment 2: censored-outcome gate correction

Filed 2026-11-01 after fixed diagnostic run `41360060164` and before any
replacement measurement. This amendment records the exact target-1 root cause
and freezes the candidate harness correction at source revision
`a34125841c2b696420bd97440d59b73c13355abc`.

## Hash-bound diagnosis

The diagnosis bound the original workflow `41343654905`, request SHA-256
`3516bd65ea320e1d5555def621f8130684312f8a9eef654dc89d5e28f790ce19`,
launcher, Job, Pod, source revision and native binary. Its selected receipts
matched the pod's `SHA256SUMS`; the audit receipt SHA-256 was
`d7ed1453c3111664914158f31e257ca09af04e4ad3d3850c26307229f1cdf0a8`.

The audit reported:

- `ok = true`, `regraded = true`, `records_bound = true`;
- 364 records, 308 with outcome status `verified`;
- all 312 measured replay candidates reproduced;
- zero audit problems.

The only failed wrapper predicate was `verified_records_equal_records`.
`ecbench` defines `verified_records` as an outcome-status count, not as the
number of structurally valid records. Genuine timeout is checked statically;
deterministic exhaustion is replayed. Both remain valid right-censoring. The
56-record difference is exactly the size of the two frozen m=2 structural arms
over four targets and seven executions (one warmup plus six rounds), consistent
with the rank-ceiling censoring required before the protocol was frozen.

## Correction

Revision `a34125841c2b696420bd97440d59b73c13355abc` changes no algorithm, factor
base, target, seed, arm, trial cap, degree cap, timeout, round count, pricing or
analysis rule. It changes only admission:

1. the native, launcher and resumable audit gates require a clean, regraded,
   records-bound audit, a positive record count and complete replay
   reproduction; they no longer mislabel every non-recovery as audit failure;
2. the L2 gate admits only a verified answer matching its target, deterministic
   exhaustion, or genuine timeout, and reports exhaustion/timeout explicitly as
   censored; wrong answers, errors, crashes and sub-L2 records still fail;
3. every portable launch uploads and re-hashes the current reviewed
   `remote/run-request`, selects it through a content-addressed S3 buildspec,
   and sets the CodeBuild terminal override to 100 minutes. The deployed bundle
   continues to supply the validated parser, readiness and build checks.

Local fixtures cover censored acceptance, wrong-answer rejection, sub-L2
rejection, content-addressed launcher selection and the 100-minute bound. The
repository's contract workflow must compile and test the embedded Rust
controllers before this correction is admitted.

## Pre-measurement publication gate

This amendment authorizes **publication only**, not an experiment. Build and
publish revision `a34125841c2b696420bd97440d59b73c13355abc` through the fixed
main-branch workflow without submitting EKS work. The resulting image digest,
native binary SHA-256 and build receipt must be retained and reviewed.

Because this revision changes no native source, the expected `ecbench` SHA-256
is the original frozen
`f3aab23417ece64cb3373e5c66d2c3be343e18c0e89347f626e0aea18ab1b774`.
If publication produces any other native hash, stop and amend before launch.

## Replacement status

No replacement measurement is authorized by this file. A final pre-data
amendment must name the published image digest and verified binary hash, bind
the current launcher/buildspec hashes, and establish the remaining amount under
the cumulative USD 10 ceiling. Only then may both frozen shards run with fresh
workflow, Job and Pod identities. No failed-attempt record may be reused.
