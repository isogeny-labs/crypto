# Paired rootfinding versus F4, F5 and F6-IC

Status: preregistration. This file and `protocol.json` precede the measurements in this study. The earlier K13 rootfinding and F4/F5/F6 studies informed the choice of engines and runtime estimate; their targets do not enter this panel.

## Question and boundary

On eight fresh public targets of the registered Koblitz E0 curve over F_(2^13), does the complete cold `ic.pipeline` using a negation-folded rootfinding pair table cost less than the same pipeline using matrix F4, matrix F5, inherited F4 and F6-IC? The m=3 rootfinding arm holds decomposition arity and factor-base construction fixed. The m=2 rootfinding arm is a separately reported whole-algorithm choice. All IC arms use the same 33-point standard-subspace base, `max_trials=200000`, one target per run, and verification of `[k]G=Q`. The strong signed-Frobenius rho arm anchors attack cost. This is a fixed-curve comparison, with no scaling or larger-curve inference.

## Primary and secondary measurements

The primary metric is the complete cold solve cost in `bitop/v1` from `ecbench work`: setup, relation collection, linear algebra, target descent, and verification. A record with `work.complete=false`, a failed answer, or a missing phase is ineligible for a cost ordering. The deterministic decision requires the m=3 rootfinding total below each of the four Groebner totals on **all eight** matched targets and all six measured rounds. Report the paired total-cost ratio and range per comparison. The m=2 arm is reported by the same rule, separately. No pooled ratio across curve sizes exists.

Secondary diagnostics are the current `S` pricing band at weights 0, 0.1, and 1 GAE per native operation, phase counts, relation yield, peak RSS, and wall time. A cost ranking in `S` needs complete counts and the same order across the entire band; otherwise state the bound. Wall time requires every compared execution to earn L2, a same-target paired 95% workload-bootstrap interval wholly below 1, and a difference outside the rootfinding A/A interval. The A/A is a duplicate rootfind-m3 arm with independent harness execution seeds. If this gate fails, report wall times as descriptive only. The four primary comparisons use Bonferroni alpha 0.0125 for any inferential language; the all-target deterministic rule is the declared decision and does not estimate a population effect.

## Design and stopping

`SPEC-K13.json` fixes eight independent public targets (seed 202611151301), six measured rounds plus one warmup, interleaved arm order, and measurement seed 202611151302. Eight arms yield 448 executions. The same release binary, source commit, build flags, target law, group, factor base, host, isolation gate, and timeout apply to every arm. Engine and m are the named differences. A wrong answer, timeout or interruption remains in the raw session; do not replace, filter, or rerun an unfavourable row. The positive control is every planted public scalar checked by ecbench and independent replay. The negative control is not applicable to a fixed-instance cost comparison. Arm-label masking is not used because the deterministic bitop analysis reads method-specific counters; the decision rule is fixed here before data.

The previous K13 cross-family panel measured matrix F4 and F5 relation phases at about 30 and 26 seconds per run on another host. Fifty-six executions each, including warmups, suggest about 53 minutes of matrix work plus setup, rho, rootfinding, launch and verification. The job deadline is 3600 seconds, the per-execution cap 1800 seconds, and the total new-job spending ceiling is $5 including failures. A timeout censors the wall comparison and remains an observation; it cannot be translated into a completed cost ratio. Use the native ecbench Iso Lab EKS L2 runner with no automatic measurement retry, preserve its private S3 receipts, audit every deterministic row, and require a second-host replay before promoting a performance claim. A Spot restore, if any, invalidates that epoch's clock; this ordinary v1 request makes no recovery claim. This study runs one frozen cell and then stops.

## Analysis and reporting

Analyze only the sealed session and its matching spec using native `ecbench verify --replay-all`, `work --summary`, `compare`, `band`, and `table`. Retain each attempted row and phase. A/A, rho, complete bitops, band, wall admission, and every censoring status appear in the report, graph and PDF. A new numerical speed claim needs a PCC record and an independent signed verifier verdict under the repository rule. Update the IC scoreboard and dashboard in the same result PR. If this cell passes, the next protocol may test n=19 with its matrix arms honestly censored at their frozen cap; this cell alone does not select an asymptotic winner.

Prior internal sources: `research/f4_f5_f6_cross_family_20261011/PROTOCOL.md` and `RESULT.md`; `research/ecbench_rootfind_faster_20261014/README.md`; `docs/units/BITOPS.md`; `docs/ecbench/README.md`. Their earlier measurements are development evidence, not confirmatory observations in this protocol.

## Amendment A1, filed 2026-11-15 before data

The initial frozen [v1 record](protocol-v1.json) declared one Rayon thread per arm. The Iso Lab L2 benchmark launcher allocates four CPUs and sets both `RAYON_NUM_THREADS` and `OMP_NUM_THREADS` to `4` in each child process (`taskq/deploy/eks-karpenter/submit-job`). [Protocol v2](protocol.json) corrects every arm's instrument environment and thread count to match that existing runner. The spec, targets, controls, complete-cost metric, and decision threshold are identical. Neither version has been used for a scientific execution. The result, if admitted, will bind to v2 only; v1 stays as a frozen superseded design record.
