# MP-lattices-6: result
Outcome: SUPPORTS, with narrow scope (accounting). Pre-registered rule: sup over shapes for m=2 within 0.003 of 1/6, and controls pass.
This covers only the determinant-exponent (Minkowski) reach of the (p,f)^l shift-lattice family on downset monomial shapes.
The ceiling over ALL auxiliary polynomials (capacity converse) is UNTESTED. So are two-polynomial independence and non-downset or multi-ideal sets.
Raw output: raw_output.txt, RESULTS.json (run.py, 52 s), RESULTS_closedform.json (m4_closed.py, under 1 s, exact fractions, l=100/200/400 with Richardson extrapolation).
Controls: univariate 1/d for d=2..5 (error under 4e-6); bivariate linear 1/2 (exact); box m=2,3,4: 0.1666656, 0.0416661, 0.0124997, against targets 1/6, 1/24, 1/80.
m=2 shapes (extrapolated): box t=0..4 0.16662-0.16666; total-degree simplex 0.12499 (that is 1/8); weighted simplices 0.11110 (1/9).
Downset hill-climb at l=30 (seed 0, 3000 steps): no improvement over the box (0.16120 = box value).
m=4 (the S_5 analogue, primary deliverable): box 0.0124997; S_4-symmetric orbit lattice 0.0124997. Both are 1/80. Symmetrisation changes only the finite-l bias, not the limit.
The needed 0.169 is out of reach by a factor of about 13.5.
Null arm: the exponent depends only on the lead monomial (d,..,d). A random f with the same support gives identical numbers by construction. No S_3-specific effect is possible in this metric.
Caveats: the run.py m=4 extrapolation at l<=4 is biased (0.0101); the closed form supersedes it.
The squarefree sanity check was not run: sympy factor_list raised NotImplementedError for multivariate polynomials mod p.
The D4 region in e-variables was not built (symmetric_semaev(4) is too large); the symmetric-orbit lattice was used as a proxy.
Classification: accounting. B*D^2 is unchanged. No route.
Next step: (1) a sectional capacity/converse computation for polynomials vanishing only on box∩V(f) (outside (p,f)^l), e.g. via p-adic interpolation determinants at small p; (2) multi-ideal and non-downset sets with an LP over k(s) assignments.
