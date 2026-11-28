# Lean base + sibling pair table (no tau check) against the incumbent on K1 n23, n29, n37 (2026-11-18) `[pcc:unverified]`

Confirmatory study of the two levers that passed validation at n29 (VAL-20261118-43658a), `lean_layout=1` on
`compact-orbit-scan` and `sib=1` on `mitm-frobenius-layout-counted`, **with `tau_verify` off**, against the validated
incumbent of each curve: `sp-c4-d5` (K1 a=1 n23), `gen-c1` (K1 a=1 n29), `sp-c8-d5` (K1 a=0 n37), with
`rho.signed_frobenius_strong` and `rho.signed_frobenius_v5` in the same sessions. Every number is `[pcc:unverified]`:
counted GAE from local L0 ecbench sessions, **no independent validation yet**. Protocol: [`PROTOCOL.md`](PROTOCOL.md),
frozen at 2d67198576 (hashes in `REGISTRATION.sha256`) before any data. No code change: both levers already apply to the
spread-seed (`generator_multiples = columns`, `multiple_doublings`) arms. Binary: the n29 study's, built from 33dccd579d,
sha256 `43f44fec…5c3e65`. Report: [`REPORT.pdf`](REPORT.pdf); figures `figures/fig1-phases-band.svg`, `figures/fig2-ratios.svg`.

Design: 3 replicate sessions per curve (fresh target seeds 20261118431-439), each 48 public targets x 3 rounds, 1 warm-up,
alternating order; 6 arms per session (2 rho references, incumbent, A/A, default-off control, candidate `ls`). 9 sessions,
10,368 records, `verify --replay-all` 0 problems on all 9 (7,776 replays reproduced). Decision rule (frozen): `ls`/incumbent
upper 95% bound < 1 at w = 0, 0.1, 1, pooled AND in each replicate; the bound is the wider of ecbench compare's
linearised-t and cluster-bootstrap intervals.

## Result (primary, one test per curve)

| ls / incumbent (paired) | w = 0 | w = 0.1 | w = 1 | verdict |
|:--|:--|:--|:--|:--|
| **n23** ls / sp-c4-d5, pooled (432 pairs) | **0.770 [0.750, 0.790]** | **0.753 [0.734, 0.772]** | **0.720 [0.704, 0.736]** | **passed** |
| r1 / r2 / r3 | 0.784 / 0.751 / 0.771 (hi 0.820 / 0.785 / 0.808) | 0.767 / 0.736 / 0.754 (hi <= 0.801) | 0.732 / 0.705 / 0.720 (hi <= 0.762) | |
| **n29** ls / gen-c1, pooled | **0.814 [0.801, 0.826]** | **0.639 [0.622, 0.656]** | **0.432 [0.417, 0.446]** | **passed** |
| r1 / r2 / r3 | 0.807 / 0.815 / 0.819 (hi <= 0.842) | 0.630 / 0.641 / 0.646 (hi <= 0.680) | 0.424 / 0.433 / 0.438 (hi <= 0.468) | |
| **n37** ls / sp-c8-d5, pooled | **0.703 [0.686, 0.721]** | **0.732 [0.717, 0.746]** | **0.796 [0.787, 0.805]** | **passed** |
| r1 / r2 / r3 | 0.705 / 0.694 / 0.711 (hi <= 0.743) | 0.733 / 0.724 / 0.738 (hi <= 0.765) | 0.797 / 0.791 / 0.800 (hi <= 0.816) | |

All three curves pass the frozen rule at every weight, pooled and in every replicate. The frozen model expectation at
w = 0 (section 2 of the protocol) was ~0.79 / ~0.83 / ~0.71; measured 0.770 / 0.814 / 0.703. At n29 the measured lean+sib
is slightly better than the validator's derived 0.834 / 0.668 / 0.457 (other seeds).

## Distance to rho, before and after (pooled over the 3 sessions; paired ratios, 95%)

| curve | arm | / rho-v5 (w = 0 / 0.1 / 1) | / rho-strong (w = 0 / 0.1 / 1) | band order vs v5 / vs strong |
|:--|:--|:--|:--|:--|
| n23 | sp-c4-d5 (before) | 1.780 [1.615, 1.969] / 2.166 [1.981, 2.381] / 3.838 [3.553, 4.164] | 0.443 / 0.556 / 1.144 [1.076, 1.219] | v5 < inc / unresolved |
| n23 | **ls (after)** | **1.370 [1.215, 1.550] / 1.631 [1.461, 1.830] / 2.762 [2.515, 3.050]** | **0.341 / 0.419 / 0.823 [0.758, 0.895]** | v5 < ls / **ls < strong** |
| n29 | gen-c1 (before) | 1.075 [0.959, 1.178] / 1.640 [1.474, 1.778] / 4.349 [3.912, 4.694] | 0.087 / 0.140 / 0.531 | v5 < inc / inc < strong |
| n29 | **ls (after)** | **0.875 [0.775, 0.969] / 1.048 [0.931, 1.157] / 1.878 [1.665, 2.062]** | 0.070 / 0.090 / 0.229 | **unresolved** / ls < strong |
| n37 | sp-c8-d5 (before) | 1.757 [1.621, 1.904] / 2.073 [1.916, 2.241] / 3.481 [3.236, 3.749] | 0.948 [0.888, 1.012] / 1.087 / 1.620 | v5 < inc / unresolved |
| n37 | **ls (after)** | **1.236 [1.121, 1.359] / 1.517 [1.385, 1.659] / 2.771 [2.559, 3.001]** | 0.667 [0.610, 0.727] / 0.795 / 1.289 [1.203, 1.384] | v5 < ls / unresolved |

