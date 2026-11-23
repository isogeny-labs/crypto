# Protocol: stride-generator targets on the binary wwalk incumbent (L5-BINSG), 2026-11-20

This protocol was frozen before any measured row. Every statement here is `[pcc:unverified]`. There is no code change: the run uses `ecbench` built from `ce71cfa8f1`, the binary used for the L4-BINWALK measurement. A 2-target B21 smoke run checked only that the arms run.

## Why
On prime Table B, `targets=stride-generator` alone cut tw2l to 0.68–0.85× at every weight (`research/ic_tableb_sib_20261119/ablation`). The binary incumbent `ww-tw` (`research/ic_tableb_binary_wwalk_20261119`) does not use it. It is 2.2–2.3× rho at n = 21–27.

## Arms and workloads
Specs are `specs/B15,B18,B21,B24,B27.json`. They use the same curves and targets as L4-BINWALK, with a new measurement seed, 2 rounds, 1 warm-up and alternate order, at L0.

| arm | role | definition |
|---|---|---|
| `rho-neg` | reference | — |
| `bsgs-neg` | reference | — |
| `ww-tw` | baseline (incumbent) | `wwalk:x=2.0,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run` on `binary-multiplier:size=2,mult=1` |
| `ww-tw-sg` | candidate | `ww-tw` plus `targets=stride-generator` |
| `ww-sg` | candidate | `wwalk:x=2.0,g=1,grow=2,seed=run` plus `targets=stride-generator` |

All comparisons are in counted GAE at w = 0, 0.1 and 1.

## Predictions (registered)
- **G-a:** every run verifies.
- **G-b:** `ww-tw-sg / ww-tw` is in [0.55, 0.90] at w = 0 at every size, with every upper 95% end below 1.
- **G-c:** the band orders `ww-tw-sg` below `ww-tw` at every weight at n = 21, 24 and 27.
- **G-d:** `ww-tw-sg / rho-neg` is in [1.2, 2.0] at w = 0 at n = 21–27.

## Decision
- `ww-tw-sg` replaces the binary incumbent at a size if the band orders it below `ww-tw` at w = 0, 0.1 and 1.
- **Kill:** any unverified run.
- **Distance to rho:** reported as the distance remaining.
- **Leaderboard:** no row without the validation gate.
