# Sparse target-check proof map

The candidate changes target relation verification from a sum over all 2,048 folded columns to a sum over the occupied signed columns. It introduces an implementation result, measured in the paired panel, without changing the group law, pair lookup, subgroup projection, or shifted-target recovery theorem.

| Structural dependency | Checked Lean declaration and hypotheses |
| --- | --- |
| A pair-index hit gives an ordered three-point relation | `H4PairIndex.pair_lookup_iff` in `../../pair_coverage_20261120/formal/PairIndexCoverage.lean`; explicit associative group operations, zero, inverses, subtraction, and base predicate. |
| A cofactor map preserves a three-point relation | `H4CofactorProjection.additive_map_preserves_three_sum` in `../../subgroup_projection_20261120/formal/CofactorProjection.lean`; an additive map and an existing relation. |
| A log of a shifted point gives the original log after subtraction | `H4ShiftedTargetRecovery.recover_from_shift` in `../../n29_full_pipeline_20261120/formal/ShiftedTargetRecovery.lean`; ring action on an additive commutative group and the shifted-point equality. |
| The packed 35-bit index key is injective and separate from identity | `H4PackedPointKey35.key_injective` and `key_lt_sentinel` in `../../n35_packed_pair_20261120/formal/PackedPointKey35.lean`; both coordinates below `2^35`. |

The source computes a three-entry signed representative map from the same witness indices as the dense row, checks its residues against all 2,048 row entries, checks both the original three-point sum and sparse representative sum on the actual curve, then independently replays the scalar on the original target. The independent verifier rebuilds the pair index in a different traversal order and checks these equalities for every measured pair. The exact n35 curve, signed folding map, modular relation row, and Sage point implementation are not imported into Lean. A Lean theorem identifying those concrete objects remains open; the existing checked declarations do not encode the finite 10-pair measurement or its timings.
