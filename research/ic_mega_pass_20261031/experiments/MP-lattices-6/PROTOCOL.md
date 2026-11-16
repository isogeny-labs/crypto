# MP-lattices-6 protocol (pre-registered 2026-10-31, before any run)
Scope: accounting/theory only. Tests the determinant-exponent (Minkowski existence) reach of the
(p,f)^l shift-lattice family over monomial shapes. It does NOT test a ceiling over all auxiliary
polynomials (untested by design), nor the multivariable capacity converse.
Method (exact, fractions; no primes, no LLL for main result):
  For downset S, lead monomial L=(d,..,d) of S_{m+1} (deg d=2^{m-1} per var), k(s)=min(l, max k: s-kL>=0).
  delta(S,l) = sum_s k(s) / sum_s deg(s)  [= (l - e/N)/avg_deg]. l up to 400, extrapolate in 1/l (linear fit on last points).
Shapes: total-degree simplex, box [0,dl+t]^m (t=0..), weighted simplices, downset hill-climb (m=2), and
  S_m-symmetric orbit lattice (one vector per orbit) for m=4 (D4/sym proxy for lever 3).
Controls: univariate deg d -> 1/d (d=2..5); bivariate linear (lead x, Newt simplex) -> 1/2 ; box m=2,3,4 -> 1/6,1/24,1/80.
Null arm: exponent depends only on lead monomial/support => random f of same support gives identical exponents (by construction; stated, not a test).
Sanity: S3(x1,x2,x3fixed) mod p squarefree for p in {1009,4093,10007}, seed 0.
Decision rule (gap-free): SUPPORTS iff extrapolated sup-over-shapes delta(m=2) within 0.003 of 1/6 and controls within 0.003.
  COUNTEREXAMPLE iff some valid shape gives extrapolated delta > 1/6+0.005 with controls passing. Else INCONCLUSIVE.
Primary deliverable: m=4 sup delta (incl. symmetric orbit lattice) vs 1/80 and vs needed 0.169.
