---
geometry: margin=17mm
fontsize: 9pt
mainfont: DejaVu Serif
monofont: DejaVu Sans Mono
---

# Binary S3: checked equivalence and a failed model substitution

Lean proves the ordinary characteristic-two formula
S3(b,x,y,z)=(xy+xz+yz)²+xyz+b for y²+xy=x³+a*x²+b. Over an algebraically
closed field, on a nonsingular model, vanishing is equivalent to the
existence of three actual affine elliptic points with those coordinates
and zero sum. No distinct-coordinate assumption remains.
[pcc:unverified]

![Proof dependencies use actual Mathlib elliptic points, including tangent and infinity cases. The earlier native F2/F4 census is not a premise of this universal theorem.](proof.pdf){width=90%}

| Case | Checked algebra and group-law behavior |
|---|---|
| Distinct x | Chord-intercept elimination, two signed-sum roots, quadratic factorization |
| Repeated nonzero x | Actual tangent; S3=x²(x(2P)−z) |
| Repeated zero x | Sum at infinity; S3=b≠0, so no finite root |
| Arbitrary coordinate lifts | Actual degree-two polynomial roots over algebraic closure; Δ=b |

The negative control uses the actual nonsingular F2curve y²+xy=x³+1.
Lean constructs P,Q,R with x-coordinates1,1,0 and P+Q+R=0. Binary S3 is0;
the short-Weierstrass expression is1. This disproves substituting the short
expression on a binary model. Neither qualified formula is refuted.
[pcc:unverified]

The final whole-project axiom gate passes 116 declarations, with 0 failing
and 0 trusted-native exceptions; its receipt is retained beside all failed
development attempts. PCC-20261109-517cb7 remains unsigned. No custom axiom, proof hole or trusted native
computation is accepted. Main declarations:
BinaryS3Audit.binaryS3_zero_iff_exists_points and
BinaryS3Audit.short_formula_binary_counterexample. Actual source:
../../lean/BinaryS3.lean and ../../lean/BinaryMisapplication.lean.
[pcc:unverified]

This is further item1progress: universal short and ordinary binary S3
equivalences are now formalized. The general resultant recursion and
degree2^(m−2) proof remain open. The original Semaev PDF remains an explicitly
unread source; this proof independently checks its imported formula.
No new native population, clock metric or paid compute. Existing frozen
native sources/data/protocols are unchanged. [pcc:unverified]
