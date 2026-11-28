# H116 scalar division-polynomial degree protocol

H116-SCALAR-DIVPOLY-DEGREE-001 is frozen before its producer, independent
verifier, or result exists. It tests the newly formalized scalar-coordinate
degree layer on the same eight extension-field cells used by the paired-factor
experiment.

For each base curve the primary scalar is its base-field point count
N in {7,5,14}; all satisfy p does not divide N. The producer must construct
Phi_N/Psi_N^2 from division-polynomial recurrences, measure exact degrees and
leading coefficients, and compare it with direct group scalar multiplication
on every affine point. The denominator/pole convention is checked separately.

Two controls are locked:

1. add one to the numerator constant and require a regular-point rejection in
   every cell;
2. construct Psi_p^2 in characteristic p and require its degree to be strictly
   below p^2-1, with zero coefficient at that expected degree.

The second control targets the exact hypothesis in Mathlib's natDegree_PsiSq
theorem. No finite success is allowed to stand in for the remaining
scheme/function-field identification or any ECDLP runtime claim.
