# R70 Lean bridge, frozen before compile

Four conditional theorems in `lean/Frobenius70.lean`: a generic nonzero
two-by-two minor kills the two-coordinate kernel; the actual saved R70
rows `(1,0)` and `(1,1)` instantiate that lemma over `ZMod 2`; a
square-stable set contains every iterated square; and a set between the
integral closure and a conductor over-lattice equals the closure if the
all-iterates integrality criterion is supplied. The criterion itself is
the Singh--Swanson theorem and is an explicit premise, not a Lean proof
of the paper or a kernel check of the native source model.

Pinned Lean 4.34.1/mathlib d13f23b, CPU0, `-j1 -M4096`, 1200 seconds
separately for compile and audit, no virtual cap. One mathematical job at
a time. No sorry, axioms, `native_decide`, or unsound evaluation. Fresh
attempt `frobenius-lean70-01` after source freeze. Keep all failed
compiles/audits; repair under a new source revision and attempt ID.
