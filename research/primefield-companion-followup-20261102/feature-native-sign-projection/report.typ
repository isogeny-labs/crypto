#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Sign-preserving multiplicative projection
2026-11-07 | Native development construction

#image("flow.svg", width: 100%)

Write p-1=q m, q a power of two and m odd. Raising nonzero field values to m preserves multiplication and both signs. Its image has order q. Every multiplicative quotient preserving minus one must have image order divisible by q. These are derived statements, not a formal proof. [pcc:unverified]

For the existing monomial, delta and beta are multiples of four and gamma is even. At q=2 only the parity-selected T factor remains. At q=4 the seed and doubled-width factor disappear. This is an algebraic opportunity; the runtime still needs projected geometry residuals. [pcc:unverified]

*Observed development checks.* Five native Rust tests pass: homomorphism, recurrence commutation, power-map image-order minimality, domain rejection and explicit sign/validity examples. Source, commands and hashes: lib.rs, receipt.json, test.log. These are producer checks; no independent review or performance experiment. [pcc:unverified]

*Boundary.* In F7, the non-sign 2 projects to 1. In F13 the quadratic character loses minus one, while the two-primary projection preserves it. Addition does not commute: in F7 the image of 1+1 differs from the sum of images. The API returns a sign candidate, not a validity certificate. [pcc:unverified]

*Next.* Integrate a projected cursor alongside the full-value reference; retain validity checking, then account for projection and residual costs. No scalar recovery was run here; the original cohort remains intact.

Full derivation and scope: DERIVATION.md. Primary source: Andrew Sutherland, MIT 18.782 Lecture 3 (2013), sections 3.1-3.2, https://math.mit.edu/classes/18.782/2013fa/LectureNotes3.pdf. The cyclic-group projection is standard; no novelty claim. KB unavailable in this runtime.
