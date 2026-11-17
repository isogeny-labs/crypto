# An actual BK torsion-field criterion for every extension degree

Lean proves the user's torsion-inclusion form of BK for the actual curve
E/F5: y²=x³+1 and r=3, for every extension degree k. All geometric
3-torsion points lie in the actual point image of a chosen finite field
of size 5^k exactly when 3 divides 5^k−1, equivalently when k is even.
For every positive k, a further theorem constructs an algebra embedding
of Mathlib's actual GaloisField 5 k and instantiates the criterion there.
[pcc:unverified]

The source hypotheses are proved: the actual curve is nonsingular, has six
F5 points, r=3 is prime and coprime to 5, r divides the curve order and
r does not divide 5−1. No geometric torsion basis, determinant formula or
Weil-pairing nondegeneracy is assumed in this special-case proof.
[pcc:unverified]

## The actual coordinate argument

1. If an actual nonzero point P=(x,y) satisfies 3P=O, then 2P=−P.
   The actual repeated-S3 equivalence implies S3(0,1,x,x,x)=0.
   Its diagonal expression is −3x⁴−12x. In characteristic five this gives
   x=0 or x³=1. [pcc:unverified]
2. The actual curve equation gives y²=1 or y²=2. These equations imply
   x²⁵=x and y²⁵=y, so the square of the actual 5-Frobenius fixes every
   actual geometric 3-torsion point, including infinity. [pcc:unverified]
3. In the algebraic closure choose beta²=2. The actual point P=(1,beta)
   is nonsingular, has exact order three and satisfies Frobenius(P)=−P≠P.
   It lies outside the actual F5 point image. This is the odd-degree
   negative control. [pcc:unverified]
4. All even iterates fix every 3-torsion point; every odd iterate moves that
   witness. Hence full torsion fixedness is equivalent to even k.
   The actual order of 5 modulo 3 is two, giving 3|(5^k−1) iff even k.
   [pcc:unverified]
5. The previously proved actual finite-extension descent theorem converts
   fixedness into membership in the actual point-map image. The final
   module constructs GaloisField 5 k, proves its cardinality 5^k and chooses
   an actual algebra embedding into the algebraic closure for k>0.
   [pcc:unverified]

The statement quantifies over the actual torsion equation and actual curve
points. It is not an assumed rank-two matrix action or a shadow rationality
predicate. The k=0 Frobenius identity is included in the iterate theorem;
the finite-field existence theorem explicitly requires k>0.
[pcc:unverified]

## Proof map

| Namespace / declaration | Exact checked obligation |
|:--|:--|
| `BKActualThreeAudit.diagonal_of_three` | Actual 3P=O implies diagonal S3 zero |
| Three: `three_coordinate_classification`, `actual_equation` | Actual x=0 or x³=1; actual curve equation |
| Three: `x_pow_twenty_five`, `y_pow_twenty_five`, `frobenius_square_fixed` | Actual coordinate powers and Frobenius-square identity |
| `BKActualWitnessAudit.beta_square`, `beta_fifth` | Actual algebraic-closure root and its Frobenius power |
| Witness: `P_order`, `P_frobenius_neg`, `P_not_fixed`, `P_not_rational` | Actual exact-order-three point and actual nonrational image obstruction |
| `BKActualEmbeddingAudit.all_torsion_fixed_iff_even`, `divisibility_iff_even` | All k actual torsion-fixedness and arithmetic parity criterion |
| Embedding: `actual_extension_torsion_criterion` | Actual point images for any chosen finite extension/cardinality 5^k and compatible embeddings |
| `BKActualGaloisFieldAudit.actual_galois_field_criterion` | Constructed actual field/embedding for every k>0 |
| `BKActualCertificateAudit.bk_actual_case_audit_certificate` | All source conditions, actual witness, all-k criterion, chosen-field and explicit Galois-field instantiations |

## Scope and skeptical controls

The [original BK paper](https://link.springer.com/content/pdf/10.1007/s001459900040.pdf),
Theorem 1, explicitly retains r not dividing q−1. Its proof and remarks on
pages 142–143 were reread; page 142 was visually inspected. The retained
primary PDF SHA-256 is
`ac837b8edc47b346dbb341f787c41de0402c95371a159f9786b747bb2c987499`.
Prior full-paper read scope remains in the parent source receipt. The
source's r²-point-count formulation and the user's geometric-torsion-inclusion
form must be distinguished: this study proves the latter in this actual
case; it does not claim a new proof of the generic rank/cardinality theorem.
[pcc:unverified]

The earlier actual F7 curve with nine rational points and nonrational
3-torsion remains the excluded-boundary counterexample: 3 divides q−1,
and even 3² dividing the curve order does not repair removal of that
hypothesis. That disproof and this positive qualified case are compatible.
The original qualified BK theorem is not refuted. The requested complete
p<500 census, generic E[r] rank two, Frobenius determinant and Weil necessity
remain open. No claim that the actual geometric torsion cardinality nine
has been separately proved here. [pcc:unverified]

All new theorems use only propext, Classical.choice and Quot.sound. Lean
4.35.0 and Mathlib 56a005855ea17e0b0958f73ec801ae140b0a1679 are pinned.
No holes, custom axioms, native_decide, linter waivers or added elaboration
settings. Failed inference, rewrite and library-name attempts are retained;
issued parent sources remain unchanged. No native experiment, timing or
paid compute. KB and Conductor were unavailable; primary/repository/Mathlib
reads supplied the bounded fallback. No novelty claim. [pcc:unverified]

The byte-identical existing scalar S3 supplement
[IDC1h392e1740fcb7cf05](s3-identity-certificate.json) covers its prior polynomial
identity only. It does not prove torsion, Frobenius or finite-field descent;
those conclusions rest on the new Lean proofs. No fresh sampling was run.
[pcc:unverified]

```sh
cd research/bk_actual_case_audit_20261110/lean
lake build
cd ../../..
tools/lean/proof-gate.sh research/bk_actual_case_audit_20261110/lean
```

Source commit `11189a29` precedes `PCC-20261110-99983d`. Full local
verification passes at 12:23:10Z; signed standing remains unverified. Final
whole gate: 37 declarations, zero failures and zero trusted-native exceptions.
The prior 36-declaration receipt is retained. All 29 attempt files and both
gates have byte-equal archive roundtrip checks. See [sources.json](sources.json)
and [verification.json](verification.json). [pcc:unverified]

See the [report](reports/proof-20261110/report.pdf) and
[editable diagram](reports/proof-20261110/proof.dot). Canonical IC performance
graphs retain their existing scope; no timing, speed, rank, cost or isogeny
edge is changed by this actual-case proof. [pcc:unverified]
