# Protocol: a mixed-source structured hybrid at m = 3 on a = 0, n = 37, frozen before the sweep

Frozen 2026-11-03, before the first sweep run of this protocol.

## Question
Two relation sources exist on a = 0, n = 37 (cofactor 596) with the same stop (first closed equation in `t`):
two-summand relations (source A; the `ic_m2_cycle` trial) and "2 + 1" three-summand relations (source B;
`ic_m3_cycle`). Per decomposer call both yield relations at the same rate (about 17,000 calls per relation
at d = 12). A needs about 142 relations to close an equation; B needs about 5,550 (`research/ic_m3_cycle_n37_20261101`).
Does **mixing** the two sources in one echelon form close an equation in fewer decomposer calls than
source A alone? Claim class: comparative (paired by seed), exploratory, not for the leaderboard.

## Design
- Producer `examples/ic_mix_cycle.rs`, d = 12 (K = 2,011). For each Frobenius conjugate p of an anchor:
  source A decomposes p; source B decomposes `p - eP` for each base point P and sign e (4,022 pairs), each
  pair included with probability q from a deterministic sampler. q = 0 is the pure two-summand cycle stop;
  q = 1 is the full 2 + 1 sweep plus source A.
- q grid (the fraction f of decomposer calls spent in source A is `1 / (1 + 4022 q)`): q = 0 (f = 1),
  8.3e-5 (0.75), 2.5e-4 (0.5), 7.5e-4 (0.25), 2.2e-3 (0.1), 8.0e-3 (0.03), 2.5e-2 (0.01), 1 (about 2.5e-4).
- Six seeds: 20261024 and 20261101 to 20261105, one target per process, 4 processes at a time, 1,800 s cap
  per run (a capped run is a censored lower bound; none is restarted after seeing results).
  The secret depends only on the seed, so every q solves the same discrete log for a given seed.
- Instrument checks: (i) q = 0 must reproduce the merged m = 2 torsion-off trial counts of the same seeds
  exactly (`research/ic_m3_cycle_n37_20261101/runs_m2_ref.txt`, d = 12); any mismatch is reported and the
  q > 0 rows are interpreted only against this producer's own q = 0 rows. (ii) a small-n correctness check
  (a = 0 at n = 13, 19, 23) must verify every target at q = 0.01 and q = 1 before the sweep starts.

## Prediction (stated before data)
Decomposer calls to the stop rise monotonically with q, and no q beats q = 0. Reason: 3-term rows add rank
and new columns without making early cycles, so each call spent on source B is worth less than a call on
source A for closing an equation. I expect the q = 1 row near the pure m = 3 value (about 95 M calls, 39x the
q = 0 value of about 2.4 M). The result would be a win only if some q has a paired ratio to q = 0 below 1 with
a 95% bootstrap interval excluding 1.

## Metrics and analysis (frozen)
- Primary: decomposer calls to recover `t` (`ic_trials`). Secondary: group additions, wall, relations from
  each source (`ic_rels_m2src`, `ic_rels_m3src`), which source closed the equation (`ic_closed_by`), rank.
- Per q: finished / 6, mean and range of trials; per seed ratio to the same seed at q = 0; paired geometric
  mean with a 95% bootstrap over seeds (10,000 resamples, explicit Park-Miller generator seeded 20261101).
- Monotone trend: the geometric means across q are ordered the same way at every step, or not; both are
  reported with the intervals. Verified recoveries only (`ic_verified`).

## Stated limits
- One curve and one d; six pairs per q. Source B is only the 2 + 1 construction; the mixing weights are
  set by q, not optimized per anchor. Local shared 4-vCPU container, not the default isolab-eks route (no
  AWS identity in this session). Exploratory; no replay certificate or validator report.
