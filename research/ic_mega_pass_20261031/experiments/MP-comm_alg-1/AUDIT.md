# AUDIT MP-comm_alg-1 (2026-10-31)
audit: partially_valid | stated outcome: supports | corrected_outcome: inconclusive | reran: true

## Verified (correct)
- Reran 11 raw cells (d=3: n=9,11,13,15,17,21,25(k9),29; d=4: n=9,11,13). res arrays bit-identical to raw/*.json (seeds deterministic). CPU ~110 s.
- check_vs_core.py: match_core True (n=7,9,10).
- Independent GF(2) closure (numpy, different monomial order, low = dim - rank(deg-d block)) reproduces dim/low/one exactly for arms A,B,C2,D1 at (9,5), (15,8) draws 0-2. Closure code (pivot=top bit, low = pivots < nlow, Boolean x^2=x via mask OR) is right; no off-by-one.
- A' construction algebra is right (T', affine, constant transform). Equality 29/29 holds.
- Field/subspace: GF(2^n) modulus irreducible by gcd test, L(x)=x^2+zx F_2-linear, V random k-subspace by rank test. No wrong-field/subgroup issue found.

## Problems
1. Control A' is real but weak. W_d is invariant under affine changes of variables and under invertible mixing of equations (Boolean ring: l^2=l, reduced degree is affine invariant). A'==A is therefore an identity, a code-consistency check only. It cannot detect selection or modelling error. The null that matters is a matched-structure null (random_matched), which was cut.
2. Pre-registered decision rule not met for D1. protocol: D_R must "differ" (>5 sigma) from A. Recomputed A-D1 z at diagonal n=17..45: -2.41,-4.38,-3.29,-2.22,-2.94,-1.79,-4.14,-2.91,-3.04,-2.52,-2.47,-2.82,-2.92,-2.91,-3.41. Max |z|=4.38; never >5. README "2..6 sigma" is wrong (6.15 is D1-B at n=17). D1 (no Galois structure, F_2-rank n) reproduces A to within +2..15 of 3n (at n=45 135.6 vs 135). The pre-registered mechanism "Galois-stable K-bar-rank-n structure is the cause" is contradicted by D1; README admits the reinterpretation ("rank n over some extension") post hoc.
3. Result is near-tautological. Any bilinear system with n rank-one terms l_r(a)m_r(b)nu_r with independent nu_r gives 2 degree-2 falls per term (l*f, m*f via l^2=l), so low_3=3n, B=n. A=C1=C2 equal by construction (same tensor class), A!=B because B is dense. The "fit" slope 2.000+-6e-16 and e_3(131)=262 are an exact identity 3n-n=2n on zero-variance data; the CI is meaningless. Already Lemma deg3.
4. Fit restricted to n>=17 post hoc (not in protocol). n=9..13 saturate B_{<=2} (A low_3=54/77/104 vs C(2k,<=2)=56/79/106); disclosed only implicitly.
5. n=15 C2 cell is a real, reproducible anomaly (20/20 draws ~133-137 = near-saturation of C(16,<=2)=137; independent code agrees, draws 0-2: 133,137,133) while A=C1=45. Contradicts "C2 matches A" and "controlling quantity is K-bar-rank" at one n; unexplained, other composite n (9,21,27,33,35,39,45) are fine. Not noted as a failure of the prediction.
6. d=4: "indistinguishable at n<=15" is saturation, not agreement: low_4 ~ C(2k,<=3) for every arm (n=15: 695 of 697). Only n=17 is informative (10 draws, one cell): A,C,D1 ~985 of 988 vs B=323=n+n(2k) (no falls). That is the sole d>3 datum and it is a single n. Frequency of 1 in W_4 pooled n=9..17 over 50 draws: A 16, B 5 (Fisher one-sided p~0.006), but C1 18, C2 8, D1 8, D2 4, D4 4; A-like arms differ among themselves (n=17: A .1 vs C1 .4). Confounded with solution count (1 in W iff no solution when saturated). Not a robust signal; no test was pre-registered for it.
7. README claim "mu_V has F_2-rank >= 2n-1" is not computed or cited for the restricted k x k x n tensor (2n-1 style bounds are for full multiplication). Unverified. D2 (rank 2n, random) shows only that random rank 2n has no falls.
8. Protocol deviations: seed is 1000*n+100*k+s (protocol 1000*n+s); d=4 reached n=17 not 19; cut items (msolve, static d_NS, semi-regular corank, random_matched) acknowledged. System has ~n equations in ~n+1 variables (near zero-dim), not the larger-ell regime, so no d_F inference.

## Overclaim check
README "Outcome: SUPPORTS" overstates. Supported narrowly: d=3 closure counts are exactly 3n for K-bar-rank-n tensors (and approx for random F_2-rank n), n for dense/high-rank, code correct. Not supported: that Galois-stable structure is the cause (D1 matches within 5 sigma everywhere), any n=131-relevant extrapolation beyond the identity 2n, any d=4 profile claim from one cell. Classification: relabelling of known Lemma deg3 / Frobenius collapse (agrees with README caveat).

corrected_outcome: inconclusive (pre-registered conjunct "D_R differs from A" fails for D1; mechanism clause contradicted; remaining data valid but known).
