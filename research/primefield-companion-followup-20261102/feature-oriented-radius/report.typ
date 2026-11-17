#set page(paper:"a4",margin:16mm)
#set text(size:10pt)
= Oriented mode relations across radii
T-1927 · Prime fields · 2026-11-08

The new adapter retains relative orientation that separate mode products discard. It reuses existing powered sections or paired-stream factors and adds a scalar-cancelling constraint without another character query for the same radius.

#image("flow.svg",width:100%)

*Normalization.* The observed mode satisfies fplus(b)=m b^(h−1) k F₁(b⁻ᵐY), Y=kᵐ. Since b^(2h)=1, normalize n_b=bχ(b)fplus(b)=mkF₁. Then n_a F₁(b⁻ᵐY)−n_b F₁(a⁻ᵐY)=0. The implementation reuses χ(b), with no extra exponentiation or inversion.

*Safe anchors.* Establish an anchor only when both measured modes are nonzero; the existing product constraint then certifies nonzero anchor F₁. Zero observations impose the corresponding zero section. Store the anchor in the common W chart and reduce it as support shrinks. Earlier non-anchor observations are not retroactively correlated.

*Qualification.* 23,552 exact direct-sign support comparisons and predecessor inclusions pass. In 4,024 checked states, cross-radius correlation makes support strictly smaller even than the zero-only oracle. 1,824 states have delayed anchors. All 3,600 public cases recover, including 228 odd-L cases. Among 3,372 predecessor comparisons, 440 routes and 400 call totals change. These are deterministic diagnostics, not timing rankings. PCC-20261108-704e15 is unsigned producer evidence.

*Custody.* Validation01 type-name collision is preserved. Validation02 passes; validation03 replaces its extra exponentiation with reused χ(b) and produces identical focused/inherited output. 146 copied files and 162 dependencies are hash-bound. Limits90/60/60 seconds; independent signed review pending.

*Costs and next construction.* Anchor storage, copies, normalization, conjugate construction, remainders and filtering remain charged. Initial phase-domain expansion remains. Next apply relative modes before dense expansion or carry them in an implicit multi-radius state.

*Sources.* Exact predecessor signed/transport/powered/balanced modules and their established Fourier identities; NIST DLMF reciprocal basis. No novelty or timing claim.
