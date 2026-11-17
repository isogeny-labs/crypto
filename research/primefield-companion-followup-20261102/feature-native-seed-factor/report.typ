#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Factoring seed dependence out of the source
Prime-field construction · 2026-11-03 · T-1573

The source ratio is a monomial in the nonzero normalization seed: R(c)=c^E R(1). A separate native candidate evaluates the seed-free residual and applies the seed numerator and denominator only at the final ratio. The constructor computes E from public parameters.

#image("pipeline.svg", width: 100%)

== Derived exponent
Let t=(r+1)/2 and G(n)=n(n-1)(n+1)/3. Baby exponents are t²+j(j+1); center exponents are t²+w(w+1)+2wi+4w²i(i+1). Their sums B and A, and the prefix sum H, give E=2wA+2kB+2s(s+e)²+2H+et². DERIVATION.md gives the recurrence induction and domain. Reduction is modulo p-1; divide the integer factor by3 before modular multiplication.

== Checked construction
One optimized native build and one bounded invocation pass all192 methods. The closed formula matches a separate symbolic recurrence in2545 cases.144 exact scalar-ratio cross-products and rescaled-pair checks pass on the three saved curves. All8 complete recovery receipts match the prior source. The raw scalar function and recovery code are byte-identical. The former pair consumer remains test-only. Exponents are312 (p353/r359),484 (p509/r479),556 (p701/r683).

== Cost and scope
Two final seed powers replace distributed seed scaling. Residual rational denominators, source geometry, direct phases, stored seed pairs and decoder terms remain. No comparative panel or cloud job ran. Full operation accounting of the candidate is next; local algebra alone does not determine complete cost. The original4563-case cohort and old panel identities remain unchanged.

== Evidence and sources
See source-lock.json, control.json, receipt.json, build.log and test.log. Standard context: #link("https://www.hyperelliptic.org/EFD/g1p/index.html")[EFD] and #link("https://leanprover-community.github.io/mathlib4_docs/Mathlib/NumberTheory/EllipticDivisibilitySequence.html")[Mathlib EDS documentation], read2026-11-03. The related ePrint2008/444 PDF returned403 and was not used for a claim. No novelty claim for monomial factoring is made.
