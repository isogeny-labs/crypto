# Iso Lab (EKS Spot, native L2) replay of the ecbench batch-parity session: frozen before the run

Date: 2026-11-02. Replays `research/ic_residual_batch_ecbench_20261102` (frozen spec `specs-local/E-p36.json`, run locally at L0; merged in PR #3293).

## Design
`spec-p36.json` = the study's spec with three changes required by the lab's admission (L2 spec, >= 5 measured rounds, warm-up, reference, matching A/A control):
`measurement.rounds` 3 -> 5, `isolation_required` L0 -> L2, `timeout_seconds` 3600 -> 1800 per execution, and one A/A control arm `pool64aa` (rho.multitarget_pooled, batch 64, role `control`).
Curve, workloads (16, target seed 20261901), measure seed 20261902, order, and every other arm unchanged. Code: main commit named in the dispatch (contains `ic.residual_batch`); image built from it by the `Iso Lab experiment` workflow.
One Spot worker (c8a.2xlarge), Kubernetes deadline 3600 s. Cost ceiling: USD 3 for this replay, all attempts included.

## Claims
- R1 (replay): counted GAE is a deterministic function of algorithm seed and instance, so rounds 1-3 reproduce the local L0 counts exactly for all five original arms; rounds 4-5 are new draws.
- R2 (isolation): the lab's native L2 gate, replay audit and A/A admission (pool64 vs pool64aa) pass.
- R3: the frozen parity rule of the original protocol (paired res/pool 95% interval contains 1 and lies in [0.9, 1.1] at w = 0, 0.1, 1) is re-evaluated on the 5-round data for L = 16 and 64, set beside the L0 result, not pooled with it.
Class: replay / infrastructure evidence. Not an algorithm speedup.

## Decision rule
Report the lab's result.json state verbatim. A failed admission (L2, replay or A/A) is reported as failed with its receipt; nothing is retried or edited to pass. One dispatch (a rerun after a Job exists is a new experiment).
