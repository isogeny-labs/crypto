# Resume: crypto @ claude/index-calculus-ecdlp-symmetry-65jh09 (round 2, 2026-11-03)

## Goal
Follow-ups to PR #3361 (research/ic_tree_gluing_20261103, merged): the next
steps listed in its README, run in a new run directory `round2/`.

## Pre-registered predictions (written before any round-2 run)
1. Same engine (msolve), m=4: F (S5 in e1..e4) has D >= 8 (input degree 8).
   T has D = 7 at t=4 (already measured). Prediction: D_F - D_T >= 1 at t=4 and t=5.
   F's widest matrix and time are lower than T's at t=4, as the lab's f4_fp record suggests.
2. Conjecture B (separators drive t-growth). Arm S = T plus X_c = planted
   value. Prediction: D_S equals the m=2 cherry D (4) at every t, while D_T
   grows. Falsified if D_S grows with t like D_T does, which would mean the
   membership towers, not the separators, drive the growth.
3. T t-slope, m=3, t=6, 2400 s, uncontended: D_T = 7 (it selected 7 before the earlier 600 s cut-off).

## Spec
gen.sing gains arm S (m=3,4) and arm F for m=4 (S5 by resultant, numeric xR).
Runs: msolve -g 2 -t 1 -v 2, 1800 s cap (2400 s for the m=3 t=6 T cell), 3 jobs max on 4 cores.
Cells: m4 t4 k0 {F,S}; m4 t5 k0 {F,T,S}; m3 t4..6 k0 {S}; m3 t6 k0 T (long).

## State
- Branch reset on main ca1c550507 (after the #3361 merge).
