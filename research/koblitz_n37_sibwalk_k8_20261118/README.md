# Sibling walk at k = 8, m = 8 + reuse of formed points on K1 n37, 2026-11-18 `[pcc:unverified]`

Separately registered confirmatory study on K1 a = 0, n = 37 against the lean+sib incumbent `ls` (sp-c8-d5,
`koblitz_lean_sib_confirm_20261118`). It follows the open thread of `koblitz_k1_sibwalk_20261118`, whose n37 test failed
at w = 1 because its pilot changed k to 9; that result stands as measured. Protocol [`PROTOCOL.md`](PROTOCOL.md), frozen
at 624775314b (hashes in `REGISTRATION.sha256`) before any data. Code 217638524d (levers default off), binary sha256
`7b182360…fd315`, reused from the earlier study (no code change since). Every number is `[pcc:unverified]`: local L0
ecbench counts, **no independent validation yet**. Report [`REPORT.pdf`](REPORT.pdf), figures
`figures/fig1-phases-band.svg`, `figures/fig2-ratios.svg`. Benchmark record
`benchmarks/records/20261118T1929Z-koblitz-n37-sibwalk-k8.json`.

## Candidate and the m rule (fixed before data)

`sw` = `ls` + `walk_sib=8`, `ladder_verify=1`, `orbit_key_reuse=1`, keeping k = 8. **m = 8 fixed, no pilot**: at k = 8
the walk tests the same targets in the same order as `ls`, so m only regroups the walk; its effect is a deterministic
function of the trial counts, which the count model reproduces exactly on every record checked before the freeze
(n37 ls, n37 sw k = 9, n23 sw k = 4: 1,296 records). On the earlier study's n37 records (other seeds) the model's m curve
is flat beyond 8 (m = 8 / 12 / 16: 0.736 / 0.733 / 0.732 at w = 0, 0.977 / 0.977 / 0.978 at w = 1), so a pilot over
m in {8, 12, 16} could gain at most ~0.004 at w = 0 while costing ~0.001 at the binding weight w = 1.

## Result (primary; rule: upper 95% bound < 1 at w = 0, 0.1, 1, pooled and in each replicate)

| sw / ls (paired) | w = 0 | w = 0.1 | w = 1 |
|:--|:--|:--|:--|
| **pooled (432 pairs)** | **0.724 [0.705, 0.745]** | **0.808 [0.794, 0.823]** | **0.977 [0.976, 0.979]** |
| r1 (144) | 0.727 [0.695, 0.761] | 0.810 [0.786, 0.834] | 0.977 [0.975, 0.980] |
| r2 (144) | 0.743 [0.705, 0.785] | 0.822 [0.794, 0.851] | 0.976 [0.973, 0.979] |
| r3 (144) | 0.706 [0.677, 0.739] | 0.795 [0.772, 0.819] | 0.979 [0.976, 0.981] |

**Passed** at every weight, pooled and in every replicate (largest upper bound 0.981, r3 at w = 1). As expected by the
frozen model (about 0.74 / 0.82 / 0.98); the model applied to this study's own `ls` records gives 0.724 / 0.808 / 0.977,
equal to the measured ratio of pooled means, and it reproduces every `ls` and `sw` record of this study exactly
(`tables/model_check.json`: 0 mismatches of 432 each). For deterministic counts on an unchanged walk that agreement is
expected and is not evidence of anything beyond correct accounting.

Per solve (means, group ops): relations 1717.3 -> 921.6 (809.7 sibling pairs, each one addition plus one
`sibling_second_points` priced at w; 0.5 lifted hits), verify 38.7 -> 12.8 (ladder check), oracle set-up 1188 and base 35
unchanged; base squarings for orbit keys 288 -> 0 (`min_key_field_squares`). Total GAE at w = 0: 3276 -> 2455. Wall
time per solve (descriptive, L0): 1145 -> 936 µs (relations 571 -> 355, verify 7.7 -> 3.0).

## Distance to rho (pooled, paired, 95%; never a gate)

| arm | / rho-v5 (w = 0 / 0.1 / 1) | / rho-strong (w = 0 / 0.1 / 1) | band order vs v5 / vs strong |
|:--|:--|:--|:--|
| ls (before) | 1.243 [1.113, 1.387] / 1.517 [1.370, 1.679] / 2.735 [2.518, 2.980] | 0.713 [0.645, 0.786] / 0.846 [0.773, 0.923] / 1.357 [1.260, 1.459] | v5 < ls / unresolved |
| **sw (after)** | **0.900 [0.825, 0.983]** / 1.225 [1.125, 1.337] / 2.673 [2.458, 2.915] | 0.516 [0.479, 0.555] / 0.683 [0.635, 0.734] / 1.327 [1.230, 1.428] | **unresolved** / unresolved |

Pooled band S (w = 0 / 0.1 / 1): ls 0.196 / 0.264 / 0.872, sw 0.142 / 0.213 / 0.852, rho-v5 0.158 / 0.174 / 0.319,
rho-strong 0.275 / 0.312 / 0.642. At w = 0 sw is below rho-v5 with the whole interval under 1 (0.900 [0.825, 0.983]),
but at w = 0.1 it is 1.23x and at w = 1 2.7x rho-v5: the §5a band does not order them, and the earlier "v5 < ls" order
becomes "unresolved". **Not "below rho".** Against rho-strong sw is below at w = 0 and 0.1 and 1.33x above at w = 1:
unresolved, as `ls` already was. At w = 1 the walk is neutral by its pricing (the gain there, 0.977, is the ladder check
and the orbit-key reuse), so the w = 1 gap to v5 barely moves (2.735 -> 2.673).

## Controls

- ls-off (new flags explicitly 0) and ls-aa identical to ls on 432/432 records (cost, phase counts and native maps
  without wall time, counters, outcome, `points_sha256`); ls-aa/ls and ls-off/ls exactly 1.000 at every weight.
- All 3,456 solves (6 arms x 576 incl. warm-ups) `verified` with `matches_target`; `verify --replay-all` 0 problems on
  all 3 sessions (`audits/`). sw did the same `targets_tried` and recovered the same log as ls on 432/432 pairs.

## What it is (and is not)

The earlier study's levers at the incumbent's column count: constant-factor engineering of the giant-step walk, the
recovery check and the orbit keying. Same targets, oracle, table and pricing; removed work is performed work (field
inversions in the walk, doublings that recomputed [2^i]G, squarings that recomputed Frobenius images); every remaining
operation is counted, the sibling second point priced at w. Not a relabelling. Still a Frobenius- and negation-folded
BSGS under the IC harness: T0 only.

## Deviations

- Local L0 run, no EKS/S3: raw sessions gzipped in the scratchpad (`sw8-sessions/n37-r*/records.jsonl.gz`); SHA-256 of
  each uncompressed `records.jsonl` in `summary.json` -> `sessions` (r1 b775c10e…3169, r2 139abac6…df6f,
  r3 e593e46d…6461); compact per-record table `tables/records-n37.csv.gz`; `results.json` gzipped.
- No fresh pilot (rule above, declared in the protocol). Binary reused, not rebuilt (code unchanged since 217638524d).
- `make_report.py` (adapted from the earlier study) and this README were written after data. Not pushed; no validator
  yet; no leaderboard or benchmarks/README row (validation gate).
