# Resume: E14 r2 (fall-degree study), claude/relaxed-lovelace-wjy1um

Kept here rather than in `.resume/RESUME.md`, which is a single slot shared by every session and
currently holds another session's checkpoint.

## Goal
Owner (2026-11-11/12): finish E14: the exact last fall degree `d_F` at (m, n, ℓ) = (3, 21, 7)
(first exact value above 4) and the three unmeasured E13b controls (6:10:3 draws 1, 3, 4); then
(4, 28, 7) windows as time allows.

## Rules
`research/fall_degree_bounds_20261008/PREREGISTRATION.md`, Amendments 17–30. Readings, gates and
predictions: Amendment 20; owner override of the balance rule: Amendment 21; 32 vCPU: 25; G2 key: 27;
CPU probe: 28–30.

## Current run
Pod `fd-e14-r2j-20261112T0457Z` (`gjr9mpprgga8eb`), cpu5m 32 vCPU 256 GB, USD 2.08/h, launched
2026-11-12T04:57:05Z, deadline 2026-11-13T16:57:18Z (USD 100 cap). Probe 14.5 of 32; build done
05:00:59; G0 started 05:00:59. Pod id and output token in `e14/pod-r2/launch.json`.

## How to resume
1. `bash research/fall_degree_bounds_20261008/e14/pull_r2.sh` (idempotent; pulls every 10 min,
   redacts the key, deletes the pod after `final_status`).
2. Read the pod's `run.log` at `https://<pod_id>-8080.proxy.runpod.net/<out_token>/out/run.log`.
3. If the pod is gone, score whatever snapshots exist (`e14/pod-r2/pulls/`, `out/`) with the
   Amendment 19/20 rules; anything cut off is "not completed". Nothing is retried without a new
   amendment.
4. Never commit `pulls/` or an unredacted trace: the pod trace can contain the RunPod key.

## After the run
Score into `e14/score.json` (schema `fall_degree_e14_score/v1`, new r2 section), report + graph +
PDF under `reports/run-<UTC>/`, append `RESULTS.md`, add Lean rows (kind 8) if a reading is
registered, session log, PR.

## Budget
Balance at launch USD 79.40; other pods on the account about USD 5.6/h; combined about USD 7.7/h,
so the account runs dry around 15:00 UTC on 2026-11-12 unless topped up.
