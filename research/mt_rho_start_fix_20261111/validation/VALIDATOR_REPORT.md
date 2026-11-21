# Validator report: rho-family restart fix and the corrected v2 IC4 comparison (2026-11-11)

Validator: independent `validator` subagent (Opus 5.5), 2026-11-11. This is the same validation session as `../../mt_fieldops_v3_20261111/validation/` and uses the same scripts, builds and nonce.

**Verdict: `valid_with_amendments` for the correction. Disposition of "the v2 headline survives: IC4 is still cheaper than rho at T = 16384": `insufficient_evidence` (blocking).**

- **The defect and the fix are as described, and the fix is correct.**
  - Restarts at fixed-bank sums capped the walks; chained starts, one counted addition per restart, remove the cap.
  - The diff touches only the five rho-family restart hunks.
- **The rerun and the merge are exact.** All 2112 merged rows replay exactly with the fixed source, including the 528 v2 `ic4`/`bsgs2` rows produced by the v2 binary. The merge is byte-exact.
- **Every number in RESULTS.md recomputes exactly.** That covers the table, the 1.4%–4.8% reference change, 0.951–0.991, the 0.9% margin and the three T = 4096 moves.
- **Two problems:**
  1. "Consequences" describes as done a claim correction and leaderboard and benchmarks updates that do not exist at the validated commit. The leaderboard still shows the pre-fix IC4/rho numbers as validated.
  2. "IC4 still cheaper than rho at T = 16384" holds only against the harness's anchored, fixed-database rho family:
     - with the fixed rho's residual restart cost and repeated lookups removed, only c_I = 3 stays cheaper (2 of 6);
     - with Kuhn–Struik retention added to rho2x (validator, 528 runs), IC4 at T = 16384 is 1.13–1.16× at h = 0 and 1.09–1.11× at h = 0.1 (c_I = 10, every 95% lower bound above 1);
     - all six T = 16384 decisions become unranked, and the other 12 dearer.

## 0. Artifact set

- **Commit:** `70d0b40bc365c1269dfc3c1251153cbc44cc589e`, read from a git-archive copy. The study directory in the working tree was identical to HEAD.
- **Rows:** the corrected rho rows (`results/rows.jsonl`, 1584) bind commit `e821c5fbaf` and binary `773fb2ca…`. The kept v2 `ic4`/`bsgs2` rows bind `6b0ac6f192` and binary `bbcc68cc…`.
- **v2 rows:** the v2 `rows.jsonl` is in the artifact store. I used the local copy, whose sha256 `cafb8c71…7a7a` matches the v2 `.artifacts.json`.
- **Builds and host:** as in the v3 report, §0.
- **HEAD moved during validation** to `a9907c9254`, outside the validated set.
  - `866d8b9d0a` adds this record's metrics and raw hashes; it stays open with `uploaded: false`.
  - `a9907c9254` changes only `scripts/build_ic_leaderboard.py`, which now points at v3 and "links the v2 correction". The rendered `docs/ic/LEADERBOARD.md` and the benchmarks README are unchanged at that HEAD, so §5 still holds there.

## 1. Dispositions

| # | requirement | disposition | evidence |
|---|---|---|---|
| 1 | Freeze: protocol, fix, run.py and merge.py committed (75dbc70b36, 08:00:17Z) before the rerun rows (run started 08:01:01Z per the bench record; rows bind e821c5fbaf) | **pass** | git log; bench record |
| 2 | Fix correct, rho-only, no free work | **pass** | §2 |
| 3 | Replay | **pass**: merged 2112/2112 (instrumented build; this includes the 528 v2 ic4/bsgs2 rows under the fixed source), 108/108 nonce sample (pristine build) | §3 |
| 4 | Certificates | **pass**: 36 runs (one per arm × cell), 258,048 targets, 0 failures | §3 |
| 5 | Recompute | **pass (exact)**: 324/324 summary entries; RESULTS table 6/6 rows (ranges and decisions); prose numbers | §4 |
| 6 | Merge integrity | **pass**: merged.jsonl = v2 non-rho lines + rows.jsonl, byte-exact; the new rho jobs have the same argv as the v2 jobs | §4 |
| 7 | "Consequences" applied | **fail**: not at the validated commit | §5 |
| 8 | Strength of the corrected reference | **fail as "rho"**: residual overheads and missing Kuhn–Struik retention change the T = 16384 decisions | §6 |

