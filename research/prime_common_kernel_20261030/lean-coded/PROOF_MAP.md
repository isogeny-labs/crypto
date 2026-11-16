# Exact proof map

Namespace `CodedProjection`. All six named declarations build and pass
axiom audit plus fresh official replay under the pinned dependencies.

| Claim | Declaration | Exact hypotheses |
| --- | --- | --- |
| Entrywise projected translation is the actual product P tau P | `projected_is_matrix_product` | Finite additive commutative groupG, syndrome homomorphismG->H, coefficient ringR, decidable equality |
| Nonzero-syndrome translation is annihilated | `nonzero_syndrome_annihilates` | Same additive groups/ring, syndrome of translation nonzero; finiteG not needed for entrywise statement |
| Zero-syndrome translation preserves code support | `zero_syndrome_preserves` | Syndrome of translation zero |
| Unequal error labels vanish after projection | `mismatched_pair_annihilates` | Injective labelsyndrome map, first error has assigned syndrome, second has its negative, unequal labels |
| Matched error labels preserve code | `matched_pair_preserves` | Same assigned/opposite-syndrome assumptions, equal label |
| Double weighted pair sum keeps exactly equal labels | `weighted_pair_diagonal` | Finite labelset, injective syndrome map, opposite error syndromes; arbitrary coefficient function and matrix entry |

The definitions are actual finite-index diagonal projectors and translation
matrices, not assumed tensor factorizations. In the intended binary
construction, opposite syndromes are equal. The Lean project does not yet
instantiate this with a particular H or prove its rank/cardinality, equate
the weighted matrix sum to an explicitly built Kronecker product, formalize
Lagrange charts/valuations, Smith factors, weighted mode count or recursive
costs. Those are the concrete missing formal bridges. The generic
projection laws are useful structural lemmas, not a formal proof of the
whole common-kernel construction.
