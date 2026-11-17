#set page(paper: "a4", margin: 19mm)
#set text(size: 10.5pt)
= Monic remainder specialization
Prime-field native source · 2026-11-03

The candidate preserves the product/remainder tree, Karatsuba multiplier and Newton reversal. It starts unit-constant series directly, evaluates linear leaves by Horner, and reconstructs only the low remainder coefficients at internal nodes.

#image("pipeline.svg", width: 100%)

== Correctness and construction
If B is monic of degree d, reversal determines Q as before; A minus QB has degree below d, so its coefficients modulo X^d determine the entire remainder. Only low coefficients of Q and B contribute. The internal quotient still exists; the code avoids full-length remainder reconstruction and an unused returned quotient pair. Linear leaves use A mod(X-x)=A(x). Every tree inverse series starts at1, so this evaluation path calls no field-inverse helper.

Four selected native methods pass once: 84 remainder cases, ten multipoint cases against original and Horner references, series/domain errors, and all eight complete public-point recovery receipts. Sixty prior tests are filtered. No candidate cost or timing comparison ran.

== Comparison readiness
The canonical protocol validator accepts the baseline/candidate/A-A draft with zero findings. It is not frozen: matched instrumentation/runner, binary identity, preflight, analysis and live EKS admission remain to be completed. The original 4563-case cohort is unchanged.

== Evidence and sources
Source locks, reversible routing map, compiler, commands and raw logs accompany receipt.json. See DERIVATION.md for proof and boundaries. The original polynomial implementation and #link("https://flintlib.org/doc/nmod_poly/index.html")[FLINT documentation] supply prior context. Protocol validation failures (system jsonschema version and draft enum values) are preserved; the final validation passes.