## 2. The fix

- **Before:** phase 1 restarted at B_i + B_j + B_k and phase 2 at B_i + B_j + Q, both from a fixed 64-point bank. That allows C(66, 3) = 45,760 phase-1 walks and C(65, 2) = 2,080 starts per target.
- **After:** s ← s + B_k, and from Q, tp ← tp + B_k. That is one counted addition per restart, with unboundedly many starts. The logs carry along (s = sa·G, tp = Q + ta·G).
- **Checks:**
  - every target of every rerun row verifies by the counted check;
  - 36 runs were verified independently;
  - `git diff f71b42cc4f 75dbc70b36 -- examples/mt_fieldops.rs` is exactly the five restart hunks (rho/rho2 phases 1 and 2, rho2x phases 1 and 2, rhoseq phase 2).
- **Per run the fix is not monotone.**
  - At c_I = 10, h = 0.1, the share of runs that became cheaper was 96.5% (rho), 95.5% (rho2), 95.7% (rho2x) and 84.3% (rhoseq).
  - The worst run became 4.3% dearer.
  - rhobest is cheaper in every cell, c_I and h, by 1.41%–4.75%.
- **What the fix leaves:**
  - one non-productive addition per 32-step restart: 2.2%–2.3% of rho2x's cost;
  - repeated phase-2 lookups: 2.1%–3.1% of rho2/rho2x lookups. Part of this is new: with probability 1/64 the chained increment equals the previous walk's first arm, and the walk retraces it;
  - phase-1 merge waste: rho at kappa 1.0 still takes 1.18–1.26 steps per new point at T = 16384.

## 3. Replay and certificates

- The instrumented build replayed all 2112 merged rows with 0 mismatches in every emitted field except `ms`. That includes the 528 v2 `ic4`/`bsgs2` rows, which shows that their code is unchanged and the merge is valid.
- The pristine source build replayed a nonce sample (nonce `7bc8adf9…`, 3 per arm × cell): 108/108 match.
- **Independent EC check:** one run per (arm, bits, T) = 36 runs, 258,048 targets, 0 failures. Planted lists are identical across arms within each configuration. The negative control is shared with Study A and was caught in all 7 arms.

## 4. Recompute

- `scripts/recompute.py` (v2 mode) on `runs.json`, an exact projection of `merged.jsonl`:
  - 324/324 `summary.json` entries are bit-identical;
  - the numpy bootstrap gives the same decisions.
- **IC4 vs rhobest, corrected:** cheaper 6, unranked 3, dearer 9.
- **v2 as published** (recomputed from the v2 rows): cheaper 6, unranked 6, dearer 6.
- **RESULTS table:** all 6 rows match (ranges to 3 decimals, decisions per c_I).
- **Prose:**
  - "1.4% to 4.8%" → 1.405%–4.753%;
  - "0.951-0.991" → 0.9513–0.9908;
  - "At 25 b, c_I = 30, h = 1 the margin is 0.9%" → 0.9908 [0.9861, 0.9927];
  - "three (cell, c_I) decisions move from unranked to dearer" → 24 b/4096/c_I 30, 25 b/4096/c_I 10, 25 b/4096/c_I 30;
  - "IC4 / bsgs2 is unchanged": the ratios are identical.

## 5. "Consequences" not applied

RESULTS says the following were done. At `70d0b40bc3` none of them is:

- PCC-20261111-fa13c8 superseded "by the claim recorded with this correction";
- the leaderboard component section corrected;
- `benchmarks/20261111-mt-fieldops-ic4/README.md` corrected.

The state at the validated commit:

- `pcc/claims` has no new claim.
- `docs/ic/LEADERBOARD.md`, `docs/ic/leaderboard.json` and the benchmarks README were last changed in `4fbb181006`, before the fix.
- `docs/ic/LEADERBOARD.md` (lines 580–592) still shows the pre-fix ratios, for example 24 b, T = 16384, 0.933 at h = 0, labelled "validated".
- The session log lists these as open threads.

