#set page(paper:"a4",margin:16mm)
#set text(size:10pt)
= One-stream reciprocal signed-mode filter
T-1896 · Prime-field construction · 2026-11-08

The new filter pairs weighted modes +1 and −1 from an existing public sign orbit. Reciprocal coefficient symmetry turns their product into a norm in W=V+1/V, evaluated using only the mode-1 coefficient stream. It reuses the existing quadratic quotient implementation.

#image("flow.svg",width:100%)

*Derivation.* Let h=(r−1)/2 and m≥3 divide h. With L=h/m and σ=(−1)^h, coefficient reversal gives F[m−1](V)=σ V^(L−1) F[1](1/V). For nonzero x, V=x^m and ε=x^h, the public weighted product equals m²σε N(W). If F[1](V)=a(W)+Vb(W), then N=a²+Wab+b². The filter returns gcd(g, m²σεN−f[1]f[m−1]). See DERIVATION.md for all assumptions and proofs.

*Qualification.* Seven parameter pairs, three block caps and three nonzero radii cover 6,633 observation cases, 13,812 quotient equalities and 672 support comparisons, including 279 zero-observation cases. Dense reference coefficients and scalar enumeration exist only in the correctness fixtures. Lean checks the norm identity over an arbitrary commutative ring, conditional on inverse and trace equations. The weighted projection and runtime refinement are not yet formalized. Producer evidence awaits independent signed review.

*Costs and exceptions.* Removes the second selected coefficient stream for this constraint; transition inversions, interval construction, quotient products, gcds and weighted observation arithmetic remain. The inverse chart excludes x=0. A zero product can retain both branches. W does not determine orientation. No recovery adapter or automatic method-selection policy changes here, and no timing or total-work comparison ran.

*Failures retained.* Two compile attempts failed because copied entry-point names collided. The third build and focused checks passed. Historical full-Cargo missing-fixture limitation is unchanged.

*Next.* Integrate with canonical reciprocal fibers and their phase/orientation state, then assess candidate retention and full cost under a common frozen comparison. Original cohort and cloud state are unchanged.

*Sources.* Existing triple-mode quadratic.rs, reciprocal.rs and canonical.rs; NIST DLMF18.5.1, https://dlmf.nist.gov/18.5.E1. KB unavailable (wrapper has no executable). No novelty claim.
