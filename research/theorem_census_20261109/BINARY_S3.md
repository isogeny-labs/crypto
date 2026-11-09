# Characteristic-two S3: theorem and domain negative control

Lean proves the ordinary binary formula
S3(b,x,y,z)=(xy+xz+yz)²+xyz+b for the actual model
y²+xy=x³+a*x²+b in characteristic2. On a nonsingular model over an
algebraically closed field, it vanishes exactly when three affine points
with the prescribed x-coordinates sum to zero in Mathlib's actual elliptic
point group. This is a universal theorem on that model, not an extrapolation
from the earlier bounded native census. [pcc:unverified]

The proof has no distinct-coordinate hypothesis. With x≠y it eliminates
the intercept of the chord, derives the two signed-sum roots, and factors
the actual quadratic. With repeated x≠0 it uses the actual tangent and
the curve equation. At x=0 the sum is infinity and S3(b,0,0,z)=b≠0,
so there is no finite third coordinate. The two possible y-lifts are
classified by a polynomial identity. Algebraic closure supplies roots of
the actual quadratic y²+xy−(x³+a*x²+b), and Δ=b is proved for this model.
[pcc:unverified]

Main declarations in [BinaryS3.lean](lean/BinaryS3.lean):

- `binaryS3_pointX_iff`: fixed actual nonsingular lifts, arbitrary field of
  characteristic2; roots are precisely x(P+Q) and x(P−Q), omitting infinity.
- `binaryS3_zero_iff_exists_points`: full algebraic-closure equivalence,
  with only Δ≠0 and the stated model coefficients.
- `discriminant_binary`: actual Weierstrass discriminant equals b.

Skepticism also requires a domain negative control. On the nonsingular
F2model y²+xy=x³+1, Lean constructs actual P,Q,R with x-coordinates1,1,0
and P+Q+R=0. The binary formula is0 while the short-Weierstrass expression
SemaevS3.S3(0,1,1,1,0) is1. Thus reusing the short expression on this binary
model fails even the forward implication. This refutes a model substitution,
not either formula on its stated model. The theorem is
`short_formula_binary_counterexample` in
[BinaryMisapplication.lean](lean/BinaryMisapplication.lean). [pcc:unverified]

All proof development/build failures are retained in evidence/binary-s3-*
and evidence/binary-misapplication-*. The first expanded audit, before the
negative control was added, passes114declarations with0failing and0trusted
native exceptions. The final whole-project receipt passes116declarations,
0failing and0trusted-native exceptions. PCC-20261109-517cb7 binds the full
equivalence and remains unsigned. The receipt accompanies the report.
No custom axioms, explicit proof holes or native_decide are used. The
automatic sorryAx shown in retained failed-build diagnostics is the Lean
elaborator's failed declaration placeholder; the final successful audit
must and does reject any such dependency. [pcc:unverified]

This independently checks the binary formula that the corpus imported from
a different source. It does not claim to have read the still-unavailable
original Semaev PDF. Inputs are the actual Mathlib formulas, polynomial
identities and existing bounded-census specification. No new native
population was executed, and no frozen native source/protocol was altered.
The general resultant recursion and degree2^(m−2) proof remain open.
[pcc:unverified]

See the [proof diagram and PDF](reports/proof-20261109-binary/report.pdf),
[final gate](evidence/binary-s3-complete-gate.log), and the complete
[fourteen-item map](README.md#original-fourteen-requirements).
[pcc:unverified]
