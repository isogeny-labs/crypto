#set page(paper: "a4", margin: 19mm)
#set text(size: 10pt)
= Boolean coordinate encodings: proposed obstruction
2026-10-30 | Theory search continuation | T-1008 | *Unvalidated candidate*

The completed radix census retains all 3,010 partitions of 27: 35 exact Hilbert records and 2,975 lower bounds. Its smallest exact original-top first vanishing degree is 42; every bounded record has lower bound at least 43. This motivates checking whether a broader coordinate-encoding family has the same obstruction. These observations are from `evidence/radix-census-r2/observations.json`; no new experiment ran for this note.

#image("diagram.svg", width: 100%)

== Proposed derivation
Take two disjoint blocks of 27 Boolean selectors and nonconstant coordinate polynomials of degrees d0 and d1. With only the unreduced short-Weierstrass m=2 Semaev equation and the Boolean equations, the proposed top quotient is `(A tensor B)/(F0^2 F1^2)`, where A and B have square-zero generators and Fi is the leading coordinate form.

Write `h(j)=binomial(27,j)`, zero outside the block's support. In bidegree (27,14), the first block multiplication rank is at most 1 and the second at most `h(14-2d1) <= h(12)`. Hence the proposed quotient dimension bound is `h(14)-h(12)>0`. Nonvanishing in total degree 41 would imply first vanishing at least 42. If either coordinate has degree at least two, orient it second and use (27,15), yielding the proposed stronger bound 43.

== Scope and next dataset
This concerns ordinary highest forms taken before any Boolean reduction. Added equations, auxiliary variables, overlapping blocks, constant coordinates and different preprocessing need separate arguments. It gives no affine FFD/LFD statement. Factor-base eligibility still needs actual domain/cardinality and curve-lift checks.

Before production: freeze a distinct protocol; build native small-block matrices including zero-factor and bad-characteristic controls; retain all 729 multilinear degree pairs in 1..27 with exact witness dimensions. Check the universal polynomial/tensor/no-gap bridge independently and formalize it. A finite arithmetic certificate alone would not prove that bridge. No new bound or formal proof is claimed here.

== Sources and reproducibility
Full derivation and planned file structure: `BOOLEAN_ENCODING_CANDIDATE.md`. Prior methods: `HILBERT_DESIGN.md`, `RADIX_PRESENTATION_DESIGN.md`. The higher-m route in `PRIME_LEFSCHETZ_CANDIDATE.md` remains separate. Primary source rechecked: Lundqvist–Nicklasson, Section 2, Theorem 2.2 and Remark 2.3, `https://arxiv.org/html/1604.06820`. It supports linear-form block ranks, not the arbitrary-form argument.

Diagram source is the editable `diagram.svg`. Render with the retained Typst toolchain via `typst.compile('report.typ', output='report.pdf')`; inspect every rendered page. Existing empirical result tables and canonical plots are unchanged.
