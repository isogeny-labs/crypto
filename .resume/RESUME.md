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

# Round 3 (2026-11-03, written before any round-3 run)

## Items
1. Arity scaling of the tree. Run arms T and S at m = 5 and 6, t = 4 and 5.
   - m = 5: cherries (1,2),(3,4) plus leaf 5; X1, X2, then Y = x(X1 ⊕ X2); root S3(Y, x5, xR).
   - m = 6: cherries (1,2),(3,4),(5,6); X1, X2, X3; Y from X1, X2; root S3(Y, X3, xR).
   - The coupling excess is E(m,t) = D_T − D_S. It measures 0, 2, 3 at m = 2, 3, 4 (t = 4).
2. Glued arm G at m = 4. Eliminate X1 and X2 by resultants:
   G = Res_X1(S3sym(c0, X1), Res_X2(S3sym(c1, X2), S3(X1, X2, xR))),
   plus both cherries' degree-2 remainder chains. G is S5 in pair coordinates, invariant
   under S2 ≀ S2. Run it at m = 4, t = 4, 5 (and m = 3 analogue not needed).

## Pre-registered predictions
- P1: D_S = 4 at m = 5 and 6, as at m = 3 and 4.
- P2 (kill test): E(5,4) ≤ 5 and E(6,4) ≤ 6, i.e. at most linear, slope ≤ 1 from m = 4.
  If E(6,4) > 6 (or T at m = 6, t = 4 is cut off above degree 10), the tree does not meet D = O(m) and this line closes.
- P3: at m = 4, t = 4, G has D in [7, 8] and a widest matrix below T's 25,742 columns.
  G beats F (10.7 s, 13,503 columns) only if its time is < 10.7 s; I expect it not to (50/50 at best).

## Spec
- inst.sing: deterministic planted instances for m = 5, 6 (LCG with a fixed seed, no Python),
  written to instances_r3.txt. Seed 0x7EE51103, curve line from instances.txt.
- gen.sing: arms T and S for m = 5, 6; arm G for m = 4; env INSTFILE.
- Runs: msolve -g 2 -t 1, 1800 s cap, at most 2 jobs, 6 GiB each.
