#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Omitting zero-exponent direct factors
2026-11-07 | PCC-20261107-7d2ab2 | Native development

#image("flow.svg", width: 100%)

*Domain argument.* A nonidentity point of prime order r cannot be n-torsion for $0 < n < r$. Every selected division index lies in this interval; the normalized seed is nonzero. Hence a zero-exponent factor is nonzero and contributes one without evaluation. Primality remains a caller precondition. Phase and full-value checks are retained. [pcc:unverified]

*Implementation.* Skip zero exponents before division-value and seed-power requests. If every exponent is zero, return the unit pair without constructing a value-only division engine. Geometry lanes and coordinate-difference checks are unchanged.

*Development evidence.* Three selected native methods pass. Complete/split checks retain 499 subgroup rows, 100 splits, 1375 advances and 450 skips. Direct top-level factor requests are 1, 0 and 2 from 3, 4 and 4 available factors over F5/r7, F7/r13 and F509/r479. The all-zero case constructs no value-only engine. Claims are unsigned producer evidence; see receipt.json and test/limits/factors.log. [pcc:unverified]

*Cost boundary.* Phase computation still runs, and retained requests can recursively evaluate other indices. The paired full-value source remains mandatory. No scalar recovery, timing comparison or cloud experiment ran. The original cohort remains intact.

*Next.* Analyze retained seed dependence and projected geometry residuals, then measure complete work.

Full argument and exact fixtures: DERIVATION.md and inherited projected-transport report. Primary source: Sage division-polynomial documentation, https://doc.sagemath.org/html/en/reference/arithmetic_curves/sage/schemes/elliptic_curves/ell_generic.html. KB unavailable. No novelty-priority claim.
