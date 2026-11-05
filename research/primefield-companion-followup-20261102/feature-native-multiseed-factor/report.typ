#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Normalized transport feeds character evaluation
2026-11-05 · T-1712

The new consumer factors the source residual into a geometry-only ratio and a monomial in four initial values: normalization seed c, half-index normalized value u, stride value v, and optional tail value z. This lets the residual consume transported homogeneous columns directly. [pcc:unverified]

#image("pipeline.svg", width: 100%)

== Exact program factorization
Write h=(r-1)/2=2wk+2s+e and G(n)=n(n-1)(n+1)/3. The exponents of (c,u,v,z) are:

- delta = 2w(kw(w+1)+wk(k-1)) + 2kG(w) + 2G(s),
- alpha = 4wk+2s+e,
- beta = 2wG(k),
- gamma = 2s.

Baby recurrence vectors are (j(j+1),1,0,0); center vectors are (w(w+1)+2wi,1,i(i+1),0). Accumulator and tail exponents give the formulas. Nonzero operands and the inherited phase/seed-law assumptions remain explicit. Integer division precedes reduction modulo p-1. See DERIVATION.md. [pcc:unverified]

== Construction and checks
The seed uses existing public point shifts; no multiplier lane is added. The same geometry window supplies residual grids and transport. Initial phase/value contexts are shared. Calibration uses homogeneous cross-products. Residual recurrences and resultants remain costs. [pcc:unverified]

Thirty-five native tests pass: 2,545 exponent-vector cases, 288 arbitrary scaled-input ratios, 499 complete-subgroup characters/ratios over F5/r7, F7/r13 and F509/r479, and 176 shifted-window characters. Actual phase and value calls match at most two direct initializations per window. Native/checker dependency closures contain 14/15 hashes. [pcc:unverified]

The separate frozen scalar-source checker accepts all 499 rows and rejects a changed numerator. It imports no new consumer or transport, but was written in the producing session. Its initial type-inference build failure is retained. Five native warnings remain. [pcc:unverified]

== Next construction and evidence
Retain the last two columns and projective geometry tail in a consuming cursor, preserving lazy skipped-character behavior for quartic decoding. Account for seed updates, final powers, grids, resultants and allocations. No timing or cloud panel; original 4,563-case cohort unchanged.

Claims PCC-20261105-70dedf and PCC-20261105-37f140 are unsigned; local verdict not_covered. See raw logs and receipts. Sources: saved scalar source, seed-factor and transport notes; #link("https://dlmf.nist.gov/23.10.E3")[DLMF23.10.3] for the transport antecedent. KB unavailable (MCP absent, wrapper executable missing). No novelty-priority claim.
