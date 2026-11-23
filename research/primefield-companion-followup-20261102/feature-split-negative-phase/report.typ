#set page(paper:"a4",margin:16mm)
#set text(size:10pt)
= Bounded negative-phase splitting
T-1916 · Optional prime-field recovery initializer · 2026-11-08

For even n and negative normalized character, a checked public eighth-root trace splits the degree-n W domain into two disjoint degree-n/2 factors. Each factor retains both original quartic phase labels through separate canonical branches. Root-search exhaustion preserves the unsplit route.

#image("flow.svg",width:100%)

*Construction.* A candidate η passes η⁴=−1; then η⁻¹=−η³ and s=η−η³ has s²=2, which is checked explicitly. At most 64 public candidates are tried. The standard identity $D_n = D_(n/2)^2 - 2$ yields two coprime factors. Runtime does not enumerate target scalars or issue a new character query for this constant.

*State invariant.* Each old u=±i branch becomes two branches with the same u. Their lifted supports multiply to the old phase support. Subsequent intersections preserve the disjoint partition. Qualification groups by u; a one-to-one canonical-signature comparison would be invalid here.

*Capacity and cost.* Successful splitting admits logical section slots at m·n rather than 2m·n for negative phases. The bound is per branch, not total RAM. Root search, additional canonical validation and phase setup, four branch vectors, allocations, quotient work, source extraction, replay and point verification remain charged. Root setup is recomputed per eligible initialization. No timing or total-work ranking.

*Sources and next construction.* NIST DLMF18.5.1 and predecessor direct-phase/canonical code. KB unavailable; exact-source fallback recorded. Next examine sharing the two V orientations in one character-phase branch per W factor. That requires changing both the lift exponent and transported character label; it is not implemented here.


*Qualification.* 20,352 state comparisons and 20,352 grouped phase-support checks pass. All 3,372 public target/capacity cases retain outcomes, routes and recorded source statistics. 208 coprime factorization checks, seven field controls and five split-capacity boundary groups pass; 281 zero-search public cases cover fallback. These are deterministic fixtures, not timing results. PCC-20261108-5e51c6 is unsigned producer evidence; independent review pending.

*Custody.* Both revisions pass build and inherited regression; earlier changed sources are preserved. 127 copied-source files and 142 compiler dependencies are hash-bound. No compiler/native failure or cloud run. A character-phase formulation may also admit odd L=h/m cases without a fourth-root search, but that admission/lift/transport change is not implemented here.
