# AUDIT MP-lattices-4 (2026-10-31)
reran=true: identity.gp, reach.py (l=1..5, ~7 s CPU), asym.py all reproduce RESULTS.json numbers exactly.
audit=partially_valid. Numbers are correct; the "supports" label and the model are not.

## Findings
1. Pre-registered rule violated, then reinterpreted. PROTOCOL: prediction_if_true = D within 5% of A/B at matched l; prediction_if_false = D exceeds B by >10%. Measured D4/S4e = 4.71,1.26,1.12 (l=1..3: >10%, rule for "false" fires), 1.077 (l=4), 1.056 (l=5; both >5%). The "true" prediction is never met at any measured l. README switches to an asymptotic criterion that was not pre-registered. l=6 was dropped (killed; reach_raw.txt starts with rc=124 and reach_out.json was never written).
2. Control arms are weak/tautological.
   - identity.gp checks Y(P+T)=-Y(P) for T=(0,0), B=c^2: this is the algebraic identity x->c^2/x, true by construction. The wrong-c control only breaks that Mobius identity; it does not test any sum-invariance or the D4 rewrite of Semaev S5.
   - Even-flip sum invariance (P_i+T twice = 0) is group-theoretic and trivial. The protocol's R=-(sum) and the "odd-2-torsion-free curve" negative control were not implemented.
   - Nothing verifies that S5 expressed in (e1,e2,e3 of Y^2, pi) has support inside 2(a+b+c)+d<=8. "105 monomials" is a lattice-point count of the assumed polytope (tautology), not a property of S5. The polytope is only a per-variable degree bound; true support may be smaller.
   - pi, e_k(Y^2) are not small integers. Smallness is a property of x_i, not Y_i=(x_i-c)/(x_i+c) mod p (a modular ratio). The D4 flips send the small region (Y near -1) to Y near +1, so invariant values at the root are arbitrary residues, not X^w-bounded. The weights (2,4,6,4) as size exponents are therefore unjustified. The model assigns the new variables the bound p^delta by fiat.
3. Claim B is vacuous. Ybox = Mobius = x+k = 1/x identical by construction (all set to the same code line r['Ybox']=r['xbox']); no computation distinguishes them, and the premise that bounds transfer under Mobius is false (see 2). Also "Ybox = xbox" is assumed, not computed.
4. Strategy mismatch. JM componentwise reach at l=1 (xbox 9.5e-6) is orders below the programme's min-cut baselines (0.005253/0.0144), so the finite-l D4 edge (62x over xbox at l=1) is an artefact of a weak strategy and says nothing about min-cut behaviour. The decay-to-1.0003 conclusion is under this strategy only; the optimal-lattice reach for non-box polytopes is not shown <=1/80. Best-of-lead-vertices selection is declared, fine.
5. MC asymptote: D4 0.012508 vs 1/80=0.0125; ratio 1.0003 is within MC noise (4e6 samples); formula check E[min]/(8E[sum]) = 0.2/16 = 1/80 for the box is correct. Equality across arms is an identity of weighted-degree normalisation, not an independent finding.
6. Dimension comparison at matched l is misleading; at matched dimension D4 is far better (D4 l=5 dim 19481 reach 0.00824 vs S4e l=3 dim 20475 reach 0.00543). The only real gain is the ~5-7x (-> 8) dimension cut, an engineering constant, and it is conditional on item 2.
7. Housekeeping: reach_raw.txt is polluted with lit2026 output (concatenated) and lacks l=6; lit2026 never cross-matched.

## Verdict
Reach <= 1/80 (<<0.169) conclusion is not contradicted, but it is not established for D4 (the invariant-variable smallness is unjustified), and the pre-registered criterion is not met. Corrected outcome: inconclusive (engineering at best; dimension saving unvalidated; no lever shown). Not "refutes": nothing shows D4 exceeds 1/80.