## 6. Strength of the corrected reference

Table at c_I = 10, from `scripts/sens.py`, `scripts/run_ks_cfg.py` and `scripts/dp_estimate.py`:

| bits | T | IC4/rhobest h = 0.1, corrected | same, restart + repeats removed | IC4 / (rhobest ∪ rho2xks), h = 0 / 0.1 / 1 | 5a with rho2xks (c_I 3/10/30) | IC4 / DP-rho, h = 1 (**estimate**) |
|---|---|---|---|---|---|---|
| 22 | 1024 | 1.086 [1.077, 1.090] | 1.118 [1.112, 1.123] | 1.331 / 1.264 / 1.131 | dearer ×3 | 1.76 |
| 24 | 1024 | 1.152 [1.142, 1.162] | 1.193 [1.181, 1.200] | 1.474 / 1.380 / 1.204 | dearer ×3 | 2.12 |
| 24 | 4096 | 1.007 [0.999, 1.013] | 1.037 [1.029, 1.043] | 1.233 / 1.170 / 1.032 | dearer ×3 | 1.60 |
| 24 | 16384 | 0.970 [0.968, 0.974] | 0.996 [0.994, 1.000] | 1.131 / 1.092 / 0.976 | unranked ×3 | 1.33 |
| 25 | 4096 | 1.022 [1.019, 1.029] | 1.055 [1.051, 1.061] | 1.283 / 1.207 / 1.051 | dearer ×3 | 1.73 |
| 25 | 16384 | 0.973 [0.969, 0.974] | 1.000 [0.996, 1.002] | 1.157 / 1.109 / 0.984 | unranked ×3 | 1.42 |

- **Residual overheads removed** (counterfactual on measured counters): T = 16384 stays cheaper only at c_I = 3. At c_I = 10 it is unranked and at c_I = 30 dearer. Overall: cheaper 2, unranked 2, dearer 14.
- **With Kuhn–Struik retention** (`rho2xks`: rho2x plus one counted hash insert per retained point of a solved walk; 528 runs at kappa 0.05/0.1/0.25/0.5, all verified):
  - IC4 is dearer at h = 0 and 0.1 in all 18 (cell, c_I);
  - at h = 1 it is cheaper only at T = 16384, where the anchored rho2x remains the reference;
  - section 5a: unranked 6, dearer 12, cheaper 0.
- **Distinguished points (estimate, not measured):** a distinguished-point Kuhn–Struik rho at h = 1 is estimated at 1/1.33 to 1/1.42 of IC4's cost at T = 16384.

## 7. Required amendments to RESULTS.md (and the public record)

1. **BLOCKING:** reword "The v2 headline survives" to: "IC4 stays cheaper than the cheapest of the 12 runs of this harness's anchored, fixed-database rho family at T = 16384". Add that this is not robust:
   - removing the fixed rho's residual restart cost and repeated lookups leaves only c_I = 3 cheaper;
   - against a rho with Kuhn–Struik retention (validator, measured), IC4 is 1.12–1.17× dearer at h = 0 and the decision is unranked.

   Correct PCC-20261111-fa13c8, the leaderboard section and the benchmarks README to this before the v2 row is cited as "IC4 cheaper than rho".
2. Either apply the "Consequences" (new pcc claim, leaderboard and benchmarks README) in the same PR, or reword them as pending. At the validated commit they are not done, and the leaderboard still shows the pre-fix 0.91–0.95 numbers as validated.
3. Say that the fix is not monotone per run (up to +4.3% on 3.5%–15.7% of runs) and that chained starts repeat a walk with probability 1/64 per restart.
4. Move `results/merged.jsonl` (1.45 MB) and `results/rows.jsonl` (1.06 MB) to the artifact store with a manifest, as AGENTS.md §13 requires. Close the bench record with raw hashes and metrics.

## 8. Scripts and evidence

- The scripts are copies of the Study A set (`validation/scripts/`); see the v3 report §9 for their roles.
- `ks_runs.json`: the 528 retention-arm runs.
- `rho_instrumentation.json`: per corrected rho row, the restart cost vectors and counts, the phase-2 lookups and the repeated lookups.
