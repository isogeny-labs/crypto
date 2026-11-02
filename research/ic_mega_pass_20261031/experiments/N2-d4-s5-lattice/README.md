# N2-d4-s5-lattice: D4-symmetrised S5 l=1 lattice, rebuilt with stated smallness (REPORT §6 #2)

Outcome: engineering only. No security claims. (Headline reworded in Repair item 1; superseded text: "Every reach measured or predicted here is at most about 0.0057 ... No lever.")
- l=1 closure on the generic-coefficient model: 0.00568 at bounding box 14, still rising (increments shrinking; extrapolated l=1 limit about 0.0058, an extrapolation, not a bound).
- l=2 and l=3 were not run. This run gives NO asymptotic ceiling.
- The closure and determinant reaches assume generic coefficients; this H is measured non-generic (coefficient rank 59 of 105), so they are not ceilings for this H.
- Below 1/80 and 0.169 only as l=1, generic-model numbers.

## What was fixed
- The MP-lattices-4 model gave the invariants the size p^delta "by fiat". That turns out to be correct in one base and wrong in the other:
  - Arm Y, the FGHR base (|Y_i| < X). The induced bounds are rigorous: |e_k(Y^2)| <= C(4,k) X^{2k} and |Y1Y2Y3Y4| <= X^4, so weights (2,4,6,4) hold.
  - Arm X, the Weierstrass base (|x_i| < X). The even flips are x -> c^2/x, and that map leaves the box. For small-x plants
    the invariants measure at log_p ~ 0.999, so they are full size. The D4 lattice then has reach 0, and the correct symmetrisation is S4 e(x).
    The old fiat lattice failed on small-x plants in 0 of 3 tries to recover them (v1 never vanished), as predicted.
- H was built honestly by interpolation, not assumed:
  - S5(x(Y), xR) * prod(1-Y_i)^8 is exactly D4-invariant (even-flip scalar 1, S4 scalar 1, odd flip not invariant).
  - It lies in the 105-monomial polytope 2(a+b+c)+d <= 8, with full support, on every seed.
  - New fact: across random (A, c, xR) the 105 coefficients span only rank 59. Fixing the curve and varying xR gives rank 9.

## Reach
- Determinant reach of the l=1 dim-105 lattice:
  - Asymptotic: 1/1680 = 5.95e-4.
  - Finite p, with constants: log2 X* = -0.36, 1.93, 3.15 and 4.36 at p = 2^256, 2^4096, 2^6144 and 2^8192.
- Closure cross-check uses MP-lattices-3/lp.py unchanged. It was validated in this run against lit2026 (S3 0.09/0.11607/0.12889; S4 0.018780/0.025591).
  - The D4 box [4,4,4,8] gives 5.95e-4. That reproduces the dim-105 lattice and MP-lattices-4.
  - Growing the bounding box gives 0.00304 (dim 797), 0.00458 (1688), 0.00527 (3157), 0.00558 (5412) and 0.00568 (8693). It is not yet bounded.
  - [SUPERSEDED, retracted in Repair item 2] "The Y-box (= x-box) l=1 closure in lit2026 is 0.005253 at bounding box 38. So at l=1 the D4 lattice passes the box at a far smaller dimension." / "This is a finite-l edge. By the MP-lattices-4 MC and the lit2026 ceilings, the asymptote is still <= 1/80." 0.005253 is lit2026's JM-extended box strategy on x (dim about 2.3M), not a closure computed here on G(Y); Y-box = x-box was assumed. No comparison is made. l=2 was not run (budget).

## fplll validation (arm Y, l=1, dim 105)
- p = 2^256: the planted arms could not run. X* < 1, so no |Y| >= 2 plant fits. Deviation: p = 2^6144 for the main run (2^8192 was over budget).
- 2^6144, 8 seeds, plants with pairwise-distinct Y_i^2:

  | arm | X | v1 vanishes |
  |---|---|---|
  | below | 6 | 8/8 |
  | null | 6 | 0/8 (cannot vanish by construction; not a calibration, see Repair item 3) |
  | above | 12 | 8/8 |

  - The pre-registered "above fails" prediction FAILED.
  - The v1 norms (2^6094 below, 2^6113 above, null 2^6095) are about 2^35 under det^(1/n). The lattice is non-generic, consistent with the rank-59 coefficient span, but this was not proved.
  - An exploratory far-above arm at X = 128 (post hoc, one value, 3 seeds) gave 0/3, with v1 norm = p.
  - Empirical threshold: log2 X in (3.58, 7), i.e. delta_emp in (5.8e-4, 1.1e-3). [SUPERSEDED: "about 5.5 by extrapolation ... delta_emp about 9e-4" had no stated method; simple extrapolations of the stored v1 norms to the HG bound give 5.0 (from X=6,12) to 6.6 (using X=128).] delta_emp is finite-p with constants, not comparable to the asymptotic 1/1680.
