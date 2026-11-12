# General geometric 3-torsion, with actual curve points

Lean proves exactly nine actual 3-torsion points on every short Weierstrass
curve y²=x³+ax+b over an algebraically closed field with 2≠0, 3≠0 and
4a³+27b²≠0. There is no assumed geometric rank, finite basis, Hasse bound
or pairing nondegeneracy. The complete local proof gate passes 20
declarations, zero failures and zero trusted-native exceptions.
[pcc:unverified]

The statement is
`ThreeTorsionRankAudit.geometric_three_torsion_cardinality` in
[GeometricCardinality.lean](lean/GeometricCardinality.lean).
`geometricTorsion a b` is the subtype of actual Mathlib curve points P
satisfying 3•P=0, including the identity. `shortCurve a b` has the actual
Weierstrass coefficients (0,0,0,a,b). This is not a count of an assumed
abstract (Z/3)² model. [pcc:unverified]

| Module | Checked step |
|:--|:--|
| QuarticRoots | ψ₃=3X⁴+6aX²+12bX−a²; derivative 12(X³+aX+b); no common root under the discriminant condition; separable degree four and four distinct roots |
| ActualTorsion | Actual 3P=0 iff ψ₃(x)=0 for every affine point, via the checked repeated-S₃ group equivalence; two roots above each x and actual nonsingular point construction |
| GeometricCardinality | Explicit inverse coordinate maps and equivalence between actual 3-torsion and identity plus the four two-point fibres; cardinality nine |

The key obstruction is ψ₃(x)=12x(x³+ax+b)−(3x²+a)². If the cubic and
quartic both vanish, a=−3x² and b=2x³, contradicting 4a³+27b²≠0.
Thus derivative roots cannot repeat. The same obstruction ensures the
y-fibres above quartic roots are nonzero, so characteristic other than two
gives two distinct square roots. Both coordinate maps precede the count.
[pcc:unverified]

This extends the earlier concrete F5/r3 geometric count to arbitrary
nonsingular short coefficients and algebraically closed fields in the
specified characteristics. It supplies one geometric component used by
Balasubramanian–Koblitz. It does not prove the all-prime torsion-rank theorem,
the Frobenius determinant identity, the full BK criterion, a p<500 census,
canonical divisor pairings or the remaining original fourteen requirements.
The qualified original BK result is not refuted here. [pcc:unverified]

Source review: reread BK1998 Theorem 1/proof/Remark 1, printed pages142–143,
PDF SHA256 ac837b8edc47b346dbb341f787c41de0402c95371a159f9786b747bb2c987499.
Its matrix proof uses the geometric torsion basis before the determinant
argument. Mathlib and previous actual S3 proofs are pinned. KB unavailable
(MCP absent, wrapper127); bounded primary/repository/Mathlib fallback, with
receipts in evidence/source-review.json. No novelty claim.
[pcc:unverified]

Lean4.35.0 / Mathlib56a005855ea17e0b0958f73ec801ae140b0a1679. Only
propext, Classical.choice and Quot.sound; no holes, native_decide, custom
axioms, linter waivers or new elaboration options. Failed source versions,
all build logs/exits and the final gate are retained losslessly. No new
native experiment or paid compute. Local replay and independent signed
standing are separate. [pcc:unverified]

```sh
cd research/three_torsion_rank_audit_20261112/lean
lake build
```
