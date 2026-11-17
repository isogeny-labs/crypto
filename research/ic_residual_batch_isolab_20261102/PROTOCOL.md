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

## Amendment 1 (2026-11-02, no Job was created; nothing measured)
Dispatch: workflow run 41496023931 (run 178, workflow on main f147d0c914), source commit 1307de808809b3ff29014b475762e2ab312e0edb, spec as frozen.
The build job built and published the image (digest sha256:6084e1ca33e5cd0f889239995f5e079c136916b15f2678acb0873cc09392847c, ecbench sha256 a776f12252172e761a8d23405a83e10c93a648e25fefda4e0858f411f54e1306).
The launcher failed before submission: result state `launcher_failed`, exit 127, no Job/Pod UID, request sha256 7258ef550fd9a5280a5d9d6d8fd8d573d484549d756210cb2561ea5acc59d9a3,
receipt s3://isolab-results-104186811093-us-east-2/runs/gh-41496023931-1/launcher/22a809fb-9936-40b5-95a0-bdac17593f88/ (readiness ready=true; overview.json empty; no submission.json).
Cause, from `taskq/deploy/eks-karpenter/remote/run-request` and the receipts: the step after `check-ready` is `isolab-eks-mcp call isolab_k8s_overview`, and exit 127 with an empty overview.json means the
`isolab-eks-mcp` binary was not found in the CodeBuild launcher environment. The launcher scripts and workflow are unchanged between the last successful workflow run (177, main a56dbd30) and this one,
so this is a launcher-environment problem outside the request. This session has no CodeBuild read or deploy permission (codebuild:BatchGetBuilds denied), so it cannot be diagnosed or fixed here.
Because no Job was created, a later dispatch with the same request after the launcher is repaired is not a rerun of a measurement. No further dispatch until then.
