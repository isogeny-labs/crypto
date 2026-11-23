# Scalar-field instance gate

1. Structure: Mathlib Field, Matrix, mulVec, vecMul and dotProduct on finite
   index types; the actual scalar field is ZMod scalarOrder, under the
   explicitly stated Fact scalarOrder.Prime hypothesis.
2. Theorems used as stated: Matrix.dotProduct_mulVec for matrix cancellation;
   CryptoLean.SingleRelation.recover_from_relation for division by the nonzero
   coefficient; CryptoLean.SingleRelation.mixed_label_fiber_card for the exact
   free-tag fiber; AddSubgroup.card_ker_mul_card_of_surjective for the zero-tag
   hyperplane, using the existing dotProductBilin object. The shared libraries
   already own these laws. No new generic
   cancellation or fiber-counting theorem is introduced in this study.
3. Instance: scalarOrder=2^252+27742317777372353535851937790883648493,
   finite row/column types Fin r and Fin b, field-valued checked row equations.
   Primality and the translation from actual curve relations into these field
   equations remain hypotheses/bridges, not hidden axioms or computations.
4. Assembly probe: anonymous generic field example applies recover_from_relation
   verbatim. The actual scalar-field endpoint rewrites the checked matrix/vector
   contracts with dotProduct_mulVec, then applies that same recovery law.
5. Missing: the point-group/log-coordinate bridge, probability distribution
   and stopping/selection instance, and nonzero-kernel existence/backend cost.
   The fiber and annihilator endpoints are not a conditional
   probability theorem. No missing component is assumed as a Lean axiom.
6. Inheritance: field operations and inverses from ZMod's Field instance under
   Fact.Prime; matrix products/dot products from Mathlib; finite-index structure
   from Fin; scalar recovery and label-fiber equivalence from pinned CryptoLean.

| Statement | Kind | Owner | Hypotheses / existing laws |
|---|---|---|---|
| Generic assembly example | Probe | Anonymous only | Existing Field recovery law used verbatim |
| extract_scalar | Endpoint | Study | Checked row and matrix-kernel equations; nonzero tag sum; prime scalar field |
| mixed_label_fiber | Endpoint | Study | Existing generic fiber-cardinality theorem at the actual scalar field |
| annihilator_card | Endpoint | Study | Existing kernel-cardinality law; nonzero scalar-field functional, with surjectivity from dotProduct_single |

Proof chain: checked C*z=a+x*b; dot-product with v; rewrite by
dotProduct_mulVec; rewrite v*C=0; invoke recover_from_relation.
The card endpoint applies mixed_label_fiber_card and reads off the existing
ZMod and Fin cardinalities. No definition of group arithmetic is recreated.
For annihilator_card, a single nonzero coefficient makes the existing
dot-product linear functional surjective by a single-coordinate vector.
Its additive kernel cardinality is read from the existing group theorem.
No new generic probability or counting law is proved in this study.

Shared source pin: aburan28/crypto-lean 5f2ca251cb34806f69fd5455175434cac3567dfa.
Mathlib pin: cfc55fc58714a59dc06e6e79edc6d1d95140d3c2; Lean 4.35.0.
No shared-library source edit is needed. An initially considered generic
cancellation lemma was abandoned after finding the existing SingleRelation laws.
