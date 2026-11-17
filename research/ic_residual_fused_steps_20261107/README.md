# Why the fused residual walk takes more steps (2026-11-07)

Status: local L0 sandbox measurements, counted units only. Not validated under
the validation gate; no leaderboard or `benchmarks/README.md` row. Every claim
below is an unverified assertion [pcc:unverified] until a pcc claim is filed.
PDF: [`REPORT.pdf`](REPORT.pdf). Figure: [`figures/ratios.svg`](figures/ratios.svg).

## The question

[`../ic_residual_fused_20261031`](../ic_residual_fused_20261031/README.md)
fused the residual walk's two additions per step into one
(`targets=residual-fused`). Additions per step fell from 2.005 to 1.004, but
the fused walk needed 1.1–1.3× more steps per relation. That study left the
cause open, with hash-bit correlation between the jump index `j` and the
base-point index `i` as its hypothesis.

## Answer: coincidence hazard, not the hash

A walk finds a relation when it reaches a point the table already holds. What
matters is how many chances of a coincidence each step creates.

- **Baseline step** `L → X = L + J_j(L) → L' = X − P_i(X)` visits two points.
  - Two walks that meet at the same intermediate `X` take the same next step,
    so they reach the same stored `L'`. The collision is caught one step later,
    although `X` is never stored.
  - Stored points are compared as sign classes `±L`, so a stored point offers
    two coincidences. Intermediates offer one, because the map from `X` is not
    negation-equivariant.
- **Fused step** `L → L + (J_j − P_i)(L)` visits one point, compared as a sign
  class.

An additive walk with r possible steps cannot reach a same-sign coincidence
through the step that produced the old point, so a same-sign coincidence
counts (r − 1)/r. A coincidence with the opposite sign is a plain landing and
keeps its full weight. Per step, with f signed base points, J = 16 jumps, and
s = 1 when the table uses sign classes (0 otherwise):

```text
hazard(base)  = (J − 1)/J + (f − 1)/f + s
hazard(fused) = (J·f − 1)/(J·f) + s
steps(fused) / steps(base) = sqrt(hazard(base) / hazard(fused))
```

For J = 16, f = 4, 8, 16 this predicts 1.164, 1.188 and 1.200 times the steps.
Additions per collision fall to that ratio over two: 0.58–0.60 of the
baseline's. The earlier study measured 0.56–0.66.

### What this means for the residual collector

- **The fused walk is not defective.** It spends one addition per point and
  compares every point as a sign class. That is rho's own rate of coincidences
  per addition, the generic optimum for a walk that stores every point. The
  extra steps buy nothing to fix in the step itself.
- **The band reversal is per-step native work.** The baseline makes about 1.2×
  fewer table probes and stores per collision. Once those are priced (w > 0),
  that outweighs the fused walk's saved additions. The remedy is to store
  fewer points without losing the sign classes: a negation-equivariant step
  with distinguished points. That is `residual-fused-eq` on main
  ([`../ic_residual_dp_eq_20261101`](../ic_residual_dp_eq_20261101/)).
  Plain distinguished points (`residual-fused-dp`) give up the sign classes,
  because walks at L and −L do not merge. The model puts that cost at √2 in
  steps; this study did not measure it.
- **Derived floor, not measured here.** Suppose a base of k unknown-log
  columns needs k + 1 independent collisions against one persistent table. The
  expected time to the c-th collision is
  E[T_c] / E[T_1] = (2c − 1)!! / (2^(c−1) (c − 1)!), which is 1.5 for c = 2.
  So `res1` (2 relations) cannot get below about 1.5× matched rho at w = 0,
  before set-up and restarts. Main reports `res1eq` at 1.3–1.7×. A base of
  known logarithms needs one collision, and then the collector is rho.

## Evidence

Three runs, each scored against predictions committed before its data:

| run | data | predictions | rule | outcome |
|:--|:--|:--|:--|:--|
| exploratory simulation, N ≈ 2^24, f = 4, 16 | `data/sim-*.json`, `data/ratios.json` | first, coarse model (√1.5, √2) | inside 95% interval | coarse model fails 6 of 10; the hazard model above was fitted after seeing this run |
| confirmatory simulation, N ≈ 2^25, (f, J) = (2, 16), (8, 16), (4, 32), new seeds | `data/confirm-*.json` | `PREDICTIONS.json` (commit 4cb30f903e) | every prediction inside its 95% interval | **not met: 13 of 15** |
| real `ecbench`, main cd9256bea0, p36, B = 2, 4, 8, 64 targets × 4 rounds | `real/scored.json`, session ECBS1h999448230f15 | `real/PREDICTIONS.json` (commit dc1665062a) | inside 95% interval | **2 of 3** |

The four misses all sit just outside their intervals:

| run | ratio | measured [95% CI] | predicted | miss |
|:--|:--|:--|--:|--:|
| confirmatory | f = 2, independent hash / fused | 0.9884 [0.9772, 0.9999] | 1.0000 | 0.0001 |
| confirmatory | f = 8, fused / base | 1.2020 [1.1889, 1.2155] | 1.1882 | 0.0007 |
| real code | B = 2, fused / base | 1.2341 [1.1644, 1.3036] | 1.1638 | 0.0006 |

The largest error over the 15 confirmatory ratios is about 1.2%. With 15
intervals at 95%, one miss is expected even under an exact model, and two or
more occur about 17% of the time. As written, the frozen rules are not met;
the model is a close approximation, not confirmed exact.

Two results bear on the question without that caveat:

- **Hash correlation.** Drawing `i` from an independent hash changes the step
  count by 0.988–1.001 (`fusedind/fused`), against a 12–23% effect.
- **Sign classes.** A table without them (`*_ns`) costs the predicted
  factor for the fused walk: 1.413–1.422 measured against 1.417–1.426 predicted.

In the real-code run the arms needed identical numbers of table hits (576,
960 and 1,728 each), so the step ratio is entirely steps per collision.

## Model and limits

- The simulation (`scripts/sim.py`) models the group as Z_N with negation, the
  hash as SplitMix64 and the steps as uniform random elements. It reproduces
  the walk logic of `ic_framework/residual.rs` (modes 0 and 1), including the
  `^ 0xA5A5` index hash, the restart after every hit and the persistent table.
  It has no curve arithmetic and no relations: it measures steps to a table
  hit.
- Real-code run: local 4-core sandbox, L0, counted units, which are
  deterministic. Wall time was not used. 1,536/1,536 records passed
  ecbench's own check [pcc:unverified].

## Reproduce

```text
python3 scripts/sim.py 16777259 4 16 1000 > data/sim-N2e24-F4.json        # exploratory
python3 scripts/sim.py 33554467 2 16 1000 16 50000 > data/confirm-F2-J16.json  # confirmatory (and F8-J16, F4-J32)
python3 scripts/analyse.py data/sim-N2e24-F4.json data/sim-N2e24-F16.json > data/ratios.json
python3 scripts/confirm.py > data/confirm-scored.json
ecbench run --spec real/spec-p36.json --out <dir> --cpus none --allow-busy
python3 real/score.py <dir>/records.jsonl real/PREDICTIONS.json > real/scored.json
python3 scripts/figure.py
```
