# Protocol: C = 1 wwalk arm with x-only siblings on Table B (L4-C1SIB), 2026-11-19

Every statement here is `[pcc:unverified]`. This protocol was frozen before any measured row. A 2-target, 1-round P20 smoke run (`ecbench run` and `verify`, 24 of 24 verified) checked that the arms run; its S values were visible in the log and are not used for any prediction below. No code change: every arm is an existing plugin parameter, run with `target/release/ecbench` (sha256 prefix `b4fcf0ecdb3d0a0f`, not rebuilt).

## Classification
The C = 1 arms have one unknown seed logarithm plus the target (K = 2 unknowns). The owner's 2026-11-04 ruling counts C = 2 arms as index calculus; C = 1 sits at the baby-step giant-step boundary (`research/curve25519_autolab_20261104/c1/README.md`: "boundary result ... does not count toward the main ratchet"). Whether C = 1 counts as index calculus is pending an owner ruling. Nothing here is claimed as an IC improvement.

## Why this test
`research/curve25519_autolab_20261104/ideas/SIBLING-POINTS-20261110.md` section 3 predicts that the C = 1 boundary track with x-only siblings costs about 0.93 of rho-neg in bit-ops (floor 0.89), against about 1.15 for C = 2 (which round 3 measured at 1.07-1.23 on Table B). The C = 1 stage was never run with siblings; the earlier C = 1 study (no siblings, `wsum`/`wwalk` grow=1) measured 1.40-1.74x rho at w = 0.

## Arms
Factor base for C = 1 arms: `prime-projected-abscissa:size=1,scan_cap=32` (one seed); for the incumbent: `size=2,scan_cap=32`. `max_trials` as in round 3. `tw=1` needs `g=1,grow=2` and a one-seed base (plugins.rs, wwalk params).

| arm | role | oracle | targets |
|---|---|---|---|
| `rho-neg` | reference | `rho.negation` | |
| `bsgs-neg` | reference | `bsgs.negation` | |
| `sib8-p256` | C = 2 incumbent | `wwalk:x=2.0,g=1,grow=2,tw=2,cj=1,seed=run,lazy=1,sib=8,pool=256` | stride-generator |
| `c1` | candidate, no siblings | `wwalk:x=2.0,g=1,grow=2,tw=1,cj=1,seed=run,lazy=1` | default |
| `c1-sib8-p256` | candidate | `wwalk:x=2.0,g=1,grow=2,tw=1,cj=1,seed=run,lazy=1,sib=8,pool=256` | default |
| `c1-sib8-p256-sg` | candidate | same oracle | stride-generator |

Specs `specs/P{20,22,24}.json`: the three Table B curves of round 3, 48 public targets per curve, 2 rounds, 1 warm-up, alternate order, new `measurement.seed` 20261119430 / 20261119432 / 20261119434. L0 requested; run `--cpus none --allow-busy` on a 4-core container shared with four other agents, so wall times are contaminated and only counted cost (GAE; `bitop/v1`) is used.

Comparisons (paired, bootstrap 95%): each C = 1 arm against `rho-neg`, `bsgs-neg` and `sib8-p256`, plus `c1-sib8-p256` vs `c1` and `c1-sib8-p256-sg` vs `c1-sib8-p256`, in GAE (w = 0, 0.1, 1) and `--unit bitop/v1`; then `ecbench band`. Audit: `verify --replay-all`.

## Predictions (registered)
- **C-a:** every run verifies.
- **C-b (siblings help at C = 1, w = 0):** `c1-sib8-p256 / c1` at most 0.65 at each size (round 3 saw 0.44-0.48 against a C = 2 walk without siblings).
- **C-c (against rho-neg, bitop/v1):** the best sibling C = 1 arm / rho-neg is in [0.80, 1.00] at P22 and P24 (memo: about 0.93), and at P20 in [0.85, 1.10].
- **C-d (against rho-neg, w = 0):** best C = 1 sibling arm / rho-neg at most 1.00 at P22 and P24 (point estimate), with the upper 95% end possibly above 1.
- **C-e (against the C = 2 incumbent):** best C = 1 sibling arm / `sib8-p256` is in [0.75, 0.95] in bitop/v1 and at w = 0 (memo: 0.93 / 1.15 = 0.81).
- **C-f (w = 1):** every C = 1 arm / rho-neg at w = 1 is above 1.3, because the second points are priced as additions.
- **C-g:** `c1-sib8-p256-sg / c1-sib8-p256` is in [0.9, 1.1]; stride-generator targets are not expected to matter at C = 1.
- **C-h (bsgs-neg):** bsgs-neg / rho-neg is in [0.9, 1.1] at w = 0; the best C = 1 sibling arm is not expected to fall below bsgs-neg by more than the memo's stored-rho floor of 0.62.

## Decision
- **Below rho (C = 1 boundary result):** a C = 1 arm has the upper 95% end of its ratio to `rho-neg` below 1 in `bitop/v1` or at w = 0. Reported as a boundary result, not as an improvement of the index-calculus baseline (classification pending an owner ruling).
- **Improvement over the C = 2 incumbent:** a C = 1 arm / `sib8-p256` with the upper end below 1 at w = 0, 0.1, 1 and bitop/v1 at all sizes; otherwise partial, with the readings named.
- **Kill:** any unverified run.
- Distance to `rho-neg` and to `bsgs-neg` is reported as distance remaining. No leaderboard row without the validation gate.
