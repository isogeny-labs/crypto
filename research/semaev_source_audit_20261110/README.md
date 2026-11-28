# The printed binary S4 display needs an addition sign

Lean disproves the literal product parsing of the binary S4 display in
Remark 1, page 5 of the recovered six-page Semaev manuscript (5 February
2004). It also proves the additive repair equals the actual degree-two
Sylvester resultant in every field of characteristic two, for every curve
parameter a and every coordinate tuple. The underlying recursive
construction and S3 formula are not refuted. [pcc:unverified]

Write s1=x+y+z+w, s3=xyz+xyw+xzw+yzw and s4=xyzw. The two continuation
lines in the source lack an addition sign. The repaired expression is:

```text
s1^4 + s3^4 + s4*(s3+s1)^2 + s4^2*s1^2 + s3^2
```

The literal expression multiplies the third and fourth terms instead.
Lean distinguishes these two functions explicitly; no punctuation is
silently repaired before testing it. [pcc:unverified]

## An actual F4 point-sum counterexample

Use F4=F2[alpha]/(alpha²+alpha+1), the actual Mathlib AdjoinRoot field with
proved cardinality four. On the nonsingular curve E: y²+xy=x³+1, define
the following actual points. [pcc:unverified]

| Actual point | Coordinates | Checked relation |
|:--|:--|:--|
| P | (1,0) | P+P=T |
| T | (0,1) | T+T=O |
| R | (alpha,0) | R+S=T |
| S | (alpha²,alpha²) | P+P+R+S=O |

Their x-coordinate tuple is (1,1,alpha,alpha²). All three symmetric quantities
s1,s3,s4 equal one. The actual resultant and additive repair are zero,
while the literal printed product is one. Lean directly negates both the
universal formula equality and the implication from an actual four-point
zero sum to literal-expression vanishing. These are actual points and
actual addition formulas, not a lookup table or abstract group substitute.
[pcc:unverified]

The two expressions agree on every one of the 16 tuples over F2; this
closed control is also kernel checked. Testing only the prime field would
miss the omission. The repaired polynomial remains independent of a, as
the universal actual-resultant identity proves. [pcc:unverified]

## Proof map

| Namespace / declaration | Exact checked obligation |
|:--|:--|
| `SemaevBinaryFourthAudit.binary_resultant_coefficients` | Actual curve invariants reduce the actual resultant to six binary coefficients |
| `SemaevBinaryFourthAudit.actual_resultant_corrected` | Universal equality with the additive repair, all characteristic-two fields / a / tuples |
| `SemaevBinaryFourthControlAudit.alpha_square`, `alpha_cube` | Actual AdjoinRoot-field relations |
| Control: `tuple_sigma1`, `tuple_sigma3`, `tuple_sigma4` | Fixed tuple has all symmetric quantities equal to one |
| Control: `actual_resultant_zero`, `literal_value_one` | Actual zero versus literal one |
| `SemaevBinaryFourthPointsAudit.curve_nonsingular`, `mkPoint` | Actual nonsingular curve and point-equation bridge |
| Points: `P_double`, `R_add_S`, `T_double`, `actual_point_witness` | Actual additions and actual four-point zero sum |
| `SemaevBinaryFourthCertificateAudit.all_F2_tuples_agree` | All 16 prime-field tuples agree |
| Certificate: `literal_printed_recursion_claim_false` | Direct negation of universal literal-resultant equality over F4 |
| Certificate: `literal_printed_vanishing_claim_false` | Direct negation of literal vanishing on actual F4 zero sums |
| Certificate: `binary_fourth_source_audit_certificate` | Generic repaired identity, actual field/curve/witness, prime-field control and both negations |

## Source review and remaining obligations

The full [author-uploaded manuscript](https://www.researchgate.net/profile/Igor-Semaev-3/publication/2869464_Summation_Polynomials_and_the_Discrete_Logarithm_Problem_on_Elliptic_Curves/links/5440f69f0cf228087b6a11ba/Summation-Polynomials-and-the-Discrete-Logarithm-Problem-on-Elliptic-Curves.pdf)
was downloaded normally: 131064 bytes, six pages, SHA-256
`1cab0ae675996cdba6393b1e369a22113fd2e74bcf1223595743dc140631c7d6`.
All six pages, including the proof, remarks and references, were read;
pages 2 and 5 were visually inspected. This is an additive read receipt
for S-SEMAEV-2004. The official IACR PDF access failure remains historical;
it no longer means that the original manuscript has not been read.
The source version is pinned by hash, without asserting that every other
version reproduces this typography. [pcc:unverified]

The original theorem uses algebraic-closure points. Section 3 explicitly
allows quadratic-extension y-coordinates. Therefore the earlier rational-only
converse counterexample concerns an extra restriction, not the source theorem.
Existing all-arity geometric equivalence and individual-degree certificates
remain unchanged. Full all-arity polynomial permutation invariance,
absolute irreducibility and arbitrary-split normalization remain open.
No generic bounded-root solver, relation independence or asymptotic attack
cost is established here. [pcc:unverified]

Relevant repository and Mathlib actual-resultant, general S3/S4, binary-model,
AdjoinRoot and actual affine-point declarations were read before construction.
KB MCP tools were unavailable and the repository wrapper returned 127;
bounded primary/repository/Mathlib and formula/errata searches were used.
No accepted source correction was found in those searches; no novelty or
global absence claim follows. No external author was contacted.
[pcc:unverified]

The existing scalar resultant supplement
[IDC1hbf1f44625ee306dc](resultant-identity-certificate.json) is copied byte for
byte from the prior audit. It supplements the six-coefficient determinant
identity only. Its fixed 61-bit-prime evaluations do not prove the
characteristic-two repair or any actual curve-point statement; those are
proved by the new Lean modules. No new native experiment or sampling run
was performed. [pcc:unverified]

Lean 4.35.0 / Mathlib 56a005855ea17e0b0958f73ec801ae140b0a1679 are pinned.
Only propext, Classical.choice and Quot.sound are admitted. No holes, custom
axioms, native_decide, linter waiver or added elaboration setting is used.
Default recursion and arithmetic-normalization failures are retained,
with the fixed tuple and target hypotheses unchanged. Zero paid compute.
[pcc:unverified]

```sh
cd research/semaev_source_audit_20261110/lean
lake build
cd ../../..
tools/lean/proof-gate.sh research/semaev_source_audit_20261110/lean
```

Source commit `44b32e67` precedes `PCC-20261110-95607f`. Full local
verification passes at 11:51:59Z; signed standing remains unverified. Whole
gate: 28 declarations, zero failures and zero trusted-native exceptions.
All source/artifact pins match; 31 attempt files and five gate files have
byte-equal archive roundtrip checks. See [sources.json](sources.json),
[verification.json](verification.json) and [coverage.json](coverage.json).
[pcc:unverified]

The [two-page report](reports/proof-20261110/report.pdf) includes the
[editable diagram](reports/proof-20261110/proof.dot). Canonical performance
graphs keep their existing scope; no timing, speed, rank, cost or isogeny
edge changes follow from this formula correction. [pcc:unverified]
