# N3-capacity-lp-converse: PROTOCOL (pre-registered 2026-11-01, written before any run)

Task: REPORT §6 step 3. Turn the MP-lattices-6 accounting ceiling (box reach 1/6, 1/24, 1/80 for m=2,3,4)
into a statement about all auxiliary polynomials, inside an explicitly stated class. Accounting only; no security claims.

## Setting and notation
- f = S_{m+1}(x_1..x_m, x_R) mod p (specialised Semaev), d = 2^{m-1} = per-variable degree. Lift f to Z.
- U = {a in Z_p^m : f(a) in pZ_p} (all p-adic points reducing to F_p-points of V(f)). The unknown root x0 lies in U.
- For a finite support M ⊂ N^m: L_l(M) = {g in Z[x]_M : g(a) ≡ 0 mod p^l for all a in U}  (the "sectional" lattice:
  every auxiliary polynomial of support M usable without knowledge of the instance root, including polys outside (p,f)^l).
- gain c(M,l) := log_p [L_l(M) : p^l Z^M]. Minkowski/HG reach: delta(M,l) = c(M,l) / Σ_{s in M} |s|  (uniform bound |x_i| < p^delta).
- k(s) = min(l, min_i floor(s_i/d)); JM triangular count Σ_s k(s) (MP-lattices-6 metric).
- Filtration identity (exact): c(M,l) = Σ_{j=1..l} dim_Fp W_j(M), W_j(M) = (L_j(M)) mod p.
- Saturation premise P(M,l,p): W_j(M) ⊆ f̄^j F_p[x] for j<=l.

## Arm A0 (theory; no computation beyond checking hypotheses)
Claim: under P, dim W_j(M) <= #{s in M : s >= j·LM(f)} for any graded order with LM(f) = (d,..,d);
hence c(M,l) <= Σ_s k(s) for ANY finite M (non-downset allowed), and delta(M,l) <= max_s k(s)/|s| = 1/totdeg(f) = 1/(md).
Weighted form: bounds X_i = X^{w_i} give delta <= 1/deg_w(f).
Check (computed): supp(f) ⊆ [0,d]^m, coefficient at (d,..,d) is a unit, (d,..,d) is the unique top-total-degree monomial,
for S3 (m=2), S4 (m=3), and S5 (m=4, via resultant in PARI if it runs in budget; else say so).
Prediction A0: hypotheses hold for all three; ceilings 1/4, 1/12, 1/32. 1/32 < 0.169 (needed) for m=4.
novelty_unverified (likely folklore: "Coppersmith reach <= 1/deg").

## Arm A (sectional p-adic interpolation determinants, small p)
Compute c_samp(M,l,p) = gain of the kernel of evaluation at N sampled points of U mod p^l (exact SNF over Z/p^l by
valuation-pivoted elimination). Λ_samp ⊇ L_l(M), so c_samp >= c(M,l); grow N until plateau (N >= 4|M| and two
successive doublings equal). If c_samp == Σ_s k(s) on a box, saturation is certified at that cell (c = Σk exactly).
Grid:
- m=2 (S3, d=2): box M=[0,2l+t]^2, l=1..3, t in {0,2}; plus 3 non-downset supports (box with interior holes, box minus
  a top corner block, random 70% subset of box); p ladder {3,5,7,11,13,17,23,31}.
- m=3 (S4, d=4): box l=1 (M=4) and l=2 (M=8) at p in {5,7,11,13,17}; l=2 only if runtime < 10 min.
- m=4: l=1 box M=8 has 6561 columns; NOT run (l=1 reach is 0 anyway; out of budget). Stated a priori.
Predictions A:
- A1: for p > total degree of M's monomials' per-variable degree (p > M_max) c_samp == Σ_s k(s) on every box cell (equality).
- A2 (negative control): for p <= M_max (e.g. p=3,5 at m=2), c_samp > Σk is expected (x^p - x type artifacts);
  such cells are flagged artifact and NOT counted as beating the box.
- A3: on non-downset supports with p > M_max, c_samp <= Σ_{s in M} k(s).
Falsifier: any cell with p > M_max and c_samp > Σ_s k(s) refutes the saturation premise there (then A0 does not apply
at that cell and the reach is reported).

## Arm B (LP / exact search over supports, m=2 and m=3, large p ~ 2^61)
- B0 calibration: reproduce MP-lattices-6 box closed form exactly (rational) and the audit staircase optimum
  (0.14129 l=5, 0.15267 l=10, 0.16158 l=30) with an exact Dinkelbach + DP over 2D staircases.
- B1 per-monomial LP over k(s) with only the pointwise cap k(s) <= min(l, min s_i/d): optimum = 1/(md), dual certificate
  k <= |s|/(md). Stated, computed by scipy linprog as a check.
- B2 exact D_j(M) = dim{h : supp(f̄^j h) ⊆ M} over F_p for the real f (not a generic stand-in), on families:
  downsets/staircases, shifted & unequal boxes, cut corners, convex polygons, and a local search (simulated annealing,
  fixed seed 0, fixed budget 2000 moves per (l), not tuned) over non-downset M ⊂ [0,2l+4]^2, score
  ρ_D(M) = Σ_j min(D_j(M), N_j(M)) / Σ_M |s| (valid upper bound on delta(M,l) under P).
  Prediction B2: no M at l in {3,5,8} exceeds the exact staircase optimum at the same l by more than 1e-9
  (i.e. downsets are optimal); if one does, it is reported as a counterexample to "downset-optimal" (still <= 1/(md)).
- B3 m=3: box and box+t only (exact D_j), check vs closed form.

## Decision rules
- Main deliverable (m=4): converse holds in class C if A0 hypotheses verified for S5 (or stated as assumed if S5 cannot be
  built) and premise P certified in arm A at m=2,3 for p above threshold. Then "no auxiliary polynomial set in class C reaches
  0.169 at m=4 (ceiling 1/32)". Whether 1/80 itself can be beaten inside C at m=4 is NOT decided by this experiment unless
  B finds the optimum = box for m=2,3 (then reported as evidence, not proof, for m=4).
- Class C (to be restated in README): single f, single modulus p^l, auxiliary polys vanishing mod p^l on all of U
  (root-oblivious), uniform (or fixed-weight) archimedean bounds, Minkowski/determinant accounting, p above saturation threshold.
Budget: < 1 CPU-hour total, <= 3 threads (OMP/OPENBLAS=3).
