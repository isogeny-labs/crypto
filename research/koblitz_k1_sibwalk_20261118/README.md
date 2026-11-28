# Sibling giant-step walk + reuse of formed points on the K1 generator-orbit IC arm (n23, n29, n37), 2026-11-18 `[pcc:unverified]`

Confirmatory study against the lean+sib incumbents (`ls`: lean_layout + sib on sp-c4-d5 / gen-c1 / sp-c8-d5,
`koblitz_lean_sib_confirm_20261118`). Protocol [`PROTOCOL.md`](PROTOCOL.md), frozen at f164bf67fc (hashes in
`REGISTRATION.sha256`) before any confirmatory data; pilot rule `pilot/SELECTION_RULE.md` committed at d321041bde before
pilot data. Code 217638524d (all levers default off). Binary sha256 `7b182360…fd315`. Every number is `[pcc:unverified]`:
local L0 ecbench counts, **no independent validation yet**. Report [`REPORT.pdf`](REPORT.pdf), figures
`figures/fig1-phases-band.svg`, `figures/fig2-ratios.svg`.

## Profile that chose the levers (from the `ls` raw sessions; band = group ops + w x native ops)

A count model (`prof/model.py`) reproduces all 1,296 `ls` records of the incumbent study exactly. Mean per solve at
w = 0 / 1: n23 base 15 / 291, pair table 186 / 1104, relations (giant steps) 390 / 1170, verify 30 / 30; n29 base 0 / 59,
table 15 / 87, relations 31 / 93, verify 19.6 / 19.6; n37 base 35 / 923, table 1188 / 7104, relations 1554 / 4661,
verify 38 / 38. The biggest single item at w = 0 is the giant-step walk (one full addition per target), so:

