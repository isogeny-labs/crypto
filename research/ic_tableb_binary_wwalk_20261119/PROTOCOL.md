# Protocol: the C = 2 wwalk walk on Table B's random-binary curves (L4-BINWALK measurement), 2026-11-19

This protocol was frozen before any measured row. Every statement here is `[pcc:unverified]`. The code change is commit `3a90dba6ae`, the wwalk port to binary curves (`research/ic_tableb_l4_binwalk_20261119/README.md`). The run uses `ecbench` built from the commit that freezes this file. A smoke run on B15 checks only that the arms run.

## Why
On the random-binary rows of Table B, the index-calculus incumbent is the k-sum table: `ksum4` at n = 15 and 18, `ksum5` at n = 21, 24 and 27. Rho is 5.3–7.5× ahead (`research/ic_tableb_binary_ksum_20261115`). On the prime rows, the same C = 2 walk cut the k-sum arm to 0.41–0.48× (round 2). Before this port, wwalk refused binary curves.

## Arms and workloads
Specs: `specs/B15,B18,B21,B24,B27.json`. They use the same curves, targets (48 public targets per curve) and target seeds as the binary k-sum study, with 2 rounds, 1 warm-up and alternate order, at L0.

| arm | role | definition |
|---|---|---|
| `rho-neg` | reference | — |
| `bsgs-neg` | reference | — |
| incumbent | baseline | that study's arm and factor base: `ksum4` at n ≤ 18, `ksum5` above |
| `ww` | candidate | `wwalk:x=2.0,g=1,grow=2,seed=run` on `binary-multiplier:size=2,mult=1` |
| `ww-tw` | candidate | `wwalk:x=2.0,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run` on the same base |

Siblings and `aut` are refused on binary curves and are not tested. Comparisons are in counted GAE at w = 0, 0.1 and 1. `BinaryGroup` does not count field operations, so there is no `bitop/v1` reading.

## Predictions (registered)
- **W-a:** every run verifies.
- **W-b:** the better wwalk arm / incumbent is at most 0.6 at w = 0 at n = 21, 24 and 27. Upper 95% ends are below 1 at every size.
- **W-c:** the better wwalk arm / rho-neg is in [1.6, 2.6] at w = 0 at n = 21–27, so the distance to rho drops from 5.3–7.5× to about 2×.
- **W-d:** the band orders the better wwalk arm below the incumbent at every weight at n = 21–27.

## Decision
- **Improvement:** a wwalk arm is an improvement over the binary incumbent at a size if `ecbench band` orders it below the incumbent at w = 0, 0.1 and 1.
- **Kill:** any unverified run.
- **Rho distance:** reported as the distance remaining.
- **Leaderboard:** no row without the validation gate.

## Addendum A (before any measured data)
The B15 smoke run on the frozen build failed every `ksum4` run with "oracle `ksum` does not run on a binary curve". Main had lost the binary `"ksum"` dispatch line in `solve_ic_binary`, which was present in `2476332d22`, the binary k-sum study's build. The line is restored in the commit that adds this addendum, and the run uses `ecbench` built from that commit. After the fix, the 2-target B15 smoke run verified 30 of 30. Nothing else changes.