Reading: rho-v5 stays ahead on n23 and n37 at every weight (gap shrinks by ~23-30%). On n29 the w = 0 point and interval
are below rho-v5 (0.875, upper 0.969), but at w = 0.1 the interval contains 1 and at w = 1 ls is 1.9x above, so the §5a band
does not rank ls against v5: **not "below rho"**. Against rho-strong (the leaderboard's reference) ls moves to below at
every weight on n23 (was unresolved), stays below on n29, and stays unresolved on n37 (below at w = 0, above at w = 1).
Pooled band S (w = 0 / 0.1 / 1): n23 inc 0.394 / 0.532 / 1.769, ls 0.303 / 0.400 / 1.274, v5 0.221 / 0.245 / 0.461;
n29 inc 0.391 / 0.649 / 2.972, ls 0.318 / 0.415 / 1.284, v5 0.364 / 0.396 / 0.684; n37 inc 0.264 / 0.343 / 1.054,
ls 0.185 / 0.251 / 0.840, v5 0.150 / 0.165 / 0.303.

## Controls

- Default-off (`lean_layout=0`, `sib=0,tau_verify=0`) and A/A identical to the incumbent on 432/432 records per curve
  (cost, phase counts and native maps without wall time, counters, outcome, `points_sha256`); ratio 1.000 at all weights.
- All 10,368 solves `verified` with `matches_target`; `ls` did the same `targets_tried` as the incumbent on 432/432
  (replicate, round, target) per curve: the levers change no walk, table entry or relation.

## Phase split (mean per solve, w = 0 GAE)

| curve | arm | factor base | pair table | relations | verify | total | factor-base unpriced native ops | sibling second points |
|:--|:--|--:|--:|--:|--:|--:|--:|--:|
| n23 | sp-c4-d5 | 18.59 | 397.62 | 420.73 | 30.10 | 867.17 | 1,196 | |
| n23 | ls | 18.59 | 211.62 | 420.73 | 30.10 | 681.17 | 184 | 182 |
| n29 | gen-c1 | 0.99 | 32.07 | 33.45 | 19.62 | 86.18 | 377 | |
| n29 | ls | 0.99 | 17.07 | 33.45 | 19.62 | 71.18 | 30 | 14 |
| n37 | sp-c8-d5 | 42.89 | 2537.62 | 1669.10 | 38.43 | 4288.16 | 3,848 | |
| n37 | ls | 42.89 | 1349.62 | 1669.10 | 38.43 | 3100.16 | 592 | 1,180 |

(Factor-base unpriced native ops = hash probes + factor-point inserts + point negations + curve-equation checks +
orbit-key squarings.) At n23/n37 with several columns the orbit key is still computed (it refuses a repeated column).
The w = 0 gain is the pair table (sib); the w > 0 gain adds the lean base. At n37, w = 1, the 1,180 sibling second points
priced at 1 GAE each take back part of the pair-table saving, so the ratio rises from 0.70 to 0.80.

Wall time (descriptive, L0, mean microseconds per solve, incumbent -> ls): n23 342 -> 292, n29 172 -> 149,
n37 1,337 -> 1,061.

## What it is (and is not)

Constant-factor engineering on the incumbent arms; same boundary, unit, targets, oracle, walk and pricing. Removed work is
performed work (field inversions, hash-map operations); the sibling second point is priced by the band at w. Not a
relabelling: `tau_verify`, whose w = 0 gain the validator classed as a count relabel, is off. The incumbents remain
Frobenius- and negation-folded baby-step giant-step under the IC harness (gen-c1) or its spread-seed variants; the
asymptotic ceiling is unchanged. T0 only.

## Deviations

- Local L0 run, no EKS/S3 (no AWS here): raw sessions gzipped in the scratchpad (`lsc-sessions/*/records.jsonl.gz`, SHA-256
  of each uncompressed `records.jsonl` in `summary.json` -> `sessions`); compact per-record tables in `tables/`.
- `summarize.py` failed on first run on a key-format mismatch ("n23" vs "23"); fixed after data by stripping the "n"
  (one line, marked in the file; no statistic changed). Its registered hash therefore no longer matches; the registered
  version is in commit 2d67198576.
- `make_report.py` and this README were written after data, as the protocol declares.
- Not pushed; no validator yet; no leaderboard or benchmarks/README row.
