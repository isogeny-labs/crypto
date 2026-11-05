# L13 plateau-deficit — PROTOCOL (written 2026-11-02 before any L13 run)

Class: explanation/accounting of a measured closure plateau. No security claim.

## Model (derived on paper from L1's round logs; disclosed: fitted AFTER seeing n=29/31 numbers)
L1 found that C1's round-1 span has 3n elements of degree <=2, consistent with g_r = λ_r μ_r + ℓ_r (r=1..n) and the
falls (1+λ_r)ℓ_r, (1+μ_r)ℓ_r. Top-degree parts: λμ, λℓ, μℓ. In A = F2[x]/(x_i^2) every linear form squares to 0
(char 2), so in coordinates y1=λ, y2=μ, y3=ℓ the r-th block is the ideal I_r = (y1y2, y1y3, y2y3).
- deg<=3 per block: 3 quadrics + monomials of degree 3 with >=2 of the y's = 3 + 3(N-3)+1 = 3N-5. Blocks cannot meet
  below degree 4 (product of two quadrics), so **low = n(3N-5)**.
- deg 4 per block: D(N) = 3C(N-3,2) + (N-3). Pairwise overlap I_r ∩ I_s in degree 4 = I_r·I_s (9-dim, Koszul
  syzygies); no degree-4 higher syzygies (triple products have degree 6). So
  **R4(n,N) = n·D(N) − 9·C(n,2)**, and the closure stalls (no forced degree-4 fall) iff R4 < C(N,4).
- Deficit(n) = C(N,4) − R4. For N=n+1: n=29: −261 (no stall, surplus 261), n=31: +1488 = 31·48, n=33: +4070.
- Threshold: R4 ~ 1.5 n^3 vs C(N,4) ~ n^4/24; on N=n+1 the sign first turns at n=31.

Already seen before writing this (NOT predictions, only consistency): n=31 deg4 34472, low 2821; n=29 C1 +261 round-3
low with full deg4; n=33 C1 dim4 45507 (= 3201 + 42306; I had read 45507 in LEADS3 but not compared it before deriving).

## Pre-registered predictions
- P1 (pure model, generator-independent): n random triples of linear forms in N vars; GF(2) rank of the degree-4
  span = min(C(N,4), R4(n,N)) and degree<=3 dimension = n(3N-5) whenever n(3N-8) <= C(N,3), on >=6 (n,N) pairs
  incl. ones with R4 just below/above C(N,4). Exact equality on every pair; one mismatch with R4<C(N,4) falsifies.
- P2 (closure, stalled, N=n+1): n=35 C1 and C2 (s=0, plus one UNSAT draw if s=0 is SAT):
  low = 3605, dim = 3605+51240 = 54845, one=0.
- P3 (off-diagonal k): n=31,k=17 (N=34): stall, low = 31·97 = 3007, dim = 3007+40021 = 43028.
  n=33,k=16 (N=32): R4=36399 > 35960, so NO plateau stall: 1 ∈ W_4 on UNSAT draws.
  n=29,k=16 (N=32): R4=32509 < 35960 -> plateau at low 2639, dim 35148 (unless solutions lower it; SAT draws excluded
  from the dim prediction if nsol>0 changes it — recorded either way).
- P4 (exploratory, weak): for non-stalled n (25,27,29, C1, macclose round logs) the low increase in the first round that
  forms degree-4 products equals the surplus R4 − C(N,4) (n=27: 1341, n=25: 1900). Failure here does not falsify P1-P3.
- Scope: model is for C1/C2 (rank-n λμ+ℓ structure). D1 (101 quadrics at n=31) and A (extra 4n Frobenius falls) are
  outside it; reported, not predicted.

Engines: N1 gf2close (via lib.call), L1 macclose (round logs). <=3 concurrent processes. Outputs in raw/.
