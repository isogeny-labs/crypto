# Resume: crypto @ claude/relaxed-lovelace-wjy1um

## Goal
Owner (2026-11-11/12): finish E14 of the fall-degree study: the exact last fall degree `d_F` at
(m, n, ℓ) = (3, 21, 7) (first exact value above 4) and the three unmeasured E13b controls
(6:10:3 draws 1, 3, 4); then (4, 28, 7) windows as time allows.

## Lead / hypothesis
`research/fall_degree_bounds_20261008/PREREGISTRATION.md`, Amendments 17–21. Frozen readings and
predictions in Amendment 20: E14a "d_F = 5" 0.50, "≥ 6" 0.25, "split" 0.05, "not completed" 0.20;
E14b "controls confirmed" 0.55; E14c "no reading" 0.80.

## Spec
One RunPod CPU pod (cpu3g, 64 vCPU, Secure, not interruptible), USD 100 cap, deadline
min(36 h, 100/costPerHr). Source: sparse clone of `isogeny-labs/crypto` at `a2be6609`
(tree sha256 `da6ee5f3…`). Engines: `hky_par newcells --cells 3:21:7 --seed 20261012 --keep 4
--cmax 5` (E14a), `dreg_linear_guess --engine pe` (E14b, threads from gate G3),
`e12/run_window.sh 4:28:7` (E14c). Gates G0–G3 and G-src-a/b/c before any draw.
Launcher `e14/launch_r2.sh` (OWNER_OVERRIDE=1 per Amendment 21), controller `e14/pull_r2.sh`.

## State
Launching 2026-11-12 ~02:10 UTC. `e14/pod-r2/launch.json` holds pod id and output token.
Owner override of the balance rule recorded (balance USD 99, sibling pod burning USD 5.29/h:
zero-balance termination after ~12 h is possible and was accepted).

## Runs in flight
Pod named `fd-e14-r2-<stamp>`; outputs pulled every 10 min to `e14/pod-r2/pulls/`, final to
`e14/pod-r2/out/`. Session: session_01AXUKfTRWdZTFrWGLHJZp11.

## Stop and success conditions
The pod stops itself at the deadline or when done. A fresh session resumes by running
`e14/pull_r2.sh` (idempotent) with the pod id from `launch.json`; if the pod is gone, score
whatever snapshots exist with the Amendment 19/20 reading rules ("not completed" for anything cut
off). Nothing is retried without a new amendment.

## Next steps
1. Pull outputs; when `final_status` arrives, delete the pod (the controller does this).
2. Score: extend `e14/score.json` (schema `fall_degree_e14_score/v1`) with an r2 section per the
   Amendment 20 rules; report + graph + PDF under `reports/run-<UTC>/`; append RESULTS.md.
3. Lean scoring table: add E14 rows (kind 8) if any reading is registered.
4. Commit on the branch, PR, session log.
