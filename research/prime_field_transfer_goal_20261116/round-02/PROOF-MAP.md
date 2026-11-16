# Round-two instance gate and formalization boundary

1. Structure above: Mathlib WeierstrassCurve over the integers, its existing
   normalized division polynomial preΨ', and finite quotient rings ZMod n.
2. Theorems used as stated: WeierstrassCurve.natDegree_preΨ' and
   CharP.dvd_of_ringHom. No new general degree or characteristic law.
3. Objects as instances: the exact round-one global models
   (1048577,0,0,15728640,6815744) and
   (309485009821345068724781057,0,0,2824050714619773752113627136,
   4052319347348236993615101952), over Z; n=19 and83. The finite
   source quotients are ZMod(17^64) and ZMod(257^64), target ZMod2.
4. Assembly probe: instantiate the existing degree formula at19, with an
   arbitrary WeierstrassCurve Z; infer 2 divides17^64 from an alleged
   quotient-ring hom. Type-check before concrete endpoint declarations.
5. Missing components: literal numeric simplification only for these
   endpoints. A complete formalization of the native Newton lift would
   additionally need a faithful connection between division-polynomial
   roots, r-torsion, and uniqueness in the kernel of good reduction.
   Mathlib has Hensel's lemma and division polynomials; the searched
   elliptic-curve/p-adic modules do not supply that assembled interface.
   No assumption of that missing bridge is counted as a proved map.
6. Inheritance below. No new law, resource-limit increase, custom axiom,
   native_decide or opened composite is needed.

| Operation | Owner / inherited line |
| --- | --- |
| Model coefficients | Mathlib WeierstrassCurve literal constructor |
| Normalized division polynomial | Mathlib WeierstrassCurve.preΨ' |
| Exact degree | natDegree_preΨ', applied as stated |
| Integer coefficient nonvanishing | kernel-checked norm_num |
| Finite quotient-ring characteristic | existing ZMod CharP instance |
| Homomorphism implies characteristic divisibility | CharP.dvd_of_ringHom |
| Impossible divisibility | kernel-checked norm_num |

| Statement | Kind / owner | Scope |
| --- | --- | --- |
| model19_degree | endpoint, LocalTorsionBoundary | degree180 for the actual model |
| model83_degree | endpoint, LocalTorsionBoundary | degree3444 for the actual model |
| no_binary_residue17 | endpoint, LocalTorsionBoundary | no unital ring hom from ZMod(17^64) to ZMod2 |
| no_binary_residue257 | endpoint, LocalTorsionBoundary | no unital ring hom from ZMod(257^64) to ZMod2 |
| Native local lift preserves the scalar | not formalized here | finite development checks and Silverman's classical written argument |
| Cross-prime point/label transport | open | no evaluator or Lean bridge is supplied |

The ring-hom endpoints reject treating a finite p-adic residue as an element
of the binary field by coefficient reduction. They do not forbid set maps,
abstract group maps, global algebraic lifting, covers or every reduction.
The degree endpoints are polynomial-degree facts, not irreducibility or
computational lower bounds. The native program deliberately evaluates these
polynomials in compressed form.
