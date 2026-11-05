# L10-kertr-family -- PROTOCOL (written 2026-11-02 before any closure run)

Toy-scale measurement. No security claims. Single closure definition (N1 gf2close W_d, round-based Macaulay
closure with x^2=x; gf2close5 for d=5), single generator convention (N1 tensor_closure helpers).

## Question
Treatise open problem 13 (population form): does the ker-Tr family T (V random k-dim subspace of K_0 = ker Tr,
z a curve x-coordinate, k = ceil(n/2)) fail degree-4 refutation (1 not in W_4) where N1's arm A (random V, uniform z)
does not, and at what rate per n in {29,31,33,35}?

## Instance families
- T (treatise gen2.py basis=tr0 zmode=curve, rebuilt with N1 conventions in src/lib10.py): a6 uniform nonzero, a2 in {0,1},
  V from uniform trace-0 elements until rank k, z uniform with Tr(z + a2 + a6/z^2) = 0. Draws with Tr(a6/z^2) = 1 are
  rejected and resampled (1 is then already in span F, Thm reform(3)); reject count logged. Own seed stream
  10_000_000+1000n+100k+s. UNSAT decided by gf2close bsols (exhaustive over a).
- A: N1 lib.system(n,k,s)['A'] unchanged. Data for n=29,31,33 reused from N1 raw / L8 RESULTS (not rerun). New run at n=35.
- R (control, only if time allows): as T but V random (treatise R family).
- Note (derived, check not run): for odd n, Tr(1)=1, so every z != 0 is a curve x-coordinate for exactly one a2; curve-z with
  uniform a2 is uniform z. A is therefore distributionally the treatise U/R family; the only difference T vs A is V in ker Tr
  (plus the rejection rule).

## Validation (before main runs)
V1 builder equality: for named instances, lib10.build == treatise s3_descent (equation sets identical); modulus(n) == modulus_hex.
V2 named reproduction with gf2close (low = dim W_d cap B_{<=d-1}, dim = dim W_d):
| instance | d | expected (low, dim, one) |
| T29a | 4 | (4526, 31931, 1) |
| T31a | 4 | (5489, 41449, 1) |
| T33a, T33b | 4 | (2968, 48068, 0) |
| T33a | 3 | (2208 low? -> W_dims[2]=64, dim 2208) i.e. (64, 2208, 0) |
| R33a | 4 | (6580, 52956, 1) |
| R35a | 4 | (3885, 61425, 0) |
Any mismatch stops the main run until explained. C21a/b (t=3 chain, N=42, cubic) has no bearing on the m=2 population
question; attempted last only if time allows, else skipped with this reason.

## Main runs
- T at n=29,31,33,35: scan seeds s=0.. until >= 8 UNSAT draws, d=4 closure on each UNSAT draw (SAT draws: record nsol only).
  n=33 extended to 16 UNSAT if time allows.
- A at n=35: 8 UNSAT draws, d=4.
- d=5: gf2close5 on ONE n=33 T draw that fails d=4 (first one), single process, CPUCAP=10800 s. Deviation from BRIEF 0.7
  (~20 CPU-min per run) recorded here in advance: L7 needed 2818 s at n=31; n=33 projected 1.5-3 CPU-h, ~6.9 GB triangular storage.
  n=35 d=5 (~444k columns, ~12 GB, many CPU-h) pre-declared infeasible in this session -> blocker, not run.
  If cap hits: report queue progress, outcome inconclusive.
- Threads: at most 3 concurrent single-thread gf2close processes; d=5 never alongside 3 d=4 workers.

## Pre-registered predictions (treatise-informed, NOT independent: the treatise's T29a/b,T31a/b,T33a/b and R33/R35 are known)
P1 structural, every T draw: F_2-rank of span F = n-1 and degree<=1 part of span F = 0 (A/R: rank n, low part 1).
P2 T refutes at d=4 on all UNSAT draws at n=29 and n=31 (>= 7/8 each).
P3 T fails d=4 (1 not in W_4) on all UNSAT draws at n=33 (>= 7/8) and n=35 (>= 7/8).
P4 at n=33 every failing T draw has low4 = 2968 and dim4 = 48068 exactly (same as T33a/b).
P5 A at n=35 fails d=4 on >= 7/8 UNSAT draws (U35/R35 are >=5); then the T-vs-A contrast is confined to n=33.
P6 the n=33 d=5 run reaches 1 in W_5 within the cap (treatise R35a is exactly 5; weak prior for T33).
Rates reported per (family, n) with exact Clopper-Pearson 95% intervals, never pooled.
