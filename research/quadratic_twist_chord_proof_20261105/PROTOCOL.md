# Chord algebra proof attempt, 2026-11-05

User checkpoint approval now covers routine checkpoints for this continuing twist workstream. Use the unchanged official uploader; no arbitrary assets, paid compute or broad credential reuse. This is an actual proof implementation follow-on to merged PR3450, preserving merged worktrees and prior evidence.

First bounded task: pinned Lean elaboration of curve-line factorization, cubic division/remainder, distinct-root membership and the repeated-root discriminant condition. These are proposed statements until elaborated; no actual point-group coverage or finite census is inferred. [pcc:unverified]

Source Chord.lean is filed before its first build. Mathlib d13f23b723b8a846827a245b89c10fc7d3f11612 / Lean4.34.1 from existing local resources. Run through the existing pinned lake environment; use at most two local workers,600s per proof/build and3600s total scientific window. Preserve exact failed sources/logs; source corrections require a fresh checkpoint before affected builds. No sorry, axioms, native_decide, proof-gate patch or scalar oracle. Print transitive axioms. Independent group/lift proofs, source-bound polynomial certificates, Rust/C++ census and signed PCC review remain separate requirements.

Primary pass: all five exact declarations elaborate and their axiom output contains only the allowed foundational axioms. Any compilation error is a failed attempt, not a weaker theorem. This component does not claim useful yield, intrinsic regularity, DLP recovery or speed. [pcc:unverified]
