# Claim-to-proof map

All new sources belong to this Lake project. Lean4.19.0; Mathlib commit
`c44e0c8ee63ca166450922a373c7409c5d26b00b`. Exact accepted sources and
unchanged reference modules are bound by `verification/inputs.sha256`.
The whole transitive auditor's final receipt, not diagnostic prints from failed
builds, determines trust acceptance.

| Claim | Declaration | Exact scope |
|---|---|---|
| Boolean XOR preserves and inverts labels | `Recoding.transform_boolean`, `transform_involution` | any field; inverse requires the retained bits Boolean |
| Both variable-slot systems transport | `Recoding.system_to_original`, `original_to_system` | F17, complete scalar secant system, no fixed column |
| Same offset image | `Recoding.same_offset_set`; `SupportOneHot.offset_image` | original four offsets; recoding F17, one-hot any field |
| XOR upper degree6 | `RecodedTop.ring_reductions`, `finite_cover`, `monomial_high`, `vanishes` | full seven-variable homogeneous ideal; all field parameters |
| XOR lower degree5 witness | `RecodedTop.dual_qx`, `dual_generator`, `dual_ideal`, `witness_not_mem`, `first_vanishing` in RecodedLower | four-term coefficient functional; all field parameters |
| Actual XOR canonical forms | `RecodedCanonical.high_forms`, `canonical_ideal`, `canonical_first_vanishing` | all actual generators; nonzero two graph quadratic coefficients |
| Actual XOR scalar roots | `RecodedCanonical.fixture_zeroes_iff` | concrete F17 coefficients; all target/scalar/selector variables |
| Actual XOR point relations | `FourTrackFixture.xor_polynomial_complete` | exact finite-target Mathlib group law on existing baseX; nonsingular endpoint/target hypotheses; all sign pairs existential |
| One-hot support exact | `SupportOneHot.classification` | exactly000,100,010,001 |
| One-hot full upper/lower degree | `SupportOneHot.high_monomials`, `witness_not_mem`, `first_vanishing` | nine variables; characteristic≠2 for matching lower witness |
| Actual one-hot canonical forms | `SupportOneHot.extra_top`, `canonical_ideal`, `canonical_first_vanishing` | every pair-product constraint included; nonzero quadratic coefficients |
| Concrete degrees6 and5 | `FourTrackFixture.xor_degree_six`, `one_hot_degree_five` | all F17 target coordinates, all nonzero e,f, hence all four signs |
| Symmetric recovery | `SymmetricChord.system_iff`, `recover_original` | any field with2≠0; arbitrary graph and sign coefficients |
| Complete rational pivot charts | `SymmetricChord.complete_charts` | any field; zero/zero chart retains base-field square-root existence |
| Exact unit-pivot membership | `Determinantal.determinant`, `normalized_kernel_iff`, `determinant_iff_kernel`, `xor_schur` | any field; fixed unit pivot gives complete normalization |
| Expanded actual top obstruction | `DeterminantalTop.quadratic_ne_zero`, `linear_top`, `top_ideal_in_kernel`, `ordinate_power_not_mem`, `no_finite_vanishing` |13 variables; ten nonzero quadratic forms plus arbitrary affine lowers and four nonzero linear kernel rows; all fields and parameters |
| Ambient monomial envelopes | `FourTrackEnvelope.linear_control`, `xor_recoding`, `one_hot`, `dense_control` | exact four Nat.choose identities; no solver assertion |

The determinant's generic arbitrary affine lower terms include each exported
row exactly, with quadratic coefficients a=4e,b=4f. Thus its canonical top
obstruction does not rely on a hand-selected subset of equations.

The specialized one-hot group join is checked natively and follows
compositionally from its exact offset image and the previous generic selector
semantics; this project does not package that composition as a separate
Mathlib point-equivalence theorem. Its canonical degree theorem is complete.
Symmetric and determinant scalar identities compose with the prior group-law
bridge; their native fixtures independently replay the concrete group pairs.
The identity target remains the old separate inverse-pair chart, not a finite
secant system silently applied at infinity.

No affine F4 degree, saturated complete symmetric degree, asymptotic exponent,
large-prime-field result, measured natural yield, rank or runtime theorem is
proved here. No novelty claim is made. The next proof obligation after matched
solver measurements is whichever actual representation and parameter growth
those measurements justify, with all variables and charts retained.