- 2^4096, 3 seeds: below 3/3 and above 3/3, null 0/3. These plants were forced to contain ±Y pairs and gave abnormally short vectors. The anomaly is unexplained and is superseded by the 2^6144 run.
- Correction: the protocol's argument that at most one vector can be shorter than p is wrong (a can be ~p). v2 also vanished.

## Classification
- Engineering, with the dimension cut real: 105 vs 6561 at l=1.
- [SUPERSEDED] "There is a finite-l closure edge over the Y-box in the FGHR base only." Retracted (Repair item 2).
- [SUPERSEDED] "Reach stays below 0.006". Only the l=1 generic-coefficient closure is below 0.006 at the boxes run; no ceiling is established (Repair item 1).

Files: PROTOCOL.md (with amendments A and B), build_h.py, lattice.py, run_fplll.py, run_far.py, closure.py, closure_val2.py,
rank_test.py, diag_degenerate.py, summarize.py, raw/.

## Repair (2026-11-01; AUDIT must_fix items 1-4 only, per user instruction "do 1-4"; items 5-8 NOT addressed)
Conductor check could not run (local conductor service unreachable from sandbox); edits confined to this directory.
1. Headline and RESULTS gate reworded: l=1 closure 0.00568 at bbox 14 and still rising (extrapolated ~0.0058, not a bound);
   l=2, l=3 not run; no asymptotic ceiling from this run; closure and determinant reaches assume generic coefficients while H is
   measured non-generic (rank 59/105). Old text kept, marked SUPERSEDED (RESULTS: gate_superseded).
2. Y-box comparison retracted, not computed. The same-code G(Y) closure at matched l and dim was not run (G(Y) is the
   non-symmetrised degree-8-per-variable pullback; out of this repair's budget). 0.005253 is lit2026's JM-extended x-box strategy at dim
   about 2.3M; "Y-box = x-box" was assumed. "passes the box" and "edge" are withdrawn. Prediction miss reported: protocol predicted
   about an 8x dimension ratio; the 62.5x figure is against the 6561-dim JM-basic box (reach 1e-5), not a matched-reach comparison.
3. Null: any lattice vector with norm < p has leading multiplier a != 0 mod p, so F(Ys) = a*H(Ys) != 0 mod p for the null; 0/8 and 0/3
   carry NO false-positive information. New probe fp_probe.py (pre-registered in PROTOCOL Amendment C) at 2^6144, rebuilt
   deterministically (v1-v3 norms reproduce the stored ones exactly): seed 1 below (X=6), seed 2 below (X=6), seed 1 above (X=12).
   - v1-v3 evaluated over Z at every other D4-invariant point from Y in [-X,X]^4 (|Y_i|>=2): 139 points (X=6), 2001 points (X=12).
     False vanishings: 0/0/0 in all three runs (prediction held).
   - h1,h2,h3 have rank 3 over Q; pairwise and triple gcd over Q[s1,s2,s3,t] = 1 (prediction held).
   - Limits: 3 instances, small box; this probes non-root small points, not the +-pair mechanism of the 2^4096 anomaly.
   - This experiment tests the Howgrave-Graham vanishing criterion. It is NOT root recovery: 3 independent coprime equations in
     4 invariants were found; recovery (eliminating to Y) was not attempted.
   raw/fp_probe_L6144_s*_{below,above}.json (with p, A, B, c, xR, vector hashes) and *_vecs.json (unscaled vectors) and logs.
   CPU: about 12 min (3 parallel jobs, about 3-4 min LLL each plus checks).
4. "about 5.5 bits / delta about 9e-4" replaced by the supported bracket log2 X in (3.58, 7), delta in (5.8e-4, 1.1e-3);
   point extrapolations of the stored v1 norms give 5.0 (X=6,12) to 6.6 (with X=128), none adopted.
