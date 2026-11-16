# L4-pkm-plateau: protocol (written before any measured row, 2026-11-01)

## Target quantity (reconciled)
The registered prediction C-step (prime_fall_degree_20261027/PREREGISTRATION.md, Amendment 6) is:
Kummer tower, **m = 2**, prime P1 = 2013265921, HKY **last fall degree d_F** = 4 + floor((t-2)/4), t >= 2.
The m=3 line D = 6,6,7,7 (N=9..18) is an F4 *solving degree*, a different quantity. C-step is scored
only on m=2 d_F. msolve's max F4 step degree D is reported as its own column (H-gap, gate G2).

## Blindness / what can be scored
- t <= 10: the prediction was fit to these medians (3,4,4,4,4,5,5,5,5,6). Rows here are a
  **reproduction** with NEW draws (own generator, not the Rust pilot's rng), i.e. distributional, not exact repeat.
- t = 11: NOT blind (round-2 had D=6 with cap 5 not refuting; Amendment 7 E).
- t = 12..14: the only blind cells. Pre-registered expectation: **not reachable** in this lane
  (2 threads, ~2 CPU-hours, ~5 GB free internal disk); verdict on C-step = **undecided (blocked: resources)**
  regardless of how t <= 11 reads. A clean t <= 11 match is NOT "C-step holds".
- Context (not ours): research/prime_fall_degree_t14_20261101 reports cap 5 does not refute at t=14, so d_F >= 6; undecided.

## Instruments
1. Own exact closure V_{F,c} (C, sparse rows, mod p) in R = F_p[z1,z2]/(z1^K-1,z2^K-1), K=2^t,
   deg z^k = popcount(k) (DERIVED D2). Variables y_{i,j} = z_i^{2^j}. d_F = least c with 1 in V_{F,c};
   also operational ffd. Per-cell cap: cmax and wall-time limit; stopped cells read censored (d_F > c or >= c).
   Rust pilot (registered engine) not used: a cargo build into $TMPDIR risks the nearly full internal disk.
2. msolve (F_p, -t 2, -v 2) on {y_j^2 - y_{j+1}, y_{t-1}^2 - 1, f}: report max F4 step degree D.

## Gates (before measured rows; any failure stops and is reported)
- Ga: ffd = 4 for every t >= 2 Kummer draw with 3x_R^2+a != 0 (D3).
- Gb: d_F <= 2t+2 (D4).
- Gc: a single summand (f depending on z1 only, no grid root) has d_F = 2 (D8).
- Gd: planted (consistent) target never reaches 1.
- Ge (G2): d_F <= D_msolve on every common cell.

## Cells
- Kummer m=2, P1, t = 1..11 as far as budget allows, 4 random targets per t; draws with a zero of f on
  mu_K^2 are decomposable: reported and excluded (Amendment 7 C), not redrawn.
- Second prime 998244353 (2^23 | p-1): same, t <= 8 (fewer targets).
- Null: random coefficients on the same {0,1,2}^2 support in (z1,z2), no grid zero, nonsingular bilinear part; same t ladder.
- Optional side: msolve D for m=3 is NOT run unless time remains (it is not part of C-step).

## Pre-registered predictions
- P-repro: Kummer P1 medians (lower median over valid draws) for t=1..10 equal 3,4,4,4,4,5,5,5,5,6 where measured.
  Any difference of >= 1 at any t is reported as a reproducibility discrepancy (not a C-step refutation at t<=10).
- P-ffd: ffd = 4 at every t >= 2.
- P-D: D_msolve >= d_F; D_msolve - d_F in {0,1,2}.
- P-null: null medians equal Kummer medians within 1 at the two largest common t ("shape-driven"), as round 2 saw.
- P-C-step: undecided (t>=12 not reachable).
Negative results are reported as is; no parameter is tuned after rows exist.
