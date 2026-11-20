# R78 chart and repeated-factor Lean proofs, frozen before archival run

Compile `lean/FullU078.lean` with pinned Lean 4.34.1 and mathlib
`d13f23b723b8a846827a245b89c10fc7d3f11612`, CPU 0, one Lean
job, `-j1 -M4096`, separate 1200-second compile and axiom audit and
no virtual-memory cap. Copy and verify the source, launcher, driver,
protocol and audit tool against frozen Git bytes. Require both exits
zero, three named authored theorems, zero failing and zero trusted.
Retain every rejected attempt.

The chart theorem derives `U=v` from `u*z=1` and `V=z^2 v`. The
characteristic-two factor theorem proves
`(lambda^2+lambda+1)^8=lambda^16+lambda^8+1`; the arithmetic theorem
checks `16+16=32`. The R50 equation and native specialization are
inputs outside Lean. Repeated model-fiber powers do not establish
normal branch ramification. Interactive compilation before this
freeze was proof preflight, not the archival axiom audit.
