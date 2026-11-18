---
title: "Protocol: an r-only (k, N) sizing rule for the k-sum arm (prune + split + stride-renew) on Table B"
date: 2026-11-18
status: registered before any confirmatory run (hashes in REGISTRATION.sha256)
---

# Protocol: retuning the k-sum factor-base size N and summand count k for the post-split cost curve `[pcc:unverified]`

**Track: generic decomposition (bounded by sqrt(r); cannot beat rho or BSGS).** Nothing here moves the algebraic track.

## 1. Incumbent, profile, lever

**Incumbent.** The `-psr` arm of `research/ic_ksum_split_20261118` (validator VAL-20261118-1e81cb): `ic.pipeline`,
`oracle = ksum:m=<k>,prune=1,split=1`, `targets = stride-renew`, the producer's factor-base sizes (copied from the prune
study, never re-tuned after split/stride-renew), and per row the k that study found cheaper at w = 0 (its README "best
after" column): k = 5 on P20, P22, P24, B21, B24, B27; k = 4 on B15, B18.

**Step-1 profile (prior data only: the split study's 24 raw sessions, measured rounds, `-psr` arms).** For every one of the
16 arm-rows: table entries `T = 2^(k-1) C(N, k)` exactly (`table_insert_attempts`); hit rate x r / T = 1.93-2.04 (so a
trial hits with probability about 2T/r); relations = N + 0.3..0.9; set-up adds = T + pair table + pruned prefixes (B24 k = 5,
N = 9: 2,216, reproduced exactly); one group addition, one lookup and one guard probe per trial; fixed work of the walk set-up
(6 scalar multiplications) and factor base independent of (k, N). Share of S at w = 0: set-up 28-57 %, relation trials
43-67 %, other at most 4 %. Run-to-run CV of S 0.10-0.30 (trial count is a sum of about N + 1 geometric waits).

**Other levers considered.** (a) The walk's per-step cost is already one addition and two native probes per trial: nothing
left to cut without changing the oracle. (b) The 6 set-up scalar multiplications are about 1 S on B15 (r = 2^14) and
0.1 S on B27: a fixed cost like lever B of the split study, shrinking with r. (c) Trial-count variance is intrinsic (sum of
N + 1 geometric waits); larger N lowers it only slowly. None is clearly bigger than balancing T against the trials, which is
the caller's lever.

**Lever: the sizing rule (`rule.py`).** `C_w(N, k; r) = build(N, k) + trials + w (T + 2 trials)`, with
`trials = (N + 0.6) r / (2 T)` and `build = T + 2 C(N-k+2, 2) + sum_{j=2}^{k-2} 2^(j-1) C(N-k+j, j)`; pick the (k, N), k in
3..7, minimising it. Inputs: r and the field family (which base family to name). Constants fitted once on the split study's
committed data; no target or seed enters. **R0** (commit c3a7472845, before any pilot run) minimised `C_0`. **R1** (this
protocol, after the pilot, section 2) minimises `C_0.1`, the band middle. As a table on log2 r (R1):
12: k4 N6, 13.5: k4 N7, 15: k5 N7, 16: k4 N8, 17: k5 N8, 19: k5 N9, 20.5: k6 N9, 21.5: k5 N10, 22.5: k6 N10, 24: k7 N10,
24.5: k5 N12, 25: k6 N11, 26: k7 N11, 27: k6 N12, 28: k7 N12, 30: k6 N14, ... (`python3 rule.py <r>`).

| row | log2 r | incumbent (k, N) | R0 | R1 |
|:--|--:|:--|:--|:--|
| P20 | 20.00 | 5, 9 | same | same |
| P22 | 21.65 | 5, 10 | same | same |
| P24 | 23.38 | 5, 11 | 6, 10 | **6, 10** |
| B15 | 13.99 | 4, 7 | same | same |
| B18 | 15.00 | 4, 7 | same | **5, 7** |
| B21 | 20.00 | 5, 9 | same | same |
| B24 | 21.00 | 5, 9 | 6, 9 | **6, 9** |
| B27 | 24.41 | 5, 12 | 7, 10 | same (5, 12) |

*Sources checked* (the research-kb MCP server and the lead atlas are unavailable in this container):
`research/ic_ksum_split_20261118/` (README, PROTOCOL, summary.json, VALIDATOR_REPORT.md, raw sessions),
`research/ksum_m67_pilot_20261115/README.md` (m = 6/7 null, measured before `split`, when each entry cost more set-up),
`src/cryptanalysis/ic_boundary.rs` (`KSumTable::build_negation_folded_split`, `pair_sums`, `extend_split`,
`KSUM_MAX_K = 9`, `ksum_index_bits`).

## 2. Pilot (done; not data)

8 sessions (one per row), fresh seeds (`target_seed = 20261118200 + i`, `seed = 20261118300 + i`, i = row index), 64 public
targets, 1 round, no warm-up; per row the k = 4..7 arms at each k's R0-optimal N and N +/- 1, plus the producer's N
(`pilot/specs/`, `run_pilot.sh`, binary below). 6,656 k-sum runs, all verified; 8/8 `verify --replay-all` audit OK. Summary
`pilot/pilot_table.md`, `pilot/pilot_summary.json` (`pilot/analyse.py`). Raw sessions: `pilot-sessions.tar.gz` (sha256
`a2ba7a4a3c976256dfa8180ba5c0a476189eea05234ef04f75192755da270ba1`, outside Git; raw tree deleted after summarising).

- The model ranks the grid as observed: its optimum is the empirical best (w = 0) on 6 of 8 rows. On B18 the empirical best
  is k5 N7 (0.969 [0.921, 1.019] against k4 N7), and on B24 it is the model's pick.
- R0 against the incumbent, w = 0 / 0.1 / 1: B24 (k6 N9) 0.932 [0.874, 0.995] / 0.919 / 0.874; P24 (k6 N10) 1.016
  [0.968, 1.066] / 1.016 / 1.019; B27 (k7 N10) 1.001 [0.952, 1.053] / 1.014 / **1.061 [1.000, 1.126]**; other rows unchanged.
- **Revision R1.** On B27, R0's pick is a model tie at w = 0 that the model itself predicts loses at w = 1 (13.31 against
  12.54), and the pilot shows that loss. R1 therefore picks at the band middle w = 0.1. This keeps B27 at the incumbent and moves
  B18 to k5 N7 (model 4.85 against 4.87 at w = 0.1; pilot 0.969 / 0.959 / 0.922). R1 is chosen after reading the pilot; the
  confirmatory run below uses seeds the pilot never used.

## 3. Confirmatory design and registered tests

**Rows.** The three rows where R1 changes (k, N): P24, B18, B24. On the other five rows the candidate is the incumbent (same
method, same parameters), so the ratio is 1 by construction and they are not run.

**Sessions.** 3 replicates per row, 9 sessions (`specs/<row>-r<rep>.json`, `make_specs.py confirm P24,B18,B24`). Curves,
targets_per_curve 48, 3 measured rounds, 1 warm-up, order `alternate`, L0, timeout and max_trials are copied from the split
study's spec of the same name. **Fresh seeds:** `target_seed = 20261118600 + idx`, `seed = 20261118700 + idx`,
`idx = 10 * (row index in make_specs.ROWS) + rep` (never used by the split study, 20261118000/500 + idx, or the pilot,
20261118200/300 + i). Arms: `rho-neg` (reference), `rho-neg-aa` (control), `incumbent`, `retuned` (rule R1), same targets
and algorithm seed per round.

**Binary.** No code changes in this study. `ecbench` sha256 `808511395723f84505d4fe8edaf32e66e8d8bff34cdad57777360c5999ffe161`:
the split study's validated binary (`cargo build --release --locked --bin ecbench`, clean worktree of e93c3cb2da;
VAL-20261118-1e81cb). `run_all.sh` refuses any other binary. The k = 6 split path is exercised by the pilot (verified).

