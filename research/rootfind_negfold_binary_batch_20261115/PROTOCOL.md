# K13 scalar versus batched binary rootfinding

Status: preregistered before data. This study measures the separate
`rootfind-negfold-batch` candidate added in PR #4354. It does not alter the
frozen F4/F5/F6 study in `../rootfind_f456_paired_20261115/`.

## Question and prediction

On eight fresh public targets of the registered Koblitz E0 curve over
`F_(2^13)`, does sharing inversions across pair-table rows and eight-point
`m=3` relation-scan blocks reduce complete cold `ic.pipeline` cost against the
scalar negation-folded engine? The primary comparison is `batch-m3` against
`scalar-m3`; `batch-m2` against `scalar-m2` isolates the table-build change.
The code probes completed sums in scalar order and charges any additions past
the first hit. Consequently the candidate may spend more abstract group
additions even if field inversions or wall time decrease. A failed prediction
is retained as an outcome.

The prior prime-field batch-table study
[`../ecbench_rootfinding_batchtable_20261013/PROTOCOL.md`](../ecbench_rootfinding_batchtable_20261013/PROTOCOL.md)
and `src/cryptanalysis/ic_framework/rootfind.rs` already use batched
inversions. The K13 `m=3` ecbench adapter is the new integration point, not a
new mathematical decomposition. The frozen F4/F5/F6 panel and its matched rho
remain the only direct test of the larger objective.

## Inputs, controls, and stopping

`SPEC-K13.json` fixes Koblitz E0 n13, the 33-point standard-subspace base at
dimension five, eight public target IDs from seed `202611151501`, and a
shared algorithm seed per workload and round from seed `202611151502`. Every
IC arm uses `max_trials=200000`, one cold target per process, the same
factor-base, linear algebra and verification. Six arms, six measured rounds
and one warmup produce 336 planned executions. Alternate arm order; use the
same release binary, source commit, build flags, host, four threads, and L2
isolation for every arm. The strong signed-Frobenius rho arm is a cost
reference. `scalar-m3-aa` duplicates the scalar `m=3` method with independent
harness executions as the wall-noise control. Every reported scalar must
satisfy `[d]G=Q`; this is the positive control. A negative control is not
applicable to an exact fixed-instance cost comparison.

Stop after this one cell and its audit. Preserve wrong answers, timeouts,
interrupted rows, setup errors and censored comparisons without replacing or
discarding a target or repeat. A timeout is not a completed cost ratio. The
per-execution cap is 120 seconds; the total job deadline is 1800 seconds, zero
automatic measurement retries, at most one infrastructure retry under a new
record, and a USD 5 new-job ceiling including builds, queue, failure, storage
and transfer. The current monthly budget gate in
[`../rootfind_f456_paired_20261115/RUN_STATUS.md`](../rootfind_f456_paired_20261115/RUN_STATUS.md)
must clear before any build or job. This design grants no spending exception.

## Metrics and decision rule

Primary metric: complete cold `bitop/v1` total from `ecbench work`, including
factor-base and pair-table setup, failed decomposition probes, relation
collection, rank updates, descent, recovery and verification. Require
`work.complete=true`, a scalar passing `[d]G=Q`, every phase present, and no unpriced
counter in both arms of a pair. Compare `batch-m3 / scalar-m3` on every matched
target and measured repeat; report the full ratio range and phase totals.
Report `batch-m2 / scalar-m2` separately. The all-target cost criterion holds
only if the candidate is strictly lower on all eight targets and all
six measured repeats; otherwise retain the per-pair outcomes without a winner.
No comparison across `m=2` and `m=3` is attributed to batching.

The exact relation control compares each same-target, same-seed pair's
scalar answer, trial count, relation count, lookup count and final rank.
The focused code test also checks the first decomposition witness. If any
control differs, investigate the first differing row and withhold a
correctness-preserving interpretation. The candidate's abstract addition
count may exceed the scalar arm by additions computed after an early hit; do
not require equality there. Report field-operation counts, native counters,
group additions, peak RSS and each phase beside the primary total.

Secondary metric: complete cold wall time and relation-phase wall time.
Admit a wall comparison only if every paired execution has L2, the same
source/binary and host, an A/A paired 95% target-cluster interval containing
one, the candidate/scalar paired 95% target-cluster interval wholly below one,
and an observed difference outside the A/A spread. The deterministic
all-target rule above governs bitops; wall intervals characterize the fixed
eight-target panel and do not imply a population or larger-curve result. A
component wall change is reported as a component, even if complete wall time
does not move.

## Execution, audit, and inference

Use the native Iso Lab EKS Spot ecbench L2 runner. Commit and publish the
source, this protocol and the exact spec first; bind the GitHub commit,
spec SHA-256, OCI image digest and ecbench binary SHA-256 in the request and
retain the check-experiment-git receipt. Re-read live daily and monthly
budget headroom, reserved spend, pool capacity and existing jobs before
submitting. An ambiguous submission result is reconciled by request and Job
UID, never retried under a new identity without checking the original.

Retain the raw session, `ecbench verify --replay-all`, `work --summary`,
`compare`, `band`, A/A and scalar checks, host and memory telemetry, and all
failure rows. Require independent replay on a second host class before
promoting a performance claim. The report includes a graph and PDF and names
the exact source/spec hashes. Any future quantified advantage assertion must carry a
machine-readable `claims/*.json` record and independent verification under
the progress-fidelity and PCC gates. This study alone cannot establish a
ranking against F4/F5/F6 or a larger-curve cost law.
