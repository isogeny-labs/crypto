# N2-d4-s5-lattice protocol (pre-registered 2026-11-01, before any run)

REPORT §6 step #2: rebuild the D4-symmetrised Semaev S5 small-root lattice with the smallness stated on the
primitive per-point coordinates (not on the invariants), derive the induced invariant bounds, compute reach
from the determinant, cross-check with the min-cut closure code (MP-lattices-3/lp.py, validated against lit2026),
fplll-validate at l=1. Fixes MP-lattices-4 AUDIT item 2 (invariant smallness by fiat). Expected class: engineering.

Setup: E: y^2 = x(x^2+Ax+c^2) over F_p, T=(0,0), x(P+T)=c^2/x, Y=(x-c)/(x+c), Y(P+T)=-Y(P).
Unknowns: 4 summands; x_R known (target). G(Y) = S5(x(Y_1..Y_4), x_R) * prod(1-Y_i)^8, degree <=8 per Y_i.
D4 invariants (W(D4)-type, (Z/2)^3 even flips x S4): s1,s2,s3 = e1,e2,e3(Y^2), t = Y1Y2Y3Y4 (e4(Y^2)=t^2; ring is free).
Candidate support: 2(a+b+c)+d <= 8, 105 monomials.

## Two smallness models (both run, kept separate)
- Arm X (literal: Weierstrass |x_i| < p^delta). Even flips act as x -> c^2/x, which does not preserve the box,
  so the induced invariant bounds are trivial (log_p bound ~1) and the D4 lattice has reach 0; the valid
  symmetrisation is S4 e(x). Empirical check: for planted small-x roots, report log_p |lift(s_k)|, log_p|lift(t)|.
  Prediction: all ~1 (>=0.95). Falsifier of the old fiat model: build the l=1 D4 lattice with bounds X^(2,4,6,4)
  at a delta the fiat model predicts succeeds, plant a small-x root; prediction: v1 does NOT vanish at the
  invariant values (8/8 fail).
- Arm Y (FGHR base |Y_i| < X = p^delta). Rigorous induced bounds |s_k| <= C(4,k) X^(2k), |t| <= X^4.
  fplll validation speaks to this arm only.

## Step 1: build H by interpolation (per seed, since x_R changes)
S5 = Res_u(S3(x1,x2,u), S4(x3,x4,x_R,u)), S4(a,b,c,u)=Res_v(S3(a,b,v),S3(c,u,v)). Evaluate G at >=200 random
Y points numerically mod p, solve for the 105 invariant-monomial coefficients; extra rows check consistency
(= invariance + support). Record: true nonzero support size; even-flip scalar lambda (semi-invariance would be a finding);
S4 symmetry. Prediction: consistent, support 105 (or a subset), lambda = 1.

## Step 2: theory reach
l=1 lattice: rows H (normalised on one monomial) + p*mu for the other support monomials; dim = |supp|.
Structural fact (pre-stated): at most one lattice vector independent over Z can be shorter than p (two would give a
combination with a=0 mod p, lying in p*Lambda). So l=1 yields at most ONE integer equation: necessary-condition reach,
not a recovery reach (recovery needs 4). Finite-p delta*: det^(1/n) * sqrt(n) = p, using true support and C(4,k) constants.
Asymptotic l=1: 1/sum(wdeg) = 1/1680 = 5.95e-4 if full support.

## Step 3: closure cross-check (MP-lattices-3 lp.solve, min-cut)
Validate in this run: reproduce lit2026 S4 l=2 closure 0.025591 (or S3 rows). Then D4 invariant H with bounds
[2d,4d,6d,4d] (log_p, constants dropped asymptotically) at l=1,2 (3 if time), grow bounding box until optimum is
interior; S4 e(x) arm with same code if affordable. Compare against Y-box = x-box lit2026 closure rows
(0.005253, 0.007281, 0.008384) at matched l AND matched dim.

## Step 4: fplll l=1 validation (arm Y)
p ~ 2^256 requested. Pre-computed: X* = p^(1/1680) < 2^0.16 at 2^256, so no nontrivial Y (|Y|>=2) can be planted;
planted arms at 2^256 declared not runnable for that reason (H-build + one null run still done at 2^256).
Deviation rule (pre-registered): time one unplanted LLL at 2^2048, 2^4096, 2^8192; fix p as the largest that fits
>=24 runs within ~40 min on 3 processes. Arms (>=8 seeds each): below (delta = 0.85 delta*), above (1.15 delta*),
null (random x_R, same planted Y values; no root). fplll -a lll default params.
Success = v1 (first reduced vector, unscaled) vanishes over Z at the lifted invariant integers (after asserting H=0 mod p).
Predictions: below >=7/8 vanish; above <=1/8; null 0/8; v2 never vanishes. Log ||v1|| vs det^(1/n).

## Gate
Reach vs 1/80 = 0.0125 and vs needed 0.169 (m=4). Prediction: D4 reach <= 1/80 asymptotically, << 0.169;
~8x dimension cut vs Y-box at matched l -> engineering. Surprise to flag: D4 closure reach > Y-box at matched dimension.
Seeds: 1..8 (python random/gp setrand). Threads: <=3. No security claims.

