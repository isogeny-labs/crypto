# Protocol: one-point (F = 1) walk-table oracle, single-target ecbench (frozen 2026-11-07)

Frozen before any measured data. Nothing below changes after the first session runs; failed
predictions stay failed and are reported as failed. Unit tests and a single `ecbench plan` (no run)
were the only executions before freezing.

## 1. Question

The lab's generic incumbent `wwalk` (`x=0.9,g=1,grow=2,seed=run` on a 2-abscissa base) has F = 2
unknown base logarithms and sits about 1.8-1.9x rho-neg (measured, `research/curve25519_autolab_20261104/rebaseline-seedrun/README.md`).
The collision floor of `research/curve25519_rho_parity_bound_20261104/README.md` is 1.5x rho at F = 1
and 1.875x at F = 2 (derived). Does a one-seed walk table (F = 1: jumps `[a]P1 + [c]G`, one free row
`du*s1 = -dz` pins `log P1`, one target row finishes) cost less than the F = 2 incumbent, measured
end to end in ecbench, one target per session?

## 2. Implementation under test

`WalkTable::build_with` (src/cryptanalysis/ic_boundary.rs) now accepts a base of exactly one
abscissa when `g=1`. Jumps carry no P2 part; `grow=1|2` walk until the verified free rows have rank 1
over the single seed column (cap 2T / 3T). The 2-seed path is unchanged: its draws, walk and rows
are those of the previous code (existing tests unchanged). The new path has its own unit test,
`wwalk_one_seed_rows_are_affine_and_grow_reaches_rank_one`.

## 3. Design (frozen)

- Binary: `ecbench` built from this working tree into `/home/user/ctarget-f1` (release); its SHA-256
  is recorded in `data/binary.sha256`.
- Curves: k = 20 (`p = 1,048,573`, `r = 131,203`, `#E = 8r`) primary; k = 24 (`p = 16,777,213`,
  `r = 2,097,349`) secondary replicate. Both are the `prime_explicit` Weierstrass models of the
  Curve25519-family miniatures already used in the lab's specs.
- Sessions: 8 per k, each with `targets_per_curve = 1` (one public target per session), its own
  `target_seed` and measurement `seed` (`scripts/make_specs.py`). 64 measured rounds per arm, 1
  warm-up, `order = alternate`.
- Arms (identical in every session):
  - `rho-neg` (reference) and `rho-aa` (A/A control): `rho.negation`.
  - `f2-inc` (baseline, F = 2): `ic.pipeline`, base `prime-projected-abscissa:size=2,scan_cap=32`,
    oracle `wwalk:x=0.9,g=1,grow=2,seed=run`, targets `stride-generator` (the rebaseline-seedrun incumbent).
  - `f1-x05`, `f1-x07`, `f1-x09` (candidates, F = 1): the same with `size=1` and `x = 0.5, 0.7, 0.9`.
    **`f1-x07` is the pre-designated primary F = 1 arm** (the model optimum below); the other two are
    secondary and no "best of three" is promoted as a headline.
- Counted unit: ecbench GAE; `S = cost/sqrt(r)`; every comparison at w = 0, 0.1, 1 (AGENTS.md section 5a).
- Audit: `ecbench verify --replay 1` per session.
- Pooling: `scripts/pooled_compare.py` (copied unchanged from
  `research/curve25519_t10_winner_ecbench_20261104/scripts/`): per session, sum S over rounds per arm;
  ratio of sums; linearised t interval with df = sessions - 1. One session = one target = one cluster.
- Runtime cap: about 90 minutes in total. If k = 24 would exceed it, k = 24 sessions are cut and the
  README says so.

## 4. Model (derived, before data)

In the uniform-walk model with negation folding (collisions as a Poisson process of mean n^2/r),
a table of `max(T, n_F)` entries (n_F = walk length to rank F, T = x*sqrt(r)) followed by probes
that hit with probability 2*entries/r costs, in units of sqrt(r),
`E[max(x, R_F) + 1/(2*max(x, R_F))]`, with `R_F^2 ~ Gamma(F, 1)`. Monte Carlo (100,000 draws):

| F | x = 0.5 | 0.7 | 0.9 | ratio to rho model 0.886 at the best x |
|--:|--:|--:|--:|--:|
| 1 | 1.571 | 1.548 | 1.568 | 1.75 |
| 2 | 1.765 | 1.761 | 1.768 | 1.99 |

The walk-table design is asymmetric (target probes are not inserted), so it sits above the
symmetric floor of 1.5x (F = 1) and 1.875x (F = 2). Model ratio F = 1 / F = 2 at x = 0.7 vs 0.9: 0.876.

## 5. Predictions and thresholds (frozen)

All on the pooled k = 20 sessions unless named; "ratio" = pooled ratio of sums, 95 % interval.

- **P1 (primary).** S(f1-x07) / S(f2-inc) < 1 at all of w = 0, 0.1, 1: the interval's upper end is
  below 1.00 at each weight. Point estimate expected in [0.80, 0.95] at w = 0. Fails if any weight's
  upper end is >= 1.00; the point band is reported held/failed separately (P1b).
- **P2.** S(f1-x07) / S(rho-neg) at w = 0 lies in [1.45, 1.85] (point estimate), i.e. near 1.5x the
  floor times the walk-table asymmetry, scaled by the arms' overhead factors. Fails outside.
- **P3 (no parity).** S(f1-x07) / S(rho-neg) interval lower end > 1.00 at w = 0, 0.1 and 1.
  (Predicting rho stays ahead; a failure here would be the surprising result.)
- **P4 (A/A).** S(rho-aa) / S(rho-neg) interval contains 1.00 at w = 0.
- **P5 (secondary arms).** S(f1-x05) and S(f1-x09) / S(f2-inc) at w = 0 also below 1 (upper end
  < 1.00). The x-ordering among F = 1 arms is predicted unresolvable (model spread about 1.5 %) and is
  not scored.
- **P6 (replicate, k = 24, if run).** P1 at w = 0 holds at k = 24 with the same threshold.
- Every record must be `verified`; every session's `ecbench verify --replay 1` must reproduce. A
  session that fails the audit is excluded and reported; it is not rerun with another seed.

## 6. Reporting

README.md tabulates measured S per arm and weight (with file), the pooled ratios, the held/failed
status of P1-P6, and the distance to matched rho at w = 0, 0.1, 1. Every result sentence ends with
[pcc:unverified]. F = 1 still has one unknown base logarithm; it is index calculus with a one-point
base, not rho relabelled, and the README says how close that is to F = 0.
