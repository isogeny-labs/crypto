# R70 Lean bridge, frozen before compile

Four conditional theorems in `lean/Frobenius70.lean`: a generic nonzero
two-by-two minor kills the two-coordinate kernel; the actual saved R70
rows `(1,0)` and `(1,1)` instantiate a stronger field-generic lemma; a
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

## Version 2 repair, frozen before the second Lean attempt

`frobenius-lean70-01` failed compilation: `Field (ZMod 2)` was not
synthesized at the concrete-row lemma, and the final set-inclusion
direction needed an explicit element argument. The new row theorem is
field-generic, which is stronger than the binary application; the set
proof applies the recorded inclusion to the element. Preserve the first
raw compile failure. Fresh `frobenius-lean70-02` uses the same pinned
toolchain and separately audited compile result.

## Version 3 direct normalization implication

The accepted native zero-kernel result identifies `V6` with the old
order. Add definitions of the actual square-preimage chain and prove
inductively that any square-closed set inside `V0` remains inside every
step. A six-step equality `V6=O`, together with the trace-dual
containment of the integral closure, therefore proves `O` is its
integral closure. This avoids needing a formalization of the full
Singh--Swanson fixed-point theorem for this particular endpoint.
The native `V6=O`, source-field/order identities, and trace-dual
containment are explicit premises; Lean does not verify their CAS
payloads. Fresh `frobenius-lean70-03` is frozen before compilation.