## Deviation log (written after timing, before planted runs)
- Timing (raw/timing_*.json, timing_8192.log): one LLL at 2^2048 13 s, 2^4096 59 s, 2^8192 216 s (+51 s H build).
  Per seed = 5 lattices (below, null, above, armX) -> 2^8192 would be ~180 CPU-min for 8 seeds, over the ~1 CPU-hour task
  budget. Chosen p ~ 2^4096 (X* = 2^1.93 ~ 3.8 incl. constants; below X=3, above X=5). Thin but nontrivial plants.
  p = 2^256 planted arms: not runnable (X* = 2^-0.36 < 1: no |Y|>=2 plant fits); one unplanted 2^256 LLL is in timing.
- build_h.S3 was first coded with the short-Weierstrass S3 (wrong model); caught by the symmetry check (lambda != 1) before
  any lattice run; fixed to the a2-model S3 and verified on 50 random point triples.
- Implementation fix (before any planted result): PARI issquare/sqrt/ellinit on 4096-bit Mod stalled (modulus checks);
  replaced by Euler criterion, p=3 mod 4 square root, and python affine addition with distinct x per plant
  (plants with repeated x are re-drawn; summands P_i distinct). p = first prime >= 2^L+777 with p=3 mod 4.
- Curve pre-check requires >=4 distinct on-curve Y in [-X_below,X_below] (needed for 4 distinct summands); this selects curves, not roots.
- eadd rewritten as full affine addition (doubling, O); verified vs PARI elladd 200/200 at p~1e6.
- plant re-draws if P1+..+P4 = O (happens with few small-Y candidates).

## Amendment A (2026-11-01, after 3 seeds at 2^4096, before any 2^6144 run)
Observed at 2^4096 (seeds 1,4,7): below AND above arms both vanish (3/3 first vectors each), null 0/3, v1 norm ~2^3830-2^4038
vs null 2^4063.5 and det^(1/n) ~ 2^4087. The "above" prediction FAILED. Diagnosis: at X<=5 only |Y| in {2..5} exist and every
plant contained a +-Y pair (e.g. {3,-2,2,-3}); P(-Y) = +-P(Y)+T, so R is a structured point (T-combinations, doublings),
not a generic decomposition target. raw/diag_degenerate_s4.json: the short vectors do not vanish on random +-pair points,
so this is specific to the special R, not a factor of H; the mechanism is NOT identified. Recorded as an anomaly; the
earlier "at most one vector < p" argument in Step 2 is WRONG (it ignored that a can be ~p) and is withdrawn.
Also: my structural one-vector claim is withdrawn; vanishing of v2,v3 is now just recorded.
Amended validation: p ~ 2^6144 (X* = 2^3.15 ~ 8.9; below X=6, above X=12), plants require pairwise distinct Y_i^2
(no +-pairs), 8 seeds x {below, null, above}; arm X not repeated (3 seeds at 2^4096 suffice). Same predictions as before.
Budget overrun accepted and reported (~70 CPU-min for this block).

## Amendment B (after all 8 seeds at 2^6144; exploratory, post hoc, labelled as such)
6144 result: above arm (X=12) vanished in all seeds; v1 norms ~2^35 below det^(1/n), null included. rank_test.py: H coefficient
vectors span only rank 59 of 105 over F_p (fixed curve, xR varying: rank 9), i.e. 46 integer linear relations -> non-generic
lattice, explains vectors shorter than the det prediction (hypothesis consistent, not proven).
Exploratory far-above arm: X=128 (log2 X = 7, well past the empirical extrapolated threshold ~2^5.3), seeds 1-3, one lattice
each. Prediction (stated now): v1 does not vanish in 3/3. No further X values will be tried.

## Amendment C (repair, 2026-11-01, written BEFORE running fp_probe.py; addresses AUDIT must_fix 1-4 only)
Scope set by the user ("do 1-4"): AUDIT must_fix items 5-8 are NOT addressed in this repair.
- Item 2: the G(Y) same-code closure at matched l and dim is NOT run (G(Y) is the non-symmetrised degree-8-per-variable S5 pullback;
  building it and a matched-dim closure is outside this repair's budget). The Y-box comparison is retracted instead.
- Item 3 probe (fp_probe.py): rebuild deterministically the 2^6144 lattices for seed 1 below (X=6), seed 2 below (X=6), seed 1 above (X=12)
  (same rng replay as run_fplll.py, null rng draws replayed without LLL; reproduction checked against stored log2norms).
  For v1..v3: (a) evaluate over Z at every Y-tuple in [-X,X]^4 (Y_i not in {0,+-1}), grouped by D4-invariant point
  (s1,s2,s3,t); count vanishing at invariant points != the planted one; (b) rank over Q of {h1,h2,h3}; (c) sympy gcd of the
  three polynomials in (s1,s2,s3,t) and of each pair.
  Predictions: (a) 0 false vanishings at non-planted invariant points for every vector; (b) rank 3; (c) gcd = 1 (constant).
  If (a) is nonzero the "8/8 vanish" result is not evidence of the HG criterion alone. Budget: 3 jobs x ~3 min, 3 threads.
