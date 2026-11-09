# General Weierstrass S3, with actual small-characteristic controls

Lean proves the S3 zero-sum equivalence for the general model
y²+a1xy+a3y=x³+a2x²+a4x+a6 over an algebraically closed field of any
characteristic, provided the actual discriminant is nonzero. The three
prescribed x-coordinates belong to actual affine points of Mathlib's
elliptic group. There is no distinct-coordinate or normal-form assumption.
This extends the prior short and ordinary binary theorems. [pcc:unverified]

Write s1=x+y+z, s2=xy+xz+yz and s3=xyz. The checked formula is
S3=s2²−(4s1+b2)s3−b4s2−b6s1−b8, using Mathlib's actual Weierstrass
invariants. The source quadratic-in-z expression and this symmetric form
are proved equal. The proof eliminates a chord intercept through the curve
equations, factors the quadratic through the actual signed sums, and uses
the general tangent numerator A=3x²+2a2x+a4−a1y and denominator
D=2y+a1x+a3. When D=0, nonsingularity forces A≠0 and S3 has no finite
third-coordinate root. No division by 2 or 3enters. [pcc:unverified]

Key declarations in [GeneralS3.lean](lean/GeneralS3.lean):

- `generalS3_vieta`, `line_elimination`, `generalS3_factor`: checked
  coefficient/Vieta identities and the actual signed-root factorization.
- `repeated_mathlib_iff`, `generalS3_pointX_iff`: actual tangent/vertical
  and fixed-lift root criteria, over arbitrary fields.
- `generalS3_zero_iff_exists_points`: full algebraic-closure equivalence
  for a nonsingular general Weierstrass model.
- `generalS3_short`, `generalS3_map`: the old short expression and actual
  coefficient-field transport are checked specializations/identities.

The [controls](lean/GeneralS3Controls.lean) prove the ordinary binary
specialization and two faithful geometric false-zero examples:
[pcc:unverified]

| Smooth actual model | Triple | Misapplied expression | Correct conclusion |
|---|---|---|---|
| F2: y²+y=x³ | (1, 0, 0) | Ordinary binary formula is 0 | General S3≠0; no corresponding geometric zero-sum triple |
| F3: y²=x³+x²+1 | (0, 0, 0) | Short formula is 0 | General S3≠0; no corresponding geometric zero-sum triple |

These are model-domain counterexamples, not refutations of either formula
on its stated normal form. The nonexistence statements concern actual
points over the algebraic closure, so the failure is not merely absence
of rational y-coordinate lifts. [pcc:unverified]

Two additional actual-point families advance the geometric p-torsion side
of item 3. In characteristic 2, a nonsingular general model with a1=0 has no
nonzero point killed by 2. In characteristic 3, a nonsingular short model
has no nonzero point killed by 3; the S3 diagonal reduces to a4²≠0.
The fields may themselves be algebraically closed. No equivalence to the
source's trace definition, general Hasse bound, embedding-degree bound or
full classification is assumed or thereby discharged. [pcc:unverified]

The whole census project axiom gate passes167 declarations, 0 failing and
0 trusted-native exceptions, including generated declarations. The earlier
134receipt remains a historical checkpoint. No custom axiom, proof hole
or native_decide is accepted. Packets PCC-20261109-4d5cd7and
PCC-20261109-b32b34 bind the universal equivalence and two model-domain
counterexamples, respectively; they remain unsigned. [pcc:unverified]

Both full local packet checks pass. The exact source commit, gate digests,
packet receipts and native supplementary check are bound in
[general-s3-verification.json](general-s3-verification.json).
[pcc:unverified]

The required polynomial-rewrite record is
`IDC1h392e1740fcb7cf05`, in
[general-s3-identity-certificate.json](general-s3-identity-certificate.json).
The unchanged repository issuer/checker accepts 32ID-derived points and 11
extra points over 2^61−1. The actual b-invariant AST has8 variables and
syntactic degree bound 5. Its proof-file digest matches GeneralS3.lean.
This one-host verification supplement does not establish the all-characteristic
or geometric conclusion; those are the Lean theorem's scope. [pcc:unverified]

Primary sources and exact read scopes are in
[general-s3-sources.json](general-s3-sources.json). [Kosters–Yeo 2015](https://arxiv.org/pdf/1503.08001), §2/p3
supplies the general invariant formula; its PDF page was visually checked.
[Semaev 2015](https://arxiv.org/pdf/1504.01175), §2supplies the ordinary binary/short normal-form assumptions.
The original Semaev 2004PDF still returns 403 and is explicitly unread.
Related source review does not silently repair that gap. [pcc:unverified]

No new native curve population, benchmark, statistical comparison or paid
compute was run. Native identity verification source was committed before
issuing its certificate; all frozen earlier native sources/protocols/data
are unchanged. The new result is universal S3 work, not a general resultant
recursion/irreducibility/degree proof, full 14-item completion or independent
second-host validation. [pcc:unverified]

See [gate](evidence/general-s3-gate.log),
[proof diagram/PDF](reports/proof-20261109-general-s3/report.pdf), and the
[all-fourteen coverage map](README.md#original-fourteen-requirements).
Reproduce with `tools/lean/proof-gate.sh research/theorem_census_20261109/lean`;
the identity supplement is regenerated from the study's identity directory
using `cargo run --release --locked --bin general_s3`, preserving the original
record and choosing a new output path. [pcc:unverified]
