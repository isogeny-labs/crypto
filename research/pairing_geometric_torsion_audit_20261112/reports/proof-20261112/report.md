---
geometry: margin=17mm
fontsize: 10pt
mainfont: DejaVu Serif
monofont: DejaVu Sans Mono
---

# Actual geometric torsion for the BK test curve

For E/F5:y²=x³+1 and r=3, Lean proves that every geometric3-torsion
point descends from the actual field F25. Over every algebraically closed
characteristic5 extension K, E(K)[3] has exactly nine points and an
actual additive basis indexed by (Z/3)². Frobenius acts as (a,b)↦(a,−b),
and the actual F5-rational image is exactly b=0. [pcc:unverified]

The finite basis is proved before transport. A general torsion-rank theorem,
an assumed basis, and a source nondegeneracy axiom are not premises.
The earlier issued Miller packet and its historical scope are unchanged.
[pcc:unverified]

![Coordinate descent, actual geometric basis and scope. [pcc:unverified]](proof.pdf){width=80%}

\newpage

# Proof map, hypotheses and assurance

Let β²=2 and ω=2+3β in the actual F25. The frozen actual3-torsion
classification gives x=0 or x³=1. Kernel-checked finite identities give
ω³=1 and ω²+ω+1=0. Over any F5-algebra embedding f:F25→K,
the universal identity
(x−1)(x−f(ω))(x−f(ω)²)=x³−1
therefore forces x to descend. The actual curve equation forces y=±1
when x=0, and y=±f(β) otherwise. Both coordinates descend. [pcc:unverified]

Nonsingularity reflects through f. The actual point map is injective, so
3R=0 reflects to the lifted point. Its restriction to actual3-torsion is
bijective and additive. The previously checked actual F25 basis transfers
through this actual homomorphism, proving cardinality9. For algebraically
closed K, Mathlib constructs f with IsAlgClosed.lift from the finite
algebraic extension; no embedding hypothesis remains. [pcc:unverified]

Frobenius commutes with the point map because f preserves powers.
Transporting the checked F25 action gives (a,b)↦(a,−b) on the geometric
basis. The actual finite-field descent criterion equates rational image
with Frobenius-fixed points, giving precisely b=0. [pcc:unverified]

| Module | Checked structural obligation |
|:--|:--|
| CoordinateDescent | Actual root identities plus universal factorization and both coordinate descents |
| GeometricTorsion | Actual torsion point-map surjectivity, additive equivalence, cardinality and basis |
| AlgebraicClosure | Embedding constructed in every algebraically closed characteristic5 extension |
| GeometricFrobenius | Actual Frobenius commutation, matrix, rational image and combined certificate |

The full project proof/axiom gate and exact statement/source pins are recorded
in sources.json. All failed source versions and build exits are retained in
lossless archives. Lean4.35.0 and Mathlib56a00585 are pinned. Only propext,
Classical.choice and Quot.sound are admitted; no holes, custom axioms,
native_decide, trusted-native exceptions, waived linters or new elaboration
options. Actual local replay and independently signed standing are recorded
separately in verification.json. [pcc:unverified]

Sources: frozen actual BK coordinate and finite Miller arithmetic proofs;
prior selected Galbraith chapter26 domain reading; pinned Mathlib actual point
map/injectivity, nonsingularity reflection and IsAlgClosed.lift. KB unavailable
(MCP absent/wrapper127), bounded repository/primary/Mathlib fallback.
No novelty or new source-refutation claim. [pcc:unverified]

This is one concrete curve and r=3. Generic all-curve BK rank/determinant,
the p<500 census, canonical principal-divisor/valuation pairing identification
and generic MOV/Frey–Rück remain open. No native experiment, benchmark,
timing, independent-host or paid-compute claim. [pcc:unverified]
