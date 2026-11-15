#set page(paper:"a4",margin:17mm)
#set text(font:"Helvetica",size:10pt,fill:rgb("233746"))
#set par(leading:.6em)
#text(size:22pt,weight:"bold")[Rational trigonal construction gate]

Two eligible kernels · Complete archived genus-3 cohort retained · 15 November 2026

*Neither eligible pair kernel admits a trigonal map over its base field.*
Both discriminants are nonsquares. This closes the direct rational Smith
construction on these pair kernels, without excluding other isogenies or descent.

#image("TRIGONAL.svg",width:100%)

#table(columns:(1fr,1fr,1fr,1fr),align:right,
 [Prime / family],[Q(u+t v) coefficients],[Discriminant],[Rational roots],
 [127 / 1],[71,25,30],[106],[0],
 [1009 / 1],[556,129,20],[413],[0])

Coefficients are low-to-high. Both quadratic leading coefficients are nonzero,
so the point at infinity is not a missed root. Complete enumeration of all
p+1 projective-line points agrees with each Legendre-symbol test (−1).

*Construction.* In each even-degree irreducible factor F of degree e, the
unique eligible pairing is x↔τ=x^(p^(e/2)) modulo F. For cubic coefficient
vectors N,D, set pᵢⱼ=nᵢdⱼ−nⱼdᵢ. Expanding
Σ pᵢⱼ(xⁱτʲ−xʲτⁱ)=0 modulo F gives equations over the base field. Add the
rational root/infinity-pair equation. This computes the rational constraint
space without extension-field root coordinates. Both nullspaces have dimension 2.

The decomposable-minor quadric in lexicographic order 01,02,03,12,13,23 is
p₀₁p₂₃−p₀₂p₁₃+p₀₃p₁₂=0. Intersect the complete projective constraint line
with this quadric. Every candidate would reconstruct numerator and denominator,
check all six minors, coprimality, degree three and original pair-value identities.
Here there are no real candidates, so real-map identity checks are vacuous.

*Controls.* Sixty-four independent constructed wedges over 5,7,11,17 satisfy
the quadric and reconstruct exactly. Three planted x³ maps over 13,19,31
survive the same nullspace/line pipeline and satisfy original pair values.
All controls pass; scientific stderr is empty. No curve or smoothness claim
is made for the polynomial controls.

*Limits.* A rational kernel does not guarantee a usable base-field curve map.
Smith’s quadratic-descent possibility is not excluded, and p127/family1 also
has non-pair kernels. No Jacobian isogeny, destination quartic, transported
scalar or runtime gain is asserted. The full genus2/3/4 mathematical 10×
objective remains active. This is an established-method construction gate.

*Custody.* Protocol d9fa1ed2eb preceded source 8c6fad97c906 and checkpoint
20261115T125024Z-8c6fad97c906. Both exact input hashes, nullspace bases,
quadratic coefficients and all output are retained. Native Rust only,
no scientific Python, cloud worker or performance benchmark. Primary source:
Smith, arXiv:0806.2995v2, Section5 and Remark5/Section9. Existing native CI
does not automatically execute this standalone gate.
