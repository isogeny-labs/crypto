# Nonzero Kummer comparator semantics

Six checked field declarations connect the literal sixteen cross-product equations to projective proportionality. For two nonzero vectors, the comparator holds exactly when one is a nonzero scalar multiple of the other. Symmetry and transitivity on that domain are proved. Eight controls and fourteen separate axiom readbacks passed.

The pivot proof chooses a nonzero coordinate of the first vector and recovers the scalar as the corresponding coordinate ratio. Nonzeroness of the second vector forces that scalar to be nonzero. The resulting equivalence supplies a source-specific meaning for same/same_k once its machine residue and nonzero-vector premises are established.

The raw comparator also matches the zero vector to every vector. Controls show this destroys transitivity if zero vectors are admitted: two distinct coordinate axes each match zero but do not match one another. Additional controls exercise positive and negative scales, a last-coordinate pivot and the impossibility of a nonzero scalar taking a nonzero vector to zero.

Source basis: retained walk_impl_uncached.rs same lines51-53, pruned.rs same_k wrapper, prior signed-pair report and its primary CCS quotient/recovery context. This proof concerns vectors over a field. It does not establish concrete Kummer projection fibers or curve membership. Canonical machine words and modular-operation correspondence are needed to identify raw nonzero checks with decoded nonzero vectors; native zero tests alone do not provide that bridge for noncanonical residues.

Failed attempts comparator1 and controls1 remain archived. Final comparator2, controls2 and readback1 pass with unchanged input bindings. Readbacks use only propext, Classical.choice and Quot.sound. Imported compiled dependencies remain a trust boundary. Independent source review is pending.

No scientific execution or target enumeration occurred. Full cohort scope remains 1,260 targets, and complete cost ratios remain null.
