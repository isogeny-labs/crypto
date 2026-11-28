# Omitting zero-exponent direct factors

Assume the inherited domain: nonsingular short Weierstrass curve over caller-
certified prime p>3; caller-certified odd prime subgroup order r>=7, r!=p,
gcd(r,p-1)=1; nonidentity Q in that subgroup. The phase routine checks psi_r(Q)=0
and nonzero derivative before constructing its nonzero normalized seed c.
It does not certify primality of r. Identity and anchors remain separate.

The indices used by the monomial are1,(r+1)/2,2w and optionally t+e. Here
h=(r-1)/2=2wk+2t+e, w<=h/2,0<=t<w,e in{0,1}. All selected indices n obey
0<n<r. Since Q has prime order r, [n]Q is not the identity. By the division-
polynomial torsion criterion, psi_n(Q) is nonzero; the even-index2y factor is
nonzero because Q cannot have order two. Thus T_n(Q)=psi_n(Q)c^(n*n) is nonzero.
A factor with projected exponent zero contributes1 and its direct evaluation
can be omitted on this domain. [pcc:unverified]

The implementation preserves the phase check at base and direct nonanchor
initialization. The twice-base anchor is a valid nonidentity subgroup point;
its seed c2=c^4*2*y(P) is nonzero. These are the private values() call sites.
When every projected exponent is zero, values() returns[1,1] without constructing
the value-only engine. Otherwise it skips zero exponents before requesting
value(n) or computing its seed normalization. Remaining factor checks stay.
The public paired wrapper still executes the full-value source and original
ratio validator, including its domain rejection. [pcc:unverified]

This omission is restricted to direct factor evaluation. All coordinate-
difference checks, geometry lanes, anchors and adaptive limits are unchanged.
The phase engine still computes the subgroup-index derivative; retained division
requests may recursively compute other indices. We therefore count top-level
requests, not eliminated internal arithmetic, memory, or elapsed time.

Three selected native methods pass: existing full-source agreement over499
complete-subgroup rows,100 splits,1375 advances and450 skips; limits/rejections;
and an instrumented direct-factor test. That test sees1/0/2 requests from3/4/4
available factors over F5/r7,F7/r13,F509/r479, with no value-only engine created
in the middle case. Evidence: PCC-20261107-7d2ab2, unsigned producer records.
This is development validation, not an independently reviewed performance panel.
[pcc:unverified]

Next: investigate whether seed dependence cancels algebraically in the retained
monomial, while retaining an adequate subgroup/domain validator. The full geometry
residual and reference validation remain major work. Complete-cost measurement,
scalar recovery and the original4563-case cohort have not been replaced by these
development checks.

Sources checked: predecessor projected transport and factor derivation, native
phase() in feature-native-division/lib.rs, and Sage reference documentation for
division_polynomial and division_polynomial_0:
https://doc.sagemath.org/html/en/reference/arithmetic_curves/sage/schemes/elliptic_curves/ell_generic.html
(read2026-11-07). The documentation distinguishes even-index two-torsion factors;
the odd-prime subgroup excludes that exception. KB unavailable in current tools;
the prior repository wrapper check reported a missing executable. No novelty
claim; this is a domain-specific use of the standard torsion criterion.
