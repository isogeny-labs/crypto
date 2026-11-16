---
geometry: margin=17mm
fontsize: 10pt
mainfont: DejaVu Serif
monofont: DejaVu Sans Mono
---

# An actual BK torsion-field criterion for every degree

For the actual E/F₅: y²=x³+1 and r=3, Lean proves that full geometric
3-torsion descends to the actual point image of a field of size 5ᵏ exactly
when 3 divides 5ᵏ−1, equivalently when k is even. For each positive k,
it constructs an actual embedding of Mathlib's GaloisField 5 k and proves
the criterion for that explicit field. The actual curve has six points;
all source arithmetic hypotheses, including r not dividing q−1, are proved.
No torsion basis, determinant or Weil-pairing theorem is assumed.
[pcc:unverified]

The actual point P=(1,beta), beta²=2, has exact order three and
Frobenius(P)=−P≠P. It supplies an actual nonrational witness for every
odd degree. Every actual 3-torsion point is fixed by Frobenius squared.
[pcc:unverified]

![Actual coordinates, all-degree criterion and explicit finite-field construction. [pcc:unverified]](proof.pdf){width=96%}

\newpage

# Proof obligations, scope and skeptical controls

If 3P=O for an actual nonzero point (x,y), then 2P=−P. The checked actual
repeated-S3 bridge gives diagonal S3 zero. Its expression −3x⁴−12x factors
in characteristic five to x=0 or x³=1. The actual curve equation gives
y²=1 or y²=2. Therefore x²⁵=x and y²⁵=y: actual Frobenius squared fixes
every geometric 3-torsion point, including infinity. [pcc:unverified]

An actual algebraic-closure point (1,beta), beta²=2, has order three by
the same repeated-S3 group bridge. Its fifth power relation beta⁵=−beta
proves it is moved by Frobenius and is outside the actual F₅ point image.
Even iterates fix all torsion; every odd iterate moves this witness.
The order of 5 modulo 3 is two, giving the arithmetic equivalence.
[pcc:unverified]

The existing actual finite-extension theorem identifies Frobenius fixedness
with membership in an actual point-map image. The new construction uses
GaloisField 5 k, its proved cardinality 5ᵏ and IsAlgClosed.lift to an actual
algebra embedding, with scalar-tower compatibility checked. It requires
k>0. The separate iterate theorem includes k=0 as an ordinary identity;
no field with one element is asserted. [pcc:unverified]

| Module / declarations | Exact checked obligation |
|:--|:--|
| `ThreeTorsion`: `diagonal_of_three`, `three_coordinate_classification` | Actual torsion equation implies diagonal S3 and the x classification |
| Three: `actual_equation`, `frobenius_square_fixed` | Actual y equation, coordinate powers and Frobenius-square identity |
| `QuadraticWitness`: `P_order`, `P_frobenius_neg`, `P_not_rational` | Actual exact-order-three point, anti-Frobenius action and nonrational image |
| `EmbeddingCriterion`: `all_torsion_fixed_iff_even`, `divisibility_iff_even` | All-k torsion criterion and exact arithmetic equivalence |
| Embedding: `actual_extension_torsion_criterion` | Actual finite-extension point image, with its stated compatible embeddings |
| `GaloisFieldCase`: `actual_galois_field_criterion` | Explicit actual finite field and algebra embedding for every positive k |
| `BKCertificate`: `bk_actual_case_audit_certificate` | All source hypotheses, witness, all-k and explicit-field conclusions |

The final whole gate checks 37 declarations, zero failing and zero
trusted-native exceptions. The earlier 36-declaration gate is retained.
Lean 4.35.0 / Mathlib 56a00585; only propext, Classical.choice and Quot.sound.
No holes, custom axioms, native_decide, linter waivers or added elaboration
settings. All 29 attempt files and both gate receipts are archived losslessly.
Source commit `11189a29` precedes packet `PCC-20261110-99983d`.
Actual local verdict and signed standing are retained in verification.json;
signed standing remains unverified. [pcc:unverified]

Original BK Theorem 1 conditions/proof/remarks, pages 141–143, were reread;
page 142 was visually inspected. Primary PDF SHA-256:

```text
ac837b8edc47b346dbb341f787c41de0402c95371a159f9786b747bb2c987499
```

The source's r²-point-count statement and the user's geometric-torsion
inclusion form are distinct formal obligations. This study proves the latter
for one actual curve at every degree. Generic rank/cardinality, determinant,
Weil necessity, actual torsion cardinality nine and the requested all-p<500
census remain unclaimed. [pcc:unverified]

The prior actual F₇/#9/nonrational 3-torsion witness remains the negative
control for removing r not dividing q−1. It does not refute the qualified
source theorem. The prior scalar S3 supplement IDC1h392e1740fcb7cf05 is
copied unchanged; its sampled polynomial identity does not prove these new
curve/descent results. No fresh sampling, native experiment, timing or paid
compute. KB/Conductor unavailable; primary/repository/Mathlib fallback.
No novelty claim. [pcc:unverified]

[Original BK paper](https://link.springer.com/content/pdf/10.1007/s001459900040.pdf).
Source pins, actual verdict, proof map and archives: study sources.json,
verification.json and README.md. [pcc:unverified]
