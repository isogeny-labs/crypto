# Protocol: sibling-pool wwalk arms on Table B (L3-SIB), 2026-11-19

This protocol was frozen before any measured row; a 2-target P20 smoke run checked only that the arms run. Every statement here is `[pcc:unverified]`. There is no code change; every arm is an existing plugin parameter, run with `ecbench` built at the round-2 freeze commit `b0c61aee53` (`target/release/ecbench`, unchanged since).

## Why this test
Round 2 (`research/ic_tableb_fewcol_20261118`) made `tw2l` the Table B incumbent at 2.1–2.2× matched rho (w = 0). The Curve25519 autolab measured x-only sibling pools on the same C = 2 walk at 28–40 bits (`research/curve25519_autolab_20261104/siblings/stage1-eks`, `stage2-primefield`, `stage3-scaling`). There `sib8-p256` and `sib8-p512` were 0.49–0.65× of `tw2cj-lazy` at w = 0, near 1.0 at w = 1, and 0.78–0.91× of rho-neg at w = 0. They have never run on Table B.

Table B is smaller (20–24 bits), so the pool set-up costs proportionally more: a·popcount + M/a + M additions for M = 256 or 512, against about √r ≈ 1,000–3,300.

## Arms and workloads
Specs: `specs/P20.json`, `specs/P22.json` and `specs/P24.json`. They use the same three Table B curves and 48 public targets per curve as rounds 1–2, with a new measurement seed. The arms are:

| arm | role | oracle | factor base |
|---|---|---|---|
| `rho-neg` | reference | — | — |
| `tw2l` | baseline (round-2 incumbent) | `wwalk:x=2.0,jumps=16,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run` | `prime-projected-abscissa:size=2,scan_cap=32` |
| `sib8-p256` | candidate | `wwalk:x=2.0,g=1,grow=2,tw=2,cj=1,seed=run,lazy=1,sib=8,pool=256`, `targets=stride-generator`, as in the autolab | same |
| `sib8-p512` | candidate | same as `sib8-p256` with `pool=512` | same |
| `tw2l-aa` | determinism control | as `tw2l` | same |

The run uses 2 rounds, 1 warm-up and alternate order. It is L0: one local 4-core container. Comparisons are made in counted GAE at w = 0, 0.1 and 1 (`ecbench compare`, `band`), and in `bitop/v1` as a secondary unit (`ecbench compare --unit bitop/v1`).

## Predictions (registered)
- **B-a:** every run verifies, and `tw2l-aa / tw2l` is exactly 1.
- **B-b (w = 0):** the better sibling arm / `tw2l` is at most 0.80 at P22 and P24, with the upper 95% end below 1 at every size. `sib8-p256` beats `sib8-p512` at P20, because the pool set-up is a larger share there.
- **B-c (w = 1):** the sibling / `tw2l` ratio is in [0.9, 1.15]. The autolab saw 0.98–1.09; the second points are priced as additions at w = 1, so I expect no band order.
- **B-d (bitop/v1):** the better sibling arm / `tw2l` is at most 0.80, and the better sibling arm / `rho-neg` is in [0.9, 1.4].

## Decision
- **Improvement over `tw2l` across the band:** B-a holds and `ecbench band` orders the sibling arm below `tw2l` at w = 0, 0.1 and 1 at all three sizes.
- **Partial result:** the order holds at w = 0 and 0.1 but not at w = 1. This is reported as an improvement in the w = 0 and field-work (`bitop/v1`) readings only, not across the band.
- **Kill:** any unverified run, or a ratio of 1 or more at w = 0.
- Distance to `rho-neg` is reported as the distance remaining. No leaderboard row without the validation gate.
