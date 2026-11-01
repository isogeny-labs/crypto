# R42 primitive least-scalar bridge, frozen before compilation

Use the accepted base CMRecovery42 object, source comparison, pinned
Lean4.34.1/mathlibd13f23b, -j1 -M4096 and 1200s compile/audit caps.
The generic scalar_divisibility lemma proves the primitive-coordinate
divisibility implication over integers for any finite coordinate vector.
The native full matrix calculation must separately establish nonsingularity,
primitive coordinates and proportionality for competing solutions.
This formalizes the last implication, not the concrete matrices, their
determinants, a universal Lucas formula or the whole modular Hom lattice.
One declaration expected, audit zero failing/trusted. Preserve failures.