1. **Sibling giant-step walk** (`walk_sib=m`): the same targets Q + [k]G in the same order, formed in blocks of 2m+1 around
   a centre C (one addition of [2m+1]G per block); C + [i]G (full) and the abscissa of C - [i]G come from ONE inversion,
   charged as one addition plus one `sibling_second_points` (priced at w by the band, the table's convention). The
   abscissa is probed by one counted canonicalisation + lookup; on a hit the point is formed by one more counted addition
   and probed again in full. Set-up: m+1 operations.
2. **Reuse of points already formed** (only where k > 1): `ladder_verify=1` makes the recovery check read the chain
   [2^i]G the base formed for its spread seeds (it re-doubled G before), and `orbit_key_reuse=1` takes each seed's orbit
   key from its already-formed Frobenius images (it squared the seed's abscissa n-1 more times before).

Not used: `tau_verify` (count relabel, VAL-20261118-43658a). Per-curve k and m from the pilot (separate seeds): n23 k=4
m=8, n29 k=1 m=1, n37 **k=9** m=8.

## Result (primary, one test per curve; rule: upper 95% bound < 1 at w = 0, 0.1, 1, pooled and in each replicate)

| sw / ls (paired) | w = 0 | w = 0.1 | w = 1 | verdict |
|:--|:--|:--|:--|:--|
| **n23** pooled (432 pairs) | **0.703 [0.686, 0.721]** | **0.787 [0.774, 0.800]** | **0.966 [0.963, 0.968]** | **passed** |
| n23 r1 / r2 / r3 | 0.714 / 0.698 / 0.697 (hi <= 0.753) | 0.795 / 0.783 / 0.783 (hi <= 0.822) | 0.964 / 0.967 / 0.966 (hi <= 0.971) | |
| n29 pooled | 0.873 [0.856, 0.889] | 0.915 [0.903, 0.927] | 1.013 [1.011, 1.014] | **failed at w = 1** (as predicted) |
| n29 r1 / r2 / r3 | 0.874 / 0.893 / 0.855 | 0.916 / 0.930 / 0.902 | 1.014 / 1.013 / 1.011 | |
| n37 pooled (k 8 -> 9) | 0.785 [0.735, 0.839] | 0.874 [0.824, 0.926] | 1.050 [1.006, 1.095] | **failed at w = 1** |
| n37 r1 / r2 / r3 | 0.808 / 0.730 / 0.830 | 0.893 / 0.823 / 0.916 (r3 hi 1.001) | 1.056 / 1.016 / 1.083 | |

- n23 passes everywhere. Same targets, same recovered log on 432/432 pairs; relations-phase group ops 380.7 -> 213.9,
  verify 30.4 -> 15.4 per solve; walk sibling pairs 180.8, lifts 0.4 per solve. Wall time (descriptive, L0) per solve
  324 -> 283 µs (relations 111 -> 78, verify 4.5 -> 2.4).
- n29 fails at w = 1 exactly as the frozen expectation said (~1.015): the sibling walk is neutral at w = 1 by its pricing,
  its set-up (2G, 3G, first centre) and lifted hits add ~3 operations, and n29 (one column) has no chain or key to reuse.
  It is an improvement at w = 0 and 0.1 only, so not an improvement under the rule.
- n37 fails at w = 1 because of the pilot's k = 9 choice, which the frozen protocol flagged as uncertain (the A1-seed
  model put k = 9 near 1.04 at w = 1): the larger table (oracle set-up 1350 -> 1708 at w = 0) costs more at w = 1 than the
  walk saves. **Derived, not measured** (count model on this study's `ls` records, which it reproduces exactly; it
  reproduces the measured n23 and n29 ratios to 3 decimals): at k = 8, m = 8 the same levers would read 0.736 / 0.817 /
  0.977 at n37 (`derived_counterfactual.json`). That is an open thread, not a result.

The model's count prediction and the measurement agree where the walk is unchanged (n23, n29), which for deterministic
counts on an unchanged walk is expected and is not evidence of peeking.

## Distance to rho (pooled, paired, 95%)

| curve | arm | / rho-v5 (w = 0 / 0.1 / 1) | / rho-strong (w = 0 / 0.1 / 1) | band order vs v5 / vs strong |
|:--|:--|:--|:--|:--|
| n23 | ls (before) | 1.318 [1.179, 1.478] / 1.572 / 2.668 | 0.336 / 0.413 / 0.815 | v5 < ls / ls < strong |
| n23 | **sw (after)** | **0.927 [0.847, 1.020] / 1.237 [1.131, 1.361] / 2.577 [2.355, 2.842]** | 0.236 / 0.325 / 0.788 | **unresolved** / sw < strong |
| n29 | ls | 1.011 [0.930, 1.100] / 1.204 / 2.136 | 0.077 / 0.097 / 0.243 | unresolved / ls < strong |
| n29 | sw (failed) | 0.882 [0.826, 0.944] / 1.101 / 2.163 | 0.067 / 0.089 / 0.246 | unresolved / sw < strong |
| n37 | ls | 1.232 [1.104, 1.378] / 1.513 / 2.764 | 0.682 / 0.815 / 1.329 | v5 < ls / unresolved |
| n37 | sw (failed) | 0.967 [0.896, 1.051] / 1.322 / 2.903 | 0.535 / 0.712 / 1.396 | unresolved / unresolved |

No arm is below rho-v5 across the band: at n23 the w = 0 point is below v5 (0.927) but its interval reaches 1.020, and at
w = 1 sw is 2.6x v5. **Not "below rho".** Against rho-strong n23 sw is below at every weight (as ls already was).
Pooled band S (w = 0 / 0.1 / 1), n23: ls 0.299 / 0.395 / 1.260, sw 0.210 / 0.311 / 1.217, v5 0.227 / 0.251 / 0.472.

## Controls

- ls-off (all new flags explicitly 0) and ls-aa identical to ls on 432/432 records per curve (cost, phase counts and
  native maps without wall time, counters, outcome, `points_sha256`).
- All 5,184 solves `verified` with `matches_target`; `verify --replay-all` 0 problems on all 9 sessions (and the 3 pilot
  sessions). Where k is unchanged (n23, n29) sw did the same `targets_tried` and recovered the same log as ls on 432/432.
- Unit test `k1_sibling_walk_ladder_verify_and_key_reuse_keep_the_solve`: off state identical, same targets / relation /
  log for m in {1,2,3,5}, x-only siblings equal two full additions, and the walk's operation count equals the model.

## What it is (and is not)

Constant-factor engineering of the incumbent's giant-step walk and recovery check; same targets, oracle, table and
pricing. Removed work is performed work (field inversions in the walk, doublings that recomputed [2^i]G, squarings that
recomputed Frobenius images); every remaining operation is counted, the sibling second point priced at w. Not a
relabelling: the w = 1 ratio at n23 (0.966) comes from the ladder check (15 fewer doublings) and the key reuse (88 fewer
squarings), not from the walk. Still a Frobenius- and negation-folded BSGS under the IC harness: T0 only.

## Deviations

- Local L0 run, no EKS/S3: raw sessions gzipped in the scratchpad (`sw-sessions/*/records.jsonl.gz`, `sw-pilot/`), SHA-256
  of each uncompressed `records.jsonl` in `summary.json` -> `sessions`; compact tables in `tables/`.
- The container restarted during the first release build (before any data); the build was restarted from the same
  commit and worktree under the CPU lock. No data affected.
- The pilot's choice of k = 9 at n37 came from 96 noisy pairs where k changes the walk; the rule was applied as frozen.
- `make_report.py` and this README were written after data. Not pushed; no validator yet; no leaderboard or
  benchmarks/README row.
