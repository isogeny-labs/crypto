# Protocol: three summands (m = 3) with the cycle stop on a = 0, n = 37, frozen before data

Frozen 2026-11-01, before the first run of the sweep in this protocol.

## Question
The m = 2 cycle stop (first closed equation in `t`) solves a = 0, n = 37 (cofactor 596) in about
2 to 100 M decomposer calls depending on d (`ic_m2_torsion_ratio_n37_20261101`, torsion off). What
does the same stop cost when relations have three summands? Claim class: comparative (paired by
seed), exploratory, not for the leaderboard.

## Method (new producer `examples/ic_m3_cycle.rs`)
- Same decomposer, factor base, anchors `[a]G + [b]T`, Frobenius conjugates and strong rho as
  `ic_m2_cycle.rs`, torsion off. Relations come from a "2 + 1" split: for an anchor point p and each
  base point P (both signs), decompose `R' = p - eP` over the base with the existing linear
  two-summand solve. Each success gives `p = eP + e2 P2 + e3 P3` (merged by column).
- With up to three terms per relation the relation graph is a hypergraph, so relations are reduced
  incrementally into a row echelon form modulo the prime r. The first relation that reduces to the
  zero row leaves `0 = alpha + beta t`; with `beta != 0` that gives `t`, checked as `[t]G = T`. This is
  the m = 2 stop (a closed equation in `t`) generalized from graph cycles to linear dependencies.
- "Trials" counts decomposer calls, the same unit as the m = 2 records. Group additions are counted
  separately (the m = 3 producer adds one per trial: `p - eP`).

## Prediction (stated before data)
Per trial the relation yield is that of the two-summand solve (`R'` is one m = 2 trial), so m = 3
cannot win in decomposer calls unless it needs fewer relations. It needs more: a linear dependency
among 3-term relations needs about 0.9 K relations (rank near the column count K), against 126 (d = 12),
27 (d = 10) and 7 (d = 8) for the m = 2 cycle stop. I therefore expect m = 3 to cost more than m = 2 in
trials at every d, by at least 10x at d = 12. A throwaway pilot on seed 777 (excluded from the analysis;
`raw/pilot/`) gave 92.4 M trials at d = 12 and 237 M at d = 10, both verified; the small-n correctness
check (`raw/smalln/`, a = 0 n = 13, 19, 23 and a = 1 n = 19, 8 targets each) verified 32 of 32.

## Design
- d in {12, 10, 8} (K = 2,011, 500, 121). Six seeds per d: 20261024 and 20261101 to 20261105, one
  target (ti = 0) each, one process per target, 4 processes at a time. The secret depends only on the
  seed, so each m = 3 run pairs with the m = 2 torsion-off record of the same seed and d, taken from
  the merged records (`research/ic_m2_torsion_n37_20261101/pairs.txt` for d = 12,
  `research/ic_m2_torsion_ratio_n37_20261101/runs.txt` for d = 10 and 8). Those are not rerun.
- Cap: 1,800 s per run (`timeout`); a capped run is a censored lower bound, reported as such. No run
  is restarted or given a longer cap after seeing results (an environment interruption that kills a
  run before it finishes is rerun identically and disclosed).
- Instrument: the binary hash is recorded. After the sweep, one run (d = 12, seed 20261024) is rerun and
  must reproduce its trial count exactly.

## Metrics and analysis (frozen)
- Primary: decomposer calls to recover `t` (`ic_trials`); secondary: group additions
  (`ic_group_additions`), wall seconds, relations used, rank at the stop, anchors used.
- Per d: finished / 6; per-seed ratio m = 3 / m = 2 of trials, group additions and wall; geometric
  mean with a 95% bootstrap interval over pairs (10,000 resamples, explicit Park-Miller generator
  seeded 20261101, as in `ic_m2_torsion_ratio_n37_20261101/analyze.sh`); mean relations and mean
  rank / K at the stop.
- A trend statement across d only if the per-d geometric means are ordered the same way at all three d.
- Verified recoveries only (`ic_verified`); a failing row is excluded and reported.

## Stated limits
- Six pairs per d, heavy-tailed trials. The m = 2 reference is the earlier merged runs (same
  decomposer source and seeds, a different binary build and run time), so wall times carry host noise;
  trials are counted and comparable.
- One 2 + 1 construction (enumerating the first summand over the whole base). Other three-summand
  constructions (for example a direct S4 solve) are not tested. Local shared 4-vCPU container, not the
  default isolab-eks route (no AWS identity in this session). Exploratory; no replay certificate or
  validator report.
