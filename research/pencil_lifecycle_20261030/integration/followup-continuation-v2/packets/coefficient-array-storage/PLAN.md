# Coefficient array storage binding
Task T-1163. No scientific campaign.
Prove array size and untouched-slot preservation, canonicality preservation, and generated coefficient update decoding at each slot. Bind to transform iter_mut enumerate take(k+1), with bounds from j<=k<length. This packet is the storage step toward full nested-loop composition, not that full theorem.
Sources: existing Loop.lean, WordCoefficient.lean, Inversion.lean; source projective_law_gate.rs transform. Lean official Array.set semantics https://lean-lang.org/doc/api/Init/Data/Array/Set.html and arrays reference. Use standard bounds-checked Array.set, no novelty claim.
Controls: first/middle/last index, untouched slots, canonical zeros, actual generated update and excluded out-of-range index. Separate axiom readback. Preserve all failed attempts. Independent fidelity review pending.
