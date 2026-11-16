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
05:00:59; G0–G3 passed by 05:33:41 (G3: `pe` at 8 threads, 341 s); E14a (`hky_par` cmax 5,
32 threads) started 05:33:41, time box until 17:33 UTC. Pod id and output token in `e14/pod-r2/launch.json`.

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

## Progress (2026-11-12 16:12 UTC)
E14a draw 0 finished at about 16:05: `d_f = 5`, 37,890 s, degree ≤ 4 equal to E10, so it is valid
(`pod-r2/out-r2j/A_3-21-7.draw0.jsonl`). E14a continues with draws 1, 3, 4 at about 10.5 h each.
The pod deadline (11-13 16:57Z) allows at most draw 3. At USD 3.43/h, the account balance
(USD 34.69 at 16:10) runs out about 02:15Z on 11-13, before draw 1 (about 02:35Z), unless topped up.

## Outcome (scored 2026-11-14 19:15 UTC)
The run is over and scored. E14a: draws 0 and 1 valid, both "`d_F = 5`". Draw 3's record was lost
with the pod, and draw 4 was cut off by the deadline. Cell reading "split" (2 measured; ≥ 3 needed).
E14b "not completed" and E14c "no reading". See `score.json` (`r2`), `RESULTS.md` and
`reports/run-20261114-e14r2/`. Pod `gjr9mpprgga8eb` returned 404 at 2026-11-14T19:09Z. Nothing
further runs without a new amendment.

## E14 r3 (Amendment 31), prepared 2026-11-14, not launched
Part a: E14a draw 3 alone via `hky_par ladder` on E10's draw-3 record. If it reads 5, the cell
reads "`d_F = 5`" (3 of 4). Part b: the three 6:10:3 `pe` controls, run concurrently at 8 threads.
Launch with `MODE=a research/fall_degree_bounds_20261008/e14/launch_r3.sh` (dry run first with
`DRY_RUN=1`). Part b goes the same way once its admission passes. Pull with
`MODE=a research/fall_degree_bounds_20261008/e14/pull_r3.sh`: it starts a stopped pod, pulls, and
deletes the pod after `final_status`. Pods stop themselves 2 h after finishing; they never delete
themselves. Expected: part a about 11 h (USD 23), part b at most about 7 h (USD 15).
Status at 19:40 UTC: the launch was blocked by the session's permission check; it waits for the owner.

## E14 r3 part a running (2026-11-15)
Pod `5nxsdf6du9au1i` (`fd-e14-r3a-20261115T0033Z`, 32 vCPU, 256 GB, USD 2.08/h), launched 00:33:09Z, deadline 12:33Z.
Gates passed by 00:41:49 (input sha256 0c6437f9… as registered; GL equal to E10 draw 3). Draw 3 started
00:41:49 and is expected about 11:20Z. After finishing, the pod serves for 18 h (Amendment 32) and then
deletes itself. Pull with `MODE=a research/fall_degree_bounds_20261008/e14/pull_r3.sh`, which deletes
the pod after `final_status`. Part b launches when its admission passes (balance USD 106.77 at 00:33).
