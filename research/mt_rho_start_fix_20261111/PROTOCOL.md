# Correction: rho-family restart bug and the v2 IC4 result (protocol, frozen before data)

## The defect (found 2026-11-11 while running the v3 holdout)

In `examples/mt_fieldops.rs` the arms `rho`, `rho2`, `rho2x` and `rhoseq` restarted a walk (every 32 steps) at a
sum of bank points: phase 1 at B_i + B_j + B_k from a fixed 64-point bank, phase 2 at B_i + B_j + Q. That allows
only C(66, 3) = 45 760 distinct phase-1 walks (at most about 1.46M distinct points) and 2 080 distinct walks per
target. Consequences:

- at 32 bits a database above about 1.46M points can never fill, so the v3 holdout hung (rho, kappa 0.7, T = 1024);
- smaller panels finished but repeated walks re-stored nothing: in the v2 holdout, phase-1 steps per new database
  point were up to 1.81 (`rho`, 25 b, T = 16384, kappa 1.0) against about 1.0 for a walk with fresh starts, and
  0.667 against about 0.52 for the sibling arms. That inflated the rho reference exactly where v2 reported IC4
  cheaper (T = 16384).

`rhoseq` phase 1 (sequential G, 2G, ...) was unaffected; its phase 2 was affected like the others.

## The fix (this commit)

Chained starts: one running start s <- s + B_k (random k) per restart, s = Q + a G in phase 2. One addition per
restart instead of two, and unboundedly many distinct starts. No other change; `ic4`, `ic5` and `bsgs2` code is
untouched. Check: 32 b, T = 16384, all four arms verify every target (rho2: 0.52 steps per database point).

## Correction rerun

Every rho-family job of the v2 holdout (6 cells x 22 configurations x 4 arms x 3 kappa = 1584 runs, same
instances, seeds and argv) with the fixed binary (`run.py`). The v2 `ic4` and `bsgs2` rows are kept (their code is
unchanged; `merge.py`) and the v2 `analyze.py` is rerun on the merged rows. Every target must verify.

Reported: the v2 table and decisions (IC4 vs rhobest and bsgs2, all c_I, the section 5a rule) before and after;
which v2 decisions change. If IC4 is no longer cheaper than rhobest at T = 16384, claim PCC-20261111-fa13c8 and
the leaderboard section are corrected in the same PR.

Prediction: the rho arms get cheaper by up to about 20% of phase-1 cost at T = 16384; IC4/rhobest at T = 16384
moves from 0.91-0.95 to about 1.0, i.e. unranked.
