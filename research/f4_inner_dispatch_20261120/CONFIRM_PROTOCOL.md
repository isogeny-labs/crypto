# Confirmation: F4 allocation changes on jv_cover (frozen before data, 2026-11-20 22:40 UTC)

**Question.** Do the two allocation changes make the real index-calculus consumer of the F_p F4
engine faster, end to end, with identical work?

**Arms** (all `jv_cover --exp dlp --sizes 53 --seeds 1 --rho-runs 0`, RAYON_NUM_THREADS=4, no F4
toggles set):

- `B0`: baseline binary built from main `3b8db9765e` (`data/baseline-binaries.sha256`).
- `B1`: this branch, default features: the final interreduction moves the basis instead of copying it.
- `B2`: this branch with `--features jemalloc` (jemalloc as `jv_cover`'s global allocator).
- `B1a`: a second copy of `B1` in each round, the A/A control.

**Schedule.** 8 rounds; each round runs the four arms once, in an order rotated by the round. Wall time
from the report's `wall_ms`; CPU time (user + system) from the shell's `time`.

**Identity check.** Every run must report the same residuals, relations, counted oracle operations and a
correct logarithm; any difference fails the run.

**Statistic.** Per round, the paired ratios `B0/B2`, `B0/B1` and `B1/B1a` of wall time and of CPU time.
An arm counts as faster than `B0` when at least 7 of 8 rounds favour it (one-sided sign test,
p = 0.035) and its median ratio exceeds the largest `B1/B1a` deviation from 1.

**Predictions.**
- Primary: `B0/B2` wall, median in [1.10, 1.30], at least 7 of 8 rounds above 1.
- `B0/B2` CPU, median in [1.08, 1.25].
- `B0/B1` wall, median in [0.98, 1.06]: the copy removed is about 4 % of F4's instructions, so the effect
  may be within noise.
- `B1/B1a` within ±5 % in every round.

**Conditions.** Run with no build or other benchmark active; one shared 4-vCPU cloud VM, isolation L0,
no PMU. All results `[pcc:unverified]`.
