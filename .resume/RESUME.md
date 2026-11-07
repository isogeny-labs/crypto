# Resume: crypto @ claude/gpu-ic-baseline-modal-20261107

## Goal
User request (2026-11-07): draft the EKS GPU pool for review and also use Modal, for the first hardware
baseline of gpu/semaev and gpu/rootfind (no kernel in either has ever run on a GPU).

## Lead / hypothesis
Engineering stage diagnostic, not a lead. Does the device agree with the verified host path, and how fast
does it run (clmad vs software product, per-row vs per-pair, sorted vs hash lookup)? Predictions P1-P4 are
in the spec.

## Spec
research/gpu_ic_baseline_20261027/PROTOCOL.md including Amendment 1 (frozen at the commit that adds
gpu/modal/). Container job: gpu/modal/ic_baseline_job.sh. Launcher: gpu/modal/modal_ic_job.py.

## State
Protocol, Modal launcher/job and the EKS GPU pool draft (taskq/deploy/eks-karpenter/gpu-draft/, NOT applied)
are committed on this branch. No Modal run yet. Modal credentials are in the session environment
(MODAL_TOKEN_ID, MODAL_TOKEN_SECRET); the `modal` client is pip-installed (1.6.3).

## Next steps
1. Checkpoint push must succeed (needs CHECKPOINT_UPLOAD_TOKEN or AWS creds; neither was present).
2. `modal run --detach gpu/modal/modal_ic_job.py --out /tmp/gpu-ic-baseline/run-01`; if the client dies,
   `--fetch-token <token>`.
3. If the selftest gate fails, that is the result: report it, no throughput.
4. Otherwise score P1-P4, write RESULTS, graph, PDF, benchmarks/ row only if the validation gate allows
   (it does not for a Modal-only run), session log, PR.

## Runs in flight
C = 1 boundary study (owner approved 2026-11-07 as a labelled boundary result outside the main ratchet): one-seed wwalk (existing mode `wwalk:...,g=1,grow=1` on a size=1 base, commit f51911e130 by another session) screened at k = 20, 24, 28, 32 on five dev curves with seed=run. Under benchrec record in scratchpad recc1, branch claude/curve25519-c1-boundary.
Specs: research/curve25519_autolab_20261104/c1/spec-screen-k*.json (20 arms each). Confirmation curves c1/confirm-curves.json untouched.
Next: pre-register ONE C = 1 arm per size from the screen, run it vs inc-c2 and rho-neg on the confirm curves (12 targets x 2 rounds), report S, ratio to rho and S relative to the C = 1 floor 1.329 as a boundary result; no new oracle, no change to rho.

## Stop and success conditions
One Modal launch, 2 h function timeout, expected under 30 min. Any selftest failure stops throughput.

## Open questions
Whether sm_120 is available on Modal as "RTX-PRO-6000" for this workspace; whether the g7e sizes and
G-family Spot quota exist for the EKS pool.
