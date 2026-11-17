# L1 — independent degree-4 closure at n=31 (PROTOCOL, written before any run)

Date: 2026-11-01. Question: is N1's n=31 stall (C1, C2, D1 at low_4 = 2821; A reaches 5489 and 1 in W_4)
an artefact of `N1-gf2-d4-profile/src/gf2close.c`, and if real, what is 2821?

## Object computed
W_4 = smallest F_2-subspace of Boolean polynomials of degree <= 4 in N = 2k variables containing the n input
quadratics and closed under g -> x_v * g (Boolean product, x^2 = x) for every g in W_4 with deg g <= 3.
Reported: dim W_4, low = dim(W_4 ∩ P_{<=3}), one = [1 ∈ W_4]. Same definition as gf2close (closure mode).

## Independent implementation (src/macclose.c), deliberately different from gf2close
- Column order: degree-4 monomials FIRST (lex within degree), then 3, 2, 1, constant LAST.
  Pivot = LOWEST set bit (gf2close: degree-compatible colex, pivot = highest bit).
- Monomial -> column via an open-addressing hash table (gf2close: combinatorial colex ranking).
- Plain one-row-at-a-time Gaussian reduction (no M4RI tables, no chunking), pivot rows stored sparsely-allocated.
- Round-based (BFS/Jacobi) closure: round 0 inserts F; round r multiplies every low row created in round r-1 by
  every x_v. Per-round, per-leading-degree pivot counts are logged (gf2close: single FIFO queue, no rounds).
- low = number of pivots whose column lies in the degree<=3 block (valid because deg-4 columns come first and
  pivot = lowest bit, so a stored row with pivot in the low block has no degree-4 term).
- Generator: N1 src/lib.py `system(n,k,s)` (unchanged, imported read-only); encoding written fresh here.

## Validation (V)
All arms {A,Ap,B,C1,C2,D1,D2,D4,RM}, n in {13,15,17,19,21} (k=(n+1)/2), draws s=0..2:
compare (dim, low, one) with gf2close called via N1 lib.call. Also a pure-Python tiny reference at n<=9.

## Main runs (M)
n=31, k=16, N=32: C1 s=0, C1 s=1, D1 s=0, A s=2 (N1 reported: C1/D1 2821/37293/0; A s=2 1/5489/41449).
Explanatory runs (E): n=29 C1 s=0 and D1 s=0 (and C1/D1 at n=27 if cheap) with round logs.

## Pre-registered predictions
- P1: macclose == gf2close on every V instance (0 mismatches). If any mismatch, stop and report.
- P2: n=31 C1 s0, C1 s1, D1 s0 give low=2821, dim=37293, one=0; A s2 gives one=1. (If not: the N1 anomaly is
  an instrument artefact of gf2close.)
- P3 (hypothesis H, formula found post hoc from N1 numbers: B arm low = n(N+1) exactly at n=27,29,31;
  2821 = 31*91 = n(3N-5) = n(N+1) + 2n(N-3)): for a rank-n bilinear tensor sum_r lambda_r(a) mu_r(b) nu_r
  each of the n equations yields 2 quadratic "degree falls" (1+lambda_r)l_r, (1+mu_r)l_r, each contributing
  N-3 new cubic multiples. GENUINE prediction: at n=29 (N=30) C1 and D1 pass through low = 29*85 = 2465
  before the degree-4 stage, and then gain further low elements (reach 1); at n=31 they get no low element
  beyond 2821 from degree-4 products. A at n=31 has low > 2821 already before the degree-4 stage.
- P4 (count-threshold reading, weaker): at n=31 the degree-4 projection rank of the products
  (dim - low) stays < C(32,4) = 35960, while at n=29 the analogous count would exceed C(30,4) = 27405 (forced falls).
Negative outcomes are reported as such; no parameter tuning. No security claims.

## Budget
1 thread per process, at most 2 concurrent. Outputs < 50 MB; logs gzipped.
