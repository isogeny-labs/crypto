# Actual geometric torsion and j=0 in characteristics two and three

Lean proves the geometric p-torsion/j-invariant criterion for every
nonsingular general Weierstrass model in characteristics p=2 and p=3.
Over every algebraically closed field, the actual subgroup killed by p
has cardinality **1 if j=0, otherwise p**. Its actual induced ZModp module
has rank **0 if j=0, otherwise 1**. Equivalently, j=0 iff every actual
point killed by p is the identity. [pcc:unverified]

For a model over any base field F and every embedding into an
algebraically closed extension L, the actual subgroup of
scalar-extended points has the same count determined by the base
model's Mathlib j-invariant. The hypotheses are explicit: fields,
characteristic, algebraic closure and actual `W.IsElliptic`, meaning
unit discriminant, equivalent to nonzero discriminant over a field.
No rank, basis, normal-form, trace or pairing premise is supplied.
[pcc:unverified]

In characteristic three, the literal division polynomial evaluates to
b2*x³+b8. If b2=0, the actual discriminant and invariant relation force
b8≠0, excluding every nonidentity killed-three point. If b2≠0, an
actual cube root and ordinate construct a nonzero nonsingular point
killed by three; the preceding general one-or-three classification
then forces count three. Combine this with Mathlib's existing
j=0 iff b2=0 identity. In characteristic two, reuse the earlier actual
torsion classification and Mathlib's j=0 iff a1=0 identity. Construct
the actual two-torsion module; finite group counts determine both
module ranks. Existing coefficient-level identities and earlier
torsion proofs are credited. [pcc:unverified]

The field condition has a concrete Lean counterexample. Over F3,
**y²=x³+2x²+1** has discriminant1, j=1 and exactly five actual rational
points. Its rational subgroup killed by three has one point, while
its geometric three-torsion has three points. Lean negates both the
corresponding concrete iff and the universal claim obtained by deleting
algebraic closure. This is a hypothesis-deletion control, not a
counterexample to the qualified geometric criterion. [pcc:unverified]

The original corpus's supersingular prime-field clause already requires
p≥5. Characteristic-three examples cannot refute that qualified clause.
The earlier trace-zero arithmetic proof is also credited rather than
repeated. The missing connection to the FST trace-based definition,
endomorphism-ring rank, Hasse's theorem and the general embedding-degree
statement remains explicit. No such equivalence is imported as an axiom.
[pcc:unverified]

The main declaration is
`SmallCharacteristicJAudit.small_characteristic_j_certificate`, in
[SmallCharacteristicCertificate.lean](lean/SmallCharacteristicCertificate.lean).
The [proof map](proof-map.json), [source plan](sources.json),
[source review](evidence/source-review.json) and
[visual report](reports/report.pdf) retain the faithful statements,
controls, failed source snapshots and build logs.
Exact local **PCC-20261114-04e0a2** replay PASS at 2026-11-14T05:33:50Z; signed standing remains unverified. See [verification receipt](verification.json).
[pcc:unverified]

```sh
tools/lean/proof-gate.sh research/small_characteristic_j_audit_20261114/lean
python3 research/small_characteristic_j_audit_20261114/restore_attempts.py /tmp/small-j-builds
```

Primary source: Milne, *Elliptic Curves*,
[author PDF](https://www.jmilne.org/math/Books/EC2.pdf), selected
Theorem3.12/context/Remark3.16, printed pp98–99, and notes on p75.
The remark describes the algebraically closed p-power torsion-rank
distinction; p75 describes the endomorphism-rank definition's origin.
These descriptions are not used as an unproved equivalence. This packet
handles p itself in characteristics two and three, not all p-powers.
PDF pages104 and80 were rendered and inspected; URL/hash retained,
raw copyrighted book kept only in the local checkpoint. No full-book
reading or republishing claim. [pcc:unverified]

Pinned Mathlib invariant/j/point APIs and the preceding actual two-/three-
torsion, normalization and characteristic-three proofs were reviewed and
reused. KB and Conductor unavailable; bounded primary/repository/library
fallback recorded before implementation. Other-prime geometry, canonical
Weil identification, the full all-k p<500 census and remaining fourteen-item
obligations remain open. No novelty, qualified-source refutation, corpus
promotion, native experiment, paid compute, upload, holes, custom axioms,
native evaluator, raised limits, new global elaboration flags or waivers.
[pcc:unverified]
