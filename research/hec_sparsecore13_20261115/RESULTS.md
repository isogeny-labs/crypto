# Sparse quotient-core kernel and cold integration

Both separately frozen correctness gates passed once. Engineering adaptation of scalar Lanczos from repository N4v2/la2.rs; no novelty, timing or scaling result.

Standalone source e5179bfd111d/checkpoint20261116T002935Z: fresh relation collection reduced314relations to232x47 with2413nonzero entries after81LPeliminations. Sparse Lanczos returned a nonzero exact original-matrix kernel at attempt0. All129reconstructed point logs match the independent dense reference up to one common nonzero scalar; all314original relation residuals pass. Four seeds each at dimensions47,128,512 pass twelve planted rank(n−1) controls. Corrupted original matrices and zero vectors reject. No breakdown occurred in this fixed cohort; no failure-rate estimate is inferred.

Cold source8eadddd88e0d/checkpoint20261116T003102Z: full fresh setup, discovered chart, calibration, alignment, equations, three actual solves, inverse recovery and conic descent passed. The candidate cold binary does not call the dense verifier. Sparse core attempt0; logs for D1/B/calibrator556422/314822/383403 differ in scale from the old basis. Four/two conic descents agree; k183049 verifies on source and destination. k+1source control rejects. All positive stages exit0, expected missing-loader101. No rerun, scientific failure or omitted input. Full raw output in run/ and cold-run/.

## Exact safeguard

AᵀDAx=0 does not imply Ax=0 over a finite field, because isotropic vectors can occur. Every returned nonzero vector is checked against original sparse A; failed or zero vectors trigger a bounded retry, never an accepted result or dense fallback. Candidate storage for this reduced step is O(nnz(A)+rows+columns), not O(rows*columns). Scalar Lanczos can break down; cap32preconditioners, n+5iterations each. No universal termination guarantee is claimed.

## Cost case for the actual objective

Diem's published plane-quartic expected soft-O(q) algorithm offers an exponent advantage over the hyperelliptic genus3 double-large-prime soft-O(q^(4/3)) baseline. These are existing results, not this study's breakthrough. Primary sources: https://jtnb.centre-mersenne.org/articles/10.5802/jtnb.815/ and https://eprint.iacr.org/2004/153.pdf . The prior N4v2 family and implementation are not same-target evidence for this transfer.

Ignoring logarithmic factors ONLY for a conditional budget, let source IC cost Ch*q^(4/3), destination IC Cd*q, and construction+transfer+verification S(q). Tenfold requires

    S(q) + Cd*q <= (Ch/10)*q^(4/3).

This is an admission inequality, not a measured crossover. If S(q) contains an unchanged positive q² term, its ratio to the source grows as q^(2/3), invalidating an eventual asymptotic-gain inference for that implementation. Source/target exhaustive enumerators and verification controls must therefore be charged, replaced by exact efficient checks, or explicitly scoped out under a justified protocol before any full-path exponent claim. One cannot simply exclude their time after measurement.

Remaining work: sparse full-row assembly (current full rows still dense), factor-base scaling instead of fixed48, field parameterization beyond127, efficient construction/verification, applicable rational kernels and source-family coverage, followed by frozen same-target charged measurements. Genus2/4 need separate mathematical mechanisms. Full mathematical10xobjective remains open; previous failed coldcost comparison unchanged.
