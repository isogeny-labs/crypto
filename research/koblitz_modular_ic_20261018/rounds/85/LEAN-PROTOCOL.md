# R85 diamond base-overlap Lean identities, frozen before data

Compile `lean/Diamond85.lean` with pinned Lean 4.34.1 and mathlib
`d13f23b723b8a846827a245b89c10fc7d3f11612`, CPU 0,
one Lean job, `-j1 -M4096`, separate 1200-second compile and
axiom audit, and no virtual-memory cap. Copy and verify source,
launcher, driver, protocol and audit tool against frozen Git bytes.
Require both exits zero, three authored theorems, zero failing and
zero trusted. Retain failures.

The theorems prove the two characteristic-two principal-open
polynomial identities and their consequence on the quadratic
source equation. They do not formalize the full degree-32 upper
field or normalized-chart gluing.
