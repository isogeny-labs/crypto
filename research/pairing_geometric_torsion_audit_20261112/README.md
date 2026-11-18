# Actual geometric 3-torsion and Frobenius for the BK test curve

For E/F5:y²=x³+1, the new Lean proof shows that every geometric3-torsion point descends from the actual field F25 used in the earlier Miller study. Over every algebraically closed characteristic5 extension K, the actual subgroup E(K)[3] has cardinality9 and an actual additive basis indexed by (Z/3)². Frobenius acts on that basis by (a,b)↦(a,−b), and the actual F5-rational image is precisely the line b=0. [pcc:unverified]

The proof also works for any field K equipped with an F5-algebra embedding of F25. The actual point map restricts to an additive equivalence between E(F25)[3] and E(K)[3]. For algebraically closed K, Mathlib's IsAlgClosed.lift supplies the embedding; neither a torsion-rank premise nor an assumed basis is used. [pcc:unverified]

The coordinate proof is explicit. The earlier actual group classification gives x=0 or x³=1. The checked primitive cube root ω=2+3β in F25, β²=2, factors x³−1 into its three linear factors over every embedding. If x=0 then y=±1; otherwise y=±β. All coordinates therefore descend. Nonsingularity reflects through the field embedding; injectivity of actual point base change reflects3U=0. This proves surjectivity on actual torsion before transferring the previously checked finite basis and cardinality. Frobenius commutes with actual point base change by preservation of powers. [pcc:unverified]

| Module / declaration | Exact scope |
|:--|:--|
| CoordinateDescent.cubic_factorization | Universal linear factorization after any actual F25 embedding |
| CoordinateDescent.cubic_coordinate_descends / y_coordinate_descends | Both coordinates descend from actual F25 roots |
| GeometricTorsion.actual_torsion_descends | Every actual3-torsion point lifts from an actual F25 torsion point |
| GeometricTorsion.torsionMap_bijective / torsionEquivalence | Actual point base change restricts to an additive equivalence |
| GeometricTorsion.all_extension_torsion_cardinality / actual_extension_torsion_basis | Cardinality9 and actual basis, assuming a field embedding only |
| AlgebraicClosure.actual_geometric_torsion_certificate | Every algebraically closed characteristic5 extension, with embedding constructed |
| GeometricFrobenius.actual_geometric_torsion_and_frobenius_certificate | Combined actual cardinality, basis, descent, Frobenius and rational-image result |

Namespace prefixes are PairingGeometricCoordinateAudit, PairingGeometricTorsionAudit, PairingGeometricClosureAudit and PairingGeometricFrobeniusAudit respectively. [pcc:unverified]

This closes the geometric coverage gap for this concrete curve and r=3. It does not prove the generic all-curve BK rank/determinant theorem, the requested full p<500 census, a canonical principal-divisor identification of the earlier Miller expressions, or generic MOV/Frey–Rück reductions. The historical Miller packet is unchanged and its earlier scope remains accurate at issue time. [pcc:unverified]

Sources: frozen actual BK coordinate classification and finite Miller torsion/basis/Frobenius proofs; Galbraith chapter26 selected pairing/torsion domains as recorded in the prior source review; pinned Mathlib actual affine point base-change injectivity/nonsingularity reflection and IsAlgClosed.lift. KB unavailable (MCP absent/wrapper127), bounded repository/primary/Mathlib fallback. No novelty or new source-theorem refutation claimed. [pcc:unverified]

Lean4.35.0 and Mathlib56a00585 are pinned. All modules and the full18-declaration gate pass with zero failures and zero trusted-native exceptions. Only propext, Classical.choice and Quot.sound. Exact statement/dependencies are in sources.json; packet replay will be recorded in verification.json. No holes, custom axioms, native_decide, new elaboration options or trusted-native exceptions. Failed source versions and all compile logs/exits are retained. No native experiment, timing, paid compute or independent-host claim. [pcc:unverified]

```sh
cd research/pairing_geometric_torsion_audit_20261112/lean
lake build
cd ../../..
tools/lean/proof-gate.sh research/pairing_geometric_torsion_audit_20261112/lean
```

Producer source d6d36fcc precedes PCC-20261112-803949. Actual full local replay passes at22:07:43Z on2026-11-12. All33 formal/config dependency pins and all artifact hashes match;18 declarations, zero failures and zero trusted-native exceptions. All29 attempt/source files and five final gate files round-trip byte-for-byte; both earlier gates are retained. Signed standing remains unverified. See verification.json and the inspected two-page report. [pcc:unverified]
