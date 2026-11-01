# Complete cold E2E phase ledger

Status: frozen protocol; the first two-shard attempt produced no admissible
result set. See [AMENDMENT_1.md](AMENDMENT_1.md) for the retained failure
classification and prospective infrastructure remediation. A fixed receipt-only
diagnostic is prepared in [`diagnostic/`](diagnostic/): it submits no EKS work
and publishes only hash-verified gate booleans, counts, and failed predicate
names for the retained target-1 audit failure. [AMENDMENT_2.md](AMENDMENT_2.md)
records the resulting censored-outcome gate defect and freezes its candidate
correction. It authorizes publication-only verification, not a retry.

This study fills a specific accounting gap in the binary-curve index-calculus
work: it measures a complete, current-source, cold solve from setup through an
independently verified discrete logarithm. It does not substitute a
relation-oracle microbenchmark for an attack.

The registered cell is the toy Koblitz E0 curve at `n = 13`. That cell is
small enough for all registered backends to complete under one measurement
envelope. It is deliberately **not** evidence about ECC2K-130/131 feasibility,
an asymptotic exponent, or novelty in the literature.

## What is charged

Every index-calculus execution uses
`incremental-gauss-full-rank-checked`. The ledger therefore includes:

1. factor-base construction;
2. oracle/table setup;
3. relation trials and accepted relations;
4. PDP/decomposition work, including F4, F5, F6-IC, or root finding;
5. relation-matrix construction;
6. full-rank linear algebra;
7. one point check per factor-base column;
8. target-log recovery and `[d]G = Q` verification.

The report will retain each record's cold wall time, exclusive phase clocks,
group operations, field operations where present, solver-native statistics,
matrix statistics, peak bytes, `bitop/v1` work block, `unpriced` list,
`known_uncounted` list, and the 0/0.1/1 GAE pricing band. Unlike native units
will not be silently added or compared.

## Arms

The common m=3 cell fixes the factor base, target stream, trial cap, node
budget, degree cap, and full-rank solver while changing only the PDP backend:

- inherited F4 (baseline), matrix F4, matrix F5;
- F6-IC and F6-IC pair-index;
- full, windowed, negation-folded, and windowed-negation-folded root finding.

Two m=2 root-finding arms are retained as separately labelled structural
rank-ceiling diagnostics. Pre-freeze smoke reached rank 10 of 11 after the
fixed trial cap for both layouts under the all-columns full-rank rule; those
outcomes are retained as censoring, not converted into successes by early
target pinning. A strong signed-Frobenius rho arm is the generic reference, and an
identical rho arm is the A/A timing control.

There are two independent four-target shards, one warmup and six measured
rounds per target. Cross-backend wall comparisons are made only within the
same L2 session; count evidence can be pooled only where its units and
coverage flags match.

The smoke used target seed `202611010130`. Both registered streams were then
rotated before freeze to `202611011130` and `202611011131`, so no registered
target or measured round was observed during instrument calibration.

See [PROTOCOL.md](PROTOCOL.md), [LITERATURE.md](LITERATURE.md), and
[`protocol.json`](protocol.json).

The diagnostic transport follows the official AWS CodeBuild
[`StartBuild`](https://docs.aws.amazon.com/codebuild/latest/APIReference/API_StartBuild.html)
and [buildspec](https://docs.aws.amazon.com/codebuild/latest/userguide/build-spec-ref.html)
contracts: an S3 ARN may supply a per-build buildspec in the project's Region.
Here both the buildspec and script are SHA-256 pinned before upload and again
before execution.
