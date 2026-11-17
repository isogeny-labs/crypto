#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Compact scalar Schur unwind frames
2026-11-02 · Derived construction / static checks · Numerical trials: zero

*Identity.* Write B in blocks (a, tᵀ; c, D), with a nonzero and
S=D−c a⁻¹tᵀ. For Sz=h, the primal head is a⁻¹(b₀−tᵀz).
For Sᵀv=0 and vᵀh=1, the dual head is −a⁻¹cᵀv. The cached borders
already contain t and c. Undo the existing column or row swap respectively.

#image("pipeline.svg",width:100%)

*Implementation.* A separate solver captures a frame before releasing the
setup cache. The frame stores two immutable border tails, a scalar inverse,
one RHS entry, permutation indices, a shared meter and a small profile.
It holds no source, Schur object, generators, full RHS or callback. It checks
input shape/canonical entries, reserves one-use ownership, computes the lift,
and clears field-value storage after success. Failed arithmetic after reservation
cannot be retried through the same frame.

*Derived per-node schedule.* Let n≥2 and w be transported generator width;
ell is one for nonzero lift input and zero otherwise. Search, compression,
child work, common setup arithmetic and root verification remain excluded.
#table(columns:(2fr,1fr,1fr),inset:4pt,
 [Quantity],[Prior route],[Compact frame],
 [Lift source applications],[ell],[0],
 [New dot field products],[0],[n−1],
 [New dot field additions],[0],[n−1],
 [Common scalar products / subtractions],[1 / 1],[1 / 1],
 [Setup + RHS + lift source applications],[3+ell],[3])

The reference skips full queries on zero lift inputs, whereas the new dot
still charges n−1 products and additions. No runtime dominance follows.
Saved instrumented native products are ell(4w+8); native and field products
are separate units. Capture additionally copies 2(n−1) border entries.

*Storage.* A live frame has exactly 2n field-value slots. Complete scalar
descent retains N(N+1)−2 such slots before unwind: still quadratic. Frames
no longer keep source generators or full parent RHS vectors alive. Temporary
copies, current operators, object overhead, meter blocks, certificates and
trace history are outside that bound. No measured heap reduction is claimed.

*Checks and provenance.* Complete normalized AST equality against the preserved
border solver; frame slots and absence of numerical source calls checked
statically. Borders/inverse are trusted internal provenance, not independently
certified. Root solution/witness verification remains unchanged. No numerical,
corruption-control, memory or performance experiment was run.

*Next validation.* Full upstream cohort; both lift directions; permutations;
zero/nonzero tails; singular terminals; malformed and consumed frames;
corrupted borders/inverses; independent counters and heap ownership. Existing
cost gate remains in force. Next construction: coordinate extraction without
retaining all border tails, with explicit recomputation accounting.

*Sources.* compact.py, solve\_compact.py, check\_preparation.py and DERIVATION.md.
Bostan et al. (2017), Sections 6.2–6.3, DOI 10.1137/16M1062855, provides the
established structured Schur context; no fast randomized bound is inherited.