**Route.** Local, `--cpus none`, L0, counted cost only, one session at a time (`ecbench run --wait` on its own lock, under
`flock` on the scratchpad CPU lock), inside `tools/benchrec.py run`. `ecbench verify --replay-all --exit-code` on every
session. Budget at most 3 CPU-hours in total (pilot included).

**Test T (primary, per row).** `R_w = cost(retuned) / cost(incumbent)`, paired by target and round:
`ecbench compare --dir <the 3 sessions> --a incumbent --b retuned --weight w --resamples 4000 --seed 20261118` (ops ratio of
totals), and the same per replicate. Upper end = the larger of the linearised-t and bootstrap 95 % upper ends. A row
**holds** if the upper end is below 1 at w = 0, 0.1 and 1, pooled and in each replicate, and
`1 - max upper > max_w |R_aa(w) - 1|` (`R_aa` = `rho-neg-aa` / `rho-neg`, expected exactly 1). A row is **significantly
worse** if the pooled lower end (smaller of the two) is above 1 at any weight.
**Rule verdict:** the rule is an improvement if at least one row holds and no row is significantly worse. A row that neither
holds nor is significantly worse is reported as no detectable change. `analysis/evaluate.py` computes all of it.

**Multiplicity.** Three row tests; Holm across rows (levels 1 - 0.05/3, 1 - 0.05/2, 0.95) is reported beside the 95 % verdict when
they differ, from the linearised-t interval rescaled to the adjusted level.

**Distance remaining (not a gate).** `S(retuned) / S(rho-neg)` and `S(incumbent) / S(rho-neg)` per weight, ratio of totals, on
the changed rows; the other rows keep the split study's distance (2.78-4.97 at w = 0 for the best arm).

**Predictions** (fixed now; from the pilot and the model): B24 `R_0` 0.93 +/- 0.05, `R_1` 0.87 +/- 0.07 (holds); P24 `R_0`
0.97-1.03 (no detectable change); B18 `R_0` 0.97-1.02, `R_1` 0.90-0.98 (likely fails the all-weights rule at w = 0; the split
study measured k5/k4 = 1.02 there at w = 0).

**Not claimed:** wall time, scaling beyond these curves, an algebraic-track result, any ranking against tables A and C. All
numbers `[pcc:unverified]`; no leaderboard or `benchmarks/README.md` row (the artifact store is down, so the validation gate
cannot be met).

## 4. Deviations (declared before data)

- Route: local L0 instead of isolab-eks; no S3 checkpoint or raw-data upload (no AWS in this container); `.resume/RESUME.md`
  belongs to another lane and was not rewritten. Conductor and research-kb unavailable.
- Binary reused rather than rebuilt: the study changes no code, and the validated binary avoids the stale-`target/` hazard.
- Raw sessions stay in the scratchpad as gzip archives pinned by sha256 (disk limit); verify reports are summarised in
  `data/`.
