#set page(paper: "a4", margin: 16mm)
#set text(size: 10pt)
#set heading(numbering: none)
= R63: degree 112 source and half-space descent
The actual marked-23 source has degree 112 over F2(x). Its 112 standard
monomials and 124 complete source functions have square pairs built from
two checked generator formulas and derivative inverses. A fresh source
replay binds all 124 saved inputs and roots, both generator bundles,
both inverse products, and altered-inverse/root controls.

#image("stages63.svg", width: 100%)
Each bar counts a different exact unit. The two 124-function checks are
producer and fresh replay of the same cohort, not independent functions.

== Terminal flow
#table(columns: (2fr,1fr,1.2fr), inset: 4pt,
[Attempt], [Status], [Native wall seconds],
[check112‑01], [accepted], [56],
[generator112‑01], [accepted], [110],
[half‑descent63‑01], [rejected], [4],
[half‑descent63‑02], [accepted], [32],
[half‑lean63‑01], [accepted], [—],
)
The first half-space attempt is rejected despite a late marker: an
unexported ambient constant caused Singular question-mark errors. Version 5
reconstructs the constant from retained R61 matrices; attempt 02 passes.
Wall intervals are receipt timing, not a performance comparison.

== Accepted half-space restriction
#image("half-spaces63.svg", width: 100%)
The mixed space has fixed dimension 6 and trace rank 4. The cubic space has
fixed dimension 64 and trace rank 64. Both contain an actual function with
source involution trace one. The 10 mixed roots lie inside the 128 cubic
roots; the two dimensions do not add.

#pagebreak()
= Exact checks and next geometry
The half-space run binds all 138 saved roots to the actual R61 source and
verifies $g^2=f$ for each. It checks 20 mixed and 256 cubic root/square
involution identities, all 70 fixed roots, and both trace-one functions
in the full source quotient. Each restricted matrix has a checked left
inverse, intertwines the ambient action, squares to the identity, and
rejects an altered action.

The pinned Lean 4.34.1/mathlib d13f23b audit compiles five authored
conditional implications: square-root uniqueness, fixedness of a root
over a fixed square, integer pole-cap halving, restriction of an
involution, and a trace separator. The audit reports zero failing and
zero trusted native decisions. The actual source equations are checked
by the native field calculations, and proper pole valuations still need
their own construction.

== New normalization route
The retained rank-32 trace Gram discriminant predicts a quotient of
F2 dimension 160 between the trace-dual lattice and the affine order.
The Singh–Swanson descending Frobenius algorithm turns each normalization
step into a complete binary kernel on that finite quotient. Equality is
an exact affine stopping certificate after the source-field bridge is
verified. The old PID variable is z=1/u relative to the actual FIELD61
coordinate. The quadratic equations match under V=z²v; the degree-16
equation and all 32 basis functions require explicit reduction. See
R64/NORMALIZATION-LEAD.md for the source-bound algorithm and proof scope.

Proper normalization, branch valuations at zero/one/infinity, complete
Riemann–Roch spaces, the level-77 map and order-11 action, quotient
saturation, arbitrary reverse divisors, node187, and selected-991 factor
action remain separate exact construction tasks.

== Reproduction
Five immutable raw attempts; four accepted and one rejected. The 29
historical source byte pins and every terminal receipt are in results.json.
Singular used seed 63, one thread, 1200 seconds and 4 GiB virtual memory
per run; Lean compile and axiom audit had separate 1200-second caps and
the pinned toolchain. Raw archive custody is recorded separately.

Primary normalization source:
#link("https://www.math.purdue.edu/~iswanso/intclSinghSwanson.pdf")[Singh–Swanson, Theorem 1.1].
