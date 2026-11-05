# Priced DP residual collectors (distinguished points, equivariant walk): preregistered protocol (before data)

Date: 2026-11-01. Branch `claude/ic-gap-20261031`. Supersedes the w > 0 numbers of `research/ic_residual_dp_20261101`,
whose `residual-fused-dp` arm left its per-step DP test out of the priced counters (`dp_tests` is not in the band's
priced list), so its w = 0.1 and w = 1 figures understated priced work. That study's records stay unchanged as a record;
the code now names the per-step counters `*_uncharged`, so the band prices them (1 GAE per op at w = 1) like rho's canonicalisation.

## Candidates (src/cryptanalysis/ic_framework/residual.rs; baseline `targets=residual` unchanged)
- `residual-fused-dp` (mode 3): one addition per step (J_j - P_i table), distinguished points b = floor(log2 r / 2) - 9 (= 8 on p36, p37),
  restart after 40*2^b steps without a DP. Priced per step: one DP test (`dp_tests_uncharged`).
- `residual-fused-eq` (mode 4): negation-equivariant walk (step index from the x-part of the key, applied with the orientation sign, so the
  paths of L and -L merge) on 64 jumps -> 2016 pair sums J_u + J_v, each minus each base point (all setup additions counted); 2-cycle escape
  (next step, one extra counted addition); DP as above, restart after 16*2^b steps. Priced per step: DP test, orientation read, plus one negation per flipped step.
Exploratory only (not claims): at B = 1 on p36, 24 targets, accounting before the fix: baseline 2.63, dp 1.99, eq 1.44 at w = 0.

## Claims
- C1 (engineering, confirmatory): on p36 and p37, for each B in {1, 2, 4} and each candidate X in {dp, eq}, the paired 95% interval of
  S(res{B}X)/S(res{B}) lies below 1 at w = 0, 0.1 and 1 (12 cells per curve pair, 24 cells in all; all must hold for "met").
- C2 (descriptive): distance to matched rho (S ratio rho -> arm) at w = 0, 0.1, 1; rank two arms only where the order holds across the band.
- C3 (descriptive): dp versus eq at the same B; report where the order holds across the band.
- Class: engineering of a generic collision search (the collector is a rho walk with B + 1 collisions needed); not structural IC, no claim to beat rho.

## Design
Curves p36, p37 (DP-active; p16/p18 omitted: b = 0 there). Arms: rho (rho.negation); baseline res1, res2, res4 (`residual`);
candidates res{1,2,4}dp and res{1,2,4}eq; res2aa (A/A copy of res2). 16 targets per curve, target seed 20261601, measure seed 20261602,
3 rounds + 1 warm-up, alternate_measured, local L0 run (counts deterministic; isolab-eks route unavailable; no wall-clock claim). `specs/` (L2) kept for a replay.
Positive control: harness verification of every answer and `ecbench verify --replay 12`. A/A interval must contain 1; all records verified; audits OK, else void.

## Decision rule (frozen)
C1 met iff all 24 cells hold. Otherwise report which cells fail. No threshold is tuned after data.
