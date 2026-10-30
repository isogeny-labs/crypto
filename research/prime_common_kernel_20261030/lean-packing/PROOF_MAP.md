# Complete theorem map

Namespace `CodedPacking`; coefficientK is a field, C,V are k-vector spaces,
and I is a finite label type. Ei:C->V, Fj:V->C are linear maps. `assemble`
is an actual linear map from the finite product I->C into V, formed by
adding Ei composed with each coordinate projection.

| Declaration | Statement and hypotheses |
| --- | --- |
| `assemble_apply` | The constructed linear map evaluates to sum_i Ei(xi) |
| `recover_component` | If FjEi=0 for everyi!=j, applying Fj to the sum returns FjEj(xj) |
| `assemble_injective` | If additionally every FjEj is injective, the assembly map is injective |
| `packing_dimension_bound` | Under the same hypotheses and finite-dimensional C,V, cardI*finrankC<=finrankV |

The central abstract inequality is fully formalized. The theorem does not
assume a syndrome space, positivity or orthogonality; only the explicitly
stated crosszero and injective-diagonal linear identities. It also does
not assume its desired final dimension bound.

The projected-operator specialization P Fj Ei, coefficient-ring/finite
precision analogues, logical-position cardinalities and asymptotic
mode-count substitution are not separate Lean declarations here. The
readable addendum supplies the exact specialization to a *fixed* C and V.
Approximate/modulo identities cannot silently instantiate a field-level
theorem. No algorithmic lower bound or new ECDLP algorithm is formalized.
