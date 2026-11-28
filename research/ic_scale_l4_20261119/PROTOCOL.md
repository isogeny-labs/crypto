# Protocol: the Table B incumbent at 28 and 32 bits (L4-SCALE), 2026-11-19

Frozen before any measured row. Two 2-target smoke runs (`specs/smoke28.json`, `specs/smoke32.json`, 1 round, 32 executions each) checked only that the arms run and took 3.3 s and 1.9 s of wall time. Every statement here is `[pcc:unverified]`. There is no code change; the binary is `target/release/ecbench` (sha256 prefix `b4fcf0ecdb3d0a0f`), no `cargo build`.

## Question
On Table B (20/22/24 bits) the incumbent `sib8-p256` / `rho-neg` at w = 0 fell with size: 1.11, 0.96, 0.93 in round 3 and 1.075, 0.972, 0.891 in the ablation re-run (`research/ic_tableb_sib_20261119/summary.json`, `.../ablation/summary.json`). Stage 3 of the Curve25519 autolab saw 0.78–0.91 at 34–40 bits. Does the ratio keep falling at 28 and 32 bits, or does it flatten near 0.8–0.9?

## Curves
The `snc-panel` k28/k32 curves in `research/curve25519_autolab_20261104` have cofactor 8 (group order 268462472 = 8 * 33557809), so they are not prime order and are not used. The engine's own generator `prime_orbit_shape` (shape `generic`, as in stage 2 and 3) makes prime-order prime-field curves; the curves are frozen by `(bits, seed)` and `ecbench plan` expands them:
- 28 bits: seeds 28001, 28002 (r = 2^27.89 and 2^27.74).
- 32 bits: seeds 32001, 32002 (r read from `ecbench plan` and written in the README).
The ecdb server is not used (no token here); the curves are not in `docs/curves/registry.json` (plan notes "registered=false"), so they are not cited in prose by registry slug beyond the engine's `icv1-...` name.

## Arms (as in `research/ic_tableb_sib_20261119`)
| arm | oracle |
|---|---|
| `rho-neg` | reference, `rho.negation` |
| `tw2l` | `wwalk:x=2.0,jumps=16,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run` |
| `sib8-p256` (incumbent) | `wwalk:x=2.0,g=1,grow=2,tw=2,cj=1,seed=run,lazy=1,sib=8,pool=256`, `targets=stride-generator` |
| `sib8-p256-aa` | A/A copy of `sib8-p256` |
All IC arms: factor base `prime-projected-abscissa:size=2,scan_cap=32`, `max_trials=13528399621`.

## Workload and run
Because a smoke execution costs about 0.05–0.1 s, the whole panel is cheap; so instead of 24 targets and 2 rounds I use 96 public targets per curve (2 curves per size), 4 rounds, 1 warm-up, `alternate`. L0, `--cpus none --allow-busy` on a shared 4-core container with four other agents running: counted cost (GAE, bitop/v1) is exact, wall times are contaminated and are not used for any claim. Seeds: target seed 2026111928 / 2026111932, measurement seed that + 7. Audit: `ecbench verify --replay-all`. Analysis: `compare` (GAE at w = 0, 0.1, 1, and `--unit bitop/v1`) for rho-neg vs {tw2l, sib8-p256}, tw2l vs sib8-p256, sib8-p256 vs sib8-p256-aa; `band`.

## Trend
A derived statistic: weighted least squares of ln(sib8-p256 / rho-neg, GAE w = 0) on log2 r, over Table B (round 3 and ablation re-run, 20/22/24 bits, ratio with its interval converted to a standard error on the log scale) plus the per-curve points at 28 and 32 bits. `ecbench trend` is also run on the saved 28/32-bit comparisons if it accepts them. Table B is fitted both with and without the 28/32 points.

## Predictions (registered)
- **S-a:** all runs verify; the A/A ratio is exactly 1 in every unit.
- **S-b (w = 0):** `sib8-p256` / `rho-neg` is 0.85 at 28 bits (plausible range 0.75–0.95) and 0.82 at 32 bits (0.72–0.92), and the upper 95% end is below 1 at both sizes.
- **S-c (tw2l):** `tw2l` / `rho-neg` is in [1.7, 2.3] at w = 0 at both sizes (round 2: 2.1–2.2).
- **S-d (sib / tw2l, w = 0):** 0.38–0.50 at both sizes.
- **S-e (w = 1):** `sib8-p256` / `rho-neg` is in [1.4, 1.8], above 1, at both sizes. `band` leaves the order against `rho-neg` unresolved or against `sib8-p256`.
- **S-f (bitop/v1):** `sib8-p256` / `rho-neg` is in [0.8, 1.2].
- **S-g (trend):** the fitted slope of ln ratio on log2 r over 20–32 bits is negative, in [-0.03, -0.005] per bit, and flatter than the 20–24 bit slope (about -0.045 per bit), i.e. the curve flattens above 24 bits and does not extrapolate to below 0.7 at 32 bits.

## Decision
- **Improvement over rho-neg at that size:** the upper 95% end of `sib8-p256` / `rho-neg` at w = 0 is below 1 and every run verifies.
- **Falling trend:** the slope interval lies wholly below 0. It is a derived statistic over two different panels, not a preregistered scaling claim.
- **Kill:** any unverified run, an A/A ratio other than 1, or a ratio of 1 or more at w = 0 with an interval above 1.
- The distance to `rho-neg` is reported as the distance remaining (ratio at w = 0, 0.1, 1). No leaderboard row without the validation gate.
