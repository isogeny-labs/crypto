# Curve25519-family m = 3 generic decompositions — results

Protocol: [`PROTOCOL.md`](PROTOCOL.md) (frozen before data; Amendment 1 added
after train). Specs: `specs/` (emitted by `mkspec.sh`). Sessions: `sessions/`.
Executed locally at L1 (`--allow-busy`, shared host), counted units only; no
wall-time claim. All 6 sessions: every record verified; `ecbench verify
--replay 8` audit OK (`analysis/verify-*.json`). Public synthetic curves, no
security claim.

## Train (16 targets x 1 round, mean S at w = 0; rho-neg 1.490 / 1.174 / 0.989)

| arm | k=20 | k=24 | k=28 |
|---|---:|---:|---:|
| inc (N_a = 32 / 81 / 203, m=2) | 11.905 | 13.018 | 16.689 |
| mitm3 f=0.25 (N = 8 / 20 / 51) | 68.791 | 101.696 | 158.825 |
| ksum3 f=0.25 | 11.875 | **7.916** | **17.018** |
| mitm3 f=0.35 (N = 11 / 28 / 71) | 49.399 | 73.979 | 111.789 |
| ksum3 f=0.35 | **8.382** | 12.297 | 41.794 |
| mitm3 f=0.5 (N = 16 / 41 / 102) | 33.944 | 49.606 | 82.886 |
| ksum3 f=0.5 | 11.365 | 32.786 | 121.597 |
| mitm3 f=0.7 (N = 22 / 57 / 142) | 27.523 | 39.407 | 60.760 |
| ksum3 f=0.7 | 22.660 | 85.255 | 327.357 |

Selected (lowest m = 3 S at w = 0): k20 `ksum3-n11`, k24 `ksum3-n20`, k28
`ksum3-n51`. The `mitm:m=3` oracle is worse than the incumbent at every N on
every k (it decreases with N but never reaches inc); `ksum:m=3` has a
minimum that moves to smaller N as k grows. At k24 and k28 the selected N is
the smallest grid value, so the true ksum optimum may lie below the grid
(not explored). Tables: `analysis/train-table-k*.md`.

## Holdout (32 fresh targets x 2 rounds; sel = `ksum:m=3`)

S = counted GAE per sqrt(r), mean over 64 runs. Ratio sel/inc is the paired
`ecbench compare` (linearised-t 95% interval over 32 targets).

| k | arm | S w=0 | S w=0.1 | S w=1 |
|---|---|---:|---:|---:|
| 20 | rho-neg | 1.578 | 1.693 | 2.724 |
| 20 | inc | 11.861 | 12.737 | 20.623 |
| 20 | sel (ksum3, N=11) | 8.987 | 9.710 | 16.217 |
| 24 | rho-neg | 1.169 | 1.275 | 2.221 |
| 24 | inc | 12.666 | 13.802 | 24.023 |
| 24 | sel (ksum3, N=20) | 7.554 | 8.173 | 13.745 |
| 28 | rho-neg | 0.965 | 1.055 | 1.864 |
| 28 | inc | 16.057 | 17.564 | 31.130 |
| 28 | sel (ksum3, N=51) | 17.034 | 17.423 | 20.924 |

| k | sel/inc w=0 [95%] | w=0.1 [95%] | w=1 [95%] | band `inc vs sel` | distance inc/rho (w=0, 0.1, 1) | distance sel/rho (w=0, 0.1, 1) | verdict |
|---|---|---|---|---|---|---|---|
| 20 | 0.758 [0.716, 0.802] | 0.762 [0.718, 0.809] | 0.786 [0.728, 0.849] | sel < inc | 7.5 / 7.5 / 7.6 | 5.7 / 5.7 / 6.0 | improvement |
| 24 | 0.596 [0.575, 0.619] | 0.592 [0.569, 0.617] | 0.572 [0.541, 0.605] | sel < inc | 10.8 / 10.8 / 10.8 | 6.5 / 6.4 / 6.2 | improvement |
| 28 | 1.061 [1.033, 1.090] | 0.992 [0.963, 1.022] | 0.672 [0.644, 0.701] | unresolved inside the band | 16.6 / 16.6 / 16.7 | 17.7 / 16.5 / 11.2 | null (regression at w=0, gain only at w=1) |

Rho stays ahead of every arm at every k and weight (`rho-neg < sel` in the
band). Controls: `inc` vs `inc-aa` and `rho-neg` vs `rho-neg-aa` are
"unresolved inside the band"; the counts are deterministic given the target,
so the A/A count ratio is exactly 1 by construction (identical S in the
band tables). The wall-clock A/A (ns/GAE, diagnostic only, L1) is 1.04 / 1.02
/ 1.00 at k = 20 / 24 / 28, and the sel/inc wall ratio is within noise at
k = 20, 24 and 0.70 [0.67, 0.73] at k = 28 (L1, not admitted; no wall claim).
Files: `analysis/compare-holdout-k*.txt` (+ `-w0.1`, `-w1`),
`analysis/holdout-band-k*.{txt,json}`, `analysis/holdout-table-k*.md`,
`sessions/holdout-k*/comparisons/`.

## Classification

- **k = 20, 24: improvement** by the protocol rule (interval below 1 at
  w = 0, band order sel < inc across w): 24% and 40% fewer counted GAE than
  the incumbent. The mechanism is a genuine change of the counted work (a
  base of 11 or 20 points with the m = 3 table `ksum:m=3`, fewer pipeline
  set-up and base-building operations), not a relabelling. Distance to rho
  falls from 7.5x to 5.7x (k=20) and from 10.8x to 6.5x (k=24).
- **k = 28: null.** The selected arm (grid-boundary N = 51) is 6% worse at
  w = 0, equal at w = 0.1 and 33% better at w = 1; the band does not order
  the pair, so no claim.
- Overall: m = 3 `ksum` improves the generic baseline at two of three sizes
  but not at the largest, and the gain does not grow with k; the trend
  across k (0.76, 0.60, 1.06) is not monotone and rests on single grid
  selections.

## Deviations and caveats

- `rootfind-mitm:m=3` is not a method; `ksum:m=3` was the second m = 3 oracle.
- Runs used `--allow-busy`: levels L1, not the L0 the spec asks for. One
  measured train round cannot be balanced (ecbench warning, accepted).
- Selection is in-sample among 8 train arms per k (train 16 targets x 1
  round); holdout targets are fresh. k24 N = 41 (not 40) from rounding.
- No Iso Lab / second-host replay and no validation-gate files
  (`VALIDATION.json`), so none of this is a leaderboard or benchmarks row.
  `ecbench compare` marks the count ratio "diagnostic only (unpriced work;
  ratio does not bound full cost)"; the band check covers the unpriced work
  at w = 0.1 and 1.
- Compute: each session took seconds to under a minute; far under budget.
  No arm timed out or failed.

## Next step

Extend the ksum3 grid below f = 0.25 at k = 24 and 28 (the optimum is at the
boundary) and add k = 32; train on fresh seeds, then a new frozen holdout.
Then price the `ksum` table-build against the m = 2 pair table at larger k,
where N^3 table growth should cap the gain. Replay on a second host before
any leaderboard entry.
